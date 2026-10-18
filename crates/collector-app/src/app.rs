use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use metrics_exporter_prometheus::PrometheusHandle;
use tokio::sync::{mpsc, watch};
use tracing::info;

use avro_kafka::{KafkaConfig, Publisher};
use buffer::BufferStore;

use crate::config::{CollectorConfig, ConfigError};
use crate::health::Health;
use crate::uplink::UplinkConfig;
use crate::{devices, ingest, metrics_server, retention, systemd, uplink};

/// Runs the collector until `shutdown` resolves, publishing to the Kafka brokers in
/// `config` (or only logging records when no brokers are configured).
pub async fn run(
    config: CollectorConfig,
    metrics: Option<PrometheusHandle>,
    shutdown: impl Future<Output = ()>,
) -> Result<()> {
    let publisher = build_publisher(&config)?;
    run_with_publisher(config, metrics, publisher, shutdown).await
}

/// Like [`run`], with a caller-supplied publisher (e.g. an in-memory one in tests).
///
/// On shutdown, tasks stop in order: device pollers first, then ingest (which drains
/// the channel into the buffer), then the uplink (after a bounded final drain).
pub async fn run_with_publisher(
    config: CollectorConfig,
    metrics: Option<PrometheusHandle>,
    publisher: Publisher,
    shutdown: impl Future<Output = ()>,
) -> Result<()> {
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let health = Arc::new(Health::default());
    let listener = metrics_server::bind(config.metrics_port)
        .await
        .with_context(|| format!("bind metrics port {}", config.metrics_port))?;
    tokio::spawn(metrics_server::serve(
        listener,
        metrics,
        health.clone(),
        shutdown_rx.clone(),
    ));

    let buffer = BufferStore::new(&config.buffer_path)
        .await
        .context("buffer init failed")?;

    let (tx, rx) = mpsc::channel(config.channel_capacity);
    let ingest = tokio::spawn(ingest::run(
        rx,
        buffer.clone(),
        publisher.topic().to_string(),
        health.clone(),
    ));
    // The uplink stops only after ingest has persisted everything, then drains once more.
    let (uplink_stop_tx, uplink_stop_rx) = watch::channel(false);
    let uplink = tokio::spawn(uplink::run(
        buffer.clone(),
        publisher,
        uplink_stop_rx,
        UplinkConfig {
            batch_size: config.buffer_batch_size,
            drain_interval: Duration::from_millis(config.buffer_drain_interval_ms),
            shutdown_drain_timeout: Duration::from_millis(config.shutdown_drain_timeout_ms),
        },
        health.clone(),
    ));
    let retention = tokio::spawn(retention::run(
        buffer,
        config.buffer_max_rows,
        Duration::from_secs(config.buffer_max_age_s),
        shutdown_rx.clone(),
    ));
    let devices = tokio::spawn(devices::run(
        config,
        tx,
        shutdown_rx.clone(),
        health.clone(),
    ));

    systemd::notify_ready();
    let watchdog = systemd::start_watchdog(health, shutdown_rx);

    shutdown.await;
    info!("shutdown signal received");
    let _ = shutdown_tx.send(true);

    // Device tasks own the only senders, so once they stop the channel closes and
    // ingest finishes after persisting everything still queued.
    let _ = devices.await;
    let _ = ingest.await;
    let _ = uplink_stop_tx.send(true);
    let _ = uplink.await;
    let _ = retention.await;
    if let Some(handle) = watchdog {
        let _ = handle.await;
    }
    info!("collector stopped");
    Ok(())
}

fn build_publisher(config: &CollectorConfig) -> Result<Publisher> {
    let topic = config
        .kafka_topic
        .clone()
        .unwrap_or_else(|| "sunspec.telemetry".to_string());
    let Some(brokers) = config.kafka_brokers.clone() else {
        return Ok(Publisher::new_log(topic));
    };

    let defaults = KafkaConfig::default();
    let kafka_config = KafkaConfig {
        brokers,
        client_id: config.kafka_client_id.clone().unwrap_or(defaults.client_id),
        acks: config.kafka_acks.clone().unwrap_or(defaults.acks),
        compression: config
            .kafka_compression
            .clone()
            .unwrap_or(defaults.compression),
        message_timeout_ms: config
            .kafka_timeout_ms
            .unwrap_or(defaults.message_timeout_ms),
        enable_idempotence: config
            .kafka_enable_idempotence
            .unwrap_or(defaults.enable_idempotence),
        security: config.kafka_security.clone(),
    };
    Publisher::new_kafka(topic, kafka_config).context(ConfigError("kafka publisher init failed"))
}
