use std::sync::Arc;
use std::time::{Duration, Instant};

use metrics::{counter, histogram};
use tokio::sync::watch;
use tokio::time::{sleep, timeout};
use tracing::{debug, info, warn};

use avro_kafka::{PublishError, Publisher};
use buffer::BufferStore;
use poller_actor::Sample;

use crate::health::Health;

const BACKOFF_BASE: Duration = Duration::from_secs(1);
const BACKOFF_MAX: Duration = Duration::from_secs(30);
/// While backing off from records that timed out, how often to ask the brokers whether
/// they answer again.
const PROBE_INTERVAL: Duration = Duration::from_secs(2);
const PROBE_TIMEOUT: Duration = Duration::from_secs(1);

/// Why a drain delivered nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DrainFailure {
    /// The buffer could not be read.
    Buffer,
    /// Kafka rejected records.
    Rejected,
    /// Kafka never answered for any record within `kafka.timeout_ms`.
    TimedOut,
}

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
    health: Arc<Health>,
) {
    let mut failures = 0u32;
    let mut last_failure = None;
    let mut backlog = false;

    loop {
        health.uplink.beat();
        let delay = next_delay(failures, backlog, config.drain_interval);
        let probe = last_failure == Some(DrainFailure::TimedOut);
        tokio::select! {
            _ = pause(delay, probe, &publisher) => {}
            _ = stop.wait_for(|stop| *stop) => break,
        }
        match drain_once(&buffer, &publisher, config.batch_size).await {
            Ok(processed) => {
                if processed > 0
                    && matches!(
                        last_failure,
                        Some(DrainFailure::Rejected | DrainFailure::TimedOut)
                    )
                {
                    info!(failed_attempts = failures, "records delivered again");
                }
                failures = 0;
                last_failure = None;
                backlog = processed as i64 >= config.batch_size;
            }
            Err(failure) => {
                failures = failures.saturating_add(1);
                last_failure = Some(failure);
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

/// Waits `delay` before the next drain. With `probe`, after records timed out while
/// broker connections were failing, it asks the brokers every [`PROBE_INTERVAL`] whether
/// they answer and returns as soon as they do, so a backlog starts draining within
/// seconds of an outage ending rather than after the rest of the backoff (up to 30 s).
async fn pause(delay: Duration, probe: bool, publisher: &Publisher) {
    if !probe || !publisher.brokers_unreachable() {
        sleep(delay).await;
        return;
    }
    let deadline = tokio::time::Instant::now() + delay;
    loop {
        let left = deadline.saturating_duration_since(tokio::time::Instant::now());
        if left <= PROBE_INTERVAL {
            sleep(left).await;
            return;
        }
        sleep(PROBE_INTERVAL).await;
        if publisher.probe(PROBE_TIMEOUT).await {
            debug!("Kafka brokers answered; draining before the backoff ends");
            return;
        }
    }
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

/// Publishes one batch, one Kafka record per sample. Returns how many rows were
/// processed (acknowledged by Kafka, or discarded because they can never be sent).
/// Rows Kafka did not acknowledge stay buffered and are retried first next time.
async fn drain_once(
    buffer: &BufferStore,
    publisher: &Publisher,
    batch_size: i64,
) -> Result<usize, DrainFailure> {
    let batch = buffer.dequeue_batch(batch_size).await.map_err(|err| {
        warn!(error = %err, "buffer dequeue failed");
        DrainFailure::Buffer
    })?;
    if batch.is_empty() {
        return Ok(0);
    }

    let mut samples = Vec::with_capacity(batch.len());
    let mut sample_ids = Vec::with_capacity(batch.len());
    let mut corrupt_ids = Vec::new();
    for message in &batch {
        match serde_json::from_slice::<Sample>(&message.payload) {
            Ok(sample) => {
                samples.push(sample);
                sample_ids.push(message.id);
            }
            Err(err) => {
                warn!(id = message.id, error = %err, "corrupt buffered sample, discarding");
                corrupt_ids.push(message.id);
            }
        }
    }

    let started = Instant::now();
    let results = publisher.publish(&samples).await;
    let mut acked = Vec::with_capacity(samples.len());
    let mut undelivered = 0usize;
    let mut all_timed_out = true;
    let mut last_error = None;
    for (id, result) in sample_ids.into_iter().zip(results) {
        match result {
            Ok(()) => acked.push(id),
            // Encoding failures are deterministic: retrying can never succeed.
            Err(err @ (PublishError::Avro(_) | PublishError::Schema(_))) => {
                warn!(id, error = %err, "sample cannot be encoded, discarding");
                corrupt_ids.push(id);
            }
            Err(err) => {
                undelivered += 1;
                all_timed_out &= err.is_timeout();
                last_error = Some(err);
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
    if !acked.is_empty() {
        histogram!("sunspec_uplink_publish_seconds").record(started.elapsed());
        counter!("sunspec_uplink_records_total").increment(acked.len() as u64);
        // Delete only what Kafka acknowledged (at-least-once delivery).
        if let Err(err) = buffer.delete_batch(&acked).await {
            warn!(error = %err, "deleting published rows failed; they may be sent again");
        }
    }
    if let Some(err) = last_error {
        counter!("sunspec_uplink_errors_total").increment(undelivered as u64);
        warn!(undelivered, error = %err, "records not delivered; they stay buffered");
        if acked.is_empty() {
            return Err(if all_timed_out {
                DrainFailure::TimedOut
            } else {
                DrainFailure::Rejected
            });
        }
    }
    debug!(
        rows = batch.len(),
        acked = acked.len(),
        "uplink batch processed"
    );
    Ok(acked.len() + corrupt_ids.len())
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
