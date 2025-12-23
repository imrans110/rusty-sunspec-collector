use std::env;
use std::fs;
use std::net::Ipv4Addr;
use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};
use serde::Deserialize;

use discovery::DiscoveryConfig;
use modbus_client::ClientConfig;
use poller_actor::ActorConfig;
use types::DeviceIdentity;

const DEFAULT_BASE_ADDRESS: u16 = 40_000;
const DEFAULT_DISCOVERY_REG_COUNT: u16 = 200;
const DEFAULT_CHANNEL_CAPACITY: usize = 256;
const DEFAULT_RESPAWN_DELAY_MS: u64 = 1_000;
const DEFAULT_BUFFER_PATH: &str = "sunspec-buffer.sqlite";
const DEFAULT_BUFFER_BATCH_SIZE: i64 = 100;
const DEFAULT_BUFFER_DRAIN_INTERVAL_MS: u64 = 500;

#[derive(Clone, Debug)]
pub struct CollectorConfig {
    pub discovery: DiscoveryConfig,
    pub modbus: ClientConfig,
    pub poller: ActorConfig,
    pub base_address: u16,
    pub discovery_register_count: u16,
    pub channel_capacity: usize,
    pub respawn_delay_ms: u64,
    pub buffer_path: String,
    pub buffer_batch_size: i64,
    pub buffer_drain_interval_ms: u64,
    pub kafka_brokers: Option<String>,
    pub kafka_client_id: Option<String>,
    pub kafka_acks: Option<String>,
    pub kafka_compression: Option<String>,
    pub kafka_timeout_ms: Option<u64>,
    pub kafka_topic: Option<String>,
    pub kafka_enable_idempotence: Option<bool>,
}

impl CollectorConfig {
    pub fn load() -> Result<Self> {
        Self::load_with_path(None)
    }

    pub fn load_with_path(config_path: Option<String>) -> Result<Self> {
        let mut config = Self::default();

        if let Some(file_config) = load_file_config(config_path.as_deref())? {
            apply_file_config(&mut config, file_config);
        }

        apply_env_overrides(&mut config);
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if self.discovery.port == 0 {
            anyhow::bail!("discovery.port must be between 1 and 65535");
        }
        if self.discovery.max_concurrency == 0 {
            anyhow::bail!("discovery.max_concurrency must be >= 1");
        }
        if self.discovery.per_host_timeout_ms == 0 {
            anyhow::bail!("discovery.per_host_timeout_ms must be >= 1");
        }
        validate_cidr(&self.discovery.subnet)?;
        if self.poller.poll_interval.as_millis() == 0 {
            anyhow::bail!("poller.poll_interval_ms must be >= 1");
        }
        if self.poller.request_timeout.as_millis() == 0 {
            anyhow::bail!("poller.request_timeout_ms must be >= 1");
        }
        if self.modbus.port == 0 {
            anyhow::bail!("modbus.port must be between 1 and 65535");
        }
        if let Some(max_batch) = self.modbus.max_batch_size {
            if max_batch == 0 {
                anyhow::bail!("modbus.max_batch_size must be >= 1");
            }
        }
        if self.modbus.timeout_ms == 0 {
            anyhow::bail!("modbus.timeout_ms must be >= 1");
        }
        if self.modbus.retry_backoff_ms == 0 {
            anyhow::bail!("modbus.retry_backoff_ms must be >= 1");
        }
        if self.modbus.retry_max_backoff_ms == 0 {
            anyhow::bail!("modbus.retry_max_backoff_ms must be >= 1");
        }
        if let Some(delay) = self.modbus.inter_read_delay_ms {
            if delay == 0 {
                anyhow::bail!("modbus.inter_read_delay_ms must be >= 1 when set");
            }
        }
        if self.base_address == 0 {
            anyhow::bail!("sunspec.base_address must be >= 1");
        }
        if self.discovery_register_count == 0 {
            anyhow::bail!("sunspec.discovery_register_count must be >= 1");
        }
        if self.channel_capacity == 0 {
            anyhow::bail!("channel_capacity must be >= 1");
        }
        if self.respawn_delay_ms == 0 {
            anyhow::bail!("respawn_delay_ms must be >= 1");
        }
        if self.buffer_batch_size <= 0 {
            anyhow::bail!("buffer.batch_size must be >= 1");
        }
        if self.buffer_drain_interval_ms == 0 {
            anyhow::bail!("buffer.drain_interval_ms must be >= 1");
        }
        if let Some(timeout_ms) = self.kafka_timeout_ms {
            if timeout_ms == 0 {
                anyhow::bail!("kafka.timeout_ms must be >= 1");
            }
        }
        if let Some(ref brokers) = self.kafka_brokers {
            if brokers.trim().is_empty() {
                anyhow::bail!("kafka.brokers must be non-empty when set");
            }
        }
        if let Some(ref topic) = self.kafka_topic {
            validate_kafka_topic(topic)?;
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
            discovery_register_count: DEFAULT_DISCOVERY_REG_COUNT,
            channel_capacity: DEFAULT_CHANNEL_CAPACITY,
            respawn_delay_ms: DEFAULT_RESPAWN_DELAY_MS,
            buffer_path: DEFAULT_BUFFER_PATH.to_string(),
            buffer_batch_size: DEFAULT_BUFFER_BATCH_SIZE,
            buffer_drain_interval_ms: DEFAULT_BUFFER_DRAIN_INTERVAL_MS,
            kafka_brokers: None,
            kafka_client_id: None,
            kafka_acks: None,
            kafka_compression: None,
            kafka_timeout_ms: None,
            kafka_topic: None,
            kafka_enable_idempotence: None,
        }
    }
}

