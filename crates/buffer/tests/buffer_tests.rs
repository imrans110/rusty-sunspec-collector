use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use buffer::BufferStore;

#[tokio::test]
async fn buffer_enqueue_dequeue_delete() {
    let path = temp_db_path("buffer_enqueue_dequeue_delete");
    let store = BufferStore::new(path.to_str().expect("path"))
        .await
        .expect("init");

    store.enqueue("topic-a", b"alpha").await.expect("enqueue");
    store.enqueue("topic-b", b"beta").await.expect("enqueue");

    let count = store.pending_count().await.expect("count");
    assert_eq!(count, 2);

    let batch = store.dequeue_batch(10).await.expect("dequeue");
    assert_eq!(batch.len(), 2);
    assert_eq!(batch[0].topic, "topic-a");
    assert_eq!(batch[0].payload, b"alpha");
    assert_eq!(batch[1].topic, "topic-b");

    let ids: Vec<i64> = batch.iter().map(|item| item.id).collect();
    store.delete_batch(&ids).await.expect("delete");

    let remaining = store.dequeue_batch(10).await.expect("dequeue");
    assert!(remaining.is_empty());

    drop(store);
    cleanup_db(&path);
}

#[tokio::test]
async fn buffer_delete_empty_is_noop() {
    let path = temp_db_path("buffer_delete_empty_is_noop");
    let store = BufferStore::new(path.to_str().expect("path"))
        .await
        .expect("init");

    let count = store.pending_count().await.expect("count");
    assert_eq!(count, 0);

    store.delete_batch(&[]).await.expect("delete");

    let count = store.pending_count().await.expect("count");
    assert_eq!(count, 0);

    drop(store);
    cleanup_db(&path);
}

#[tokio::test]
async fn enqueue_batch_writes_every_row_in_order() {
    let path = temp_db_path("enqueue_batch");
    let store = BufferStore::new(path.to_str().expect("path"))
        .await
        .expect("init");

    let payloads: Vec<Vec<u8>> = (0..250).map(|i: u32| i.to_be_bytes().to_vec()).collect();
    store
        .enqueue_batch("topic", &payloads)
        .await
        .expect("batch");
    store
        .enqueue_batch::<Vec<u8>>("topic", &[])
        .await
        .expect("empty batch is a no-op");

    let rows = store.dequeue_batch(1_000).await.expect("dequeue");
    assert_eq!(rows.len(), 250);
    assert!(rows
        .iter()
        .zip(&payloads)
        .all(|(row, payload)| &row.payload == payload));

    drop(store);
    cleanup_db(&path);
}

#[tokio::test]
async fn trim_keeps_only_the_newest_rows() {
    let path = temp_db_path("trim_rows");
    let store = BufferStore::new(path.to_str().expect("path"))
        .await
        .expect("init");
    let payloads: Vec<Vec<u8>> = (0..10u8).map(|i| vec![i]).collect();
    store
        .enqueue_batch("topic", &payloads)
        .await
        .expect("batch");

    assert_eq!(store.trim_to_max_rows(4).await.expect("trim"), 6);
    assert_eq!(store.trim_to_max_rows(4).await.expect("trim again"), 0);

    let rows = store.dequeue_batch(100).await.expect("dequeue");
    let kept: Vec<u8> = rows.iter().map(|row| row.payload[0]).collect();
    assert_eq!(kept, vec![6, 7, 8, 9]);

    drop(store);
    cleanup_db(&path);
}

#[tokio::test]
async fn age_based_retention_deletes_only_old_rows() {
    let path = temp_db_path("age_retention");
    let store = BufferStore::new(path.to_str().expect("path"))
        .await
        .expect("init");
    assert_eq!(store.oldest_created_at().await.expect("oldest"), None);

    store.enqueue("topic", b"old").await.expect("old row");
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    let cutoff = now_ms();
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    store.enqueue("topic", b"new").await.expect("new row");

    let oldest = store
        .oldest_created_at()
        .await
        .expect("oldest")
        .expect("rows");
    assert!(oldest < cutoff);
    assert_eq!(store.delete_older_than(cutoff).await.expect("delete"), 1);

    let rows = store.dequeue_batch(10).await.expect("dequeue");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].payload, b"new");

    drop(store);
    cleanup_db(&path);
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
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
