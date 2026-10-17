//! Publishes decoded samples to Kafka: one record per sample, keyed by device, encoded
//! with Avro single-object encoding (`C3 01`, the schema's 8-byte Rabin fingerprint,
//! then the datum). The schema is `schemas/sunspec_sample.avsc`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use apache_avro::headers::{HeaderBuilder, RabinFingerprintHeader};
use apache_avro::types::Value;
use apache_avro::{GenericSingleObjectReader, GenericSingleObjectWriter, Schema};
use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use thiserror::Error;
use tracing::debug;
use types::{DeviceIdentity, PointValue, Sample};

/// Avro schema of every published record.
pub const SCHEMA_JSON: &str = include_str!("../schemas/sunspec_sample.avsc");
/// Value of the `content-type` header on every record.
pub const CONTENT_TYPE: &str = "application/avro; encoding=single-object";

#[derive(Debug, Error)]
pub enum PublishError {
    #[error("avro error: {0}")]
    Avro(String),
    #[error("kafka config error: {0}")]
    KafkaConfig(rdkafka::error::KafkaError),
    #[error("kafka publish error: {0}")]
    Kafka(rdkafka::error::KafkaError),
    #[error("kafka delivery was cancelled")]
    Cancelled,
    #[error("record does not match the sample schema: {0}")]
    Schema(&'static str),
}

impl From<apache_avro::Error> for PublishError {
    fn from(err: apache_avro::Error) -> Self {
        PublishError::Avro(err.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct KafkaConfig {
    pub brokers: String,
    pub client_id: String,
    pub acks: String,
    pub compression: String,
    pub message_timeout_ms: u64,
    pub enable_idempotence: bool,
    pub security: KafkaSecurity,
}

/// Connection security: TLS and SASL settings passed to librdkafka.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KafkaSecurity {
    /// `plaintext`, `ssl`, `sasl_plaintext` or `sasl_ssl` (`None` = librdkafka's default,
    /// plaintext).
    pub protocol: Option<String>,
    /// CA certificate file (PEM) or directory used to verify the broker.
    pub ssl_ca_location: Option<String>,
    /// Client certificate (PEM) for mutual TLS.
    pub ssl_certificate_location: Option<String>,
    /// Client private key (PEM) for mutual TLS.
    pub ssl_key_location: Option<String>,
    pub ssl_key_password: Option<Secret>,
    /// `PLAIN`, `SCRAM-SHA-256` or `SCRAM-SHA-512`.
    pub sasl_mechanism: Option<String>,
    pub sasl_username: Option<String>,
    pub sasl_password: Option<Secret>,
}

/// A secret value whose `Debug` output is redacted, so it cannot leak into logs.
#[derive(Clone, PartialEq, Eq)]
pub struct Secret(String);

impl Secret {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Secret(<redacted>)")
    }
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
            security: KafkaSecurity::default(),
        }
    }
}

/// The parsed record schema.
pub fn schema() -> &'static Schema {
    static SCHEMA: OnceLock<Schema> = OnceLock::new();
    SCHEMA.get_or_init(|| Schema::parse_str(SCHEMA_JSON).expect("bundled avro schema is valid"))
}

/// The 10-byte single-object header: `C3 01` and the schema's Rabin fingerprint.
pub fn single_object_header() -> &'static [u8] {
    static HEADER: OnceLock<Vec<u8>> = OnceLock::new();
    HEADER.get_or_init(|| RabinFingerprintHeader::from_schema(schema()).build_header())
}

/// The schema's Rabin fingerprint as hex, sent in the `schema-fingerprint` header.
pub fn schema_fingerprint_hex() -> &'static str {
    static HEX: OnceLock<String> = OnceLock::new();
    HEX.get_or_init(|| {
        single_object_header()[2..]
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    })
}

/// Encodes a sample as one Avro single-object message.
pub fn encode(sample: &Sample) -> Result<Vec<u8>, PublishError> {
    let mut writer = GenericSingleObjectWriter::new_with_capacity(schema(), 256)?;
    let mut out = Vec::new();
    writer.write_value(to_value(sample), &mut out)?;
    Ok(out)
}

/// Decodes a message produced by [`encode`]. Fails if the fingerprint does not match.
pub fn decode(bytes: &[u8]) -> Result<Sample, PublishError> {
    let reader = GenericSingleObjectReader::builder()
        .schema(schema().clone())
        .build()?;
    let value = reader.read_value(&mut &bytes[..])?;
    from_value(value)
}

