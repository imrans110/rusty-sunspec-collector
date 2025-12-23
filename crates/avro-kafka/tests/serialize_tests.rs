use avro_kafka::Publisher;
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

#[test]
fn serialize_default_schema() {
    let publisher = Publisher::new_mock(Publisher::default_schema(), "topic");
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

    let bytes = publisher.serialize(&payload).expect("serialize ok");
    assert!(!bytes.is_empty());
    assert_eq!(publisher.topic(), "topic");
}
