use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Serialize;

use poller_actor::DeviceHealth;

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
    devices: RwLock<HashMap<String, Arc<DeviceHealth>>>,
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
    pub fn register_device(&self, key: String, health: Arc<DeviceHealth>) {
        self.devices
            .write()
            .expect("health lock poisoned")
            .insert(key, health);
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
            devices_up: devices.values().filter(|device| device.is_up()).count(),
            devices_total: devices.len(),
        }
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
        health.register_device("10.0.0.5/1".into(), Arc::new(DeviceHealth::default()));

        let report = health.report();
        assert!(report.healthy);
        assert_eq!((report.devices_up, report.devices_total), (0, 1));
    }
}
