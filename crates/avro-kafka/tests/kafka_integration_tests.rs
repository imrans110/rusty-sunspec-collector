use avro_kafka::{KafkaConfig, Publisher};
use types::{DeviceIdentity, PointValue, Sample};

#[tokio::test]
#[ignore = "needs a Kafka broker: SUNSPEC_KAFKA_BROKERS=host:port cargo test -- --ignored"]
async fn publishes_keyed_records_to_kafka() {
    let brokers = std::env::var("SUNSPEC_KAFKA_BROKERS")
        .expect("SUNSPEC_KAFKA_BROKERS must point at a Kafka broker");
    let topic =
        std::env::var("SUNSPEC_KAFKA_TOPIC").unwrap_or_else(|_| "sunspec.telemetry".to_string());
    let config = KafkaConfig {
        brokers,
        client_id: "sunspec-collector-tests".to_string(),
        ..Default::default()
    };
    let publisher = Publisher::new_kafka(&topic, config).expect("publisher init");

    let sample = Sample {
        device: DeviceIdentity {
            ip: "127.0.0.1".to_string(),
            unit_id: 1,
        },
        model_id: 103,
        model_name: "inverter_three_phase".to_string(),
        start: 40_070,
        registers: vec![103, 50],
        collected_at_ms: 1_700_000_000_000,
        points: vec![("W".to_string(), PointValue::Float(432.1))],
    };

    for result in publisher.publish(&[sample.clone(), sample]).await {
        result.expect("delivered");
    }
}
