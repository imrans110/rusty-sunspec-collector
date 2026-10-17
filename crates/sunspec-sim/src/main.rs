use std::env;
use std::net::SocketAddr;
use std::process::ExitCode;

use sunspec_sim::{parse_models, read_dump, SimConfig, UnitSpec, Unmapped};
use types::DeviceDump;

const USAGE: &str =
    "usage: sunspec-sim [--bind ADDR:PORT] [--units 1,2] [--models 1,103,160:48] [--base 40000]
       sunspec-sim [--bind ADDR:PORT] --fixture DUMP.json   (replay a collector-app --probe dump)";

#[tokio::main]
async fn main() -> ExitCode {
    if env::args()
        .skip(1)
        .any(|arg| arg == "--version" || arg == "-V")
    {
        println!("sunspec-sim {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let config = match parse_args() {
        Ok(config) => config,
        Err(message) => {
            eprintln!("{message}\n{USAGE}");
            return ExitCode::from(2);
        }
    };

    let handle = match sunspec_sim::start(config.sim, config.bind).await {
        Ok(handle) => handle,
        Err(err) => {
            eprintln!("failed to start simulator on {}: {err}", config.bind);
            return ExitCode::FAILURE;
        }
    };
    println!("sunspec-sim listening on {}", handle.local_addr());
    match &config.fixture {
        Some(dump) => {
            handle.load_dump(dump.unit_id, dump);
            let models: Vec<String> = dump
                .models
                .iter()
                .map(|model| model.id.to_string())
                .collect();
            println!(
                "replaying {} unit {}: base {}, models {}",
                dump.host,
                dump.unit_id,
                dump.base_address,
                models.join(", ")
            );
        }
        None => sunspec_sim::run_demo(handle.clone(), config.unit_ids),
    }

    wait_for_shutdown().await;
    ExitCode::SUCCESS
}

struct Args {
    bind: SocketAddr,
    sim: SimConfig,
    unit_ids: Vec<u8>,
    /// A probe dump to replay instead of the demo.
    fixture: Option<DeviceDump>,
}

fn parse_args() -> Result<Args, String> {
    let mut bind: SocketAddr = "0.0.0.0:1502".parse().expect("valid default address");
    let mut units = vec![1u8];
    let mut models = parse_models("1,103,120,160:48")?;
    let mut base = 40_000u16;
    let mut fixture = None;
    let mut chain_set = false;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = || args.next().ok_or_else(|| format!("{arg} needs a value"));
        match arg.as_str() {
            "--bind" => {
                bind = value()?
                    .parse()
                    .map_err(|_| "--bind must be ADDR:PORT".to_string())?
            }
            "--units" => {
                units = value()?
                    .split(',')
                    .map(|unit| unit.trim().parse::<u8>())
                    .collect::<Result<_, _>>()
                    .map_err(|_| "--units must be a list of 0-255".to_string())?;
                chain_set = true;
            }
            "--models" => {
                models = parse_models(&value()?)?;
                chain_set = true;
            }
            "--fixture" => fixture = Some(read_dump(std::path::Path::new(&value()?))?),
            "--base" => {
                base = value()?
                    .parse()
                    .map_err(|_| "--base must be 0-65535".to_string())?;
                chain_set = true;
            }
            "--help" | "-h" => return Err(String::new()),
            other => return Err(format!("unknown argument {other}")),
        }
    }

    if let Some(dump) = fixture {
        if chain_set {
            return Err(
                "--fixture sets the unit, base and models; drop --units, --models and --base"
                    .to_string(),
            );
        }
        let unit = UnitSpec::from_dump(&dump)?;
        return Ok(Args {
            bind,
            sim: SimConfig::single(unit),
            unit_ids: vec![dump.unit_id],
            fixture: Some(dump),
        });
    }

    let unit_ids = units.clone();
    let units = units
        .into_iter()
        .map(|unit_id| UnitSpec {
            unit_id,
            base,
            models: models.clone(),
        })
        .collect();
    Ok(Args {
        bind,
        sim: SimConfig {
            units,
            unmapped: Unmapped::Exception,
        },
        unit_ids,
        fixture: None,
    })
}

async fn wait_for_shutdown() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        if let Ok(mut terminate) = signal(SignalKind::terminate()) {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {}
                _ = terminate.recv() => {}
            }
            return;
        }
    }
    let _ = tokio::signal::ctrl_c().await;
}
