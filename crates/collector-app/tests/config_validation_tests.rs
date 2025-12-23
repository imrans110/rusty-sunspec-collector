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

fn fixture_path(name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(name);
    path.to_string_lossy().to_string()
}
