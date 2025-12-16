#![allow(dead_code)]

use serde::Serialize;
use thiserror::Error;
use tracing::info;

#[derive(Debug, Clone)]
pub struct Publisher;

impl Publisher {
    pub fn new_mock() -> Self {
        Self
    }

    pub async fn publish<T: Serialize>(&self, _value: &T) -> Result<(), PublishError> {
        // TODO: emit Avro-encoded payloads to Kafka using rdkafka.
        info!("mock publish invoked");
        Err(PublishError::NotImplemented)
    }
}

#[derive(Debug, Error)]
pub enum PublishError {
    #[error("publish not implemented")]
    NotImplemented,
}