fn to_value(sample: &Sample) -> Value {
    let points = sample
        .points
        .iter()
        .map(|(name, value)| {
            let value = match value {
                PointValue::Null => Value::Union(0, Box::new(Value::Null)),
                PointValue::Float(value) => Value::Union(1, Box::new(Value::Double(*value))),
                PointValue::Int(value) => Value::Union(2, Box::new(Value::Long(*value))),
                PointValue::Text(value) => Value::Union(3, Box::new(Value::String(value.clone()))),
            };
            (name.clone(), value)
        })
        .collect();
    Value::Record(vec![
        ("device_key".into(), Value::String(sample.device.key())),
        ("ip".into(), Value::String(sample.device.ip.clone())),
        ("unit_id".into(), Value::Int(sample.device.unit_id.into())),
        ("model_id".into(), Value::Int(sample.model_id.into())),
        (
            "model_name".into(),
            Value::String(sample.model_name.clone()),
        ),
        ("start".into(), Value::Int(sample.start.into())),
        (
            "collected_at_ms".into(),
            Value::TimestampMillis(sample.collected_at_ms as i64),
        ),
        ("points".into(), Value::Map(points)),
        (
            "labels".into(),
            Value::Map(
                sample
                    .labels
                    .iter()
                    .map(|(name, label)| (name.clone(), Value::String(label.clone())))
                    .collect(),
            ),
        ),
        (
            "registers".into(),
            Value::Array(
                sample
                    .registers
                    .iter()
                    .map(|register| Value::Int((*register).into()))
                    .collect(),
            ),
        ),
    ])
}

fn from_value(value: Value) -> Result<Sample, PublishError> {
    let Value::Record(fields) = value else {
        return Err(PublishError::Schema("expected a record"));
    };
    let mut fields: HashMap<String, Value> = fields.into_iter().collect();
    let mut take = |name: &'static str| fields.remove(name).ok_or(PublishError::Schema(name));

    let ip = string(take("ip")?)?;
    let unit_id = int(take("unit_id")?)?;
    let model_id = int(take("model_id")?)?;
    let model_name = string(take("model_name")?)?;
    let start = int(take("start")?)?;
    let collected_at_ms = match take("collected_at_ms")? {
        Value::TimestampMillis(ms) | Value::Long(ms) => ms as u64,
        _ => return Err(PublishError::Schema("collected_at_ms")),
    };
    let Value::Map(points) = take("points")? else {
        return Err(PublishError::Schema("points"));
    };
    let mut points: Vec<(String, PointValue)> = points
        .into_iter()
        .map(|(name, value)| {
            let value = match value {
                Value::Union(_, inner) => *inner,
                other => other,
            };
            let value = match value {
                Value::Null => PointValue::Null,
                Value::Double(value) => PointValue::Float(value),
                Value::Long(value) => PointValue::Int(value),
                Value::String(value) => PointValue::Text(value),
                _ => return Err(PublishError::Schema("points value")),
            };
            Ok((name, value))
        })
        .collect::<Result<_, _>>()?;
    points.sort_by(|a, b| a.0.cmp(&b.0));
    let mut labels: Vec<(String, String)> = match take("labels")? {
        Value::Map(labels) => labels
            .into_iter()
            .map(|(name, label)| Ok((name, string(label)?)))
            .collect::<Result<_, PublishError>>()?,
        _ => return Err(PublishError::Schema("labels")),
    };
    labels.sort_by(|a, b| a.0.cmp(&b.0));
    let Value::Array(registers) = take("registers")? else {
        return Err(PublishError::Schema("registers"));
    };
    let registers = registers
        .into_iter()
        .map(int)
        .collect::<Result<Vec<u16>, _>>()?;

    Ok(Sample {
        device: DeviceIdentity { ip, unit_id },
        model_id,
        model_name,
        start,
        registers,
        collected_at_ms,
        points,
        labels,
    })
}

fn string(value: Value) -> Result<String, PublishError> {
    match value {
        Value::String(value) => Ok(value),
        _ => Err(PublishError::Schema("expected a string")),
    }
}

fn int<T: TryFrom<i32>>(value: Value) -> Result<T, PublishError> {
    match value {
        Value::Int(value) => {
            T::try_from(value).map_err(|_| PublishError::Schema("int out of range"))
        }
        _ => Err(PublishError::Schema("expected an int")),
    }
}

/// A record handed to the in-memory sink (see [`Publisher::new_memory`]).
#[derive(Debug, Clone)]
pub struct PublishedRecord {
    pub topic: String,
    pub key: String,
    pub payload: Vec<u8>,
}

/// Collects records published through a [`Publisher::new_memory`] publisher.
#[derive(Debug, Clone, Default)]
pub struct MemorySink(Arc<Mutex<Vec<PublishedRecord>>>);

impl MemorySink {
    pub fn records(&self) -> Vec<PublishedRecord> {
        self.0.lock().expect("memory sink poisoned").clone()
    }
}

