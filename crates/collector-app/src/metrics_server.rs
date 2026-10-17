use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Json, Router};
use metrics::{describe_counter, describe_gauge, describe_histogram, Unit};
use metrics_exporter_prometheus::{BuildError, Matcher, PrometheusBuilder, PrometheusHandle};
use tokio::sync::watch;
use tracing::{info, warn};

use crate::health::Health;

/// Bucket bounds (seconds) for every `*_seconds` histogram.
const LATENCY_BUCKETS: &[f64] = &[
    0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
];

/// Installs the global Prometheus recorder, with real histograms for `*_seconds`
/// metrics, and registers help text for every metric the collector emits.
pub fn install_recorder() -> Result<PrometheusHandle, BuildError> {
    let handle = PrometheusBuilder::new()
        .set_buckets_for_metric(Matcher::Suffix("_seconds".to_string()), LATENCY_BUCKETS)?
        .install_recorder()?;
    describe_metrics();
    Ok(handle)
}

fn describe_metrics() {
    describe_counter!(
        "sunspec_poll_success_total",
        "Successful model reads, by device."
    );
    describe_counter!(
        "sunspec_poll_errors_total",
        "Failed model reads, by device and kind (exception, timeout, transport, connect)."
    );
    describe_histogram!(
        "sunspec_poll_duration_seconds",
        Unit::Seconds,
        "Time to read every model of a device once."
    );
    describe_gauge!(
        "sunspec_device_up",
        "1 while a device is being polled successfully."
    );
    describe_gauge!(
        "sunspec_device_last_success_timestamp_seconds",
        Unit::Seconds,
        "Unix time of a device's last successful read."
    );
    describe_counter!(
        "sunspec_poller_restarts_total",
        "Device sessions restarted after a failure."
    );
    describe_counter!(
        "sunspec_poller_stalls_total",
        "Device sessions restarted because no read succeeded within the stall timeout."
    );
    describe_gauge!("sunspec_devices_known", "Devices being supervised.");
    describe_counter!(
        "sunspec_samples_dropped_total",
        "Samples dropped before reaching the buffer, by reason."
    );
    describe_counter!(
        "sunspec_buffer_writes_total",
        "Samples written to the buffer."
    );
    describe_counter!(
        "sunspec_buffer_dropped_total",
        "Buffered rows dropped without being published, by reason."
    );
    describe_gauge!("sunspec_buffer_rows", "Rows waiting in the buffer.");
    describe_gauge!(
        "sunspec_buffer_oldest_age_seconds",
        Unit::Seconds,
        "Age of the oldest buffered row."
    );
    describe_counter!(
        "sunspec_uplink_records_total",
        "Records acknowledged by Kafka."
    );
    describe_counter!(
        "sunspec_uplink_errors_total",
        "Records Kafka did not acknowledge (they stay buffered)."
    );
    describe_histogram!(
        "sunspec_uplink_publish_seconds",
        Unit::Seconds,
        "Time to publish one batch and receive acknowledgements."
    );
}

/// Serves `/healthz` and, when a recorder is installed, `/metrics` until shutdown.
pub async fn serve(
    handle: Option<PrometheusHandle>,
    health: Arc<Health>,
    mut shutdown: watch::Receiver<bool>,
    port: u16,
) {
    let mut app = Router::new().route(
        "/healthz",
        get(move || {
            let report = health.report();
            async move {
                let status = if report.healthy {
                    StatusCode::OK
                } else {
                    StatusCode::SERVICE_UNAVAILABLE
                };
                (status, Json(report)).into_response()
            }
        }),
    );
    if let Some(handle) = handle {
        app = app.route("/metrics", get(move || async move { handle.render() }));
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            warn!(%addr, error = %err, "failed to bind metrics port");
            return;
        }
    };
    if let Ok(addr) = listener.local_addr() {
        info!(%addr, "serving /metrics and /healthz");
    }

    if let Err(err) = axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = shutdown.wait_for(|stop| *stop).await;
        })
        .await
    {
        warn!(error = %err, "metrics server error");
    }
}
