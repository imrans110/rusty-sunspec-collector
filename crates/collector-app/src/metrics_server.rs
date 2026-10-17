use std::future;
use std::net::SocketAddr;

use axum::{routing::get, Router};
use metrics_exporter_prometheus::PrometheusHandle;
use tokio::sync::watch;
use tracing::{info, warn};

pub async fn serve(handle: PrometheusHandle, mut shutdown: watch::Receiver<bool>, port: u16) {
    let app = Router::new().route("/metrics", get(move || future::ready(handle.render())));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!(%addr, "metrics server listening");

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            warn!(error = %e, "failed to bind metrics port");
            return;
        }
    };

    if let Err(err) = axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = shutdown.changed().await;
        })
        .await
    {
        warn!(error = %err, "metrics server error");
    }
}
