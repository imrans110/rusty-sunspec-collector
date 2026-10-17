use std::env;
use std::fmt::Display;
use std::fs;
use std::net::Ipv4Addr;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use tracing::warn;

use discovery::DiscoveryConfig;
use modbus_client::ClientConfig;
use poller_actor::ActorConfig;
use types::DeviceIdentity;

const DEFAULT_BASE_ADDRESS: u16 = 40_000;
const DEFAULT_RESCAN_INTERVAL_S: u64 = 600;
const DEFAULT_CHANNEL_CAPACITY: usize = 256;
const DEFAULT_RESPAWN_DELAY_MS: u64 = 1_000;
const DEFAULT_BUFFER_PATH: &str = "sunspec-buffer.sqlite";
const DEFAULT_BUFFER_BATCH_SIZE: i64 = 100;
const DEFAULT_BUFFER_DRAIN_INTERVAL_MS: u64 = 500;
const DEFAULT_BUFFER_MAX_ROWS: i64 = 1_000_000;
const DEFAULT_BUFFER_MAX_AGE_S: u64 = 7 * 24 * 3_600;
const DEFAULT_SHUTDOWN_DRAIN_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_METRICS_PORT: u16 = 9090;

const KAFKA_COMPRESSION: &[&str] = &["none", "gzip", "snappy", "lz4", "zstd"];
const KAFKA_ACKS: &[&str] = &["0", "1", "-1", "all"];

#[derive(Clone, Debug)]
pub struct CollectorConfig {
    pub discovery: DiscoveryConfig,
    pub modbus: ClientConfig,
    pub poller: ActorConfig,
    pub base_address: u16,
    /// How often subnet discovery is repeated to pick up new devices.
    pub rescan_interval_s: u64,
    pub channel_capacity: usize,
    pub respawn_delay_ms: u64,
    pub buffer_path: String,
    pub buffer_batch_size: i64,
    pub buffer_drain_interval_ms: u64,
    /// Oldest rows beyond this count are dropped during long uplink outages.
    pub buffer_max_rows: i64,
    /// Rows older than this are dropped during long uplink outages.
    pub buffer_max_age_s: u64,
    /// How long to keep draining to Kafka after a stop signal (0 = stop at once).
    pub shutdown_drain_timeout_ms: u64,
    pub kafka_brokers: Option<String>,
    pub kafka_client_id: Option<String>,
    pub kafka_acks: Option<String>,
    pub kafka_compression: Option<String>,
    pub kafka_timeout_ms: Option<u64>,
    pub kafka_topic: Option<String>,
    pub kafka_enable_idempotence: Option<bool>,
    /// Port for `/metrics` and `/healthz` (0 picks a free port).
    pub metrics_port: u16,
}

impl CollectorConfig {
    pub fn load() -> Result<Self> {
        Self::load_with_path(None)
    }

