use apache_avro::rabin::Rabin;
use avro_kafka::{decode, encode, schema, single_object_header, Publisher};
use types::{DeviceIdentity, PointValue, Sample};

fn sample(unit_id: u8) -> Sample {
    Sample {
        device: DeviceIdentity {
            ip: "10.0.0.5".to_string(),
            unit_id,
        },
        model_id: 103,
        model_name: "inverter_three_phase".to_string(),
        start: 40_070,
        registers: vec![103, 50, 1234, 65_535],
        collected_at_ms: 1_760_000_000_123,
        points: vec![
            ("A".to_string(), PointValue::Float(12.34)),
            ("Mn".to_string(), PointValue::Text("Rusty".to_string())),
            ("St".to_string(), PointValue::Int(4)),
            ("VA".to_string(), PointValue::Null),
        ],
    }
}

#[test]
fn single_object_encoding_round_trips() {
    let original = sample(1);

    let bytes = encode(&original).expect("encode");
    let decoded = decode(&bytes).expect("decode");

    assert_eq!(decoded, original);
}

#[test]
fn messages_start_with_the_schema_fingerprint() {
    let bytes = encode(&sample(1)).expect("encode");
    let fingerprint = schema().fingerprint::<Rabin>().bytes;

    assert_eq!(&bytes[..2], &[0xC3, 0x01], "single-object marker");
    let mut little_endian = fingerprint.clone();
    little_endian.reverse();
    assert!(
        bytes[2..10] == fingerprint[..] || bytes[2..10] == little_endian[..],
        "header carries the 64-bit Rabin fingerprint"
    );
    assert_eq!(&bytes[..10], single_object_header());
}

#[test]
fn decoding_rejects_a_foreign_fingerprint() {
    let mut bytes = encode(&sample(1)).expect("encode");
    bytes[5] ^= 0xFF;

    assert!(decode(&bytes).is_err());
}

#[tokio::test]
async fn each_sample_is_its_own_record_keyed_by_device() {
    let (publisher, sink) = Publisher::new_memory("sunspec.telemetry");

    let results = publisher.publish(&[sample(1), sample(2), sample(1)]).await;

    assert!(results.iter().all(Result::is_ok));
    let records = sink.records();
    let keys: Vec<&str> = records.iter().map(|record| record.key.as_str()).collect();
    assert_eq!(keys, vec!["10.0.0.5/1", "10.0.0.5/2", "10.0.0.5/1"]);
    assert_eq!(decode(&records[1].payload).unwrap().device.unit_id, 2);
}
