//! Per-device lifecycle: connect, discover the SunSpec model chain, poll on a fixed
//! schedule, and recover from failures (including stalls) until shutdown.

use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use metrics::{counter, gauge, histogram};
use modbus_client::{ClientConfig, ClientError, ExceptionCode, ModbusClient};
use sunspec_parser::{
    decode_model_labeled, model_name, ModelDefinition, SUNSPEC_END_ID, SUNSPEC_ID,
};
use thiserror::Error;
use tokio::sync::mpsc::error::SendTimeoutError;
use tokio::sync::{mpsc, watch};
use tokio::time::{interval, interval_at, sleep, Instant, MissedTickBehavior};
use tracing::{debug, info, warn};
use types::DeviceIdentity;
pub use types::Sample;

/// Base addresses the SunSpec specification allows, in the order it recommends probing.
pub const STANDARD_BASE_ADDRESSES: [u16; 3] = [40_000, 0, 50_000];

/// Upper bound on models in one chain, so a corrupt chain cannot loop forever.
const MAX_MODELS: usize = 256;
const MAX_CONSECUTIVE_FAILED_CYCLES: u32 = 10;
const MAX_RESPAWN_DELAY: Duration = Duration::from_secs(60);
const MIN_STALL_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone)]
pub struct ActorConfig {
    pub poll_interval: Duration,
    pub request_timeout: Duration,
    /// Maximum random offset for each device's polling schedule, so devices sharing a
    /// network do not all poll at the same instant.
    pub jitter_ms: u64,
    /// Restart a device session that has gone this long without a successful read.
    /// `None` means the larger of 10 poll intervals and 30 seconds.
    pub stall_timeout: Option<Duration>,
    /// Base delay before retrying a failed device session; doubles per consecutive
    /// failure up to 60 seconds.
    pub respawn_delay: Duration,
    /// Which discovered models are polled, and which are read less often.
    pub models: ModelSchedule,
}

impl Default for ActorConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(1),
            request_timeout: Duration::from_secs(1),
            jitter_ms: 0,
            stall_timeout: None,
            respawn_delay: Duration::from_secs(1),
            models: ModelSchedule::default(),
        }
    }
}

impl ActorConfig {
    pub fn effective_stall_timeout(&self) -> Duration {
        self.stall_timeout
            .unwrap_or_else(|| (self.poll_interval * 10).max(MIN_STALL_TIMEOUT))
    }
}

/// Which of a device's models are polled, and how often. By default every discovered
/// model is read on every poll cycle.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModelSchedule {
    /// When set, only these models are polled.
    pub include: Option<BTreeSet<u16>>,
    /// Models that are never polled.
    pub exclude: BTreeSet<u16>,
    /// Models read at most once per interval instead of every cycle, e.g. nameplate
    /// and settings models that rarely change.
    pub intervals: BTreeMap<u16, Duration>,
}

impl ModelSchedule {
    pub fn polls(&self, model_id: u16) -> bool {
        !self.exclude.contains(&model_id)
            && self
                .include
                .as_ref()
                .is_none_or(|include| include.contains(&model_id))
    }

    /// The model's slow interval, if it has one.
    pub fn interval(&self, model_id: u16) -> Option<Duration> {
        self.intervals.get(&model_id).copied()
    }
}

/// Everything needed to run one device.
#[derive(Debug, Clone)]
pub struct DeviceConfig {
    pub identity: DeviceIdentity,
    /// Connection settings; `host` is taken from `identity`.
    pub modbus: ClientConfig,
    pub actor: ActorConfig,
    /// Preferred SunSpec base address; the other standard bases are tried after it.
    pub base_address: u16,
    /// A client shared with the other units behind the same RTU-over-TCP converter, so
    /// their requests never overlap on the serial bus. `None` opens a connection per
    /// session.
    pub shared_client: Option<Arc<ModbusClient>>,
}