fn apply_env_overrides(config: &mut CollectorConfig) {
    if let Ok(value) = env::var("SUNSPEC_SUBNET") {
        config.discovery.subnet = value;
    }

    if let Some(port) = parse_env_u16("SUNSPEC_PORT") {
        config.discovery.port = port;
        config.modbus.port = port;
    }

    if let Some(timeout_ms) = parse_env_u64("SUNSPEC_REQUEST_TIMEOUT_MS") {
        config.poller.request_timeout = Duration::from_millis(timeout_ms);
    }

    if let Some(interval_ms) = parse_env_u64("SUNSPEC_POLL_INTERVAL_MS") {
        config.poller.poll_interval = Duration::from_millis(interval_ms);
    }

    if let Some(jitter_ms) = parse_env_u64("SUNSPEC_JITTER_MS") {
        config.poller.jitter_ms = jitter_ms;
    }

    if let Some(max_batch) = parse_env_u16("SUNSPEC_MAX_BATCH_SIZE") {
        config.modbus.max_batch_size = Some(max_batch);
    }

    if let Some(timeout_ms) = parse_env_u64("SUNSPEC_MODBUS_TIMEOUT_MS") {
        config.modbus.timeout_ms = timeout_ms;
    }

    if let Some(value) = env::var("SUNSPEC_STATIC_DEVICES").ok() {
        config.discovery.static_devices = parse_static_devices(&value);
    }

    if let Some(value) = env::var("SUNSPEC_BUFFER_PATH").ok() {
        config.buffer_path = value;
    }

    if let Some(value) = parse_env_i64("SUNSPEC_BUFFER_BATCH_SIZE") {
        config.buffer_batch_size = value.max(1);
    }

    if let Some(value) = parse_env_u64("SUNSPEC_BUFFER_DRAIN_MS") {
        config.buffer_drain_interval_ms = value;
    }

    config.base_address =
        parse_env_u16("SUNSPEC_BASE_ADDRESS").unwrap_or(config.base_address);
    config.discovery_register_count = parse_env_u16("SUNSPEC_DISCOVERY_REG_COUNT")
        .unwrap_or(config.discovery_register_count);
    config.channel_capacity =
        parse_env_usize("SUNSPEC_CHANNEL_CAPACITY").unwrap_or(config.channel_capacity);
    config.respawn_delay_ms =
        parse_env_u64("SUNSPEC_RESPAWN_DELAY_MS").unwrap_or(config.respawn_delay_ms);

    config.kafka_brokers = env::var("SUNSPEC_KAFKA_BROKERS").ok().or(config.kafka_brokers);
    config.kafka_client_id =
        env::var("SUNSPEC_KAFKA_CLIENT_ID").ok().or(config.kafka_client_id);
    config.kafka_acks = env::var("SUNSPEC_KAFKA_ACKS").ok().or(config.kafka_acks);
    config.kafka_compression =
        env::var("SUNSPEC_KAFKA_COMPRESSION").ok().or(config.kafka_compression);
    config.kafka_timeout_ms =
        parse_env_u64("SUNSPEC_KAFKA_TIMEOUT_MS").or(config.kafka_timeout_ms);
    config.kafka_topic =
        env::var("SUNSPEC_KAFKA_TOPIC").ok().or(config.kafka_topic);
    config.kafka_enable_idempotence =
        parse_env_bool("SUNSPEC_KAFKA_IDEMPOTENCE").or(config.kafka_enable_idempotence);
}

