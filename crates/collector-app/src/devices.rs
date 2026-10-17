use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use metrics::gauge;
use tokio::sync::{mpsc, watch};
use tokio::task::JoinSet;
use tokio::time::{sleep, timeout};
use tracing::{info, warn};

use discovery::discover_subnet;
use poller_actor::{run_device, DeviceConfig, DeviceHealth, Sample};
use types::DeviceIdentity;

use crate::config::CollectorConfig;
use crate::health::Health;

/// How long device tasks get to stop on their own before they are aborted.
const STOP_GRACE: Duration = Duration::from_secs(10);

/// Starts one supervised task per device and keeps the device set current: a static
/// list is started once, while subnet mode rescans periodically and adds new hosts.
/// Returns after shutdown, once every device task has stopped.
pub async fn run(
    config: CollectorConfig,
    sender: mpsc::Sender<Sample>,
    mut shutdown: watch::Receiver<bool>,
    health_registry: Arc<Health>,
) {
    let mut tasks = JoinSet::new();
    let mut known: HashMap<DeviceIdentity, Arc<DeviceHealth>> = HashMap::new();
    let static_mode = !config.discovery.static_devices.is_empty();
    let rescan = Duration::from_secs(config.rescan_interval_s);

    loop {
        let found = if static_mode {
            config.discovery.static_devices.clone()
        } else {
            tokio::select! {
                result = discover_subnet(config.discovery.clone()) => match result {
                    Ok(devices) => devices,
                    Err(err) => {
                        warn!(error = %err, "subnet discovery failed");
                        Vec::new()
                    }
                },
                _ = shutdown.changed() => break,
            }
        };

        for identity in found {
            if known.contains_key(&identity) {
                continue;
            }
            info!(device = %identity.key(), "starting device poller");
            let health = Arc::new(DeviceHealth::default());
            known.insert(identity.clone(), health.clone());
            health_registry.register_device(identity.key(), health.clone());
            tasks.spawn(run_device(
                device_config(&config, identity),
                sender.clone(),
                shutdown.clone(),
                health,
            ));
        }
        gauge!("sunspec_devices_known").set(known.len() as f64);
        if known.is_empty() {
            warn!(retry_in_s = rescan.as_secs(), "no devices found");
        }

        if static_mode {
            let _ = shutdown.wait_for(|stop| *stop).await;
            break;
        }
        tokio::select! {
            _ = sleep(rescan) => {}
            _ = shutdown.wait_for(|stop| *stop) => break,
        }
    }

    drop(sender);
    let stopped = timeout(STOP_GRACE, async {
        while tasks.join_next().await.is_some() {}
    })
    .await;
    if stopped.is_err() {
        warn!("device pollers did not stop in time; aborting them");
        tasks.abort_all();
        while tasks.join_next().await.is_some() {}
    }
}

fn device_config(config: &CollectorConfig, identity: DeviceIdentity) -> DeviceConfig {
    let mut actor = config.poller.clone();
    actor.respawn_delay = Duration::from_millis(config.respawn_delay_ms);
    DeviceConfig {
        identity,
        modbus: config.modbus.clone(),
        actor,
        base_address: config.base_address,
    }
}
