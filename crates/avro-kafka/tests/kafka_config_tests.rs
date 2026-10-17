use avro_kafka::{KafkaConfig, Publisher};

// librdkafka validates producer settings, including that the compression codec was
// compiled in, when the producer is created. No broker is needed.
#[test]
fn producer_accepts_every_supported_compression_codec() {
    for codec in ["none", "gzip", "snappy", "lz4", "zstd"] {
        let config = KafkaConfig {
            compression: codec.to_string(),
            ..Default::default()
        };
        let result = Publisher::new_kafka(Publisher::default_schema(), "sunspec.telemetry", config);
        assert!(result.is_ok(), "codec {codec} rejected: {:?}", result.err());
    }
}

#[test]
fn default_config_creates_a_producer() {
    let result = Publisher::new_kafka(
        Publisher::default_schema(),
        "sunspec.telemetry",
        KafkaConfig::default(),
    );
    assert!(result.is_ok(), "{:?}", result.err());
}
