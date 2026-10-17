use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Json, Router};
use metrics::{describe_counter, describe_gauge, describe_histogram, Unit};
use metrics_exporter_prometheus::{BuildError, Matcher, PrometheusBuilder, PrometheusHandle};
use tokio::sync::watch;
use tracing::{info, warn};

use crate::health::Health;
use types::DeviceIdentity;

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
        "Failed model reads, by device and kind (exception, timeout, transport, connect, address)."
    );
    describe_histogram!(
        "sunspec_poll_duration_seconds",
        Unit::Seconds,
        "Time to read the models due in one poll cycle of a device."
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
        "Record delivery attempts Kafka did not acknowledge (records stay buffered and are retried, so a record can count more than once)."
    );
    describe_histogram!(
        "sunspec_uplink_publish_seconds",
        Unit::Seconds,
        "Time to publish one batch and receive acknowledgements."
    );
}

/// Binds the HTTP port up front so a port conflict fails startup instead of leaving the
/// collector running without `/metrics` and `/healthz`.
pub async fn bind(port: u16) -> std::io::Result<tokio::net::TcpListener> {
    tokio::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).await
}

/// Serves `/healthz`, `/devices`, `/models` and, when a recorder is installed,
/// `/metrics` until shutdown.
pub async fn serve(
    listener: tokio::net::TcpListener,
    handle: Option<PrometheusHandle>,
    health: Arc<Health>,
    mut shutdown: watch::Receiver<bool>,
) {
    let devices = health.clone();
    let device = health.clone();
    let mut app = Router::new()
        .route(
            "/devices",
            get(move || {
                let devices = devices.devices();
                async move { Json(serde_json::json!({ "devices": devices })) }
            }),
        )
        .route(
            "/devices/{host}/{unit}",
            get(move |Path((host, unit_id)): Path<(String, u8)>| {
                let identity = DeviceIdentity { ip: host, unit_id };
                let view = device.device(&identity);
                async move {
                    match view {
                        Some(view) => Json(view).into_response(),
                        None => (
                            StatusCode::NOT_FOUND,
                            Json(serde_json::json!({
                                "error": format!("unknown device {}", identity.key())
                            })),
                        )
                            .into_response(),
                    }
                }
            }),
        );
    app = app
        .route(
            "/models",
            get(|| async {
                let index: Vec<serde_json::Value> = sunspec_parser::catalog()
                    .into_iter()
                    .map(|model| {
                        serde_json::json!({ "id": model.id, "name": model.name, "label": model.label })
                    })
                    .collect();
                Json(serde_json::json!({ "models": index }))
            }),
        )
        .route(
            "/models/{id}",
            get(|Path(id): Path<u16>| async move {
                match sunspec_parser::model_info(id) {
                    Some(model) => Json(model).into_response(),
                    None => (
                        StatusCode::NOT_FOUND,
                        Json(serde_json::json!({ "error": format!("no bundled definition for model {id}") })),
                    )
                        .into_response(),
                }
            }),
        );
    app = app.route(
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

    if let Ok(addr) = listener.local_addr() {
        info!(%addr, "serving /metrics, /healthz, /devices and /models");
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