#[derive(Clone)]
enum Sink {
    Kafka(FutureProducer),
    Log,
    Memory(MemorySink),
}

#[derive(Clone)]
pub struct Publisher {
    topic: String,
    sink: Sink,
}

impl Publisher {
    pub fn new_kafka(topic: impl Into<String>, config: KafkaConfig) -> Result<Self, PublishError> {
        let mut client = ClientConfig::new();
        let security = &config.security;
        let optional = [
            ("security.protocol", security.protocol.as_deref()),
            ("ssl.ca.location", security.ssl_ca_location.as_deref()),
            (
                "ssl.certificate.location",
                security.ssl_certificate_location.as_deref(),
            ),
            ("ssl.key.location", security.ssl_key_location.as_deref()),
            (
                "ssl.key.password",
                security.ssl_key_password.as_ref().map(Secret::expose),
            ),
            ("sasl.mechanism", security.sasl_mechanism.as_deref()),
            ("sasl.username", security.sasl_username.as_deref()),
            (
                "sasl.password",
                security.sasl_password.as_ref().map(Secret::expose),
            ),
        ];
        for (key, value) in optional {
            if let Some(value) = value {
                client.set(key, value);
            }
        }
        let producer: FutureProducer = client
            .set("bootstrap.servers", &config.brokers)
            .set("client.id", &config.client_id)
            .set("acks", &config.acks)
            .set("compression.type", &config.compression)
            .set(
                "enable.idempotence",
                if config.enable_idempotence {
                    "true"
                } else {
                    "false"
                },
            )
            .set("message.timeout.ms", config.message_timeout_ms.to_string())
            // Wait briefly so records to the same partition are batched and compressed together.
            .set("linger.ms", "50")
            .create()
            .map_err(PublishError::KafkaConfig)?;
        Ok(Self {
            topic: topic.into(),
            sink: Sink::Kafka(producer),
        })
    }

    /// A publisher that only logs, for running without a broker.
    pub fn new_log(topic: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            sink: Sink::Log,
        }
    }

    /// A publisher that keeps records in memory, for tests.
    pub fn new_memory(topic: impl Into<String>) -> (Self, MemorySink) {
        let sink = MemorySink::default();
        let publisher = Self {
            topic: topic.into(),
            sink: Sink::Memory(sink.clone()),
        };
        (publisher, sink)
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    /// Encodes and publishes each sample as its own record keyed by device. All records
    /// are enqueued before waiting for acknowledgements so Kafka can batch them.
    /// Returns one result per sample, in order.
    pub async fn publish(&self, samples: &[Sample]) -> Vec<Result<(), PublishError>> {
        let encoded: Vec<Result<(String, Vec<u8>), PublishError>> = samples
            .iter()
            .map(|sample| encode(sample).map(|payload| (sample.device.key(), payload)))
            .collect();

        match &self.sink {
            Sink::Kafka(producer) => {
                let pending: Vec<_> = encoded
                    .into_iter()
                    .map(|record| {
                        let (key, payload) = record?;
                        let headers = OwnedHeaders::new()
                            .insert(Header {
                                key: "content-type",
                                value: Some(CONTENT_TYPE),
                            })
                            .insert(Header {
                                key: "schema-fingerprint",
                                value: Some(schema_fingerprint_hex()),
                            });
                        let record = FutureRecord::to(&self.topic)
                            .key(&key)
                            .payload(&payload)
                            .headers(headers);
                        producer
                            .send_result(record)
                            .map_err(|(err, _)| PublishError::Kafka(err))
                    })
                    .collect();
                let mut results = Vec::with_capacity(pending.len());
                for delivery in pending {
                    results.push(match delivery {
                        Ok(delivery) => match delivery.await {
                            Ok(Ok(_)) => Ok(()),
                            Ok(Err((err, _))) => Err(PublishError::Kafka(err)),
                            Err(_) => Err(PublishError::Cancelled),
                        },
                        Err(err) => Err(err),
                    });
                }
                results
            }
            Sink::Log => encoded
                .into_iter()
                .map(|record| {
                    let (key, payload) = record?;
                    debug!(topic = %self.topic, key = %key, bytes = payload.len(), "record published (log sink)");
                    Ok(())
                })
                .collect(),
            Sink::Memory(sink) => encoded
                .into_iter()
                .map(|record| {
                    let (key, payload) = record?;
                    sink.0
                        .lock()
                        .expect("memory sink poisoned")
                        .push(PublishedRecord {
                            topic: self.topic.clone(),
                            key,
                            payload,
                        });
                    Ok(())
                })
                .collect(),
        }
    }
}
