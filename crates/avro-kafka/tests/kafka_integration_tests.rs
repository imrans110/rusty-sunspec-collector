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
        labels: vec![("St".to_string(), "MPPT".to_string())],
    };

    for result in publisher.publish(&[sample.clone(), sample]).await {
        result.expect("delivered");
    }
    assert!(
        publisher.probe(std::time::Duration::from_secs(5)).await,
        "a reachable broker answers the probe"
    );
}

/// Run by the CI Docker E2E job after the compose stack (simulator → collector →
/// Kafka) has published records: everything on the topic must decode, carry the
/// expected headers, be keyed by its device, and include decoded inverter power.
/// `SUNSPEC_E2E_DEVICE_HOST` is the host the collector polls (default `simulator`, the
/// compose service); set it to `127.0.0.1` for a collector polling a local simulator.
#[tokio::test]
#[ignore = "needs a broker a running collector publishes to (CI Docker E2E job)"]
async fn published_records_decode_to_samples() {
    use std::time::{Duration, Instant};

    use rdkafka::consumer::{Consumer, StreamConsumer};
    use rdkafka::message::Headers;
    use rdkafka::{ClientConfig, Message};

    let brokers = std::env::var("SUNSPEC_KAFKA_BROKERS")
        .expect("SUNSPEC_KAFKA_BROKERS must point at a Kafka broker");
    let topic =
        std::env::var("SUNSPEC_KAFKA_TOPIC").unwrap_or_else(|_| "sunspec.telemetry".to_string());
    let device_host =
        std::env::var("SUNSPEC_E2E_DEVICE_HOST").unwrap_or_else(|_| "simulator".to_string());
    let device_prefix = format!("{device_host}/");
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &brokers)
        .set("group.id", format!("e2e-{}", std::process::id()))
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false")
        .create()
        .expect("consumer");
    consumer.subscribe(&[&topic]).expect("subscribe");

    let deadline = Instant::now() + Duration::from_secs(60);
    let mut decoded = 0;
    let mut collector_records = 0;
    let mut saw_scaled_power = false;
    while decoded < 50 && Instant::now() < deadline {
        let Ok(Ok(message)) = tokio::time::timeout(Duration::from_secs(10), consumer.recv()).await
        else {
            break;
        };
        let key = std::str::from_utf8(message.key().expect("record key")).expect("utf-8 key");
        let sample = avro_kafka::decode(message.payload().expect("payload")).expect("decodes");
        assert_eq!(key, sample.device.key(), "record key matches its device");
        let headers = message.headers().expect("headers");
        let content_type = headers
            .iter()
            .find(|header| header.key == "content-type")
            .and_then(|header| header.value)
            .expect("content-type header");
        assert_eq!(content_type, avro_kafka::CONTENT_TYPE.as_bytes());

        if key.starts_with(&device_prefix) {
            collector_records += 1;
            if sample.model_id == 103 {
                saw_scaled_power |= sample.points.iter().any(|(name, value)| {
                    name == "W" && matches!(value, types::PointValue::Float(_))
                });
            }
        }
        decoded += 1;
    }

    println!("decoded {decoded} records, {collector_records} from the collector");
    assert!(
        collector_records > 0,
        "no records from devices on {device_host}"
    );
    assert!(saw_scaled_power, "no decoded model 103 power reading");
}
