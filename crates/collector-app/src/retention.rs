use std::time::{Duration, SystemTime, UNIX_EPOCH};

use metrics::{counter, gauge};
use tokio::sync::watch;
use tokio::time::{interval, MissedTickBehavior};
use tracing::warn;

use buffer::BufferStore;

const MAINTENANCE_INTERVAL: Duration = Duration::from_secs(10);

/// Keeps the buffer bounded during long uplink outages by dropping the oldest rows
/// beyond `max_rows` or older than `max_age`, and publishes buffer gauges.
pub async fn run(
    buffer: BufferStore,
    max_rows: i64,
    max_age: Duration,
    mut shutdown: watch::Receiver<bool>,
) {
    let mut tick = interval(MAINTENANCE_INTERVAL);
    tick.set_missed_tick_behavior(MissedTickBehavior::Delay);
    loop {
        tokio::select! {
            _ = tick.tick() => {}
            _ = shutdown.wait_for(|stop| *stop) => break,
        }
        enforce(&buffer, max_rows, max_age).await;
    }
}

async fn enforce(buffer: &BufferStore, max_rows: i64, max_age: Duration) {
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;
    let cutoff = now_ms.saturating_sub(max_age.as_millis() as i64);

    match buffer.delete_older_than(cutoff).await {
        Ok(0) => {}
        Ok(dropped) => {
            warn!(
                dropped,
                max_age_s = max_age.as_secs(),
                "buffer retention: dropped rows past max age"
            );
            counter!("sunspec_buffer_dropped_total", "reason" => "max_age").increment(dropped);
        }
        Err(err) => warn!(error = %err, "buffer age retention failed"),
    }
    match buffer.trim_to_max_rows(max_rows).await {
        Ok(0) => {}
        Ok(dropped) => {
            warn!(
                dropped,
                max_rows, "buffer retention: dropped oldest rows over max rows"
            );
            counter!("sunspec_buffer_dropped_total", "reason" => "max_rows").increment(dropped);
        }
        Err(err) => warn!(error = %err, "buffer row retention failed"),
    }

    if let Ok(rows) = buffer.pending_count().await {
        gauge!("sunspec_buffer_rows").set(rows as f64);
    }
    if let Ok(oldest) = buffer.oldest_created_at().await {
        let age_s = oldest.map_or(0.0, |oldest| (now_ms - oldest).max(0) as f64 / 1000.0);
        gauge!("sunspec_buffer_oldest_age_seconds").set(age_s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn enforces_row_and_age_limits() {
        let path = std::env::temp_dir().join(format!(
            "retention-{}-{}.sqlite",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        let buffer = BufferStore::new(path.to_str().unwrap()).await.unwrap();
        let rows: Vec<Vec<u8>> = (0..20u8).map(|i| vec![i]).collect();
        buffer.enqueue_batch("topic", &rows).await.unwrap();

        enforce(&buffer, 5, Duration::from_secs(3_600)).await;
        assert_eq!(buffer.pending_count().await.unwrap(), 5);

        tokio::time::sleep(Duration::from_millis(30)).await;
        enforce(&buffer, 5, Duration::from_millis(10)).await;
        assert_eq!(buffer.pending_count().await.unwrap(), 0);

        for suffix in ["", "-wal", "-shm"] {
            let _ = std::fs::remove_file(format!("{}{suffix}", path.display()));
        }
    }
}
