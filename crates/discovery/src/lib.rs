#![allow(dead_code)]

use thiserror::Error;
use tokio::time::{sleep, Duration};
use tracing::info;

use types::DeviceIdentity;

#[cfg_attr(feature = "config", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    pub subnet: String,
    pub port: u16,
    pub max_concurrency: usize,
    pub per_host_timeout_ms: u64,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            subnet: "192.168.1.0/24".to_string(),
            port: 502,
            max_concurrency: 64,
            per_host_timeout_ms: 200,
        }
    }
}

#[derive(Debug, Error)]
pub enum DiscoveryError {
    #[error("discovery not implemented")]
    NotImplemented,
}

pub async fn discover_subnet(_config: DiscoveryConfig) -> Result<Vec<DeviceIdentity>, DiscoveryError> {
    // TODO: implement async scan with concurrency limits and per-host timeouts.
    info!("placeholder subnet scan");
    sleep(Duration::from_millis(10)).await;
    Err(DiscoveryError::NotImplemented)
}
