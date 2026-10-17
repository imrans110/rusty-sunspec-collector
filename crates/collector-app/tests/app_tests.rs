use std::collections::BTreeSet;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use buffer::BufferStore;
use collector_app::CollectorConfig;
use poller_actor::PollSample;
use sunspec_sim::{parse_models, SimConfig, SimHandle, UnitSpec, Unmapped};
use types::DeviceIdentity;

/// A collector polling the simulator every 100 ms whose uplink never drains, so the
/// buffer holds everything collected.
fn config(sim: &SimHandle, buffer_path: &str) -> CollectorConfig {
    let mut config = CollectorConfig::default();
    config.modbus.port = sim.port();
    config.discovery.port = sim.port();
    config.modbus.retry_count = 0;
    config.poller.poll_interval = Duration::from_millis(100);
    config.poller.request_timeout = Duration::from_millis(300);
    config.respawn_delay_ms = 100;
    config.buffer_path = buffer_path.to_string();
    config.buffer_drain_interval_ms = 3_600_000;
    config
}

fn static_devices(units: &[u8]) -> Vec<DeviceIdentity> {
    units
        .iter()
        .map(|&unit_id| DeviceIdentity {
            ip: "127.0.0.1".to_string(),
            unit_id,
        })
        .collect()
}

async fn run_for(config: CollectorConfig, duration: Duration) {
    collector_app::run(config, None, tokio::time::sleep(duration))
        .await
        .expect("collector run");
}

async fn buffered_samples(path: &str) -> Vec<PollSample> {
    let buffer = BufferStore::new(path).await.expect("open buffer");
    buffer
        .dequeue_batch(100_000)
        .await
        .expect("read buffer")
        .iter()
        .map(|row| serde_json::from_slice(&row.payload).expect("sample json"))
        .collect()
}

#[tokio::test]
async fn every_unit_behind_a_gateway_is_polled() {
    let sim = sunspec_sim::start(
        SimConfig {
            units: (1..=3).map(UnitSpec::inverter).collect(),
            unmapped: Unmapped::Exception,
        },
        "127.0.0.1:0".parse().unwrap(),
    )
    .await
    .unwrap();
    let path = temp_db_path("gateway");
    let mut config = config(&sim, &path);
    config.discovery.static_devices = static_devices(&[1, 2, 3]);

    run_for(config, Duration::from_millis(1_500)).await;

    let units: BTreeSet<u8> = buffered_samples(&path)
        .await
        .iter()
        .map(|sample| sample.device.unit_id)
        .collect();
    assert_eq!(units, BTreeSet::from([1, 2, 3]));
    cleanup(&path);
}

#[tokio::test]
async fn every_model_in_a_long_chain_is_polled() {
    let chain = parse_models("1:66,103:50,120:26,121:30,122:44,123:24,160:48").unwrap();
    let sim = sunspec_sim::start(
        SimConfig::single(UnitSpec::with_models(1, chain)),
        "127.0.0.1:0".parse().unwrap(),
    )
    .await
    .unwrap();
    let path = temp_db_path("models");
    let mut config = config(&sim, &path);
    config.discovery.static_devices = static_devices(&[1]);

    run_for(config, Duration::from_millis(1_500)).await;

    let models: BTreeSet<u16> = buffered_samples(&path)
        .await
        .iter()
        .map(|sample| sample.model_id)
        .collect();
    assert_eq!(models, BTreeSet::from([1, 103, 120, 121, 122, 123, 160]));
    cleanup(&path);
}

#[tokio::test]
async fn subnet_mode_keeps_running_and_finds_devices_on_rescan() {
    let sim = sunspec_sim::start(
        SimConfig::single(UnitSpec::inverter(1)),
        "127.0.0.1:0".parse().unwrap(),
    )
    .await
    .unwrap();
    sim.go_offline();
    let path = temp_db_path("rescan");
    let mut config = config(&sim, &path);
    config.discovery.subnet = "127.0.0.1/32".to_string();
    config.rescan_interval_s = 1;

    let online = sim.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(1_300)).await;
        online.go_online();
    });
    let started = std::time::Instant::now();
    run_for(config, Duration::from_millis(3_500)).await;

    assert!(
        started.elapsed() >= Duration::from_millis(3_500),
        "collector must not exit while no devices are found"
    );
    let samples = buffered_samples(&path).await;
    assert!(
        !samples.is_empty(),
        "device that appeared later was found by a rescan"
    );
    cleanup(&path);
}

fn temp_db_path(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir()
        .join(format!(
            "app-{prefix}-{}-{nanos}.sqlite",
            std::process::id()
        ))
        .display()
        .to_string()
}

fn cleanup(path: &str) {
    for suffix in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(PathBuf::from(format!("{path}{suffix}")));
    }
}