    /// Defaults, then the config file (`--config` or `SUNSPEC_CONFIG`), then
    /// `SUNSPEC_*` environment variables. Unknown file keys and unparsable values are
    /// errors, so typos fail at startup instead of silently using defaults.
    pub fn load_with_path(config_path: Option<String>) -> Result<Self> {
        let mut config = Self::default();

        if let Some(file_config) = load_file_config(config_path.as_deref())? {
            apply_file_config(&mut config, file_config)?;
        }

        apply_env_overrides(&mut config)?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if self.discovery.port == 0 {
            bail!("discovery.port must be between 1 and 65535");
        }
        if self.discovery.max_concurrency == 0 {
            bail!("discovery.max_concurrency must be >= 1");
        }
        if self.discovery.per_host_timeout_ms == 0 {
            bail!("discovery.per_host_timeout_ms must be >= 1");
        }
        if self.discovery.unit_ids.is_empty() {
            bail!("discovery.unit_ids must list at least one unit ID");
        }
        validate_cidr(&self.discovery.subnet)?;
        if self.poller.poll_interval.as_millis() == 0 {
            bail!("poller.poll_interval_ms must be >= 1");
        }
        if self.poller.request_timeout.as_millis() == 0 {
            bail!("poller.request_timeout_ms must be >= 1");
        }
        if self.modbus.port == 0 {
            bail!("modbus.port must be between 1 and 65535");
        }
        if let Some(max_batch) = self.modbus.max_batch_size {
            if max_batch == 0 || max_batch > modbus_client::MAX_REGISTERS_PER_READ {
                bail!(
                    "modbus.max_batch_size must be between 1 and {} (Modbus protocol limit)",
                    modbus_client::MAX_REGISTERS_PER_READ
                );
            }
        }
        if self.modbus.retry_backoff_ms == 0 {
            bail!("modbus.retry_backoff_ms must be >= 1");
        }
        if self.modbus.retry_max_backoff_ms == 0 {
            bail!("modbus.retry_max_backoff_ms must be >= 1");
        }
        if self.modbus.inter_read_delay_ms == Some(0) {
            bail!("modbus.inter_read_delay_ms must be >= 1 when set");
        }
        if self.rescan_interval_s == 0 {
            bail!("discovery.rescan_interval_s must be >= 1");
        }
        if self
            .poller
            .stall_timeout
            .is_some_and(|stall| stall.is_zero())
        {
            bail!("poller.stall_timeout_ms must be >= 1 when set");
        }
        if self.channel_capacity == 0 {
            bail!("runtime.channel_capacity must be >= 1");
        }
        if self.respawn_delay_ms == 0 {
            bail!("runtime.respawn_delay_ms must be >= 1");
        }
        if self.buffer_batch_size <= 0 {
            bail!("buffer.batch_size must be >= 1");
        }
        if self.buffer_drain_interval_ms == 0 {
            bail!("buffer.drain_interval_ms must be >= 1");
        }
        if self.buffer_max_rows <= 0 {
            bail!("buffer.max_rows must be >= 1");
        }
        if self.buffer_max_age_s == 0 {
            bail!("buffer.max_age_s must be >= 1");
        }
        if self.kafka_timeout_ms == Some(0) {
            bail!("kafka.timeout_ms must be >= 1");
        }
        if self
            .kafka_brokers
            .as_ref()
            .is_some_and(|brokers| brokers.trim().is_empty())
        {
            bail!("kafka.brokers must be non-empty when set");
        }
        if let Some(ref topic) = self.kafka_topic {
            validate_kafka_topic(topic)?;
        }
        if let Some(ref compression) = self.kafka_compression {
            ensure_one_of("kafka.compression", compression, KAFKA_COMPRESSION)?;
        }
        if let Some(ref acks) = self.kafka_acks {
            ensure_one_of("kafka.acks", acks, KAFKA_ACKS)?;
            // librdkafka refuses to create an idempotent producer without acks=all.
            let idempotent = self.kafka_enable_idempotence.unwrap_or(true);
            if idempotent && acks != "all" && acks != "-1" {
                bail!("kafka.acks must be all (or -1) while kafka.enable_idempotence is true");
            }
        }

        Ok(())
    }
}

impl Default for CollectorConfig {
    fn default() -> Self {
        Self {
            discovery: DiscoveryConfig::default(),
            modbus: ClientConfig::default(),
            poller: ActorConfig::default(),
            base_address: DEFAULT_BASE_ADDRESS,
            rescan_interval_s: DEFAULT_RESCAN_INTERVAL_S,
            channel_capacity: DEFAULT_CHANNEL_CAPACITY,
            respawn_delay_ms: DEFAULT_RESPAWN_DELAY_MS,
            buffer_path: DEFAULT_BUFFER_PATH.to_string(),
            buffer_batch_size: DEFAULT_BUFFER_BATCH_SIZE,
            buffer_drain_interval_ms: DEFAULT_BUFFER_DRAIN_INTERVAL_MS,
            buffer_max_rows: DEFAULT_BUFFER_MAX_ROWS,
            buffer_max_age_s: DEFAULT_BUFFER_MAX_AGE_S,
            shutdown_drain_timeout_ms: DEFAULT_SHUTDOWN_DRAIN_TIMEOUT_MS,
            kafka_brokers: None,
            kafka_client_id: None,
            kafka_acks: None,
            kafka_compression: None,
            kafka_timeout_ms: None,
            kafka_topic: None,
            kafka_enable_idempotence: None,
            metrics_port: DEFAULT_METRICS_PORT,
        }
    }
}

