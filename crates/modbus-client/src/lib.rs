use std::cmp::min;
use std::fmt;
use std::net::SocketAddr;
use std::time::Duration;

use thiserror::Error;
use tokio::net::lookup_host;
use tokio::sync::Mutex;
use tokio::time::{sleep, timeout};
use tokio_modbus::client::{tcp, Context, Reader};
use tokio_modbus::slave::{Slave, SlaveContext};
pub use tokio_modbus::ExceptionCode;
use tracing::{debug, info, warn};

/// Protocol limit for a single read holding registers (0x03) request.
pub const MAX_REGISTERS_PER_READ: u16 = 125;

/// Configuration options for connecting and polling a Modbus TCP device.
#[cfg_attr(feature = "config", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub host: String,
    pub port: u16,
    /// Maximum number of registers to read in a single request; devices with quirks may require lower batch sizes.
    /// Defaults to, and is capped at, [`MAX_REGISTERS_PER_READ`].
    pub max_batch_size: Option<u16>,
    /// Per-request timeout in milliseconds.
    pub timeout_ms: u64,
    /// Number of retries per request after the initial attempt.
    pub retry_count: usize,
    /// Base delay between retries in milliseconds (exponential backoff).
    pub retry_backoff_ms: u64,
    /// Upper bound for retry backoff delay in milliseconds.
    pub retry_max_backoff_ms: u64,
    /// Optional delay between split reads to placate slower devices.
    pub inter_read_delay_ms: Option<u64>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 502,
            max_batch_size: None,
            timeout_ms: 1_000,
            retry_count: 2,
            retry_backoff_ms: 100,
            retry_max_backoff_ms: 2_000,
            inter_read_delay_ms: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("could not resolve {0}:{1}")]
    InvalidAddress(String, u16),
    #[error("connect to {addr} failed: {source}")]
    Connect {
        addr: SocketAddr,
        #[source]
        source: std::io::Error,
    },
    #[error("modbus transport error: {0}")]
    Transport(#[from] tokio_modbus::Error),
    #[error("device returned modbus exception: {0}")]
    Exception(ExceptionCode),
    #[error("request timed out after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
    #[error("register address overflow")]
    AddressOverflow,
}

impl ClientError {
    /// Whether the connection is unusable after this error and must be re-established.
    /// A Modbus exception is a valid response, so the connection stays up.
    pub fn is_connection_error(&self) -> bool {
        matches!(
            self,
            Self::InvalidAddress(..)
                | Self::Connect { .. }
                | Self::Transport(_)
                | Self::Timeout { .. }
        )
    }

    fn is_retryable(&self) -> bool {
        match self {
            Self::Exception(code) => *code == ExceptionCode::ServerDeviceBusy,
            Self::AddressOverflow => false,
            _ => true,
        }
    }
}

/// A Modbus TCP client that reconnects on its own after connection errors.
pub struct ModbusClient {
    config: ClientConfig,
    context: Mutex<Option<Context>>,
}

impl fmt::Debug for ModbusClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModbusClient")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

impl ModbusClient {
    /// Connects to the device. Later connection errors are recovered by reconnecting
    /// on the next read.
    pub async fn connect(config: ClientConfig) -> Result<Self, ClientError> {
        let context = open(&config).await?;
        Ok(Self {
            config,
            context: Mutex::new(Some(context)),
        })
    }

    pub async fn read_range(
        &self,
        unit_id: u8,
        start: u16,
        count: u16,
    ) -> Result<Vec<u16>, ClientError> {
        if count == 0 {
            return Ok(Vec::new());
        }

        let mut slot = self.context.lock().await;
        let batch_size = self
            .config
            .max_batch_size
            .unwrap_or(MAX_REGISTERS_PER_READ)
            .clamp(1, MAX_REGISTERS_PER_READ);
        let mut remaining = count;
        let mut offset = 0u16;
        let mut out = Vec::with_capacity(count as usize);

        while remaining > 0 {
            let chunk = min(remaining, batch_size);
            let chunk_start = u16::try_from(u32::from(start) + u32::from(offset))
                .map_err(|_| ClientError::AddressOverflow)?;
            let values = self
                .read_chunk(&mut slot, unit_id, chunk_start, chunk)
                .await?;
            out.extend(values);
            remaining -= chunk;
            offset += chunk;

            if remaining > 0 {
                if let Some(delay_ms) = self.config.inter_read_delay_ms {
                    sleep(Duration::from_millis(delay_ms)).await;
                }
            }
        }

        Ok(out)
    }

    async fn read_chunk(
        &self,
        slot: &mut Option<Context>,
        unit_id: u8,
        start: u16,
        count: u16,
    ) -> Result<Vec<u16>, ClientError> {
        let mut attempts = 0usize;

        loop {
            let error = match self.try_read(slot, unit_id, start, count).await {
                Ok(values) => {
                    debug!(unit_id, start, count, "modbus read ok");
                    return Ok(values);
                }
                Err(error) => error,
            };
            warn!(unit_id, start, count, error = %error, "modbus read failed");

            if error.is_connection_error() {
                // A timed-out or broken connection can deliver stale responses; start over.
                *slot = None;
            }
            if !error.is_retryable() || attempts >= self.config.retry_count {
                return Err(error);
            }

            let delay_ms = self.retry_delay_ms(attempts);
            attempts += 1;
            sleep(Duration::from_millis(delay_ms)).await;
        }
    }

    async fn try_read(
        &self,
        slot: &mut Option<Context>,
        unit_id: u8,
        start: u16,
        count: u16,
    ) -> Result<Vec<u16>, ClientError> {
        let ctx = match slot {
            Some(ctx) => ctx,
            None => {
                let ctx = open(&self.config).await?;
                info!(host = %self.config.host, port = self.config.port, "modbus reconnected");
                slot.insert(ctx)
            }
        };
        ctx.set_slave(Slave(unit_id));
        match timeout(
            Duration::from_millis(self.config.timeout_ms),
            ctx.read_holding_registers(start, count),
        )
        .await
        {
            Ok(Ok(Ok(values))) => Ok(values),
            Ok(Ok(Err(code))) => Err(ClientError::Exception(code)),
            Ok(Err(err)) => Err(ClientError::Transport(err)),
            Err(_) => Err(ClientError::Timeout {
                timeout_ms: self.config.timeout_ms,
            }),
        }
    }

    fn retry_delay_ms(&self, attempt: usize) -> u64 {
        let base = self.config.retry_backoff_ms.max(1);
        let shift = u32::try_from(attempt).unwrap_or(63).min(63);
        let factor = 1u64 << shift;
        let delay = base.saturating_mul(factor);
        let max = self.config.retry_max_backoff_ms.max(base);
        min(delay, max)
    }
}

/// Resolves the host and tries each address in turn (e.g. `localhost` may resolve to
/// `::1` before `127.0.0.1`).
async fn open(config: &ClientConfig) -> Result<Context, ClientError> {
    let invalid = || ClientError::InvalidAddress(config.host.clone(), config.port);
    let addrs = lookup_host((config.host.as_str(), config.port))
        .await
        .map_err(|_| invalid())?;
    let mut last_error = None;
    for addr in addrs {
        match timeout(Duration::from_millis(config.timeout_ms), tcp::connect(addr)).await {
            Ok(Ok(context)) => return Ok(context),
            Ok(Err(source)) => last_error = Some(ClientError::Connect { addr, source }),
            Err(_) => {
                last_error = Some(ClientError::Timeout {
                    timeout_ms: config.timeout_ms,
                })
            }
        }
    }
    Err(last_error.unwrap_or_else(invalid))
}
