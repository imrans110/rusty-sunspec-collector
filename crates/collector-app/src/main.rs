use std::env;

use anyhow::{Context, Result};
use metrics_exporter_prometheus::PrometheusBuilder;
use tracing::warn;

use collector_app::CollectorConfig;

#[tokio::main]
async fn main() -> Result<()> {
    if env::args().any(|arg| arg == "--print-schema") {
        println!("{}", avro_kafka::SCHEMA_JSON);
        return Ok(());
    }
    tracing_subscriber::fmt::init();

    // Install signal handlers first so a stop during startup is still graceful.
    let shutdown = tokio::spawn(shutdown_signal());

    let config =
        CollectorConfig::load_with_path(parse_config_arg()).context("load config failed")?;
    config.validate().context("config validation failed")?;

    let metrics = PrometheusBuilder::new()
        .install_recorder()
        .context("failed to install metrics recorder")?;

    collector_app::run(config, Some(metrics), async {
        let _ = shutdown.await;
    })
    .await
}

/// Resolves on SIGINT (Ctrl-C) or SIGTERM, which systemd and Docker send on stop.
async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        match signal(SignalKind::terminate()) {
            Ok(mut terminate) => {
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {}
                    _ = terminate.recv() => {}
                }
                return;
            }
            Err(err) => warn!(error = %err, "failed to install SIGTERM handler"),
        }
    }
    let _ = tokio::signal::ctrl_c().await;
}

fn parse_config_arg() -> Option<String> {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--config" {
            return args.next();
        }
        if let Some(path) = arg.strip_prefix("--config=") {
            return Some(path.to_string());
        }
    }
    None
}
