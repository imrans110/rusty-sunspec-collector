use std::collections::HashMap;
use std::env;
use std::time::Duration;

use anyhow::{Context, Result};
use tokio::sync::{mpsc, watch};
use tokio::task::JoinSet;
use tokio::time::{sleep, timeout};
use tracing::{info, warn};

use axum::{routing::get, Router};
use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::future;
use std::net::SocketAddr;

use avro_kafka::{KafkaConfig, Publisher};
use buffer::BufferStore;
use collector_app::CollectorConfig;
use discovery::discover;
use modbus_client::{ClientConfig, ModbusClient};
use poller_actor::{ActorConfig, PollerActor, PollerError, PollSample};
use sunspec_parser::{parse_models_from_registers_lenient, ModelDefinition};
use types::DeviceIdentity;

const DEFAULT_UPLINK_BACKOFF_MS: u64 = 1_000;
const DEFAULT_UPLINK_BACKOFF_MAX_MS: u64 = 30_000;
const MAX_RESPAWN_DELAY_MS: u64 = 60_000;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config_path = parse_config_arg();
    let config = CollectorConfig::load_with_path(config_path).context("load config failed")?;
    config.validate().context("config validation failed")?;
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let builder = PrometheusBuilder::new();
    let handle = builder
        .install_recorder()
        .context("failed to install metrics recorder")?;
    let _metrics_handle = tokio::spawn(metrics_task(handle, shutdown_rx.clone(), config.metrics_port));

    let devices = discover(config.discovery.clone())
        .await
        .context("device discovery failed")?;
    if devices.is_empty() {
        warn!("no devices discovered");
    }

    let (tx, rx) = mpsc::channel(config.channel_capacity);
    let publisher = if let Some(brokers) = config.kafka_brokers.clone() {
        let mut kafka_config = KafkaConfig::default();
        kafka_config.brokers = brokers;
        kafka_config.client_id = config
            .kafka_client_id
            .clone()
            .unwrap_or_else(|| "sunspec-collector".to_string());
        kafka_config.acks = config.kafka_acks.clone().unwrap_or_else(|| "all".to_string());
        kafka_config.compression = config
            .kafka_compression
            .clone()
            .unwrap_or_else(|| "zstd".to_string());
        kafka_config.message_timeout_ms = config.kafka_timeout_ms.unwrap_or(5_000);
        if let Some(enable_idempotence) = config.kafka_enable_idempotence {
            kafka_config.enable_idempotence = enable_idempotence;
        }

        Publisher::new_kafka(
            Publisher::default_schema(),
            config.kafka_topic.clone().unwrap_or_else(|| "sunspec.telemetry".to_string()),
            kafka_config,
        )
        .context("kafka publisher init failed")?
    } else {
        Publisher::new_mock(Publisher::default_schema(), "sunspec.telemetry")
    };
    let buffer = BufferStore::new(&config.buffer_path)
        .await
        .context("buffer init failed")?;
    let buffer_handle = tokio::spawn(buffer_task(
        rx,
        buffer.clone(),
        publisher.clone(),
        shutdown_rx.clone(),
    ));
    let uplink_handle = tokio::spawn(uplink_task(
        buffer.clone(),
        publisher.clone(),
        shutdown_rx.clone(),
        config.buffer_batch_size,
        Duration::from_millis(config.buffer_drain_interval_ms),
    ));

    let specs = build_poller_specs(&config, &devices, tx.clone(), shutdown_rx.clone()).await;

    let mut join_set = JoinSet::new();
    let mut respawn_counts: HashMap<String, u32> = HashMap::new();
    for spec in specs.values() {
        spawn_poller(spec.clone(), &mut join_set, Duration::from_millis(0));
    }

    notify_ready();
    let watchdog_handle = start_watchdog(shutdown_rx.clone());

    let mut shutdown_signal = tokio::signal::ctrl_c();
    loop {
        tokio::select! {
            _ = &mut shutdown_signal => {
                info!("shutdown signal received");
                let _ = shutdown_tx.send(true);
                break;
            }
            maybe_result = join_set.join_next() => {
                if let Some(result) = maybe_result {
                    match result {
                        Ok((id, outcome)) => {
                            if let Err(ref err) = outcome {
                                warn!(device = %id, error = %err, "poller exited with error");
                            } else {
                                info!(device = %id, "poller exited cleanly");
                            }
                            if let Some(spec) = specs.get(&id) {
                                let count = respawn_counts.entry(id.clone()).or_insert(0);
                                let delay = if outcome.is_err() {
                                    *count = count.saturating_add(1);
                                    respawn_delay(config.respawn_delay_ms, *count)
                                } else {
                                    *count = 0;
                                    Duration::from_millis(config.respawn_delay_ms)
                                };
                                info!(device = %id, delay_ms = delay.as_millis(), respawn_count = *count, "respawning poller");
                                spawn_poller(
                                    spec.clone(),
                                    &mut join_set,
                                    delay,
                                );
                            }
                        }
                        Err(err) => {
                            warn!(error = %err, "poller task failed");
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    join_set.abort_all();
    while let Some(result) = join_set.join_next().await {
        if let Err(err) = result {
            warn!(error = %err, "poller task join failed");
        }
    }

    let _ = buffer_handle.await;
    let _ = uplink_handle.await;
    if let Some(handle) = watchdog_handle {
        let _ = handle.await;
    }
    Ok(())
}

#[derive(Clone)]
struct PollerSpec {
    identity: DeviceIdentity,
    modbus_config: ClientConfig,
    models: Vec<ModelDefinition>,
    poller_config: ActorConfig,
    sender: mpsc::Sender<PollSample>,
    shutdown: watch::Receiver<bool>,
}

async fn build_poller_specs(
    config: &CollectorConfig,
    devices: &[DeviceIdentity],
    sender: mpsc::Sender<PollSample>,
    shutdown: watch::Receiver<bool>,
) -> HashMap<String, PollerSpec> {
    let mut specs = HashMap::new();

    for device in devices {
        match discover_models_for_device(config, device).await {
            Ok(models) if models.is_empty() => {
                warn!(ip = %device.ip, "no models discovered");
            }
            Ok(models) => {
                let mut modbus_config = config.modbus.clone();
                modbus_config.host = device.ip.clone();

                let spec = PollerSpec {
                    identity: device.clone(),
                    modbus_config,
                    models,
                    poller_config: config.poller.clone(),
                    sender: sender.clone(),
                    shutdown: shutdown.clone(),
                };
                specs.insert(device.ip.clone(), spec);
            }
            Err(err) => {
                warn!(ip = %device.ip, error = %err, "model discovery failed");
            }
        }
    }

    specs
}

fn spawn_poller(
    spec: PollerSpec,
    join_set: &mut JoinSet<(String, Result<(), PollerError>)>,
    delay: Duration,
) {
    let identity = spec.identity.clone();
    join_set.spawn(async move {
        if delay > Duration::from_millis(0) {
            sleep(delay).await;
        }
        let actor = PollerActor::new(
            spec.identity,
            spec.modbus_config,
            spec.models,
            spec.sender,
            spec.shutdown,
            spec.poller_config,
        );
        (identity.ip, actor.run().await)
    });
}

async fn discover_models_for_device(
    config: &CollectorConfig,
    device: &DeviceIdentity,
) -> Result<Vec<ModelDefinition>> {
    let mut modbus_config = config.modbus.clone();
    modbus_config.host = device.ip.clone();

    let client = ModbusClient::connect(modbus_config)
        .await
        .context("modbus connect failed")?;
    let registers = client
        .read_range(
            device.unit_id,
            config.base_address,
            config.discovery_register_count,
        )
        .await
        .context("read sunspec model list failed")?;

    parse_models_from_registers_lenient(config.base_address, &registers)
        .map_err(|err| anyhow::anyhow!(err))
}

async fn buffer_task(
    mut rx: mpsc::Receiver<PollSample>,
    buffer: BufferStore,
    publisher: Publisher,
    mut shutdown: watch::Receiver<bool>,
) {
    loop {
        tokio::select! {
            maybe_sample = rx.recv() => {
                match maybe_sample {
                    Some(sample) => {
                        // Store lightweight JSON in buffer instead of Avro
                        match serde_json::to_vec(&sample) {
                            Ok(payload) => {
                                if let Err(err) = buffer.enqueue(publisher.topic(), &payload).await {
                                    warn!(error = %err, "buffer enqueue failed");
                                    counter!("buffer_enqueue_error").increment(1);
                                } else {
                                    counter!("buffer_enqueue_success").increment(1);
                                }
                            }
                            Err(err) => {
                                warn!(error = %err, "json serialization failed");
                            }
                        }
                    }
                    None => break,
                }
            }
            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    info!("buffer shutdown requested");
                    break;
                }
            }
        }
    }
}

async fn uplink_task(
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
                                     histogram!("uplink_publish_latency", duration);
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
                        gauge!("buffer_size", count as f64);
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

async fn metrics_task(handle: PrometheusHandle, mut shutdown: watch::Receiver<bool>, port: u16) {
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
    let factor = 1u64 << shift;
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

fn respawn_delay(base_ms: u64, failure_count: u32) -> Duration {
    if failure_count == 0 {
        return Duration::from_millis(base_ms);
    }
    let shift = failure_count.saturating_sub(1).min(31);
    let factor = 1u64 << shift;
    let candidate = base_ms.saturating_mul(factor);
    let capped = candidate.min(MAX_RESPAWN_DELAY_MS);
    Duration::from_millis(capped.max(base_ms))
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

#[cfg(target_os = "linux")]
fn notify_ready() {
    if let Err(err) = sd_notify::notify(true, &[sd_notify::NotifyState::Ready]) {
        warn!(error = %err, "systemd ready notify failed");
    }
}

#[cfg(not(target_os = "linux"))]
fn notify_ready() {}

#[cfg(target_os = "linux")]
fn start_watchdog(
    mut shutdown: watch::Receiver<bool>,
) -> Option<tokio::task::JoinHandle<()>> {
    let interval = watchdog_interval()?;
    Some(tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = sleep(interval) => {
                    if let Err(err) = sd_notify::notify(false, &[sd_notify::NotifyState::Watchdog]) {
                        warn!(error = %err, "systemd watchdog notify failed");
                    }
                }
                _ = shutdown.changed() => {
                    if *shutdown.borrow() {
                        break;
                    }
                }
            }
        }
    }))
}

#[cfg(not(target_os = "linux"))]
fn start_watchdog(_shutdown: watch::Receiver<bool>) -> Option<tokio::task::JoinHandle<()>> {
    None
}

#[cfg(target_os = "linux")]
fn watchdog_interval() -> Option<Duration> {
    let watchdog_usec = env::var("WATCHDOG_USEC").ok()?.parse::<u64>().ok()?;
    if let Some(pid) = env::var("WATCHDOG_PID")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
    {
        if pid != std::process::id() {
            return None;
        }
    }

    let interval = watchdog_usec.saturating_div(2).max(100_000);
    Some(Duration::from_micros(interval))
}
