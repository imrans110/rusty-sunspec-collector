use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
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

use avro_kafka::{KafkaSecurity, Secret};
use discovery::DiscoveryConfig;
use modbus_client::{ClientConfig, Framing};
use poller_actor::{ActorConfig, DeviceConfig, ModelSchedule};
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

/// SunSpec model IDs run from 1 to 65534; 65535 marks the end of the model chain.
const MAX_MODEL_ID: u16 = 65_534;

const KAFKA_COMPRESSION: &[&str] = &["none", "gzip", "snappy", "lz4", "zstd"];
const KAFKA_ACKS: &[&str] = &["0", "1", "-1", "all"];
const KAFKA_SECURITY_PROTOCOLS: &[&str] = &["plaintext", "ssl", "sasl_plaintext", "sasl_ssl"];
const KAFKA_SASL_MECHANISMS: &[&str] = &["PLAIN", "SCRAM-SHA-256", "SCRAM-SHA-512"];

#[derive(Clone, Debug)]
pub struct CollectorConfig {
    pub discovery: DiscoveryConfig,
    pub modbus: ClientConfig,
    /// Poll settings for every device, including the model schedule
    /// (`poller.include_models`, `exclude_models`, `model_intervals_s`).
    pub poller: ActorConfig,
    /// Settings a `[[discovery.static_devices]]` entry overrides for that device.
    pub device_overrides: HashMap<DeviceIdentity, DeviceOverrides>,
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
    /// TLS and SASL settings; secrets are redacted in `Debug` output.
    pub kafka_security: KafkaSecurity,
    /// Port for `/metrics` and `/healthz` (0 picks a free port).
    pub metrics_port: u16,
}

