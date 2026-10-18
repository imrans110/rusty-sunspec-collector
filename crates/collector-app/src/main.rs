use std::env;
use std::io::IsTerminal;
use std::process::ExitCode;

use anyhow::{bail, Context, Result};
use tracing::warn;
use tracing_subscriber::EnvFilter;

use collector_app::{CollectorConfig, ConfigError};

/// Exit status for an error in the settings (`EX_CONFIG` from sysexits.h). The systemd
/// unit sets `RestartPreventExitStatus=78`, so a bad setting stops the service once
/// instead of restarting it every few seconds.
const EXIT_CONFIG: u8 = 78;

const USAGE: &str = "\
usage: collector-app [--config PATH]
       collector-app --probe HOST[:UNIT] [--keep-identifiers] [--config PATH]
       collector-app --print-schema | --print-models | --version | --help

--probe reads one device's SunSpec map once and prints it as JSON (for device reports and
sunspec-sim --fixture). Serial numbers and network addresses are blanked unless
--keep-identifiers is given. The port, timeouts and base address come from the config.
--print-models prints the metadata of every bundled SunSpec model (labels, units,
descriptions, enum and bitfield symbols) as JSON.

Settings come from the config file (--config or SUNSPEC_CONFIG) and SUNSPEC_* environment
variables; see https://imrans110.github.io/rusty-sunspec-collector/configuration.html";

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            if err.downcast_ref::<ConfigError>().is_some() {
                ExitCode::from(EXIT_CONFIG)
            } else {
                ExitCode::FAILURE
            }
        }
    }
}

async fn run() -> Result<()> {
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--print-schema" => {
                println!("{}", avro_kafka::SCHEMA_JSON);
                return Ok(());
            }
            "--print-models" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&sunspec_parser::catalog())?
                );
                return Ok(());
            }
            "--version" | "-V" => {
                println!("collector-app {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "--help" | "-h" => {
                println!("{USAGE}");
                return Ok(());
            }
            _ => {}
        }
    }
    if let Some(target) = arg_value("--probe") {
        return run_probe(&target).await;
    }
    init_logging().context(ConfigError("logging setup failed"))?;

    // Install signal handlers first so a stop during startup is still graceful.
    let shutdown = tokio::spawn(shutdown_signal());

    let config = load_config()?;

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
    arg_value("--config")
}

fn load_config() -> Result<CollectorConfig> {
    let config = CollectorConfig::load_with_path(parse_config_arg())
        .context(ConfigError("load config failed"))?;
    config
        .validate()
        .context(ConfigError("config validation failed"))?;
    Ok(config)
}

/// The value of `--name VALUE` or `--name=VALUE`.
fn arg_value(name: &str) -> Option<String> {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == name {
            return args.next();
        }
        if let Some(value) = arg
            .strip_prefix(name)
            .and_then(|rest| rest.strip_prefix('='))
        {
            return Some(value.to_string());
        }
    }
    None
}

/// Prints the dump as JSON on stdout; progress and warnings go to stderr, so the output
/// can be redirected to a file.
async fn run_probe(target: &str) -> Result<()> {
    let identity = collector_app::config::parse_device(target).context("--probe")?;
    let config = load_config()?;
    let keep_identifiers = env::args().any(|arg| arg == "--keep-identifiers");

    let dump = collector_app::probe::probe(&config, identity, keep_identifiers).await?;
    println!("{}", serde_json::to_string_pretty(&dump)?);

    let models: Vec<String> = dump
        .models
        .iter()
        .map(|model| model.id.to_string())
        .collect();
    eprintln!(
        "{}/{}: SunSpec map at {}, models {}",
        dump.host,
        dump.unit_id,
        dump.base_address,
        models.join(", ")
    );
    for model in dump.models.iter().filter(|model| model.error.is_some()) {
        eprintln!(
            "warning: model {} could not be read: {}",
            model.id,
            model.error.as_deref().unwrap_or_default()
        );
    }
    if !dump.redacted.is_empty() {
        eprintln!(
            "blanked {} (use --keep-identifiers to keep them)",
            dump.redacted.join(", ")
        );
    }
    Ok(())
}
