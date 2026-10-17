use std::env;

use std::io::IsTerminal;

use anyhow::{bail, Context, Result};
use tracing::warn;
use tracing_subscriber::EnvFilter;

use collector_app::CollectorConfig;

#[tokio::main]
async fn main() -> Result<()> {
    if env::args().any(|arg| arg == "--print-schema") {
        println!("{}", avro_kafka::SCHEMA_JSON);
        return Ok(());
    }
    init_logging()?;

    // Install signal handlers first so a stop during startup is still graceful.
    let shutdown = tokio::spawn(shutdown_signal());

    let config =
        CollectorConfig::load_with_path(parse_config_arg()).context("load config failed")?;
    config.validate().context("config validation failed")?;

    let metrics = collector_app::metrics_server::install_recorder()
        .context("failed to install metrics recorder")?;

    collector_app::run(config, Some(metrics), async {
        let _ = shutdown.await;
    })
    .await
}

/// `RUST_LOG` filters (default `info`); `SUNSPEC_LOG_FORMAT=json` emits one JSON
/// object per line for log shippers. Colours are used only on a terminal.
fn init_logging() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let builder = tracing_subscriber::fmt().with_env_filter(filter);
    match env::var("SUNSPEC_LOG_FORMAT").as_deref() {
        Ok("json") => builder.json().init(),
        Ok("text") | Err(_) => builder.with_ansi(std::io::stdout().is_terminal()).init(),
        Ok(other) => bail!("SUNSPEC_LOG_FORMAT must be text or json, got {other:?}"),
    }
    Ok(())
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
