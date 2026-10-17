use std::env;
use std::path::PathBuf;
use std::sync::Mutex;

use collector_app::CollectorConfig;

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn toml_config_validates() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    env::set_var("SUNSPEC_CONFIG", fixture_path("config-valid.toml"));

    let config = CollectorConfig::load().expect("load config");
    config.validate().expect("validate config");

    env::remove_var("SUNSPEC_CONFIG");
}

#[test]
fn json_config_validates() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    env::set_var("SUNSPEC_CONFIG", fixture_path("config-valid.json"));

    let config = CollectorConfig::load().expect("load config");
    config.validate().expect("validate config");

    env::remove_var("SUNSPEC_CONFIG");
}

#[test]
fn invalid_config_fails_validation() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    env::set_var("SUNSPEC_CONFIG", fixture_path("config-invalid.toml"));

    let config = CollectorConfig::load().expect("load config");
    assert!(config.validate().is_err());

    env::remove_var("SUNSPEC_CONFIG");
}

#[test]
fn unknown_file_keys_are_rejected() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let path = write_temp("typo.toml", "[poller]\npoll_intervall_ms = 500\n");

    let err = CollectorConfig::load_with_path(Some(path.clone())).expect_err("typo in key");

    assert!(format!("{err:#}").contains("unknown field"), "{err:#}");
    let _ = std::fs::remove_file(path);
}

#[test]
fn unparsable_env_values_name_the_variable() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    env::set_var("SUNSPEC_POLL_INTERVAL_MS", "fast");

    let err = CollectorConfig::load().expect_err("not a number");

    env::remove_var("SUNSPEC_POLL_INTERVAL_MS");
    assert!(
        format!("{err:#}").contains("SUNSPEC_POLL_INTERVAL_MS"),
        "{err:#}"
    );
}

#[test]
fn invalid_static_device_unit_is_rejected() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    env::set_var("SUNSPEC_STATIC_DEVICES", "10.0.0.5:1,10.0.0.6:300");

    let err = CollectorConfig::load().expect_err("unit 300 does not fit a unit ID");

    env::remove_var("SUNSPEC_STATIC_DEVICES");
    assert!(format!("{err:#}").contains("10.0.0.6:300"), "{err:#}");
}

#[test]
fn static_devices_accept_hostnames_and_default_unit() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    env::set_var("SUNSPEC_STATIC_DEVICES", "inverter.local, 10.0.0.6:3");

    let config = CollectorConfig::load().expect("valid list");

    env::remove_var("SUNSPEC_STATIC_DEVICES");
    let keys: Vec<String> = config
        .discovery
        .static_devices
        .iter()
        .map(|device| device.key())
        .collect();
    assert_eq!(keys, vec!["inverter.local/1", "10.0.0.6/3"]);
}

#[test]
fn kafka_compression_and_acks_must_be_valid() {
    let mut config = CollectorConfig {
        kafka_compression: Some("brotli".to_string()),
        ..Default::default()
    };
    assert!(config.validate().is_err());

    config.kafka_compression = Some("lz4".to_string());
    config.kafka_acks = Some("most".to_string());
    assert!(config.validate().is_err());

    config.kafka_acks = Some("all".to_string());
    config.validate().expect("valid kafka settings");
}

#[test]
fn metrics_and_runtime_sections_are_applied() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let path = write_temp(
        "sections.toml",
        "[metrics]\nport = 9100\n\n[runtime]\nchannel_capacity = 512\nrespawn_delay_ms = 250\n",
    );

    let config = CollectorConfig::load_with_path(Some(path.clone())).expect("load");

    assert_eq!(config.metrics_port, 9100);
    assert_eq!(config.channel_capacity, 512);
    assert_eq!(config.respawn_delay_ms, 250);
    let _ = std::fs::remove_file(path);
}

fn write_temp(name: &str, content: &str) -> String {
    let path = std::env::temp_dir().join(format!("{}-{name}", std::process::id()));
    std::fs::write(&path, content).expect("write temp config");
    path.display().to_string()
}

fn fixture_path(name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(name);
    path.to_string_lossy().to_string()
}
