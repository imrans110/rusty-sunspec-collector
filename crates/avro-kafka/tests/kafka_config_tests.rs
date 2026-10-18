use avro_kafka::{KafkaConfig, KafkaSecurity, Publisher, Secret};

// librdkafka validates producer settings, including that the compression codec was
// compiled in, when the producer is created. No broker is needed.
#[test]
fn producer_accepts_every_supported_compression_codec() {
    for codec in ["none", "gzip", "snappy", "lz4", "zstd"] {
        let config = KafkaConfig {
            compression: codec.to_string(),
            ..Default::default()
        };
        let result = Publisher::new_kafka("sunspec.telemetry", config);
        assert!(result.is_ok(), "codec {codec} rejected: {:?}", result.err());
    }
}

#[test]
fn default_config_creates_a_producer() {
    let result = Publisher::new_kafka("sunspec.telemetry", KafkaConfig::default());
    assert!(result.is_ok(), "{:?}", result.err());
}

// librdkafka reports a broker it cannot connect to; the publisher remembers that until the
// brokers answer again (which the uplink uses to end its backoff early), and probing an
// unreachable broker fails.
#[tokio::test]
async fn an_unreachable_broker_is_reported_and_fails_the_probe() {
    use std::time::{Duration, Instant};

    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().expect("addr").port();
    drop(listener);
    let config = KafkaConfig {
        brokers: format!("127.0.0.1:{port}"),
        message_timeout_ms: 500,
        ..Default::default()
    };
    let publisher = Publisher::new_kafka("sunspec.telemetry", config).expect("publisher");

    assert!(!publisher.probe(Duration::from_millis(300)).await);
    let deadline = Instant::now() + Duration::from_secs(10);
    while !publisher.brokers_unreachable() {
        assert!(
            Instant::now() < deadline,
            "librdkafka never reported the broker unreachable"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

#[tokio::test]
async fn log_and_memory_publishers_always_answer_the_probe() {
    let probe = std::time::Duration::from_millis(10);
    let log = Publisher::new_log("sunspec.telemetry");
    let (memory, _) = Publisher::new_memory("sunspec.telemetry");
    for publisher in [log, memory] {
        assert!(publisher.probe(probe).await);
        assert!(!publisher.brokers_unreachable());
    }
}

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"))
}

fn secured(security: KafkaSecurity) -> Result<Publisher, avro_kafka::PublishError> {
    Publisher::new_kafka(
        "sunspec.telemetry",
        KafkaConfig {
            security,
            ..Default::default()
        },
    )
}

#[test]
fn producer_accepts_each_security_protocol() {
    let ca = Some(fixture("test-ca.pem"));
    let cases = [
        KafkaSecurity {
            protocol: Some("plaintext".into()),
            ..Default::default()
        },
        KafkaSecurity {
            protocol: Some("ssl".into()),
            ssl_ca_location: ca.clone(),
            ..Default::default()
        },
        KafkaSecurity {
            protocol: Some("sasl_plaintext".into()),
            sasl_mechanism: Some("PLAIN".into()),
            sasl_username: Some("collector".into()),
            sasl_password: Some(Secret::new("collector-secret")),
            ..Default::default()
        },
        KafkaSecurity {
            protocol: Some("sasl_ssl".into()),
            ssl_ca_location: ca,
            sasl_mechanism: Some("SCRAM-SHA-512".into()),
            sasl_username: Some("collector".into()),
            sasl_password: Some(Secret::new("collector-secret")),
            ..Default::default()
        },
    ];
    for security in cases {
        let protocol = security.protocol.clone();
        assert!(
            secured(security).is_ok(),
            "{protocol:?} rejected by librdkafka"
        );
    }
}

#[test]
fn producer_rejects_an_unreadable_ca_certificate() {
    let result = secured(KafkaSecurity {
        protocol: Some("ssl".into()),
        ssl_ca_location: Some(fixture("not-a-certificate.pem")),
        ..Default::default()
    });

    assert!(
        result.is_err(),
        "librdkafka loads the CA when the producer is created"
    );
}

#[test]
fn secrets_are_redacted_in_debug_output() {
    let config = KafkaConfig {
        security: KafkaSecurity {
            sasl_password: Some(Secret::new("hunter2")),
            ssl_key_password: Some(Secret::new("hunter3")),
            ..Default::default()
        },
        ..Default::default()
    };

    let debug = format!("{config:?}");

    assert!(
        !debug.contains("hunter2") && !debug.contains("hunter3"),
        "{debug}"
    );
    assert!(debug.contains("<redacted>"));
}