#[derive(Debug, Error)]
pub enum PollerError {
    #[error(transparent)]
    Modbus(#[from] ClientError),
    #[error("no SunSpec register map at base addresses {bases:?}{}", last_answer(.exception))]
    NoSunSpecMap {
        bases: Vec<u16>,
        /// The device's last Modbus exception while looking, such as a gateway's "target
        /// device failed to respond" for a unit ID with nothing behind it.
        exception: Option<ExceptionCode>,
    },
    #[error("SunSpec model chain is malformed: {0}")]
    MalformedChain(&'static str),
    #[error("none of the discovered models {0:?} is selected for polling")]
    NothingToPoll(Vec<u16>),
    #[error("{0} consecutive poll cycles failed")]
    TooManyFailedCycles(u32),
    #[error("no successful read for {0:?}")]
    Stalled(Duration),
    #[error("telemetry channel closed")]
    ChannelClosed,
}

/// What a device's latest session discovered, and its latest reading of each polled
/// model: the live view behind `/devices`. Holds one sample per model at most.
#[derive(Debug, Clone, Default)]
pub struct DeviceSnapshot {
    /// Address of the `SunS` marker, once a session has found it.
    pub base_address: Option<u16>,
    /// The model chain, in device order.
    pub models: Vec<u16>,
    /// The models the polling profile selects, in device order.
    pub polled_models: Vec<u16>,
    /// Latest successful read of each polled model.
    pub readings: BTreeMap<u16, Sample>,
}

/// One device's state, shared with metrics, health reporting and `/devices`. The
/// counters are lock-free; the snapshot takes a short lock once per read.
#[derive(Debug, Default)]
pub struct DeviceHealth {
    up: AtomicBool,
    last_success_ms: AtomicU64,
    /// Last successful read or poll cycle with nothing due; drives stall detection.
    last_progress_ms: AtomicU64,
    restarts: AtomicU64,
    snapshot: Mutex<DeviceSnapshot>,
}

impl DeviceHealth {
    pub fn is_up(&self) -> bool {
        self.up.load(Ordering::Relaxed)
    }

    /// Unix time in milliseconds of the last successful read (0 if none yet).
    pub fn last_success_ms(&self) -> u64 {
        self.last_success_ms.load(Ordering::Relaxed)
    }

    pub fn restarts(&self) -> u64 {
        self.restarts.load(Ordering::Relaxed)
    }

    /// A copy of the latest discovery and readings.
    pub fn snapshot(&self) -> DeviceSnapshot {
        self.snapshot
            .lock()
            .expect("snapshot lock poisoned")
            .clone()
    }

    fn record_discovery(&self, base_address: u16, models: Vec<u16>, polled_models: Vec<u16>) {
        let mut snapshot = self.snapshot.lock().expect("snapshot lock poisoned");
        // A firmware update can change the chain; forget models no longer polled.
        snapshot
            .readings
            .retain(|model_id, _| polled_models.contains(model_id));
        snapshot.base_address = Some(base_address);
        snapshot.models = models;
        snapshot.polled_models = polled_models;
    }

    fn record_reading(&self, sample: &Sample) {
        self.snapshot
            .lock()
            .expect("snapshot lock poisoned")
            .readings
            .insert(sample.model_id, sample.clone());
    }

    fn record_success(&self, device: &str) {
        let now = unix_ms();
        self.last_success_ms.store(now, Ordering::Relaxed);
        self.last_progress_ms.store(now, Ordering::Relaxed);
        if !self.up.swap(true, Ordering::Relaxed) {
            gauge!("sunspec_device_up", "device" => device.to_string()).set(1.0);
        }
        gauge!("sunspec_device_last_success_timestamp_seconds", "device" => device.to_string())
            .set(now as f64 / 1000.0);
    }

    /// A poll cycle in which no model was due (all remaining ones are on slow
    /// intervals) is not a stall.
    fn record_idle_cycle(&self) {
        self.last_progress_ms.store(unix_ms(), Ordering::Relaxed);
    }

    fn mark_down(&self, device: &str) {
        if self.up.swap(false, Ordering::Relaxed) {
            gauge!("sunspec_device_up", "device" => device.to_string()).set(0.0);
        }
    }
}

/// Where [`discover_models`] reads registers from: the Modbus client, or an in-memory
/// register map in tests and fuzzing.
pub trait ReadRegisters {
    /// Reads `count` holding registers starting at `start`.
    fn read_registers(
        &self,
        unit_id: u8,
        start: u16,
        count: u16,
    ) -> impl Future<Output = Result<Vec<u16>, ClientError>> + Send;
}

impl ReadRegisters for ModbusClient {
    fn read_registers(
        &self,
        unit_id: u8,
        start: u16,
        count: u16,
    ) -> impl Future<Output = Result<Vec<u16>, ClientError>> + Send {
        self.read_range(unit_id, start, count)
    }
}

/// Finds the SunSpec model chain by reading one model header at a time. Reads stay
/// small and never run past the end of the map, unlike a fixed-size window.
pub async fn discover_models<R: ReadRegisters>(
    client: &R,
    unit_id: u8,
    bases: &[u16],
) -> Result<(u16, Vec<ModelDefinition>), PollerError> {
    let mut exception = None;
    for &base in bases {
        match client.read_registers(unit_id, base, 2).await {
            Ok(marker) if marker == SUNSPEC_ID => {
                return walk_chain(client, unit_id, base)
                    .await
                    .map(|models| (base, models));
            }
            // A miss at a candidate base is expected; the caller reports a map found nowhere.
            Ok(marker) => debug!(unit_id, base, ?marker, "no SunSpec marker at this base"),
            Err(ClientError::Exception(code)) => {
                debug!(unit_id, base, exception = %code, "no SunSpec map at this base");
                exception = Some(code);
            }
            Err(err) => return Err(err.into()),
        }
    }
    Err(PollerError::NoSunSpecMap {
        bases: bases.to_vec(),
        exception,
    })
}

fn last_answer(exception: &Option<ExceptionCode>) -> String {
    exception
        .map(|code| format!(" (last answer: {code})"))
        .unwrap_or_default()
}

async fn walk_chain<R: ReadRegisters>(
    client: &R,
    unit_id: u8,
    base: u16,
) -> Result<Vec<ModelDefinition>, PollerError> {
    let mut models = Vec::new();
    let mut addr = base
        .checked_add(2)
        .ok_or(PollerError::MalformedChain("address overflow"))?;
    loop {
        let header = client.read_registers(unit_id, addr, 2).await?;
        let [id, len] = header[..] else {
            return Err(PollerError::MalformedChain("short model header"));
        };
        if id == SUNSPEC_END_ID {
            return Ok(models);
        }
        if models.len() == MAX_MODELS {
            return Err(PollerError::MalformedChain("too many models"));
        }
        let length = len
            .checked_add(2)
            .ok_or(PollerError::MalformedChain("model length overflow"))?;
        models.push(ModelDefinition {
            id,
            name: model_name(id),
            start: addr,
            length,
        });
        addr = addr
            .checked_add(length)
            .ok_or(PollerError::MalformedChain("address overflow"))?;
    }
}

/// The configured base first, then the remaining standard bases.
pub fn base_candidates(preferred: u16) -> Vec<u16> {
    let mut bases = vec![preferred];
    bases.extend(
        STANDARD_BASE_ADDRESSES
            .iter()
            .copied()
            .filter(|base| *base != preferred),
    );
    bases
}

/// Runs a device until shutdown: each session connects, discovers models and polls;
/// when a session fails or stalls, it is retried with exponential backoff.
pub async fn run_device(
    config: DeviceConfig,
    sender: mpsc::Sender<Sample>,
    mut shutdown: watch::Receiver<bool>,
    health: Arc<DeviceHealth>,
) {
    let device = config.identity.key();
    let salt = device_hash(&config.identity);
    let mut failures = 0u32;
    // Publish the series from the start so a device that never answers shows up as down.
    gauge!("sunspec_device_up", "device" => device.clone()).set(0.0);

    loop {
        if *shutdown.borrow() {
            break;
        }
        let mut polled = false;
        let outcome = run_session(&config, &sender, &mut shutdown, &health, &mut polled).await;
        health.mark_down(&device);
        let err = match outcome {
            Some(err) => err,
            None => break,
        };

        if polled {
            failures = 0;
        }
        failures = failures.saturating_add(1);
        if matches!(err, PollerError::Stalled(_)) {
            counter!("sunspec_poller_stalls_total", "device" => device.clone()).increment(1);
        }
        counter!("sunspec_poller_restarts_total", "device" => device.clone()).increment(1);
        health.restarts.fetch_add(1, Ordering::Relaxed);

        let delay = respawn_delay(config.actor.respawn_delay, failures, salt);
        warn!(
            device = %device,
            error = %err,
            retry_in_ms = delay.as_millis() as u64,
            "device session ended"
        );
        tokio::select! {
            _ = sleep(delay) => {}
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() {
                    break;
                }
            }
        }
    }
    info!(device = %device, "device poller stopped");
}

/// Returns `None` on shutdown, otherwise the reason the session ended.
async fn run_session(
    config: &DeviceConfig,
    sender: &mpsc::Sender<Sample>,
    shutdown: &mut watch::Receiver<bool>,
    health: &DeviceHealth,
    polled: &mut bool,
) -> Option<PollerError> {
    let stall_timeout = config.actor.effective_stall_timeout();
    let mut last_progress = Instant::now();
    let mut progress_seen = health.last_progress_ms.load(Ordering::Relaxed);
    let mut stall_check = interval((stall_timeout / 4).max(Duration::from_millis(50)));
    stall_check.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let session = poll_device(config, sender, health, polled);
    tokio::pin!(session);
    loop {
        tokio::select! {
            err = &mut session => return Some(err),
            _ = stall_check.tick() => {
                let seen = health.last_progress_ms.load(Ordering::Relaxed);
                if seen != progress_seen {
                    progress_seen = seen;
                    last_progress = Instant::now();
                } else if last_progress.elapsed() >= stall_timeout {
                    return Some(PollerError::Stalled(stall_timeout));
                }
            }
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() {
                    return None;
                }
            }
        }
    }
}

