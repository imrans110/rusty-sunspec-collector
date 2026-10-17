use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Serialize;

use poller_actor::DeviceHealth;
use types::{DeviceIdentity, PointValue};

/// A pipeline task that has not checked in for this long is considered stuck. The
/// uplink checks in at least every 30 s (its longest backoff), ingest every 5 s.
pub const STALE_AFTER: Duration = Duration::from_secs(120);

/// Timestamp of a task's last sign of life.
#[derive(Debug, Default)]
pub struct Heartbeat(AtomicU64);

impl Heartbeat {
    pub fn beat(&self) {
        self.0.store(now_ms(), Ordering::Relaxed);
    }

    fn age(&self, now_ms: u64) -> Option<Duration> {
        match self.0.load(Ordering::Relaxed) {
            0 => None,
            beat => Some(Duration::from_millis(now_ms.saturating_sub(beat))),
        }
    }
}

/// Shared health state behind `/healthz` and the systemd watchdog.
#[derive(Debug, Default)]
pub struct Health {
    pub ingest: Heartbeat,
    pub uplink: Heartbeat,
    devices: RwLock<HashMap<String, (DeviceIdentity, Arc<DeviceHealth>)>>,
}

/// One device in `/devices`: its state, what the latest session discovered, and the
/// latest decoded reading of each polled model.
#[derive(Debug, Clone, Serialize)]
pub struct DeviceView {
    pub device: String,
    pub host: String,
    pub unit_id: u8,
    pub up: bool,
    /// Unix time of the last successful read, if any.
    pub last_success_ms: Option<u64>,
    pub restarts: u64,
    /// Address of the `SunS` marker, once found.
    pub base_address: Option<u16>,
    /// The model chain, in device order.
    pub models: Vec<u16>,
    /// The models the polling profile selects.
    pub polled_models: Vec<u16>,
    /// Latest reading of each polled model that has been read, in device order.
    pub readings: Vec<ReadingView>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadingView {
    pub model_id: u16,
    pub model_name: String,
    pub collected_at_ms: u64,
    /// Seconds since the reading was taken.
    pub age_s: f64,
    /// Decoded points in definition order.
    #[serde(with = "types::ordered_points")]
    pub points: Vec<(String, PointValue)>,
    /// Names of enum values and set bitfield bits, by point.
    #[serde(with = "types::ordered_points")]
    pub labels: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthReport {
    /// Ingest and uplink are running; device outages do not make the collector
    /// unhealthy (restarting it would not bring a device back).
    pub healthy: bool,
    pub ingest_age_s: Option<f64>,
    pub uplink_age_s: Option<f64>,
    pub devices_up: usize,
    pub devices_total: usize,
}

impl Health {
    pub fn register_device(&self, identity: &DeviceIdentity, health: Arc<DeviceHealth>) {
        self.devices
            .write()
            .expect("health lock poisoned")
            .insert(identity.key(), (identity.clone(), health));
    }

    /// Every device, sorted by key.
    pub fn devices(&self) -> Vec<DeviceView> {
        let now = now_ms();
        let devices = self.devices.read().expect("health lock poisoned");
        let mut views: Vec<DeviceView> = devices
            .values()
            .map(|(identity, health)| device_view(identity, health, now))
            .collect();
        views.sort_by(|a, b| a.device.cmp(&b.device));
        views
    }

    /// One device, by host and unit ID.
    pub fn device(&self, identity: &DeviceIdentity) -> Option<DeviceView> {
        let devices = self.devices.read().expect("health lock poisoned");
        devices
            .get(&identity.key())
            .map(|(identity, health)| device_view(identity, health, now_ms()))
    }

    pub fn report(&self) -> HealthReport {
        let now = now_ms();
        let ingest = self.ingest.age(now);
        let uplink = self.uplink.age(now);
        let devices = self.devices.read().expect("health lock poisoned");
        HealthReport {
            healthy: is_fresh(ingest) && is_fresh(uplink),
            ingest_age_s: ingest.map(|age| age.as_secs_f64()),
            uplink_age_s: uplink.map(|age| age.as_secs_f64()),
            devices_up: devices
                .values()
                .filter(|(_, device)| device.is_up())
                .count(),
            devices_total: devices.len(),
        }
    }
}

fn device_view(identity: &DeviceIdentity, health: &DeviceHealth, now: u64) -> DeviceView {
    let mut snapshot = health.snapshot();
    let readings = snapshot
        .polled_models
        .iter()
        .filter_map(|model_id| snapshot.readings.remove(model_id))
        .map(|sample| ReadingView {
            model_id: sample.model_id,
            model_name: sample.model_name,
            collected_at_ms: sample.collected_at_ms,
            age_s: now.saturating_sub(sample.collected_at_ms) as f64 / 1000.0,
            points: sample.points,
            labels: sample.labels,
        })
        .collect();
    DeviceView {
        device: identity.key(),
        host: identity.ip.clone(),
        unit_id: identity.unit_id,
        up: health.is_up(),
        last_success_ms: Some(health.last_success_ms()).filter(|ms| *ms > 0),
        restarts: health.restarts(),
        base_address: snapshot.base_address,
        models: snapshot.models,
        polled_models: snapshot.polled_models,
        readings,
    }
}

fn is_fresh(age: Option<Duration>) -> bool {
    age.is_some_and(|age| age < STALE_AFTER)
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn healthy_only_while_both_pipeline_tasks_check_in() {
        let health = Health::default();
        assert!(!health.report().healthy, "no heartbeat yet");

        health.ingest.beat();
        assert!(!health.report().healthy, "uplink has not checked in");

        health.uplink.beat();
        assert!(health.report().healthy);

        let stale = now_ms() - STALE_AFTER.as_millis() as u64 - 1_000;
        health.uplink.0.store(stale, Ordering::Relaxed);
        let report = health.report();
        assert!(!report.healthy, "stale uplink");
        assert!(report.uplink_age_s.unwrap() > STALE_AFTER.as_secs_f64());
    }

    #[test]
    fn reports_device_counts_without_affecting_health() {
        let health = Health::default();
        health.ingest.beat();
        health.uplink.beat();
        health.register_device(
            &DeviceIdentity {
                ip: "10.0.0.5".into(),
                unit_id: 1,
            },
            Arc::new(DeviceHealth::default()),
        );

        let report = health.report();
        assert!(report.healthy);
        assert_eq!((report.devices_up, report.devices_total), (0, 1));
    }
}
