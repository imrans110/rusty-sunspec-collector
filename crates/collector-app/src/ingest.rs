use metrics::counter;
use tokio::sync::mpsc;
use tracing::{info, warn};

use buffer::BufferStore;
use poller_actor::PollSample;

/// Persists samples from the pollers into the buffer. Runs until every sender is
/// dropped, so samples still queued at shutdown are written before it returns.
pub async fn run(mut rx: mpsc::Receiver<PollSample>, buffer: BufferStore, topic: String) {
    while let Some(sample) = rx.recv().await {
        // Store lightweight JSON in the buffer; Avro is produced at uplink time.
        match serde_json::to_vec(&sample) {
            Ok(payload) => {
                if let Err(err) = buffer.enqueue(&topic, &payload).await {
                    warn!(error = %err, "buffer enqueue failed");
                    counter!("buffer_enqueue_error").increment(1);
                } else {
                    counter!("buffer_enqueue_success").increment(1);
                }
            }
            Err(err) => warn!(error = %err, "json serialization failed"),
        }
    }
    info!("ingest stopped");
}
