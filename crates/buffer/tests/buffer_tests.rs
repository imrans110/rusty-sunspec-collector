use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use buffer::BufferStore;

#[tokio::test]
async fn buffer_enqueue_dequeue_delete() {
    let path = temp_db_path("buffer_enqueue_dequeue_delete");
    let store = BufferStore::new(path.to_str().expect("path")).await.expect("init");

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
    let store = BufferStore::new(path.to_str().expect("path")).await.expect("init");

    let count = store.pending_count().await.expect("count");
    assert_eq!(count, 0);

    store.delete_batch(&[]).await.expect("delete");

    let count = store.pending_count().await.expect("count");
    assert_eq!(count, 0);

    drop(store);
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
