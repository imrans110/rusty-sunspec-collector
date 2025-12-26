#![allow(dead_code)]

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use thiserror::Error;
use tokio::sync::{mpsc, watch};
use tokio::time::sleep;
use tracing::{info, warn};

use modbus_client::{ClientConfig, ClientError, ModbusClient};
use metrics::counter;
use serde::{Deserialize, Serialize};
use sunspec_parser::ModelDefinition;
use types::DeviceIdentity;

#[derive(Debug, Clone)]
pub struct ActorConfig {
    pub poll_interval: Duration,
    pub request_timeout: Duration,
    pub jitter_ms: u64,
}

impl Default for ActorConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(1),
            request_timeout: Duration::from_secs(1),
            jitter_ms: 0,
        }
    }
}

#[derive(Debug, Error)]
pub enum PollerError {
    #[error("failed to connect to modbus device: {0}")]
    Connect(#[from] modbus_client::ClientError),
    #[error("too many consecutive errors ({0})")]
    TooManyErrors(u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PollSample {
    pub device: DeviceIdentity,
    pub model_id: u16,
    pub model_name: String,
    pub start: u16,
    pub registers: Vec<u16>,
    pub collected_at_ms: u64,
}

pub struct PollerActor {
    identity: DeviceIdentity,
    modbus_config: ClientConfig,
    models: Vec<ModelDefinition>,
    sender: mpsc::Sender<PollSample>,
    shutdown: watch::Receiver<bool>,
    config: ActorConfig,
}

const MAX_CONSECUTIVE_ERRORS: u32 = 10;

impl PollerActor {
    pub fn new(
        identity: DeviceIdentity,
        modbus_config: ClientConfig,
        models: Vec<ModelDefinition>,
        sender: mpsc::Sender<PollSample>,
        shutdown: watch::Receiver<bool>,
        config: ActorConfig,
    ) -> Self {
        Self {
            identity,
            modbus_config,
            models,
            sender,
            shutdown,
            config,
        }
    }

    pub async fn run(mut self) -> Result<(), PollerError> {
        let mut modbus_config = self.modbus_config.clone();
        modbus_config.timeout_ms = self.config.request_timeout.as_millis() as u64;
        let client = ModbusClient::connect(modbus_config).await?;
        let mut iteration = 0u64;
        let mut consecutive_errors = 0u32;

        loop {
            if *self.shutdown.borrow() {
                info!(ip = %self.identity.ip, "poller shutdown requested");
                break;
            }

            let cycle_start = Instant::now();
            let mut timeout_count = 0u64;
            let mut cycle_had_error = false;

            for model in &self.models {
                if model.length == 0 {
                    continue;
                }

                match client
                    .read_range(self.identity.unit_id, model.start, model.length)
                    .await
                {
                    Ok(registers) => {
                        // Reset error counter on successful read (at least partial success keeps us alive)
                        if consecutive_errors > 0 {
                             info!(ip = %self.identity.ip, "connection recovered");
                             consecutive_errors = 0;
                        }

                        let sample = PollSample {
                            device: self.identity.clone(),
                            model_id: model.id,
                            model_name: model.name.clone(),
                            start: model.start,
                            registers,
                            collected_at_ms: unix_ms(),
                        };

                        if let Err(err) = self.sender.send(sample).await {
                             warn!(
                                ip = %self.identity.ip,
                                unit_id = self.identity.unit_id,
                                model_id = model.id,
                                error = %err,
                                "telemetry channel send failed"
                            );
                            counter!("poller_error", "ip" => self.identity.ip.clone(), "type" => "channel").increment(1);
                        } else {
                            counter!("poller_success", "ip" => self.identity.ip.clone()).increment(1);
                        }
                    }
                    Err(err) => {
                        cycle_had_error = true;
                        if matches!(err, ClientError::Timeout { .. }) {
                            timeout_count += 1;
                        }
                        warn!(
                            ip = %self.identity.ip,
                            unit_id = self.identity.unit_id,
                            model_id = model.id,
                            error = %err,
                            "modbus read failed"
                        );
                        counter!("poller_error", "ip" => self.identity.ip.clone(), "type" => "modbus").increment(1);
                    }
                }
            }

            if cycle_had_error {
                consecutive_errors += 1;
                if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
                    warn!(ip = %self.identity.ip, errors = consecutive_errors, "max errors exceeded, exiting");
                    return Err(PollerError::TooManyErrors(consecutive_errors));
                }
            }

            iteration = iteration.wrapping_add(1);
            let elapsed = cycle_start.elapsed();
            let lag = elapsed.saturating_sub(self.config.poll_interval);
            let delay = jittered_delay(self.config.poll_interval, self.config.jitter_ms, iteration);
            info!(
                ip = %self.identity.ip,
                unit_id = self.identity.unit_id,
                elapsed_ms = elapsed.as_millis(),
                lag_ms = lag.as_millis(),
                timeout_count,
                consecutive_errors,
                delay_ms = delay.as_millis(),
                "poll cycle complete"
            );

            tokio::select! {
                _ = sleep(delay) => {},
                _ = self.shutdown.changed() => {
                    if *self.shutdown.borrow() {
                        info!(ip = %self.identity.ip, "poller shutdown requested");
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}

fn jittered_delay(base: Duration, jitter_ms: u64, iteration: u64) -> Duration {
    if jitter_ms == 0 {
        return base;
    }

    let jitter_window = jitter_ms.max(1);
    let seed = unix_ms().wrapping_add(iteration.wrapping_mul(1_664_525));
    let offset = seed % jitter_window;
    base + Duration::from_millis(offset)
}

fn unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
