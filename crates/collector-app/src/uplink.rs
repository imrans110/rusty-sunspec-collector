use std::time::Duration;

use metrics::{counter, gauge, histogram};
use tokio::sync::watch;
use tokio::time::sleep;
use tracing::{info, warn};

use avro_kafka::Publisher;
use buffer::BufferStore;
use poller_actor::PollSample;

const DEFAULT_UPLINK_BACKOFF_MS: u64 = 1_000;
const DEFAULT_UPLINK_BACKOFF_MAX_MS: u64 = 30_000;

pub async fn run(
    buffer: BufferStore,
    publisher: Publisher,
    mut shutdown: watch::Receiver<bool>,
    batch_size: i64,
    drain_interval: Duration,
) {
    let mut failure_count: u32 = 0;
    let mut total_sent: u64 = 0;
    let mut total_failed: u64 = 0;

    loop {
        let delay = uplink_delay(
            drain_interval,
            failure_count,
            Duration::from_millis(DEFAULT_UPLINK_BACKOFF_MS),
            Duration::from_millis(DEFAULT_UPLINK_BACKOFF_MAX_MS),
        );

        tokio::select! {
            _ = sleep(delay) => {
                let batch = match buffer.dequeue_batch(batch_size).await {
                    Ok(batch) => batch,
                    Err(err) => {
                        warn!(error = %err, "buffer dequeue failed");
                        failure_count = failure_count.saturating_add(1);
                        total_failed = total_failed.saturating_add(1);
                        continue;
                    }
                };

                if batch.is_empty() {
                    failure_count = 0;
                    continue;
                }

                let mut samples = Vec::with_capacity(batch.len());
                let mut valid_ids = Vec::with_capacity(batch.len());
                let mut corrupt_ids = Vec::new();

                // Deserialization phase: separate valid from corrupt
                for message in &batch {
                    match serde_json::from_slice::<PollSample>(&message.payload) {
                        Ok(sample) => {
                            samples.push(sample);
                            valid_ids.push(message.id);
                        }
                        Err(err) => {
                             warn!(id = message.id, error = %err, "json deserialize failed, discarding");
                             corrupt_ids.push(message.id);
                        }
                    }
                }

                // Always delete corrupt messages to prevent head-of-line blocking
                if !corrupt_ids.is_empty() {
                    if let Err(err) = buffer.delete_batch(&corrupt_ids).await {
                        warn!(error = %err, "buffer delete corrupt failed");
                    }
                }

                let valid_count = samples.len();
                let mut publish_failed = false;

                if !samples.is_empty() {
                    match publisher.serialize_batch(&samples) {
                        Ok(avro_payload) => {
                             let start = std::time::Instant::now();
                             match publisher.publish_bytes(publisher.topic(), &avro_payload).await {
                                 Ok(()) => {
                                     let duration = start.elapsed();
                                     histogram!("uplink_publish_latency").record(duration);
                                     counter!("uplink_messages_sent", "batch_size" => valid_count.to_string()).increment(valid_count as u64);
                                 }
                                 Err(err) => {
                                     warn!(error = %err, "uplink publish batch failed");
                                     publish_failed = true;
                                     counter!("uplink_publish_error").increment(1);
                                 }
                             }
                        }
                        Err(err) => {
                            warn!(error = %err, "avro batch serialization failed");
                            publish_failed = true;
                        }
                    }
                }

                if publish_failed {
                    // Valid messages stay in buffer for retry next cycle
                    failure_count = failure_count.saturating_add(1);
                    total_failed = total_failed.saturating_add(valid_count as u64);
                } else {
                    // Ack only valid messages on successful publish
                    if !valid_ids.is_empty() {
                        if let Err(err) = buffer.delete_batch(&valid_ids).await {
                            warn!(error = %err, "buffer delete failed");
                        }
                    }

                    total_sent = total_sent.saturating_add(valid_count as u64);
                    failure_count = 0;
                }

                let queue_depth = match buffer.pending_count().await {
                    Ok(count) => {
                        gauge!("buffer_size").set(count as f64);
                        Some(count)
                    }
                    Err(err) => {
                        warn!(error = %err, "buffer count failed");
                        None
                    }
                };

                info!(
                    batch_size = batch.len(),
                    valid_samples = valid_count,
                    queue_depth = queue_depth.unwrap_or(-1),
                    total_sent,
                    total_failed,
                    failure_count,
                    next_delay_ms = delay.as_millis(),
                    "uplink drain complete"
                );
            }
            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    info!("uplink shutdown requested");
                    break;
                }
            }
        }
    }
}

fn uplink_delay(
    base: Duration,
    failures: u32,
    backoff_base: Duration,
    backoff_max: Duration,
) -> Duration {
    if failures == 0 {
        return base;
    }

    let shift = failures.saturating_sub(1).min(31);
    let factor = 1u32 << shift;
    let candidate = backoff_base.saturating_mul(factor);
    let backoff = if candidate > backoff_max {
        backoff_max
    } else {
        candidate
    };
    if backoff > base {
        backoff
    } else {
        base
    }
}
