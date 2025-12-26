#![allow(dead_code)]

use std::cmp::min;
use std::net::SocketAddr;
use std::time::Duration;

use thiserror::Error;
use tokio::sync::Mutex;
use tokio::time::{sleep, timeout};
use tokio_modbus::client::tcp;
use tokio_modbus::client::Context;
use tokio_modbus::prelude::{Reader, Slave, SlaveContext};
use tracing::{debug, warn};

/// Configuration options for connecting and polling a Modbus TCP device.
#[cfg_attr(feature = "config", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub host: String,
    pub port: u16,
    /// Maximum number of registers to read in a single request; devices with quirks may require lower batch sizes.
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
    #[error("invalid socket address {0}:{1}")]
    InvalidAddress(String, u16),
    #[error("modbus transport error: {0}")]
    Modbus(std::io::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("request timed out after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
    #[error("register address overflow")]
    AddressOverflow,
}

#[derive(Debug)]
pub struct ModbusClient {
    config: ClientConfig,
    context: Mutex<Context>,
}

impl ModbusClient {
    pub async fn connect(config: ClientConfig) -> Result<Self, ClientError> {
        let addr = format!("{}:{}", config.host, config.port)
            .parse::<SocketAddr>()
            .map_err(|_| ClientError::InvalidAddress(config.host.clone(), config.port))?;
        let context = tcp::connect(addr).await?;
        Ok(Self {
            config,
            context: Mutex::new(context),
        })
    }

    pub async fn read_range(&self, unit_id: u8, start: u16, count: u16) -> Result<Vec<u16>, ClientError> {
        if count == 0 {
            return Ok(Vec::new());
        }

        let mut ctx = self.context.lock().await;
        let batch_size = self
            .config
            .max_batch_size
            .unwrap_or(count)
            .max(1u16);
        let mut remaining = count;
        let mut offset = 0u16;
        let mut out = Vec::with_capacity(count as usize);

        while remaining > 0 {
            let chunk = min(remaining, batch_size);
            let chunk_start = u16::try_from(u32::from(start) + u32::from(offset))
                .map_err(|_| ClientError::AddressOverflow)?;
            let values = self
                .read_chunk(&mut ctx, unit_id, chunk_start, chunk)
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
        ctx: &mut Context,
        unit_id: u8,
        start: u16,
        count: u16,
    ) -> Result<Vec<u16>, ClientError> {
        ctx.set_slave(Slave(unit_id));
        let mut attempts = 0usize;
        let mut last_error = None;

        loop {
            let request = ctx.read_holding_registers(start, count);
            let result = timeout(Duration::from_millis(self.config.timeout_ms), request).await;
            match result {
                Ok(Ok(values)) => {
                    debug!(unit_id, start, count, "modbus read ok");
                    return Ok(values);
                }
                Ok(Err(err)) => {
                    warn!(unit_id, start, count, error = %err, "modbus read error");
                    last_error = Some(ClientError::Modbus(err));
                }
                Err(_) => {
                    warn!(unit_id, start, count, "modbus read timeout");
                    last_error = Some(ClientError::Timeout {
                        timeout_ms: self.config.timeout_ms,
                    });
                }
            }

            if attempts >= self.config.retry_count {
                return Err(last_error.unwrap_or(ClientError::Timeout {
                    timeout_ms: self.config.timeout_ms,
                }));
            }

            let delay_ms = self.retry_delay_ms(attempts);
            attempts += 1;
            sleep(Duration::from_millis(delay_ms)).await;
        }
    }

    fn retry_delay_ms(&self, attempt: usize) -> u64 {
        let base = self.config.retry_backoff_ms.max(1);
        let shift = u32::try_from(attempt).unwrap_or(u32::MAX);
        // saturating_shl is unstable/nightly. Use checked_shl or just shl if u32 is small enough.
        // We clamp shift to 31 anyway in other places, but here let's be safe.
        // If shift >= 64, 1 << shift wraps or panics? u64 args.
        // Let's use checked_shl
        let factor = 1u64.checked_shl(shift).unwrap_or(u64::MAX); 
        let delay = base.saturating_mul(factor);
        let max = self.config.retry_max_backoff_ms.max(base);
        min(delay, max)
    }
}
