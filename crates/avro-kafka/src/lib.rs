#![allow(dead_code)]

use std::time::Duration;

use apache_avro::{Schema, Writer};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use serde::Serialize;
use thiserror::Error;
use tracing::info;

#[derive(Debug, Clone)]
pub struct Publisher {
    schema: Schema,
    topic: String,
    producer: Option<FutureProducer>,
    timeout: Duration,
}

#[derive(Debug, Clone)]
pub struct KafkaConfig {
    pub brokers: String,
    pub client_id: String,
    pub acks: String,
    pub compression: String,
    pub message_timeout_ms: u64,
    pub enable_idempotence: bool,
}

impl Publisher {
    pub fn new_mock(schema: Schema, topic: impl Into<String>) -> Self {
        Self {
            schema,
            topic: topic.into(),
            producer: None,
            timeout: Duration::from_millis(0),
        }
    }

    pub fn new_kafka(
        schema: Schema,
        topic: impl Into<String>,
        config: KafkaConfig,
    ) -> Result<Self, PublishError> {
        let timeout = Duration::from_millis(config.message_timeout_ms);
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("client.id", &config.client_id)
            .set("acks", &config.acks)
            .set("compression.type", &config.compression)
            .set(
                "enable.idempotence",
                if config.enable_idempotence { "true" } else { "false" },
            )
            .set("message.timeout.ms", &config.message_timeout_ms.to_string())
            .create()
            .map_err(PublishError::KafkaConfig)?;

        Ok(Self {
            schema,
            topic: topic.into(),
            producer: Some(producer),
            timeout,
        })
    }

    pub async fn publish<T: Serialize>(&self, value: &T) -> Result<(), PublishError> {
        let payload = self.serialize(value)?;
        self.publish_bytes(&self.topic, &payload).await
    }

    pub async fn publish_bytes(&self, topic: &str, payload: &[u8]) -> Result<(), PublishError> {
        match &self.producer {
            Some(producer) => {
                let record = FutureRecord::to(topic).payload(payload);
                producer
                    .send(record, Timeout::After(self.timeout))
                    .await
                    .map_err(|(err, _)| PublishError::Kafka(err))?;
                Ok(())
            }
            None => {
                info!(topic = %topic, bytes = payload.len(), "mock publish invoked");
                Ok(())
            }
        }
    }

    pub fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, PublishError> {
        let avro_value = apache_avro::to_value(value)
            .map_err(|err| PublishError::Encode(err.to_string()))?;
        let mut writer = Writer::with_codec(&self.schema, Vec::new(), apache_avro::Codec::Deflate);
        writer
            .append(avro_value)
            .map_err(|err| PublishError::Encode(err.to_string()))?;
        writer
            .flush()
            .map_err(|err| PublishError::Encode(err.to_string()))?;
        Ok(writer.into_inner())
    }

    pub fn default_schema() -> Schema {
        Schema::parse_str(DEFAULT_SCHEMA).expect("valid avro schema")
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }
}

#[derive(Debug, Error)]
pub enum PublishError {
    #[error("avro encode error: {0}")]
    Encode(String),
    #[error("kafka config error: {0}")]
    KafkaConfig(rdkafka::error::KafkaError),
    #[error("kafka publish error: {0}")]
    Kafka(rdkafka::error::KafkaError),
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self {
            brokers: "localhost:9092".to_string(),
            client_id: "sunspec-collector".to_string(),
            acks: "all".to_string(),
            compression: "zstd".to_string(),
            message_timeout_ms: 5_000,
            enable_idempotence: true,
        }
    }
}

const DEFAULT_SCHEMA: &str = r#"
{
  "type": "record",
  "name": "SunspecTelemetry",
  "namespace": "com.rusty.sunspec",
  "fields": [
    {
      "name": "device",
      "type": {
        "type": "record",
        "name": "DeviceIdentity",
        "fields": [
          {"name": "ip", "type": "string"},
          {"name": "unit_id", "type": "int"}
        ]
      }
    },
    {"name": "model_id", "type": "int"},
    {"name": "model_name", "type": "string"},
    {"name": "start", "type": "int"},
    {"name": "registers", "type": {"type": "array", "items": "int"}},
    {"name": "collected_at_ms", "type": "long"}
  ]
}
"#;
