use std::time::Duration;

use metrics::counter;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{info, warn};

use buffer::BufferStore;
use poller_actor::PollSample;

/// Most samples written in one transaction.
const MAX_BATCH: usize = 256;
/// How long to wait for more samples before committing a partial batch.
const COALESCE_WINDOW: Duration = Duration::from_millis(100);

/// Persists samples from the pollers into the buffer, grouping samples that arrive
/// close together into one transaction. Runs until every sender is dropped, so
/// samples still queued at shutdown are written before it returns.
pub async fn run(mut rx: mpsc::Receiver<PollSample>, buffer: BufferStore, topic: String) {
    let mut batch = Vec::with_capacity(MAX_BATCH);
    let mut closed = false;

    while !closed {
        if rx.recv_many(&mut batch, MAX_BATCH).await == 0 {
            break;
        }
        let window = sleep(COALESCE_WINDOW);
        tokio::pin!(window);
        while batch.len() < MAX_BATCH {
            let room = MAX_BATCH - batch.len();
            tokio::select! {
                biased;
                received = rx.recv_many(&mut batch, room) => {
                    if received == 0 {
                        closed = true;
                        break;
                    }
                }
                _ = &mut window => break,
            }
        }
        persist(&buffer, &topic, &mut batch).await;
    }
    info!("ingest stopped");
}

async fn persist(buffer: &BufferStore, topic: &str, batch: &mut Vec<PollSample>) {
    // Store lightweight JSON in the buffer; Avro is produced at uplink time.
    let payloads: Vec<Vec<u8>> = batch
        .drain(..)
        .filter_map(|sample| match serde_json::to_vec(&sample) {
            Ok(payload) => Some(payload),
            Err(err) => {
                warn!(error = %err, "json serialization failed");
                None
            }
        })
        .collect();
    match buffer.enqueue_batch(topic, &payloads).await {
        Ok(()) => counter!("buffer_enqueue_success").increment(payloads.len() as u64),
        Err(err) => {
            warn!(error = %err, rows = payloads.len(), "buffer write failed; samples dropped");
            counter!("sunspec_buffer_dropped_total", "reason" => "write_error")
                .increment(payloads.len() as u64);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::DeviceIdentity;

    fn sample(n: u16) -> PollSample {
        PollSample {
            device: DeviceIdentity {
                ip: "127.0.0.1".to_string(),
                unit_id: 1,
            },
            model_id: 103,
            model_name: "three_phase_inverter".to_string(),
            start: 40_070,
            registers: vec![n],
            collected_at_ms: u64::from(n),
        }
    }

    #[tokio::test]
    async fn persists_everything_queued_before_the_channel_closes() {
        let path = std::env::temp_dir().join(format!(
            "ingest-{}-{}.sqlite",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        let buffer = BufferStore::new(path.to_str().unwrap()).await.unwrap();
        let (tx, rx) = mpsc::channel(1_000);
        for n in 0..600 {
            tx.send(sample(n)).await.unwrap();
        }
        // Shutdown: the pollers' senders are dropped while samples are still queued.
        drop(tx);

        run(rx, buffer.clone(), "topic".to_string()).await;

        let rows = buffer.dequeue_batch(10_000).await.unwrap();
        assert_eq!(rows.len(), 600);
        let first: PollSample = serde_json::from_slice(&rows[0].payload).unwrap();
        let last: PollSample = serde_json::from_slice(&rows[599].payload).unwrap();
        assert_eq!((first.registers[0], last.registers[0]), (0, 599));
        for suffix in ["", "-wal", "-shm"] {
            let _ = std::fs::remove_file(format!("{}{suffix}", path.display()));
        }
    }
}