/// Polls until something fails; never returns on success.
async fn poll_device(
    config: &DeviceConfig,
    sender: &mpsc::Sender<Sample>,
    health: &DeviceHealth,
    polled: &mut bool,
) -> PollerError {
    let identity = &config.identity;
    let device = identity.key();
    let mut modbus = config.modbus.clone();
    modbus.host = identity.ip.clone();
    modbus.timeout_ms = config.actor.request_timeout.as_millis() as u64;

    let client = match &config.shared_client {
        // Connects on its first read; failures surface from discovery below.
        Some(client) => client.clone(),
        None => match ModbusClient::connect(modbus).await {
            Ok(client) => Arc::new(client),
            Err(err) => {
                count_error(&device, &err);
                return err.into();
            }
        },
    };
    let bases = base_candidates(config.base_address);
    let (base, models) = match discover_models(client.as_ref(), identity.unit_id, &bases).await {
        Ok(found) => found,
        Err(err) => {
            if let PollerError::Modbus(ref modbus_err) = err {
                count_error(&device, modbus_err);
            }
            return err;
        }
    };
    let schedule = &config.actor.models;
    let discovered: Vec<u16> = models.iter().map(|model| model.id).collect();
    let models: Vec<ModelDefinition> = models
        .into_iter()
        .filter(|model| schedule.polls(model.id))
        .collect();
    info!(
        device = %device,
        base,
        models = ?discovered,
        polled = ?models.iter().map(|model| model.id).collect::<Vec<_>>(),
        slow = ?models
            .iter()
            .filter_map(|model| schedule.interval(model.id).map(|every| (model.id, every)))
            .collect::<Vec<_>>(),
        "discovered SunSpec models"
    );
    if discovered.is_empty() {
        return PollerError::MalformedChain("no models before the end marker");
    }
    health.record_discovery(
        base,
        discovered.clone(),
        models.iter().map(|model| model.id).collect(),
    );
    if models.is_empty() {
        return PollerError::NothingToPoll(discovered);
    }

    let offset = Duration::from_millis(device_hash(identity) % (config.actor.jitter_ms + 1));
    let mut ticker = interval_at(Instant::now() + offset, config.actor.poll_interval);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
    let mut failed_cycles = 0u32;
    // When each slow model is next due; `None` (every model at first) means now.
    let mut next_due: Vec<Option<Instant>> = vec![None; models.len()];

    loop {
        // The scheduled tick, not the wake-up time, so slow intervals stay aligned
        // with the poll schedule.
        let tick = ticker.tick().await;
        let due: Vec<usize> = (0..models.len())
            .filter(|&index| next_due[index].is_none_or(|due| tick >= due))
            .collect();
        if due.is_empty() {
            health.record_idle_cycle();
            continue;
        }
        let started = Instant::now();
        let mut cycle_failed = false;

        for index in due {
            let model = &models[index];
            match client
                .read_range(identity.unit_id, model.start, model.length)
                .await
            {
                Ok(registers) => {
                    health.record_success(&device);
                    *polled = true;
                    // A failed read stays due, so it is retried on the next cycle.
                    next_due[index] = schedule.interval(model.id).map(|every| tick + every);
                    counter!("sunspec_poll_success_total", "device" => device.clone()).increment(1);
                    // Decode at the edge so consumers get scaled, named points.
                    let decoded = decode_model_labeled(model.id, &registers).unwrap_or_default();
                    let sample = Sample {
                        device: identity.clone(),
                        model_id: model.id,
                        model_name: model.name.clone(),
                        start: model.start,
                        registers,
                        collected_at_ms: unix_ms(),
                        points: decoded.points,
                        labels: decoded.labels,
                    };
                    health.record_reading(&sample);
                    // Never block polling for long on a full channel; drop and count.
                    match sender
                        .send_timeout(sample, config.actor.poll_interval)
                        .await
                    {
                        Ok(()) => {}
                        Err(SendTimeoutError::Timeout(_)) => {
                            warn!(
                                device = %device,
                                model_id = model.id,
                                "telemetry channel full, sample dropped"
                            );
                            counter!("sunspec_samples_dropped_total", "reason" => "channel_full")
                                .increment(1);
                        }
                        Err(SendTimeoutError::Closed(_)) => return PollerError::ChannelClosed,
                    }
                }
                Err(err) => {
                    cycle_failed = true;
                    count_error(&device, &err);
                    warn!(device = %device, model_id = model.id, error = %err, "modbus read failed");
                }
            }
        }

        histogram!("sunspec_poll_duration_seconds", "device" => device.clone())
            .record(started.elapsed());
        if cycle_failed {
            failed_cycles += 1;
            if failed_cycles >= MAX_CONSECUTIVE_FAILED_CYCLES {
                return PollerError::TooManyFailedCycles(failed_cycles);
            }
        } else {
            failed_cycles = 0;
        }
        debug!(
            device = %device,
            elapsed_ms = started.elapsed().as_millis() as u64,
            "poll cycle complete"
        );
    }
}