/// Reads and parses `key`; a set but unparsable value is an error naming the variable.
fn env_parse<T>(key: &str) -> Result<Option<T>>
where
    T: FromStr,
    T::Err: Display,
{
    match env::var(key) {
        Ok(value) => value
            .trim()
            .parse()
            .map(Some)
            .map_err(|err| anyhow!("{key}={value:?} is invalid: {err}")),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(err) => Err(anyhow!("{key} is not valid unicode: {err}")),
    }
}

fn env_string(key: &str) -> Result<Option<String>> {
    env_parse::<String>(key)
}

fn apply_env_overrides(config: &mut CollectorConfig) -> Result<()> {
    if let Some(value) = env_string("SUNSPEC_SUBNET")? {
        config.discovery.subnet = value;
    }
    if let Some(ids) = env_string("SUNSPEC_DISCOVERY_UNIT_IDS")? {
        config.discovery.unit_ids =
            parse_unit_id_list(&ids).context("SUNSPEC_DISCOVERY_UNIT_IDS")?;
    }
    if let Some(port) = env_parse("SUNSPEC_PORT")? {
        config.discovery.port = port;
        config.modbus.port = port;
    }
    if let Some(value) = env_parse("SUNSPEC_RESCAN_INTERVAL_S")? {
        config.rescan_interval_s = value;
    }
    if let Some(value) = env_string("SUNSPEC_STATIC_DEVICES")? {
        config.discovery.static_devices =
            parse_static_devices(&value).context("SUNSPEC_STATIC_DEVICES")?;
    }

    if let Some(ms) = env_parse("SUNSPEC_REQUEST_TIMEOUT_MS")? {
        config.poller.request_timeout = Duration::from_millis(ms);
    }
    if let Some(ms) = env_parse("SUNSPEC_POLL_INTERVAL_MS")? {
        config.poller.poll_interval = Duration::from_millis(ms);
    }
    if let Some(ms) = env_parse("SUNSPEC_JITTER_MS")? {
        config.poller.jitter_ms = ms;
    }
    if let Some(ms) = env_parse("SUNSPEC_STALL_TIMEOUT_MS")? {
        config.poller.stall_timeout = Some(Duration::from_millis(ms));
    }

    if let Some(value) = env_parse("SUNSPEC_MAX_BATCH_SIZE")? {
        config.modbus.max_batch_size = Some(value);
    }
    if env::var_os("SUNSPEC_MODBUS_TIMEOUT_MS").is_some() {
        warn!(
            "SUNSPEC_MODBUS_TIMEOUT_MS is deprecated and ignored; use SUNSPEC_REQUEST_TIMEOUT_MS"
        );
    }

    if let Some(value) = env_parse("SUNSPEC_BASE_ADDRESS")? {
        config.base_address = value;
    }
    if env::var_os("SUNSPEC_DISCOVERY_REG_COUNT").is_some() {
        warn!("SUNSPEC_DISCOVERY_REG_COUNT is deprecated and ignored; models are discovered by walking the model chain");
    }

    if let Some(value) = env_string("SUNSPEC_BUFFER_PATH")? {
        config.buffer_path = value;
    }
    if let Some(value) = env_parse("SUNSPEC_BUFFER_BATCH_SIZE")? {
        config.buffer_batch_size = value;
    }
    if let Some(value) = env_parse("SUNSPEC_BUFFER_DRAIN_MS")? {
        config.buffer_drain_interval_ms = value;
    }
    if let Some(value) = env_parse("SUNSPEC_BUFFER_MAX_ROWS")? {
        config.buffer_max_rows = value;
    }
    if let Some(value) = env_parse("SUNSPEC_BUFFER_MAX_AGE_S")? {
        config.buffer_max_age_s = value;
    }
    if let Some(value) = env_parse("SUNSPEC_SHUTDOWN_DRAIN_TIMEOUT_MS")? {
        config.shutdown_drain_timeout_ms = value;
    }

    if let Some(value) = env_parse("SUNSPEC_CHANNEL_CAPACITY")? {
        config.channel_capacity = value;
    }
    if let Some(value) = env_parse("SUNSPEC_RESPAWN_DELAY_MS")? {
        config.respawn_delay_ms = value;
    }
    if let Some(port) = env_parse("SUNSPEC_METRICS_PORT")? {
        config.metrics_port = port;
    }

    if let Some(value) = env_string("SUNSPEC_KAFKA_BROKERS")? {
        config.kafka_brokers = Some(value);
    }
    if let Some(value) = env_string("SUNSPEC_KAFKA_CLIENT_ID")? {
        config.kafka_client_id = Some(value);
    }
    if let Some(value) = env_string("SUNSPEC_KAFKA_ACKS")? {
        config.kafka_acks = Some(value);
    }
    if let Some(value) = env_string("SUNSPEC_KAFKA_COMPRESSION")? {
        config.kafka_compression = Some(value);
    }
    if let Some(value) = env_parse("SUNSPEC_KAFKA_TIMEOUT_MS")? {
        config.kafka_timeout_ms = Some(value);
    }
    if let Some(value) = env_string("SUNSPEC_KAFKA_TOPIC")? {
        config.kafka_topic = Some(value);
    }
    if let Some(value) = env_parse("SUNSPEC_KAFKA_IDEMPOTENCE")? {
        config.kafka_enable_idempotence = Some(value);
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    discovery: Option<FileDiscoveryConfig>,
    poller: Option<FilePollerConfig>,
    modbus: Option<FileModbusConfig>,
    sunspec: Option<FileSunspecConfig>,
    buffer: Option<FileBufferConfig>,
    kafka: Option<FileKafkaConfig>,
    metrics: Option<FileMetricsConfig>,
    runtime: Option<FileRuntimeConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileDiscoveryConfig {
    subnet: Option<String>,
    port: Option<u16>,
    max_concurrency: Option<usize>,
    per_host_timeout_ms: Option<u64>,
    unit_ids: Option<Vec<u8>>,
    rescan_interval_s: Option<u64>,
    static_devices: Option<Vec<FileDeviceConfig>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileDeviceConfig {
    ip: String,
    unit_id: Option<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FilePollerConfig {
    poll_interval_ms: Option<u64>,
    request_timeout_ms: Option<u64>,
    jitter_ms: Option<u64>,
    stall_timeout_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileModbusConfig {
    port: Option<u16>,
    max_batch_size: Option<u16>,
    /// Deprecated and ignored (device sessions use poller.request_timeout_ms);
    /// accepted so existing files keep loading.
    timeout_ms: Option<u64>,
    retry_count: Option<usize>,
    retry_backoff_ms: Option<u64>,
    retry_max_backoff_ms: Option<u64>,
    inter_read_delay_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileSunspecConfig {
    base_address: Option<u16>,
    /// Deprecated and ignored; accepted so existing files keep loading.
    discovery_register_count: Option<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileBufferConfig {
    path: Option<String>,
    batch_size: Option<i64>,
    drain_interval_ms: Option<u64>,
    max_rows: Option<i64>,
    max_age_s: Option<u64>,
    shutdown_drain_timeout_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileKafkaConfig {
    brokers: Option<String>,
    topic: Option<String>,
    client_id: Option<String>,
    acks: Option<String>,
    compression: Option<String>,
    timeout_ms: Option<u64>,
    enable_idempotence: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileMetricsConfig {
    port: Option<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileRuntimeConfig {
    channel_capacity: Option<usize>,
    respawn_delay_ms: Option<u64>,
}

fn load_file_config(config_path: Option<&str>) -> Result<Option<FileConfig>> {
    let path = match config_path {
        Some(path) => path.to_string(),
        None => match env_string("SUNSPEC_CONFIG")? {
            Some(value) => value,
            None => return Ok(None),
        },
    };

    let content = fs::read_to_string(&path).with_context(|| format!("read config file {path}"))?;
    let ext = Path::new(&path)
        .extension()
        .and_then(|value| value.to_str());

    let config = match ext {
        Some("json") => {
            serde_json::from_str(&content).with_context(|| format!("parse json config {path}"))?
        }
        _ => toml::from_str(&content).with_context(|| format!("parse toml config {path}"))?,
    };

    Ok(Some(config))
}

fn set<T>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}

fn apply_file_config(config: &mut CollectorConfig, file: FileConfig) -> Result<()> {
    if let Some(discovery) = file.discovery {
        set(&mut config.discovery.subnet, discovery.subnet);
        set(&mut config.discovery.unit_ids, discovery.unit_ids);
        if let Some(port) = discovery.port {
            config.discovery.port = port;
            config.modbus.port = port;
        }
        set(
            &mut config.discovery.max_concurrency,
            discovery.max_concurrency,
        );
        set(
            &mut config.discovery.per_host_timeout_ms,
            discovery.per_host_timeout_ms,
        );
        set(&mut config.rescan_interval_s, discovery.rescan_interval_s);
        if let Some(devices) = discovery.static_devices {
            config.discovery.static_devices = devices
                .into_iter()
                .map(|device| {
                    validate_host(&device.ip)?;
                    Ok(DeviceIdentity {
                        ip: device.ip,
                        unit_id: device.unit_id.unwrap_or(1),
                    })
                })
                .collect::<Result<_>>()
                .context("discovery.static_devices")?;
        }
    }

    if let Some(poller) = file.poller {
        set(
            &mut config.poller.poll_interval,
            poller.poll_interval_ms.map(Duration::from_millis),
        );
        set(
            &mut config.poller.request_timeout,
            poller.request_timeout_ms.map(Duration::from_millis),
        );
        set(&mut config.poller.jitter_ms, poller.jitter_ms);
        if let Some(ms) = poller.stall_timeout_ms {
            config.poller.stall_timeout = Some(Duration::from_millis(ms));
        }
    }

    if let Some(modbus) = file.modbus {
        if let Some(port) = modbus.port {
            config.modbus.port = port;
            config.discovery.port = port;
        }
        if let Some(max_batch) = modbus.max_batch_size {
            config.modbus.max_batch_size = Some(max_batch);
        }
        if modbus.timeout_ms.is_some() {
            warn!("modbus.timeout_ms is deprecated and ignored; use poller.request_timeout_ms");
        }
        set(&mut config.modbus.retry_count, modbus.retry_count);
        set(&mut config.modbus.retry_backoff_ms, modbus.retry_backoff_ms);
        set(
            &mut config.modbus.retry_max_backoff_ms,
            modbus.retry_max_backoff_ms,
        );
        if let Some(delay) = modbus.inter_read_delay_ms {
            config.modbus.inter_read_delay_ms = Some(delay);
        }
    }

    if let Some(sunspec) = file.sunspec {
        set(&mut config.base_address, sunspec.base_address);
        if sunspec.discovery_register_count.is_some() {
            warn!("sunspec.discovery_register_count is deprecated and ignored; models are discovered by walking the model chain");
        }
    }

    if let Some(buffer) = file.buffer {
        set(&mut config.buffer_path, buffer.path);
        set(&mut config.buffer_batch_size, buffer.batch_size);
        set(
            &mut config.buffer_drain_interval_ms,
            buffer.drain_interval_ms,
        );
        set(&mut config.buffer_max_rows, buffer.max_rows);
        set(&mut config.buffer_max_age_s, buffer.max_age_s);
        set(
            &mut config.shutdown_drain_timeout_ms,
            buffer.shutdown_drain_timeout_ms,
        );
    }

    if let Some(kafka) = file.kafka {
        if kafka.brokers.is_some() {
            config.kafka_brokers = kafka.brokers;
        }
        if kafka.topic.is_some() {
            config.kafka_topic = kafka.topic;
        }
        if kafka.client_id.is_some() {
            config.kafka_client_id = kafka.client_id;
        }
        if kafka.acks.is_some() {
            config.kafka_acks = kafka.acks;
        }
        if kafka.compression.is_some() {
            config.kafka_compression = kafka.compression;
        }
        if kafka.timeout_ms.is_some() {
            config.kafka_timeout_ms = kafka.timeout_ms;
        }
        if kafka.enable_idempotence.is_some() {
            config.kafka_enable_idempotence = kafka.enable_idempotence;
        }
    }

    if let Some(metrics) = file.metrics {
        set(&mut config.metrics_port, metrics.port);
    }

    if let Some(runtime) = file.runtime {
        set(&mut config.channel_capacity, runtime.channel_capacity);
        set(&mut config.respawn_delay_ms, runtime.respawn_delay_ms);
    }
    Ok(())
}

/// Parses `host[:unit_id]` entries such as `192.168.1.20:1,inverter.local`.
fn parse_static_devices(value: &str) -> Result<Vec<DeviceIdentity>> {
    value
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let (host, unit_id) = match entry.rsplit_once(':') {
                Some((host, unit)) => (
                    host,
                    unit.parse::<u8>()
                        .map_err(|_| anyhow!("'{entry}': unit ID must be 0-255"))?,
                ),
                None => (entry, 1),
            };
            validate_host(host).with_context(|| format!("'{entry}'"))?;
            Ok(DeviceIdentity {
                ip: host.to_string(),
                unit_id,
            })
        })
        .collect()
}

fn parse_unit_id_list(value: &str) -> Result<Vec<u8>> {
    value
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            entry
                .parse::<u8>()
                .map_err(|_| anyhow!("'{entry}' is not a unit ID (0-255)"))
        })
        .collect()
}

fn validate_host(host: &str) -> Result<()> {
    if host.is_empty() || host.contains(char::is_whitespace) {
        bail!("device host must be a non-empty IP address or hostname");
    }
    Ok(())
}

fn ensure_one_of(name: &str, value: &str, allowed: &[&str]) -> Result<()> {
    if allowed.contains(&value) {
        Ok(())
    } else {
        bail!(
            "{name} must be one of {}, got {value:?}",
            allowed.join(", ")
        )
    }
}

fn validate_cidr(value: &str) -> Result<()> {
    let (addr, prefix) = value
        .split_once('/')
        .ok_or_else(|| anyhow!("discovery.subnet must be CIDR (e.g. 192.168.1.0/24)"))?;
    addr.parse::<Ipv4Addr>()
        .map_err(|_| anyhow!("discovery.subnet must be a valid IPv4 CIDR"))?;
    let prefix: u8 = prefix
        .parse()
        .map_err(|_| anyhow!("discovery.subnet must include a valid prefix length"))?;
    if prefix > 32 {
        bail!("discovery.subnet prefix must be between 0 and 32");
    }
    Ok(())
}

fn validate_kafka_topic(topic: &str) -> Result<()> {
    if topic.trim().is_empty() {
        bail!("kafka.topic must be non-empty when set");
    }
    if topic.len() > 249 {
        bail!("kafka.topic must be <= 249 characters");
    }
    if topic
        .chars()
        .any(|ch| !ch.is_ascii_alphanumeric() && ch != '.' && ch != '_' && ch != '-')
    {
        bail!("kafka.topic contains invalid characters");
    }
    if topic == "." || topic == ".." {
        bail!("kafka.topic cannot be '.' or '..'");
    }
    Ok(())
}
