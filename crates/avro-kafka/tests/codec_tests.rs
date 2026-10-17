use apache_avro::rabin::Rabin;
use avro_kafka::{decode, encode, schema, schema_fingerprint_hex, single_object_header, Publisher};
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
        labels: vec![
            ("Evt1".to_string(), String::new()),
            ("St".to_string(), "MPPT".to_string()),
        ],
    }
}

/// Schema 2.1 added `labels` with an empty default, so readers resolving 2.0 data with
/// the 2.1 schema (and the reverse) still work.
#[test]
fn labels_were_added_with_a_default() {
    let schema: serde_json::Value =
        serde_json::from_str(avro_kafka::SCHEMA_JSON).expect("schema is JSON");
    let labels = schema["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|field| field["name"] == "labels")
        .expect("labels field");
    assert_eq!(labels["default"], serde_json::json!({}));
    assert_eq!(labels["type"]["values"], "string");
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

/// A sample whose maps have one entry each: Avro writes map entries in `HashMap` order,
/// so only single-entry maps encode to the same bytes every time.
fn golden_sample(name: &str, value: PointValue) -> Sample {
    Sample {
        device: DeviceIdentity {
            ip: "192.168.1.50".to_string(),
            unit_id: 3,
        },
        model_id: 103,
        model_name: "inverter_three_phase".to_string(),
        start: 40_070,
        registers: vec![103, 50, 0, 1, 0x7fff, 0x8000, 0xffff],
        collected_at_ms: 1_792_252_800_123,
        points: vec![(name.to_string(), value)],
        labels: vec![("St".to_string(), "MPPT".to_string())],
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
        .collect()
}

/// Records are a public contract (schema 2.1, `com.rusty.sunspec.v2`). These bytes were
/// written by apache-avro 0.21; a dependency update or schema change that alters what is
/// on the wire fails here. A deliberate schema change updates them, with a CHANGELOG entry.
#[test]
fn records_keep_their_wire_format() {
    let cases = [
        ("VA", PointValue::Null, "c301e49fd96733758f2f1c3139322e3136382e312e35302f33183139322e3136382e312e353006ce0128696e7665727465725f74687265655f70686173658cf204f6b1bba9a96802045641000002045374084d505054000ece01640002feff03808004feff0700"),
        ("W", PointValue::Float(-1234.5), "c301e49fd96733758f2f1c3139322e3136382e312e35302f33183139322e3136382e312e353006ce0128696e7665727465725f74687265655f70686173658cf204f6b1bba9a9680202570200000000004a93c00002045374084d505054000ece01640002feff03808004feff0700"),
        ("WH", PointValue::Int(65_541), "c301e49fd96733758f2f1c3139322e3136382e312e35302f33183139322e3136382e312e353006ce0128696e7665727465725f74687265655f70686173658cf204f6b1bba9a96802045748048a80080002045374084d505054000ece01640002feff03808004feff0700"),
        ("Mn", PointValue::Text("Rüsty".to_string()), "c301e49fd96733758f2f1c3139322e3136382e312e35302f33183139322e3136382e312e353006ce0128696e7665727465725f74687265655f70686173658cf204f6b1bba9a96802044d6e060c52c3bc7374790002045374084d505054000ece01640002feff03808004feff0700"),
    ];
    for (name, value, expected) in cases {
        let sample = golden_sample(name, value);
        assert_eq!(hex(&encode(&sample).unwrap()), expected, "encoding {name}");
        assert_eq!(decode(&unhex(expected)).unwrap(), sample, "decoding {name}");
    }
    assert_eq!(schema_fingerprint_hex(), "e49fd96733758f2f");
}