fn count_error(device: &str, err: &ClientError) {
    counter!(
        "sunspec_poll_errors_total",
        "device" => device.to_string(),
        "kind" => error_kind(err)
    )
    .increment(1);
}

fn error_kind(err: &ClientError) -> &'static str {
    match err {
        ClientError::Exception(_) => "exception",
        ClientError::Timeout { .. } => "timeout",
        ClientError::Transport(_) | ClientError::Disconnected => "transport",
        ClientError::Connect { .. } | ClientError::InvalidAddress(..) => "connect",
        ClientError::AddressOverflow => "address",
    }
}

/// `base * 2^(failures - 1)`, capped at 60 s, plus up to 10% device-salted jitter so
/// devices that fail together do not retry in lockstep.
fn respawn_delay(base: Duration, failures: u32, salt: u64) -> Duration {
    let shift = failures.saturating_sub(1).min(16);
    let delay = base
        .saturating_mul(1u32 << shift)
        .clamp(Duration::from_millis(1), MAX_RESPAWN_DELAY);
    let jitter_window = (delay.as_millis() as u64 / 10).max(1);
    let jitter = (salt ^ u64::from(failures)).wrapping_mul(0x9E37_79B9_7F4A_7C15) % jitter_window;
    delay + Duration::from_millis(jitter)
}