/// Per-device settings from a `[[discovery.static_devices]]` entry; unset fields use
/// the global value.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceOverrides {
    pub poll_interval: Option<Duration>,
    pub request_timeout: Option<Duration>,
    pub base_address: Option<u16>,
    pub max_batch_size: Option<u16>,
    pub framing: Option<Framing>,
    /// Replaces `poller.include_models`.
    pub include_models: Option<BTreeSet<u16>>,
    /// Replaces `poller.exclude_models`.
    pub exclude_models: Option<BTreeSet<u16>>,
    /// Merged over `poller.model_intervals_s`, model by model.
    pub model_intervals: BTreeMap<u16, Duration>,
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

    /// Defaults, then a config file's contents, without environment overrides or
    /// validation: the file step of [`CollectorConfig::load_with_path`].
    pub fn from_file_contents(content: &str, format: ConfigFormat) -> Result<Self> {
        let mut config = Self::default();
        apply_file_config(&mut config, parse_file_config(content, format)?)?;
        Ok(config)
    }

    /// The settings one device runs with: the global ones, then its overrides.
    pub fn device_config(&self, identity: DeviceIdentity) -> DeviceConfig {
        let mut actor = self.poller.clone();
        actor.respawn_delay = Duration::from_millis(self.respawn_delay_ms);
        let mut modbus = self.modbus.clone();
        let mut base_address = self.base_address;
        if let Some(device) = self.device_overrides.get(&identity) {
            set(&mut actor.poll_interval, device.poll_interval);
            set(&mut actor.request_timeout, device.request_timeout);
            set(&mut base_address, device.base_address);
            if device.max_batch_size.is_some() {
                modbus.max_batch_size = device.max_batch_size;
            }
            set(&mut modbus.framing, device.framing);
            if device.include_models.is_some() {
                actor.models.include = device.include_models.clone();
            }
            set(&mut actor.models.exclude, device.exclude_models.clone());
            actor.models.intervals.extend(&device.model_intervals);
        }
        DeviceConfig {
            identity,
            modbus,
            actor,
            base_address,
            shared_client: None,
        }
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
        let mut listed = HashSet::new();
        for device in &self.discovery.static_devices {
            if !listed.insert(device) {
                bail!("discovery.static_devices lists {} twice", device.key());
            }
        }
        self.validate_rtu_converters()?;
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
        validate_schedule("poller", &self.poller.models)?;
        for (identity, device) in &self.device_overrides {
            validate_device(identity, device)?;
            let schedule = self.device_config(identity.clone()).actor.models;
            if let Some(model) = schedule
                .include
                .iter()
                .flatten()
                .find(|model| schedule.exclude.contains(model))
            {
                bail!(
                    "discovery.static_devices {}: model {model} is both in include_models and \
                     exclude_models (from the device entry or [poller])",
                    identity.key()
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
        validate_kafka_security(&self.kafka_security)?;

        Ok(())
    }
}

impl CollectorConfig {
    /// Units behind one RTU-over-TCP converter share a single connection, so they must
    /// agree on the settings that connection uses.
    fn validate_rtu_converters(&self) -> Result<()> {
        let mut converters: HashMap<(&str, u16), (String, Duration, Option<u16>)> = HashMap::new();
        for identity in &self.discovery.static_devices {
            let device = self.device_config(identity.clone());
            if device.modbus.framing != Framing::Rtu {
                continue;
            }
            let settings = (
                identity.key(),
                device.actor.request_timeout,
                device.modbus.max_batch_size,
            );
            match converters.get(&(identity.ip.as_str(), device.modbus.port)) {
                Some((first, timeout, batch)) if (*timeout, *batch) != (settings.1, settings.2) => {
                    bail!(
                        "discovery.static_devices {} and {first} share the RTU converter at {}:{}, \
                         so they need the same request_timeout_ms and max_batch_size",
                        identity.key(),
                        identity.ip,
                        device.modbus.port
                    );
                }
                Some(_) => {}
                None => {
                    converters.insert((identity.ip.as_str(), device.modbus.port), settings);
                }
            }
        }
        Ok(())
    }
}

/// `tcp` (Modbus TCP) or `rtu` (RTU frames over TCP, for serial converters).
fn parse_framing(name: &str, value: &str) -> Result<Framing> {
    match value.trim() {
        "tcp" => Ok(Framing::Tcp),
        "rtu" => Ok(Framing::Rtu),
        other => bail!("{name} must be tcp or rtu, got {other:?}"),
    }
}

impl Default for CollectorConfig {
    fn default() -> Self {
        Self {
            discovery: DiscoveryConfig::default(),
            modbus: ClientConfig::default(),
            poller: ActorConfig::default(),
            device_overrides: HashMap::new(),
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
            kafka_security: KafkaSecurity::default(),
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
        if !config.device_overrides.is_empty() {
            warn!("SUNSPEC_STATIC_DEVICES replaces discovery.static_devices; per-device settings from the config file are ignored");
            config.device_overrides.clear();
        }
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
    if let Some(value) = env_string("SUNSPEC_INCLUDE_MODELS")? {
        let models = parse_model_list(&value).context("SUNSPEC_INCLUDE_MODELS")?;
        // Empty (as in the shipped environment file) means every model.
        config.poller.models.include = (!models.is_empty()).then_some(models);
    }
    if let Some(value) = env_string("SUNSPEC_EXCLUDE_MODELS")? {
        config.poller.models.exclude =
            parse_model_list(&value).context("SUNSPEC_EXCLUDE_MODELS")?;
    }
    if let Some(value) = env_string("SUNSPEC_MODEL_INTERVALS_S")? {
        config.poller.models.intervals =
            parse_model_intervals_env(&value).context("SUNSPEC_MODEL_INTERVALS_S")?;
    }

    if let Some(value) = env_parse("SUNSPEC_MAX_BATCH_SIZE")? {
        config.modbus.max_batch_size = Some(value);
    }
    if let Some(value) = env_string("SUNSPEC_MODBUS_FRAMING")? {
        config.modbus.framing = parse_framing("SUNSPEC_MODBUS_FRAMING", &value)?;
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
    let security = &mut config.kafka_security;
    if let Some(value) = env_string("SUNSPEC_KAFKA_SECURITY_PROTOCOL")? {
        security.protocol = Some(value);
    }
    if let Some(value) = env_string("SUNSPEC_KAFKA_SSL_CA_LOCATION")? {
        security.ssl_ca_location = Some(value);
    }
    if let Some(value) = env_string("SUNSPEC_KAFKA_SSL_CERTIFICATE_LOCATION")? {
        security.ssl_certificate_location = Some(value);
    }
    if let Some(value) = env_string("SUNSPEC_KAFKA_SSL_KEY_LOCATION")? {
        security.ssl_key_location = Some(value);
    }
    if let Some(secret) = secret_setting(
        "SUNSPEC_KAFKA_SSL_KEY_PASSWORD",
        env_string("SUNSPEC_KAFKA_SSL_KEY_PASSWORD")?,
        env_string("SUNSPEC_KAFKA_SSL_KEY_PASSWORD_FILE")?,
    )? {
        security.ssl_key_password = Some(secret);
    }
    if let Some(value) = env_string("SUNSPEC_KAFKA_SASL_MECHANISM")? {
        security.sasl_mechanism = Some(value);
    }
    if let Some(value) = env_string("SUNSPEC_KAFKA_SASL_USERNAME")? {
        security.sasl_username = Some(value);
    }
    if let Some(secret) = secret_setting(
        "SUNSPEC_KAFKA_SASL_PASSWORD",
        env_string("SUNSPEC_KAFKA_SASL_PASSWORD")?,
        env_string("SUNSPEC_KAFKA_SASL_PASSWORD_FILE")?,
    )? {
        security.sasl_password = Some(secret);
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
    poll_interval_ms: Option<u64>,
    request_timeout_ms: Option<u64>,
    base_address: Option<u16>,
    max_batch_size: Option<u16>,
    framing: Option<String>,
    include_models: Option<Vec<u16>>,
    exclude_models: Option<Vec<u16>>,
    model_intervals_s: Option<BTreeMap<String, u64>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FilePollerConfig {
    poll_interval_ms: Option<u64>,
    request_timeout_ms: Option<u64>,
    jitter_ms: Option<u64>,
    stall_timeout_ms: Option<u64>,
    include_models: Option<Vec<u16>>,
    exclude_models: Option<Vec<u16>>,
    /// Model ID (as a string key, as TOML and JSON require) to seconds.
    model_intervals_s: Option<BTreeMap<String, u64>>,
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
    /// `tcp` or `rtu` (RTU frames over TCP, for serial-to-Ethernet converters).
    framing: Option<String>,
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
    security_protocol: Option<String>,
    ssl_ca_location: Option<String>,
    ssl_certificate_location: Option<String>,
    ssl_key_location: Option<String>,
    ssl_key_password: Option<String>,
    ssl_key_password_file: Option<String>,
    sasl_mechanism: Option<String>,
    sasl_username: Option<String>,
    sasl_password: Option<String>,
    sasl_password_file: Option<String>,
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
    let (format, name) = match ext {
        Some("json") => (ConfigFormat::Json, "json"),
        _ => (ConfigFormat::Toml, "toml"),
    };
    parse_file_config(&content, format)
        .with_context(|| format!("parse {name} config {path}"))
        .map(Some)
}

/// How a config file is written: `.json` files are JSON, anything else TOML.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    Toml,
    Json,
}

fn parse_file_config(content: &str, format: ConfigFormat) -> Result<FileConfig> {
    Ok(match format {
        ConfigFormat::Json => serde_json::from_str(content)?,
        ConfigFormat::Toml => toml::from_str(content)?,
    })
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
            config.discovery.static_devices.clear();
            config.device_overrides.clear();
            for device in devices {
                validate_host(&device.ip).context("discovery.static_devices")?;
                let identity = DeviceIdentity {
                    ip: device.ip,
                    unit_id: device.unit_id.unwrap_or(1),
                };
                let name = format!("discovery.static_devices {}", identity.key());
                let overrides = DeviceOverrides {
                    poll_interval: device.poll_interval_ms.map(Duration::from_millis),
                    request_timeout: device.request_timeout_ms.map(Duration::from_millis),
                    base_address: device.base_address,
                    max_batch_size: device.max_batch_size,
                    framing: device
                        .framing
                        .as_deref()
                        .map(|value| parse_framing(&format!("{name}: framing"), value))
                        .transpose()?,
                    include_models: device.include_models.map(BTreeSet::from_iter),
                    exclude_models: device.exclude_models.map(BTreeSet::from_iter),
                    model_intervals: parse_model_intervals(&name, device.model_intervals_s)?,
                };
                if overrides != DeviceOverrides::default() {
                    config.device_overrides.insert(identity.clone(), overrides);
                }
                config.discovery.static_devices.push(identity);
            }
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
        let models = &mut config.poller.models;
        if let Some(include) = poller.include_models {
            models.include = Some(BTreeSet::from_iter(include));
        }
        set(
            &mut models.exclude,
            poller.exclude_models.map(BTreeSet::from_iter),
        );
        if poller.model_intervals_s.is_some() {
            models.intervals = parse_model_intervals("poller", poller.model_intervals_s)?;
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
        if let Some(framing) = modbus.framing {
            config.modbus.framing = parse_framing("modbus.framing", &framing)?;
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
        let security = &mut config.kafka_security;
        if kafka.security_protocol.is_some() {
            security.protocol = kafka.security_protocol;
        }
        if kafka.ssl_ca_location.is_some() {
            security.ssl_ca_location = kafka.ssl_ca_location;
        }
        if kafka.ssl_certificate_location.is_some() {
            security.ssl_certificate_location = kafka.ssl_certificate_location;
        }
        if kafka.ssl_key_location.is_some() {
            security.ssl_key_location = kafka.ssl_key_location;
        }
        if let Some(secret) = secret_setting(
            "kafka.ssl_key_password",
            kafka.ssl_key_password,
            kafka.ssl_key_password_file,
        )? {
            security.ssl_key_password = Some(secret);
        }
        if kafka.sasl_mechanism.is_some() {
            security.sasl_mechanism = kafka.sasl_mechanism;
        }
        if kafka.sasl_username.is_some() {
            security.sasl_username = kafka.sasl_username;
        }
        if let Some(secret) = secret_setting(
            "kafka.sasl_password",
            kafka.sasl_password,
            kafka.sasl_password_file,
        )? {
            security.sasl_password = Some(secret);
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

/// Parses one `host[:unit_id]` device, as used by `--probe`; the unit defaults to 1.
pub fn parse_device(value: &str) -> Result<DeviceIdentity> {
    match parse_static_devices(value)?.as_slice() {
        [device] => Ok(device.clone()),
        _ => bail!("expected one host[:unit], got {value:?}"),
    }
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

/// `model_intervals_s` keys are model IDs written as strings (TOML and JSON keys).
fn parse_model_intervals(
    section: &str,
    intervals: Option<BTreeMap<String, u64>>,
) -> Result<BTreeMap<u16, Duration>> {
    intervals
        .unwrap_or_default()
        .into_iter()
        .map(|(model, seconds)| {
            let model = model
                .trim()
                .parse::<u16>()
                .map_err(|_| anyhow!("{section}.model_intervals_s: '{model}' is not a model ID"))?;
            Ok((model, Duration::from_secs(seconds)))
        })
        .collect()
}

/// Parses `1=3600,120=3600` (model ID = seconds).
fn parse_model_intervals_env(value: &str) -> Result<BTreeMap<u16, Duration>> {
    value
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let (model, seconds) = entry
                .split_once('=')
                .ok_or_else(|| anyhow!("'{entry}' must be model=seconds"))?;
            let model = model
                .trim()
                .parse::<u16>()
                .map_err(|_| anyhow!("'{entry}': '{model}' is not a model ID"))?;
            let seconds = seconds
                .trim()
                .parse::<u64>()
                .map_err(|_| anyhow!("'{entry}': '{seconds}' is not a number of seconds"))?;
            Ok((model, Duration::from_secs(seconds)))
        })
        .collect()
}

fn parse_model_list(value: &str) -> Result<BTreeSet<u16>> {
    value
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            entry
                .parse::<u16>()
                .map_err(|_| anyhow!("'{entry}' is not a model ID"))
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

fn validate_model_ids<'a>(name: &str, models: impl IntoIterator<Item = &'a u16>) -> Result<()> {
    if let Some(model) = models
        .into_iter()
        .find(|model| !(1..=MAX_MODEL_ID).contains(*model))
    {
        bail!("{name}: {model} is not a SunSpec model ID (1-{MAX_MODEL_ID})");
    }
    Ok(())
}

/// Checks one set of model settings; `section` is `poller` or a device entry.
fn validate_model_settings(
    section: &str,
    include: Option<&BTreeSet<u16>>,
    exclude: Option<&BTreeSet<u16>>,
    intervals: &BTreeMap<u16, Duration>,
) -> Result<()> {
    if let Some(include) = include {
        if include.is_empty() {
            bail!("{section}.include_models must list at least one model (omit it to poll every model)");
        }
        validate_model_ids(&format!("{section}.include_models"), include)?;
    }
    if let Some(exclude) = exclude {
        validate_model_ids(&format!("{section}.exclude_models"), exclude)?;
    }
    validate_model_ids(&format!("{section}.model_intervals_s"), intervals.keys())?;
    if let Some((model, _)) = intervals.iter().find(|(_, every)| every.is_zero()) {
        bail!("{section}.model_intervals_s: model {model} needs an interval of at least 1 second (omit it to poll every cycle)");
    }
    Ok(())
}

fn validate_schedule(section: &str, schedule: &ModelSchedule) -> Result<()> {
    validate_model_settings(
        section,
        schedule.include.as_ref(),
        Some(&schedule.exclude),
        &schedule.intervals,
    )?;
    if let Some(model) = schedule
        .include
        .iter()
        .flatten()
        .find(|model| schedule.exclude.contains(model))
    {
        bail!("{section}: model {model} is in both include_models and exclude_models");
    }
    Ok(())
}

fn validate_device(identity: &DeviceIdentity, device: &DeviceOverrides) -> Result<()> {
    let section = format!("discovery.static_devices {}", identity.key());
    if device.poll_interval.is_some_and(|every| every.is_zero()) {
        bail!("{section}: poll_interval_ms must be >= 1");
    }
    if device.request_timeout.is_some_and(|limit| limit.is_zero()) {
        bail!("{section}: request_timeout_ms must be >= 1");
    }
    if let Some(max_batch) = device.max_batch_size {
        if max_batch == 0 || max_batch > modbus_client::MAX_REGISTERS_PER_READ {
            bail!(
                "{section}: max_batch_size must be between 1 and {} (Modbus protocol limit)",
                modbus_client::MAX_REGISTERS_PER_READ
            );
        }
    }
    validate_model_settings(
        &section,
        device.include_models.as_ref(),
        device.exclude_models.as_ref(),
        &device.model_intervals,
    )
}

/// A secret given either directly or as `<name>_file` (a file holding only the secret,
/// e.g. a Docker secret or a systemd credential). Setting both is an error.
fn secret_setting(
    name: &str,
    value: Option<String>,
    file: Option<String>,
) -> Result<Option<Secret>> {
    match (value, file) {
        (Some(_), Some(_)) => bail!("set only one of {name} and {name}_file"),
        (Some(value), None) => Ok(Some(Secret::new(value))),
        (None, Some(path)) => {
            let content =
                fs::read_to_string(&path).with_context(|| format!("read {name}_file {path}"))?;
            let secret = content.trim_end_matches(['\r', '\n']);
            if secret.is_empty() {
                bail!("{name}_file {path} is empty");
            }
            Ok(Some(Secret::new(secret)))
        }
        (None, None) => Ok(None),
    }
}

fn validate_kafka_security(security: &KafkaSecurity) -> Result<()> {
    let protocol = security.protocol.as_deref().unwrap_or("plaintext");
    ensure_one_of(
        "kafka.security_protocol",
        protocol,
        KAFKA_SECURITY_PROTOCOLS,
    )?;
    let uses_sasl = protocol.starts_with("sasl_");
    let uses_ssl = protocol == "ssl" || protocol == "sasl_ssl";

    let sasl_set = security.sasl_mechanism.is_some()
        || security.sasl_username.is_some()
        || security.sasl_password.is_some();
    if uses_sasl {
        let mechanism = security
            .sasl_mechanism
            .as_deref()
            .ok_or_else(|| anyhow!("kafka.sasl_mechanism is required with {protocol}"))?;
        ensure_one_of("kafka.sasl_mechanism", mechanism, KAFKA_SASL_MECHANISMS)?;
        if security.sasl_username.as_deref().unwrap_or("").is_empty() {
            bail!("kafka.sasl_username is required with {protocol}");
        }
        if security.sasl_password.is_none() {
            bail!("kafka.sasl_password (or kafka.sasl_password_file) is required with {protocol}");
        }
        if protocol == "sasl_plaintext" {
            warn!("kafka.security_protocol is sasl_plaintext: credentials and data are sent unencrypted");
        }
    } else if sasl_set {
        bail!("kafka.sasl_* settings need kafka.security_protocol sasl_plaintext or sasl_ssl (got {protocol})");
    }

    let ssl_set = security.ssl_ca_location.is_some()
        || security.ssl_certificate_location.is_some()
        || security.ssl_key_location.is_some()
        || security.ssl_key_password.is_some();
    if !uses_ssl && ssl_set {
        bail!("kafka.ssl_* settings need kafka.security_protocol ssl or sasl_ssl (got {protocol})");
    }
    if security.ssl_certificate_location.is_some() != security.ssl_key_location.is_some() {
        bail!("kafka.ssl_certificate_location and kafka.ssl_key_location must be set together");
    }
    for (name, path) in [
        ("kafka.ssl_ca_location", &security.ssl_ca_location),
        (
            "kafka.ssl_certificate_location",
            &security.ssl_certificate_location,
        ),
        ("kafka.ssl_key_location", &security.ssl_key_location),
    ] {
        if let Some(path) = path {
            if !Path::new(path).exists() {
                bail!("{name} {path} does not exist");
            }
        }
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
