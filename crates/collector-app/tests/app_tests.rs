use std::collections::BTreeSet;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use buffer::BufferStore;
use collector_app::CollectorConfig;
use poller_actor::Sample;
use sunspec_sim::{parse_models, SimConfig, SimHandle, UnitSpec, Unmapped};
use types::{DeviceIdentity, PointValue};

/// A collector polling the simulator every 100 ms whose uplink never drains (no
/// periodic drain, no final drain), so the buffer holds everything collected.
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
    config.shutdown_drain_timeout_ms = 0;
    // Each test gets a free port for /metrics and /healthz.
    config.metrics_port = 0;
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

async fn buffered_samples(path: &str) -> Vec<Sample> {
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

#[tokio::test]
async fn uplink_drains_a_backlog_without_pausing_between_batches() {
    let path = temp_db_path("drain");
    let sample = Sample {
        device: static_devices(&[1]).remove(0),
        model_id: 103,
        model_name: "three_phase_inverter".to_string(),
        start: 40_070,
        registers: vec![0; 52],
        collected_at_ms: 0,
        points: Vec::new(),
        labels: Vec::new(),
    };
    let payload = serde_json::to_vec(&sample).unwrap();
    {
        let buffer = BufferStore::new(&path).await.unwrap();
        buffer
            .enqueue_batch("sunspec.telemetry", &vec![payload; 5_000])
            .await
            .unwrap();
    }
    let mut config = CollectorConfig::default();
    // No devices: a /32 with nothing listening on the Modbus port.
    config.discovery.subnet = "127.0.0.1/32".to_string();
    config.discovery.port = 9;
    config.buffer_path = path.clone();
    config.buffer_batch_size = 100;
    config.buffer_drain_interval_ms = 500;
    config.shutdown_drain_timeout_ms = 0;
    config.metrics_port = 0;

    // With the old fixed pause between batches this drained 100 rows per 500 ms,
    // i.e. about 400 rows in 2 s.
    run_for(config, Duration::from_secs(2)).await;

    let remaining = BufferStore::new(&path)
        .await
        .unwrap()
        .pending_count()
        .await
        .unwrap();
    assert_eq!(remaining, 0, "5,000-row backlog should drain within 2 s");
    cleanup(&path);
}

#[tokio::test]
async fn publishes_decoded_records_keyed_by_device() {
    let sim = sunspec_sim::start(
        SimConfig {
            units: vec![UnitSpec::inverter(1), UnitSpec::inverter(2)],
            unmapped: Unmapped::Exception,
        },
        "127.0.0.1:0".parse().unwrap(),
    )
    .await
    .unwrap();
    sim.set_text(1, 1, "Mn", "Rusty SunSpec");
    sim.set_point(1, 103, "W", &[4321]);
    sim.set_point(1, 103, "W_SF", &[(-1i16) as u16]);
    sim.set_point(1, 103, "St", &[7]);
    sim.set_point(1, 103, "Evt1", &[0, 0b11]);
    let path = temp_db_path("records");
    let mut config = config(&sim, &path);
    config.discovery.static_devices = static_devices(&[1, 2]);
    config.buffer_drain_interval_ms = 100;
    config.shutdown_drain_timeout_ms = 2_000;
    let (publisher, sink) = avro_kafka::Publisher::new_memory("sunspec.telemetry");

    collector_app::run_with_publisher(
        config,
        None,
        publisher,
        tokio::time::sleep(Duration::from_millis(1_500)),
    )
    .await
    .expect("collector run");

    let records = sink.records();
    let keys: BTreeSet<&str> = records.iter().map(|record| record.key.as_str()).collect();
    assert_eq!(keys, BTreeSet::from(["127.0.0.1/1", "127.0.0.1/2"]));
    let samples: Vec<_> = records
        .iter()
        .map(|record| avro_kafka::decode(&record.payload).expect("valid single-object record"))
        .collect();
    for (record, sample) in records.iter().zip(&samples) {
        assert_eq!(
            record.key,
            sample.device.key(),
            "record key is the payload's device"
        );
    }

    let point = |model_id: u16, name: &str| {
        samples
            .iter()
            .find(|sample| sample.device.unit_id == 1 && sample.model_id == model_id)
            .and_then(|sample| sample.points.iter().find(|(point, _)| point == name))
            .map(|(_, value)| value.clone())
    };
    assert_eq!(point(103, "W"), Some(PointValue::Float(432.1)));
    assert_eq!(
        point(1, "Mn"),
        Some(PointValue::Text("Rusty SunSpec".to_string()))
    );
    let inverter = samples
        .iter()
        .find(|sample| sample.model_id == 103)
        .unwrap();
    assert_eq!(inverter.model_name, "inverter_three_phase");

    let unit1_inverter = samples
        .iter()
        .find(|sample| sample.device.unit_id == 1 && sample.model_id == 103)
        .unwrap();
    let label = |name: &str| {
        unit1_inverter
            .labels
            .iter()
            .find(|(point, _)| point == name)
            .map(|(_, label)| label.as_str())
    };
    assert_eq!(
        label("St"),
        Some("FAULT"),
        "enum names travel in the record"
    );
    assert_eq!(label("Evt1"), Some("GROUND_FAULT,DC_OVER_VOLT"));
    cleanup(&path);
}

#[tokio::test]
async fn a_taken_metrics_port_fails_startup() {
    let taken = std::net::TcpListener::bind("0.0.0.0:0").unwrap();
    let path = temp_db_path("port");
    let mut config = CollectorConfig::default();
    config.discovery.subnet = "127.0.0.1/32".to_string();
    config.discovery.port = 9;
    config.buffer_path = path.clone();
    config.metrics_port = taken.local_addr().unwrap().port();

    let result = collector_app::run(config, None, tokio::time::sleep(Duration::from_secs(5))).await;

    let err = result.expect_err("port already in use");
    assert!(format!("{err:#}").contains("bind metrics port"), "{err:#}");
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
