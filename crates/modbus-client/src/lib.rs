use std::cmp::min;
use std::fmt;
use std::net::SocketAddr;
use std::time::Duration;

use thiserror::Error;
use tokio::net::{lookup_host, TcpStream};
use tokio::sync::Mutex;
use tokio::time::{sleep, timeout};
use tokio_modbus::client::{rtu, tcp, Context, Reader};
use tokio_modbus::slave::{Slave, SlaveContext};
pub use tokio_modbus::ExceptionCode;
use tracing::{debug, info};

/// Protocol limit for a single read holding registers (0x03) request.
pub const MAX_REGISTERS_PER_READ: u16 = 125;

/// How Modbus requests are framed on the TCP connection.
#[cfg_attr(
    feature = "config",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Framing {
    /// Modbus TCP (MBAP header with transaction IDs).
    #[default]
    Tcp,
    /// Modbus RTU frames (unit ID, PDU, CRC) carried over TCP, as transparent
    /// serial-to-Ethernet converters pass them to an RS-485 bus. There are no
    /// transaction IDs, so only one request may be in flight per connection.
    Rtu,
}

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
    /// Modbus TCP, or RTU frames over TCP for serial converters.
    pub framing: Framing,
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
            framing: Framing::Tcp,
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
    #[error("connection closed by the device")]
    Disconnected,
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
                | Self::Disconnected
                | Self::Timeout { .. }
        )
    }

    /// tokio-modbus reports a connection the device closed as `io::Error::last_os_error()`,
    /// which is whatever error number the thread saw last ("Operation now in progress (os
    /// error 36)", "Undefined error: 0"). Errors meaning the connection is gone, and error
    /// numbers no failed socket read returns, become [`ClientError::Disconnected`].
    fn from_transport(err: tokio_modbus::Error) -> Self {
        use std::io::ErrorKind;

        let tokio_modbus::Error::Transport(io) = &err else {
            return Self::Transport(err);
        };
        let closed = matches!(
            io.kind(),
            ErrorKind::UnexpectedEof
                | ErrorKind::BrokenPipe
                | ErrorKind::NotConnected
                | ErrorKind::ConnectionReset
                | ErrorKind::ConnectionAborted
                | ErrorKind::WouldBlock
        );
        let stale = match io.raw_os_error() {
            Some(0) => true,
            #[cfg(unix)]
            Some(code) => code == libc::EINPROGRESS,
            _ => false,
        };
        if closed || stale {
            Self::Disconnected
        } else {
            Self::Transport(err)
        }
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

    /// A client that connects on its first read. Requests from every holder of a
    /// shared client run one at a time, which RTU over TCP needs when several units
    /// sit behind one serial converter.
    pub fn new(config: ClientConfig) -> Self {
        Self {
            config,
            context: Mutex::new(None),
        }
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
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
            // Callers report the final outcome; an exception can also be expected, as when
            // discovery tries a base address the device does not use.
            debug!(unit_id, start, count, error = %error, attempt = attempts + 1, "modbus read failed");

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
                info!(host = %self.config.host, port = self.config.port, "modbus connected");
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
            Ok(Err(err)) => Err(ClientError::from_transport(err)),
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
        let connect = async {
            match config.framing {
                Framing::Tcp => tcp::connect(addr).await,
                Framing::Rtu => {
                    let stream = TcpStream::connect(addr).await?;
                    stream.set_nodelay(true)?;
                    Ok(rtu::attach(stream))
                }
            }
        };
        match timeout(Duration::from_millis(config.timeout_ms), connect).await {
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