#[derive(Debug, Deserialize)]
struct FileConfig {
    discovery: Option<FileDiscoveryConfig>,
    poller: Option<FilePollerConfig>,
    modbus: Option<FileModbusConfig>,
    sunspec: Option<FileSunspecConfig>,
    buffer: Option<FileBufferConfig>,
    kafka: Option<FileKafkaConfig>,
}

#[derive(Debug, Deserialize)]
struct FileDiscoveryConfig {
    subnet: Option<String>,
    port: Option<u16>,
    max_concurrency: Option<usize>,
    per_host_timeout_ms: Option<u64>,
    static_devices: Option<Vec<FileDeviceConfig>>,
}

#[derive(Debug, Deserialize)]
struct FileDeviceConfig {
    ip: String,
    unit_id: Option<u8>,
}

#[derive(Debug, Deserialize)]
struct FilePollerConfig {
    poll_interval_ms: Option<u64>,
    request_timeout_ms: Option<u64>,
    jitter_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct FileModbusConfig {
    port: Option<u16>,
    max_batch_size: Option<u16>,
    timeout_ms: Option<u64>,
    retry_count: Option<usize>,
    retry_backoff_ms: Option<u64>,
    retry_max_backoff_ms: Option<u64>,
    inter_read_delay_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct FileSunspecConfig {
    base_address: Option<u16>,
    discovery_register_count: Option<u16>,
}

#[derive(Debug, Deserialize)]
struct FileBufferConfig {
    path: Option<String>,
    batch_size: Option<i64>,
    drain_interval_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct FileKafkaConfig {
    brokers: Option<String>,
    topic: Option<String>,
    client_id: Option<String>,
    acks: Option<String>,
    compression: Option<String>,
    timeout_ms: Option<u64>,
    enable_idempotence: Option<bool>,
}

fn load_file_config(config_path: Option<&str>) -> Result<Option<FileConfig>> {
    let path = match config_path {
        Some(path) => path.to_string(),
        None => match env::var("SUNSPEC_CONFIG") {
            Ok(value) => value,
            Err(_) => return Ok(None),
        },
    };

    let content = fs::read_to_string(&path)
        .with_context(|| format!("read config file {path}"))?;
    let ext = Path::new(&path).extension().and_then(|value| value.to_str());

    let config = match ext {
        Some("json") => serde_json::from_str(&content).context("parse json config")?,
        _ => toml::from_str(&content).context("parse toml config")?,
    };

    Ok(Some(config))
}

fn apply_file_config(config: &mut CollectorConfig, file: FileConfig) {
    if let Some(discovery) = file.discovery {
        if let Some(subnet) = discovery.subnet {
            config.discovery.subnet = subnet;
        }
        if let Some(port) = discovery.port {
            config.discovery.port = port;
            config.modbus.port = port;
        }
        if let Some(max) = discovery.max_concurrency {
            config.discovery.max_concurrency = max;
        }
        if let Some(timeout) = discovery.per_host_timeout_ms {
            config.discovery.per_host_timeout_ms = timeout;
        }
        if let Some(devices) = discovery.static_devices {
            config.discovery.static_devices = devices
                .into_iter()
                .map(|device| DeviceIdentity {
                    ip: device.ip,
                    unit_id: device.unit_id.unwrap_or(1),
                })
                .collect();
        }
    }

    if let Some(poller) = file.poller {
        if let Some(interval_ms) = poller.poll_interval_ms {
            config.poller.poll_interval = Duration::from_millis(interval_ms);
        }
        if let Some(timeout_ms) = poller.request_timeout_ms {
            config.poller.request_timeout = Duration::from_millis(timeout_ms);
        }
        if let Some(jitter_ms) = poller.jitter_ms {
            config.poller.jitter_ms = jitter_ms;
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
        if let Some(timeout_ms) = modbus.timeout_ms {
            config.modbus.timeout_ms = timeout_ms;
        }
        if let Some(retry_count) = modbus.retry_count {
            config.modbus.retry_count = retry_count;
        }
        if let Some(backoff) = modbus.retry_backoff_ms {
            config.modbus.retry_backoff_ms = backoff;
        }
        if let Some(max_backoff) = modbus.retry_max_backoff_ms {
            config.modbus.retry_max_backoff_ms = max_backoff;
        }
        if let Some(delay) = modbus.inter_read_delay_ms {
            config.modbus.inter_read_delay_ms = Some(delay);
        }
    }

    if let Some(sunspec) = file.sunspec {
        if let Some(base) = sunspec.base_address {
            config.base_address = base;
        }
        if let Some(count) = sunspec.discovery_register_count {
            config.discovery_register_count = count;
        }
    }

    if let Some(buffer) = file.buffer {
        if let Some(path) = buffer.path {
            config.buffer_path = path;
        }
        if let Some(batch) = buffer.batch_size {
            config.buffer_batch_size = batch.max(1);
        }
        if let Some(interval) = buffer.drain_interval_ms {
            config.buffer_drain_interval_ms = interval;
        }
    }

    if let Some(kafka) = file.kafka {
        if let Some(brokers) = kafka.brokers {
            config.kafka_brokers = Some(brokers);
        }
        if let Some(topic) = kafka.topic {
            config.kafka_topic = Some(topic);
        }
        if let Some(client_id) = kafka.client_id {
            config.kafka_client_id = Some(client_id);
        }
        if let Some(acks) = kafka.acks {
            config.kafka_acks = Some(acks);
        }
        if let Some(compression) = kafka.compression {
            config.kafka_compression = Some(compression);
        }
        if let Some(timeout_ms) = kafka.timeout_ms {
            config.kafka_timeout_ms = Some(timeout_ms);
        }
        if let Some(enable_idempotence) = kafka.enable_idempotence {
            config.kafka_enable_idempotence = Some(enable_idempotence);
        }
    }
}

fn parse_env_u16(key: &str) -> Option<u16> {
    env::var(key).ok().and_then(|value| value.parse().ok())
}

fn parse_env_u64(key: &str) -> Option<u64> {
    env::var(key).ok().and_then(|value| value.parse().ok())
}

fn parse_env_usize(key: &str) -> Option<usize> {
    env::var(key).ok().and_then(|value| value.parse().ok())
}

fn parse_env_i64(key: &str) -> Option<i64> {
    env::var(key).ok().and_then(|value| value.parse().ok())
}

fn parse_env_bool(key: &str) -> Option<bool> {
    env::var(key).ok().and_then(|value| value.parse().ok())
}

fn parse_static_devices(value: &str) -> Vec<DeviceIdentity> {
    value
        .split(',')
        .filter_map(|entry| {
            let trimmed = entry.trim();
            if trimmed.is_empty() {
                return None;
            }
            let (ip, unit) = match trimmed.split_once(':') {
                Some((ip, unit)) => (ip, unit.parse::<u8>().unwrap_or(1)),
                None => (trimmed, 1),
            };
            Some(DeviceIdentity {
                ip: ip.to_string(),
                unit_id: unit,
            })
        })
        .collect()
}

fn validate_cidr(value: &str) -> Result<()> {
    let (addr, prefix) = value
        .split_once('/')
        .ok_or_else(|| anyhow::anyhow!("discovery.subnet must be CIDR (e.g. 192.168.1.0/24)"))?;
    addr.parse::<Ipv4Addr>()
        .map_err(|_| anyhow::anyhow!("discovery.subnet must be a valid IPv4 CIDR"))?;
    let prefix: u8 = prefix
        .parse()
        .map_err(|_| anyhow::anyhow!("discovery.subnet must include a valid prefix length"))?;
    if prefix > 32 {
        anyhow::bail!("discovery.subnet prefix must be between 0 and 32");
    }
    Ok(())
}

fn validate_kafka_topic(topic: &str) -> Result<()> {
    if topic.trim().is_empty() {
        anyhow::bail!("kafka.topic must be non-empty when set");
    }
    if topic.len() > 249 {
        anyhow::bail!("kafka.topic must be <= 249 characters");
    }
    if topic
        .chars()
        .any(|ch| !ch.is_ascii_alphanumeric() && ch != '.' && ch != '_' && ch != '-')
    {
        anyhow::bail!("kafka.topic contains invalid characters");
    }
    if !topic
        .chars()
        .next()
        .map(|ch| ch.is_ascii_alphanumeric())
        .unwrap_or(false)
        || !topic
            .chars()
            .last()
            .map(|ch| ch.is_ascii_alphanumeric())
            .unwrap_or(false)
    {
        anyhow::bail!("kafka.topic must start and end with an alphanumeric character");
    }
    let bytes = topic.as_bytes();
    for idx in 0..bytes.len() {
        if bytes[idx] == b'.' {
            let prev = idx.checked_sub(1).and_then(|i| bytes.get(i)).copied();
            let next = bytes.get(idx + 1).copied();
            let prev_ok = prev.map(|b| b.is_ascii_alphanumeric()).unwrap_or(false);
            let next_ok = next.map(|b| b.is_ascii_alphanumeric()).unwrap_or(false);
            if !prev_ok || !next_ok {
                anyhow::bail!("kafka.topic '.' must be between alphanumeric characters");
            }
        }
    }
    Ok(())
}
