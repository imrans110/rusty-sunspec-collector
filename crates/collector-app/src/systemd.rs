#[cfg(target_os = "linux")]
use std::time::Duration;

use std::sync::Arc;

use tokio::sync::watch;

use crate::health::Health;
#[cfg(target_os = "linux")]
use tokio::time::sleep;
#[cfg(target_os = "linux")]
use tracing::warn;

/// Tells systemd the service is up. `NOTIFY_SOCKET` stays set: the watchdog pings go to
/// the same socket.
#[cfg(target_os = "linux")]
pub fn notify_ready() {
    if let Err(err) = sd_notify::notify(&[sd_notify::NotifyState::Ready]) {
        warn!(error = %err, "systemd ready notify failed");
    }
}

#[cfg(target_os = "linux")]
fn notify_watchdog() {
    if let Err(err) = sd_notify::notify(&[sd_notify::NotifyState::Watchdog]) {
        warn!(error = %err, "systemd watchdog notify failed");
    }
}

#[cfg(not(target_os = "linux"))]
pub fn notify_ready() {}

/// Pings the systemd watchdog only while the pipeline is healthy, so systemd restarts
/// the service if ingest or the uplink gets stuck.
#[cfg(target_os = "linux")]
pub fn start_watchdog(
    health: Arc<Health>,
    mut shutdown: watch::Receiver<bool>,
) -> Option<tokio::task::JoinHandle<()>> {
    let interval = watchdog_interval()?;
    Some(tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = sleep(interval) => {
                    if !health.report().healthy {
                        warn!("pipeline unhealthy; withholding systemd watchdog ping");
                        continue;
                    }
                    notify_watchdog();
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
pub fn start_watchdog(
    _health: Arc<Health>,
    _shutdown: watch::Receiver<bool>,
) -> Option<tokio::task::JoinHandle<()>> {
    None
}

/// Half the watchdog timeout systemd set for this process (`WATCHDOG_USEC`, if
/// `WATCHDOG_PID` is ours), at least 100 ms.
#[cfg(target_os = "linux")]
fn watchdog_interval() -> Option<Duration> {
    let timeout = sd_notify::watchdog_enabled()?;
    Some((timeout / 2).max(Duration::from_millis(100)))
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use std::os::unix::net::UnixDatagram;
    use std::time::Duration;

    use super::{notify_ready, notify_watchdog};

    /// Readiness used to unset `NOTIFY_SOCKET`, so every later watchdog ping went
    /// nowhere and systemd killed the service after `WatchdogSec`.
    #[test]
    fn watchdog_pings_reach_systemd_after_ready() {
        let dir = std::env::temp_dir().join(format!("sunspec-notify-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("notify.sock");
        let _ = std::fs::remove_file(&path);
        let socket = UnixDatagram::bind(&path).unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        // The only test in this crate that touches the environment.
        std::env::set_var("NOTIFY_SOCKET", &path);

        notify_ready();
        notify_watchdog();

        let mut received = Vec::new();
        let mut buf = [0u8; 256];
        for _ in 0..2 {
            let len = socket.recv(&mut buf).expect("a notification arrives");
            received.push(String::from_utf8_lossy(&buf[..len]).into_owned());
        }
        assert_eq!(received, ["READY=1\n", "WATCHDOG=1\n"]);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
