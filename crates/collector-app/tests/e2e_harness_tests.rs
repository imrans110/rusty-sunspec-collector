use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use avro_kafka::Publisher;
use buffer::BufferStore;
use poller_actor::PollSample;
use types::DeviceIdentity;

#[tokio::test]
async fn e2e_harness_serializes_and_buffers() {
    let path = temp_db_path("e2e_harness");
    let buffer = BufferStore::new(path.to_str().expect("path"))
        .await
        .expect("buffer init");
    let publisher = Publisher::new_mock(Publisher::default_schema(), "sunspec.telemetry");

    let sample = PollSample::new(
        DeviceIdentity {
            ip: "127.0.0.1".to_string(),
            unit_id: 1,
        },
        103,
        "three_phase_inverter",
        40_002,
        vec![1, 2, 3, 4],
        unix_ms(),
    );

    let payload = publisher.serialize(&sample).expect("serialize");
    buffer
        .enqueue(publisher.topic(), &payload)
        .await
        .expect("enqueue");

    let batch = buffer.dequeue_batch(10).await.expect("dequeue");
    assert_eq!(batch.len(), 1);

    publisher
        .publish_bytes(&batch[0].topic, &batch[0].payload)
        .await
        .expect("publish bytes");
    buffer
        .delete_batch(&[batch[0].id])
        .await
        .expect("delete");

    let remaining = buffer.dequeue_batch(10).await.expect("dequeue");
    assert!(remaining.is_empty());

    drop(buffer);
    cleanup_db(&path);
}

fn temp_db_path(prefix: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let pid = std::process::id();
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    path.push(format!("{prefix}-{pid}-{ts}.sqlite"));
    path
}

fn cleanup_db(path: &PathBuf) {
    let _ = std::fs::remove_file(path);
    let wal = PathBuf::from(format!("{}-wal", path.display()));
    let shm = PathBuf::from(format!("{}-shm", path.display()));
    let _ = std::fs::remove_file(wal);
    let _ = std::fs::remove_file(shm);
}

fn unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
