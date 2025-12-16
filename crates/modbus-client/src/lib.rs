#![allow(dead_code)]

use thiserror::Error;

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
            inter_read_delay_ms: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("connection logic not implemented yet")]
    NotImplemented,
}

#[derive(Debug)]
pub struct ModbusClient {
    config: ClientConfig,
}

impl ModbusClient {
    pub async fn connect(config: ClientConfig) -> Result<Self, ClientError> {
        // TODO: wire up tokio-modbus client with timeouts and retries.
        Ok(Self { config })
    }

    pub async fn read_range(&self, _unit_id: u8, _start: u16, _count: u16) -> Result<Vec<u16>, ClientError> {
        // TODO: implement batched reads with max_batch_size handling.
        Err(ClientError::NotImplemented)
    }
}
