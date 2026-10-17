//! Devices behind an RTU-over-TCP serial converter.

use std::collections::BTreeSet;
use std::env;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use buffer::BufferStore;
use collector_app::CollectorConfig;
use modbus_client::Framing;
use poller_actor::Sample;
use sunspec_sim::{SimConfig, SimHandle, UnitSpec, Unmapped};
use types::DeviceIdentity;

static ENV_LOCK: Mutex<()> = Mutex::new(());

async fn converter(units: &[u8]) -> SimHandle {
    sunspec_sim::start_rtu_over_tcp(
        SimConfig {
            units: units.iter().copied().map(UnitSpec::inverter).collect(),
            unmapped: Unmapped::Exception,
        },
        "127.0.0.1:0".parse().unwrap(),
    )
    .await
    .expect("start simulator")
}

fn load(toml: &str) -> anyhow::Result<CollectorConfig> {
    let path = env::temp_dir().join(format!(
        "{}-rtu-{}.toml",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::write(&path, toml).unwrap();
    let result = {
        let _guard = ENV_LOCK.lock().unwrap();
        CollectorConfig::load_with_path(Some(path.display().to_string()))
    };
    let _ = std::fs::remove_file(path);
    let config = result?;
    config.validate()?;
    Ok(config)
}

fn device(ip: &str, unit_id: u8) -> DeviceIdentity {
    DeviceIdentity {
        ip: ip.to_string(),
        unit_id,
    }
}

#[test]
fn framing_comes_from_the_file_the_environment_and_device_entries() {
    let config = load(
        r#"
[modbus]
framing = "rtu"

[[discovery.static_devices]]
ip = "10.0.0.9"
unit_id = 1

[[discovery.static_devices]]
ip = "10.0.0.10"
framing = "tcp"
"#,
    )
    .expect("valid");
    assert_eq!(config.modbus.framing, Framing::Rtu);
    assert_eq!(
        config.device_config(device("10.0.0.9", 1)).modbus.framing,
        Framing::Rtu
    );
    assert_eq!(
        config.device_config(device("10.0.0.10", 1)).modbus.framing,
        Framing::Tcp
    );

    let _guard = ENV_LOCK.lock().unwrap();
    env::set_var("SUNSPEC_MODBUS_FRAMING", "rtu");
    let from_env = CollectorConfig::load();
    env::set_var("SUNSPEC_MODBUS_FRAMING", "ascii");
    let invalid = CollectorConfig::load();
    env::remove_var("SUNSPEC_MODBUS_FRAMING");
    assert_eq!(from_env.unwrap().modbus.framing, Framing::Rtu);
    let err = invalid.expect_err("ascii is not supported");
    assert!(
        format!("{err:#}").contains("SUNSPEC_MODBUS_FRAMING must be tcp or rtu"),
        "{err:#}"
    );
}

#[test]
fn units_on_one_converter_must_agree_on_connection_settings() {
    let base = r#"
[modbus]
framing = "rtu"

[[discovery.static_devices]]
ip = "10.0.0.9"
unit_id = 1
"#;
    let err = load(&format!(
        "{base}\n[[discovery.static_devices]]\nip = \"10.0.0.9\"\nunit_id = 2\nrequest_timeout_ms = 3000\n"
    ))
    .expect_err("different timeouts on one converter");
    assert!(
        format!("{err:#}").contains("share the RTU converter at 10.0.0.9:502"),
        "{err:#}"
    );

    // Another host, or Modbus TCP on the same host, does not share the converter.
    load(&format!(
        "{base}\n[[discovery.static_devices]]\nip = \"10.0.0.10\"\nunit_id = 2\nrequest_timeout_ms = 3000\n"
    ))
    .expect("different converters");
    load(&format!(
        "{base}\n[[discovery.static_devices]]\nip = \"10.0.0.9\"\nunit_id = 2\nframing = \"tcp\"\nrequest_timeout_ms = 3000\n"
    ))
    .expect("a Modbus TCP device on the same host");
    // Per-device poll intervals may differ: requests are serialised either way.
    load(&format!(
        "{base}\n[[discovery.static_devices]]\nip = \"10.0.0.9\"\nunit_id = 2\npoll_interval_ms = 5000\n"
    ))
    .expect("different poll intervals");
}

/// The R13 acceptance test: two units behind one simulated converter are discovered
/// and polled end to end over a single connection.
#[tokio::test]
async fn units_behind_one_converter_are_polled_over_one_connection() {
    let sim = converter(&[1, 2]).await;
    let buffer_path = temp_db_path();
    let mut config = CollectorConfig::default();
    config.modbus.port = sim.port();
    config.discovery.port = sim.port();
    config.modbus.framing = Framing::Rtu;
    config.modbus.retry_count = 0;
    config.poller.poll_interval = Duration::from_millis(100);
    config.poller.request_timeout = Duration::from_millis(300);
    config.respawn_delay_ms = 100;
    config.discovery.static_devices = vec![device("127.0.0.1", 1), device("127.0.0.1", 2)];
    config.buffer_path = buffer_path.clone();
    config.buffer_drain_interval_ms = 3_600_000;
    config.shutdown_drain_timeout_ms = 0;
    config.metrics_port = 0;
    config.validate().expect("valid");

    collector_app::run(
        config,
        None,
        tokio::time::sleep(Duration::from_millis(1_500)),
    )
    .await
    .expect("collector run");

    let samples = buffered_samples(&buffer_path).await;
    let polled: BTreeSet<(u8, u16)> = samples
        .iter()
        .map(|sample| (sample.device.unit_id, sample.model_id))
        .collect();
    assert_eq!(polled, BTreeSet::from([(1, 1), (1, 103), (2, 1), (2, 103)]));
    for unit in [1, 2] {
        let polls = samples
            .iter()
            .filter(|sample| sample.device.unit_id == unit)
            .count();
        assert!(polls >= 10, "unit {unit} polled only {polls} times");
    }
    assert_eq!(
        sim.connections_accepted(),
        1,
        "one connection to the converter"
    );
    cleanup(&buffer_path);
}

#[tokio::test]
async fn probe_works_through_a_converter() {
    let sim = converter(&[4]).await;
    let mut config = CollectorConfig::default();
    config.modbus.port = sim.port();
    config.modbus.framing = Framing::Rtu;
    config.modbus.retry_count = 0;

    let dump = collector_app::probe::probe(&config, device("127.0.0.1", 4), false)
        .await
        .expect("probe");

    let models: Vec<u16> = dump.models.iter().map(|model| model.id).collect();
    assert_eq!(models, vec![1, 103]);
    assert!(dump.models.iter().all(|model| model.error.is_none()));
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

fn temp_db_path() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    env::temp_dir()
        .join(format!("rtu-{}-{nanos}.sqlite", std::process::id()))
        .display()
        .to_string()
}

fn cleanup(path: &str) {
    for suffix in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(PathBuf::from(format!("{path}{suffix}")));
    }
}
