#[cfg(target_os = "linux")]
use std::env;
#[cfg(target_os = "linux")]
use std::time::Duration;

use tokio::sync::watch;
#[cfg(target_os = "linux")]
use tokio::time::sleep;
#[cfg(target_os = "linux")]
use tracing::warn;

#[cfg(target_os = "linux")]
pub fn notify_ready() {
    if let Err(err) = sd_notify::notify(true, &[sd_notify::NotifyState::Ready]) {
        warn!(error = %err, "systemd ready notify failed");
    }
}

#[cfg(not(target_os = "linux"))]
pub fn notify_ready() {}

#[cfg(target_os = "linux")]
pub fn start_watchdog(mut shutdown: watch::Receiver<bool>) -> Option<tokio::task::JoinHandle<()>> {
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
pub fn start_watchdog(_shutdown: watch::Receiver<bool>) -> Option<tokio::task::JoinHandle<()>> {
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
