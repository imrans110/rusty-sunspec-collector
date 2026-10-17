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

#[test]
fn shipped_example_config_file_is_valid() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let path = repo_path("docs/config.example.toml");

    let config = CollectorConfig::load_with_path(Some(path)).expect("example config loads");

    config.validate().expect("example config validates");
}

/// systemd `EnvironmentFile=` and `docker --env-file` take everything after `=` as the
/// value (no trailing comments), so every assignment in the shipped env file must parse.
#[test]
fn shipped_example_env_file_is_valid() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let content =
        std::fs::read_to_string(repo_path("docs/sunspec-collector.env")).expect("read env file");
    let vars: Vec<(String, String)> = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (key, value) = line.split_once('=').expect("KEY=value");
            (key.to_string(), value.to_string())
        })
        .collect();
    assert!(!vars.is_empty());
    for (key, value) in &vars {
        env::set_var(key, value);
    }

    let result = CollectorConfig::load().and_then(|config| config.validate());

    for (key, _) in &vars {
        env::remove_var(key);
    }
    result.expect("example env file loads and validates");
}

fn repo_path(relative: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../..");
    path.push(relative);
    path.to_string_lossy().to_string()
}

#[test]
fn acks_below_all_require_idempotence_off() {
    let mut config = CollectorConfig {
        kafka_acks: Some("1".to_string()),
        ..Default::default()
    };
    let err = config.validate().expect_err("idempotence is on by default");
    assert!(err.to_string().contains("enable_idempotence"), "{err}");

    config.kafka_enable_idempotence = Some(false);
    config.validate().expect("acks=1 without idempotence");
}

#[test]
fn modbus_batch_size_is_limited_to_the_protocol_maximum() {
    let mut config = CollectorConfig::default();
    config.modbus.max_batch_size = Some(126);
    assert!(config.validate().is_err());

    config.modbus.max_batch_size = Some(125);
    config.validate().expect("125 is the protocol limit");
}

#[test]
fn sasl_needs_a_mechanism_username_and_password() {
    let mut config = CollectorConfig::default();
    config.kafka_security.protocol = Some("sasl_ssl".to_string());
    let err = config.validate().expect_err("no mechanism");
    assert!(err.to_string().contains("kafka.sasl_mechanism"), "{err}");

    config.kafka_security.sasl_mechanism = Some("SCRAM-SHA-512".to_string());
    config.kafka_security.sasl_username = Some("collector".to_string());
    let err = config.validate().expect_err("no password");
    assert!(err.to_string().contains("kafka.sasl_password"), "{err}");

    config.kafka_security.sasl_password = Some(avro_kafka::Secret::new("secret"));
    config.validate().expect("complete SASL settings");

    config.kafka_security.sasl_mechanism = Some("GSSAPI".to_string());
    assert!(config.validate().is_err(), "unsupported mechanism");
}

#[test]
fn security_settings_must_match_the_protocol() {
    let mut config = CollectorConfig::default();
    config.kafka_security.sasl_username = Some("collector".to_string());
    let err = config
        .validate()
        .expect_err("SASL settings without a SASL protocol");
    assert!(
        err.to_string().contains("sasl_plaintext or sasl_ssl"),
        "{err}"
    );

    let mut config = CollectorConfig::default();
    config.kafka_security.protocol = Some("ssl".to_string());
    config.kafka_security.ssl_ca_location = Some("/no/such/ca.pem".to_string());
    let err = config.validate().expect_err("missing CA file");
    assert!(err.to_string().contains("/no/such/ca.pem"), "{err}");

    config.kafka_security.ssl_ca_location = None;
    config.kafka_security.ssl_certificate_location = Some(write_temp("client.pem", "cert"));
    let err = config.validate().expect_err("certificate without key");
    assert!(err.to_string().contains("set together"), "{err}");

    config.kafka_security.protocol = Some("tls".to_string());
    assert!(config.validate().is_err(), "unknown protocol");
}

#[test]
fn sasl_password_is_read_from_a_file_and_never_printed() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let password_file = write_temp("kafka-password", "s3cr3t-from-file\n");
    env::set_var("SUNSPEC_KAFKA_SECURITY_PROTOCOL", "sasl_ssl");
    env::set_var("SUNSPEC_KAFKA_SASL_MECHANISM", "PLAIN");
    env::set_var("SUNSPEC_KAFKA_SASL_USERNAME", "collector");
    env::set_var("SUNSPEC_KAFKA_SASL_PASSWORD_FILE", &password_file);

    let loaded = CollectorConfig::load();
    env::set_var("SUNSPEC_KAFKA_SASL_PASSWORD", "also-set");
    let both = CollectorConfig::load();

    for key in [
        "SUNSPEC_KAFKA_SECURITY_PROTOCOL",
        "SUNSPEC_KAFKA_SASL_MECHANISM",
        "SUNSPEC_KAFKA_SASL_USERNAME",
        "SUNSPEC_KAFKA_SASL_PASSWORD_FILE",
        "SUNSPEC_KAFKA_SASL_PASSWORD",
    ] {
        env::remove_var(key);
    }
    let _ = std::fs::remove_file(&password_file);

    let config = loaded.expect("load");
    config.validate().expect("valid SASL config");
    let password = config
        .kafka_security
        .sasl_password
        .as_ref()
        .expect("password");
    assert_eq!(
        password.expose(),
        "s3cr3t-from-file",
        "trailing newline trimmed"
    );
    assert!(
        !format!("{config:?}").contains("s3cr3t"),
        "secret in Debug output"
    );

    let err = both.expect_err("password and password file together");
    assert!(format!("{err:#}").contains("only one of"), "{err:#}");
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