fn device_hash(identity: &DeviceIdentity) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    identity.hash(&mut hasher);
    hasher.finish()
}

fn unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_candidates_put_preferred_first_without_duplicates() {
        assert_eq!(base_candidates(40_000), vec![40_000, 0, 50_000]);
        assert_eq!(base_candidates(0), vec![0, 40_000, 50_000]);
        assert_eq!(base_candidates(1_000), vec![1_000, 40_000, 0, 50_000]);
    }

    #[test]
    fn respawn_delay_doubles_and_caps() {
        let base = Duration::from_secs(1);
        let first = respawn_delay(base, 1, 7);
        let third = respawn_delay(base, 3, 7);
        assert!(first >= base && first < base + Duration::from_millis(101));
        assert!(third >= base * 4 && third < base * 4 + Duration::from_millis(401));
        let capped = respawn_delay(base, 30, 7);
        assert!(capped >= MAX_RESPAWN_DELAY && capped <= MAX_RESPAWN_DELAY * 11 / 10);
    }

    #[test]
    fn model_schedule_applies_include_then_exclude() {
        let mut schedule = ModelSchedule::default();
        assert!(schedule.polls(1) && schedule.polls(103));
        schedule.exclude = BTreeSet::from([1]);
        assert!(!schedule.polls(1) && schedule.polls(103));
        schedule.include = Some(BTreeSet::from([1, 160]));
        assert!(!schedule.polls(1), "exclude wins");
        assert!(!schedule.polls(103), "not included");
        assert!(schedule.polls(160));
        schedule.intervals = BTreeMap::from([(120, Duration::from_secs(3600))]);
        assert_eq!(schedule.interval(120), Some(Duration::from_secs(3600)));
        assert_eq!(schedule.interval(103), None);
    }

    #[test]
    fn default_stall_timeout_scales_with_poll_interval() {
        let mut config = ActorConfig::default();
        assert_eq!(config.effective_stall_timeout(), MIN_STALL_TIMEOUT);
        config.poll_interval = Duration::from_secs(10);
        assert_eq!(config.effective_stall_timeout(), Duration::from_secs(100));
        config.stall_timeout = Some(Duration::from_secs(5));
        assert_eq!(config.effective_stall_timeout(), Duration::from_secs(5));
    }
}
