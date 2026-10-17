use avro_kafka::{KafkaConfig, Publisher};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Sample {
    device: Device,
    model_id: i32,
    model_name: String,
    start: i32,
    registers: Vec<i32>,
    collected_at_ms: i64,
}

#[derive(Debug, Serialize)]
struct Device {
    ip: String,
    unit_id: i32,
}

#[tokio::test]
#[ignore = "needs a Kafka broker: SUNSPEC_KAFKA_BROKERS=host:port cargo test -- --ignored"]
async fn kafka_publish_integration() {
    let brokers = std::env::var("SUNSPEC_KAFKA_BROKERS")
        .expect("SUNSPEC_KAFKA_BROKERS must point at a Kafka broker");
    let topic =
        std::env::var("SUNSPEC_KAFKA_TOPIC").unwrap_or_else(|_| "sunspec.telemetry".to_string());

    let config = KafkaConfig {
        brokers,
        client_id: std::env::var("SUNSPEC_KAFKA_CLIENT_ID")
            .unwrap_or_else(|_| "sunspec-collector-tests".to_string()),
        ..Default::default()
    };

    let publisher =
        Publisher::new_kafka(Publisher::default_schema(), &topic, config).expect("publisher init");

    let payload = Sample {
        device: Device {
            ip: "127.0.0.1".to_string(),
            unit_id: 1,
        },
        model_id: 103,
        model_name: "three_phase_inverter".to_string(),
        start: 40002,
        registers: vec![1, 2, 3],
        collected_at_ms: 1_700_000_000,
    };

    publisher.publish(&payload).await.expect("publish");
}
