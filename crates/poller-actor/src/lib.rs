#![allow(dead_code)]

use std::time::Duration;

use thiserror::Error;
use tokio::time::sleep;
use tracing::warn;

use avro-kafka::Publisher;
use modbus-client::{ClientConfig, ModbusClient};
use sunspec-parser::ModelDefinition;
use types::DeviceIdentity;

#[derive(Debug, Clone)]
pub struct ActorConfig {
    pub poll_interval: Duration,
    pub request_timeout: Duration,
    pub jitter_ms: u64,
}

impl Default for ActorConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(1),
            request_timeout: Duration::from_secs(1),
            jitter_ms: 0,
        }
    }
}

#[derive(Debug, Error)]
pub enum PollerError {
    #[error("poller loop not implemented")]
    NotImplemented,
}

/// A lightweight polling task responsible for one device.
pub struct PollerActor {
    identity: DeviceIdentity,
    modbus_config: ClientConfig,
    models: Vec<ModelDefinition>,
    publisher: Publisher,
    config: ActorConfig,
}

impl PollerActor {
    pub fn new(
        identity: DeviceIdentity,
        modbus_config: ClientConfig,
        models: Vec<ModelDefinition>,
        publisher: Publisher,
        config: ActorConfig,
    ) -> Self {
        Self {
            identity,
            modbus_config,
            models,
            publisher,
            config,
        }
    }

    pub async fn run(mut self) -> Result<(), PollerError> {
        let _client = ModbusClient::connect(self.modbus_config.clone())
            .await
            .map_err(|_| PollerError::NotImplemented)?;

        loop {
            // TODO: issue reads, parse responses, publish.
            warn!("poll loop not implemented for {}", self.identity.ip);
            sleep(self.config.poll_interval).await;
            return Err(PollerError::NotImplemented);
        }
    }
}
