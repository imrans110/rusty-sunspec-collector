use std::time::{Duration, Instant};

use metrics::{counter, histogram};
use tokio::sync::watch;
use tokio::time::{sleep, timeout};
use tracing::{debug, info, warn};

use avro_kafka::Publisher;
use buffer::BufferStore;
use poller_actor::PollSample;

const BACKOFF_BASE: Duration = Duration::from_secs(1);
const BACKOFF_MAX: Duration = Duration::from_secs(30);

#[derive(Debug, Clone)]
pub struct UplinkConfig {
    pub batch_size: i64,
    /// Pause between drains once the backlog is empty.
    pub drain_interval: Duration,
    /// How long to keep draining after `stop` before giving up (0 = stop at once).
    pub shutdown_drain_timeout: Duration,
}

/// Drains the buffer to Kafka. While a backlog exists it drains batch after batch
/// without pausing; once the buffer is empty it waits `drain_interval`. After `stop`
/// it makes one bounded final drain attempt.
pub async fn run(
    buffer: BufferStore,
    publisher: Publisher,
    mut stop: watch::Receiver<bool>,
    config: UplinkConfig,
) {
    let mut failures = 0u32;
    let mut backlog = false;

    loop {
        let delay = next_delay(failures, backlog, config.drain_interval);
        tokio::select! {
            _ = sleep(delay) => {}
            _ = stop.wait_for(|stop| *stop) => break,
        }
        match drain_once(&buffer, &publisher, config.batch_size).await {
            Ok(processed) => {
                failures = 0;
                backlog = processed as i64 >= config.batch_size;
            }
            Err(()) => {
                failures = failures.saturating_add(1);
                backlog = false;
            }
        }
    }

    if !config.shutdown_drain_timeout.is_zero() {
        let drained = timeout(config.shutdown_drain_timeout, async {
            while let Ok(processed) = drain_once(&buffer, &publisher, config.batch_size).await {
                if processed == 0 {
                    break;
                }
            }
        })
        .await;
        if drained.is_err() {
            warn!("final uplink drain timed out; remaining rows stay buffered for the next start");
        }
    }
    info!("uplink stopped");
}

fn next_delay(failures: u32, backlog: bool, drain_interval: Duration) -> Duration {
    if failures > 0 {
        let shift = (failures - 1).min(16);
        return BACKOFF_BASE
            .saturating_mul(1u32 << shift)
            .min(BACKOFF_MAX)
            .max(drain_interval);
    }
    if backlog {
        Duration::ZERO
    } else {
        drain_interval
    }
}

/// Publishes one batch. Returns how many rows were processed (published or discarded
/// as corrupt); rows stay buffered when publishing fails.
async fn drain_once(
    buffer: &BufferStore,
    publisher: &Publisher,
    batch_size: i64,
) -> Result<usize, ()> {
    let batch = buffer.dequeue_batch(batch_size).await.map_err(|err| {
        warn!(error = %err, "buffer dequeue failed");
    })?;
    if batch.is_empty() {
        return Ok(0);
    }

    let mut samples = Vec::with_capacity(batch.len());
    let mut valid_ids = Vec::with_capacity(batch.len());
    let mut corrupt_ids = Vec::new();
    for message in &batch {
        match serde_json::from_slice::<PollSample>(&message.payload) {
            Ok(sample) => {
                samples.push(sample);
                valid_ids.push(message.id);
            }
            Err(err) => {
                warn!(id = message.id, error = %err, "corrupt buffered sample, discarding");
                corrupt_ids.push(message.id);
            }
        }
    }

    // Corrupt rows can never be published; delete them so they cannot block the queue.
    if !corrupt_ids.is_empty() {
        counter!("sunspec_buffer_dropped_total", "reason" => "corrupt")
            .increment(corrupt_ids.len() as u64);
        if let Err(err) = buffer.delete_batch(&corrupt_ids).await {
            warn!(error = %err, "deleting corrupt rows failed");
        }
    }
    if samples.is_empty() {
        return Ok(batch.len());
    }

    let payload = publisher.serialize_batch(&samples).map_err(|err| {
        warn!(error = %err, "avro batch serialization failed");
        counter!("uplink_publish_error").increment(1);
    })?;
    let started = Instant::now();
    publisher
        .publish_bytes(publisher.topic(), &payload)
        .await
        .map_err(|err| {
            warn!(error = %err, "uplink publish failed; rows stay buffered");
            counter!("uplink_publish_error").increment(1);
        })?;
    histogram!("uplink_publish_latency").record(started.elapsed());
    counter!("uplink_messages_sent").increment(samples.len() as u64);

    // Acknowledge only after Kafka accepted the batch (at-least-once delivery).
    if let Err(err) = buffer.delete_batch(&valid_ids).await {
        warn!(error = %err, "deleting published rows failed; they may be sent again");
    }
    debug!(rows = batch.len(), "uplink batch published");
    Ok(batch.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drains_back_to_back_while_a_backlog_remains() {
        let interval = Duration::from_millis(500);
        assert_eq!(next_delay(0, true, interval), Duration::ZERO);
        assert_eq!(next_delay(0, false, interval), interval);
    }

    #[test]
    fn backs_off_on_failures_up_to_the_cap() {
        let interval = Duration::from_millis(500);
        assert_eq!(next_delay(1, true, interval), Duration::from_secs(1));
        assert_eq!(next_delay(3, false, interval), Duration::from_secs(4));
        assert_eq!(next_delay(20, false, interval), BACKOFF_MAX);
    }
}
