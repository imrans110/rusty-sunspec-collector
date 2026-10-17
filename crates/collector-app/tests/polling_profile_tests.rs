use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use buffer::BufferStore;
use collector_app::CollectorConfig;
use poller_actor::Sample;
use sunspec_sim::{parse_models, SimConfig, SimHandle, UnitSpec, Unmapped};
use types::DeviceIdentity;

static ENV_LOCK: Mutex<()> = Mutex::new(());

const PROFILE_TOML: &str = r#"
[poller]
poll_interval_ms = 1000
exclude_models = [123]

[poller.model_intervals_s]
1 = 3600
120 = 3600
121 = 600

[[discovery.static_devices]]
ip = "10.0.0.5"

[[discovery.static_devices]]
ip = "10.0.0.6"
unit_id = 126
poll_interval_ms = 5000
request_timeout_ms = 2500
base_address = 0
max_batch_size = 60
include_models = [1, 103, 160]

[discovery.static_devices.model_intervals_s]
121 = 60
160 = 10
"#;

fn device(ip: &str, unit_id: u8) -> DeviceIdentity {
    DeviceIdentity {
        ip: ip.to_string(),
        unit_id,
    }
}

fn secs(seconds: u64) -> Duration {
    Duration::from_secs(seconds)
}

#[test]
fn profile_settings_load_from_toml_and_merge_per_device() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let path = write_temp("profile.toml", PROFILE_TOML);

    let config = CollectorConfig::load_with_path(Some(path.clone())).expect("load");
    config.validate().expect("valid profile");

    let global = &config.poller.models;
    assert_eq!(global.include, None);
    assert_eq!(global.exclude, BTreeSet::from([123]));
    assert_eq!(
        global.intervals,
        BTreeMap::from([(1, secs(3600)), (120, secs(3600)), (121, secs(600))])
    );

    let plain = config.device_config(device("10.0.0.5", 1));
    assert_eq!(plain.actor.poll_interval, Duration::from_millis(1000));
    assert_eq!(plain.actor.models, *global);
    assert_eq!(plain.base_address, 40_000);

    let tuned = config.device_config(device("10.0.0.6", 126));
    assert_eq!(tuned.actor.poll_interval, Duration::from_millis(5000));
    assert_eq!(tuned.actor.request_timeout, Duration::from_millis(2500));
    assert_eq!(tuned.base_address, 0);
    assert_eq!(tuned.modbus.max_batch_size, Some(60));
    assert_eq!(
        tuned.actor.models.include,
        Some(BTreeSet::from([1, 103, 160]))
    );
    assert_eq!(
        tuned.actor.models.exclude,
        BTreeSet::from([123]),
        "not overridden, so the global list applies"
    );
    assert_eq!(
        tuned.actor.models.intervals,
        BTreeMap::from([
            (1, secs(3600)),
            (120, secs(3600)),
            (121, secs(60)),
            (160, secs(10))
        ]),
        "merged model by model, the device winning"
    );
    let _ = std::fs::remove_file(path);
}

#[test]
fn profile_settings_load_from_json() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let path = write_temp(
        "profile.json",
        r#"{
          "poller": { "include_models": [1, 103], "model_intervals_s": { "1": 900 } },
          "discovery": { "static_devices": [ { "ip": "10.0.0.5", "exclude_models": [1] } ] }
        }"#,
    );

    let config = CollectorConfig::load_with_path(Some(path.clone())).expect("load");
    config
        .validate()
        .expect_err("model 1 is included globally and excluded on the device");

    assert_eq!(config.poller.models.include, Some(BTreeSet::from([1, 103])));
    assert_eq!(
        config.poller.models.intervals,
        BTreeMap::from([(1, secs(900))])
    );
    let _ = std::fs::remove_file(path);
}

#[test]
fn environment_variables_set_the_global_profile() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    env::set_var("SUNSPEC_INCLUDE_MODELS", "1, 103,160");
    env::set_var("SUNSPEC_EXCLUDE_MODELS", "160");
    env::set_var("SUNSPEC_MODEL_INTERVALS_S", "1=3600, 120=600");

    let loaded = CollectorConfig::load();
    env::set_var("SUNSPEC_INCLUDE_MODELS", "");
    let empty_include = CollectorConfig::load();
    env::set_var("SUNSPEC_MODEL_INTERVALS_S", "1:3600");
    let bad_interval = CollectorConfig::load();
    for key in [
        "SUNSPEC_INCLUDE_MODELS",
        "SUNSPEC_EXCLUDE_MODELS",
        "SUNSPEC_MODEL_INTERVALS_S",
    ] {
        env::remove_var(key);
    }

    let models = loaded.expect("load").poller.models;
    assert_eq!(models.include, Some(BTreeSet::from([1, 103, 160])));
    assert_eq!(models.exclude, BTreeSet::from([160]));
    assert_eq!(
        models.intervals,
        BTreeMap::from([(1, secs(3600)), (120, secs(600))])
    );
    assert_eq!(
        empty_include.expect("load").poller.models.include,
        None,
        "an empty value means every model"
    );
    let err = bad_interval.expect_err("model:seconds is not model=seconds");
    assert!(
        format!("{err:#}").contains("SUNSPEC_MODEL_INTERVALS_S"),
        "{err:#}"
    );
}

#[test]
fn env_static_devices_replace_the_file_list_and_its_overrides() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let path = write_temp("profile-env.toml", PROFILE_TOML);
    env::set_var("SUNSPEC_STATIC_DEVICES", "10.0.0.6:126");

    let loaded = CollectorConfig::load_with_path(Some(path.clone()));

    env::remove_var("SUNSPEC_STATIC_DEVICES");
    let config = loaded.expect("load");
    assert!(config.device_overrides.is_empty());
    let device = config.device_config(device("10.0.0.6", 126));
    assert_eq!(device.actor.poll_interval, Duration::from_millis(1000));
    let _ = std::fs::remove_file(path);
}

#[test]
fn invalid_profile_settings_are_rejected() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let device = "[[discovery.static_devices]]\nip = \"10.0.0.5\"\n";
    let cases = [
        ("[poller]\ninclude_models = []\n", "at least one model"),
        (
            "[poller]\nexclude_models = [65535]\n",
            "not a SunSpec model ID",
        ),
        ("[poller]\ninclude_models = [0]\n", "not a SunSpec model ID"),
        (
            "[poller]\ninclude_models = [103]\nexclude_models = [103]\n",
            "in both",
        ),
        ("[poller.model_intervals_s]\n120 = 0\n", "at least 1 second"),
        (
            "[poller.model_intervals_s]\nnameplate = 60\n",
            "'nameplate' is not a model ID",
        ),
        (
            &format!("{device}poll_interval_ms = 0\n"),
            "poll_interval_ms must be >= 1",
        ),
        (
            &format!("{device}request_timeout_ms = 0\n"),
            "request_timeout_ms must be >= 1",
        ),
        (&format!("{device}max_batch_size = 126\n"), "max_batch_size"),
        (&format!("{device}pollinterval_ms = 500\n"), "unknown field"),
        (
            &format!("[poller]\nexclude_models = [123]\n\n{device}include_models = [123]\n"),
            "both in include_models and exclude_models",
        ),
        (&format!("{device}\n{device}"), "10.0.0.5/1 twice"),
    ];

    for (index, (toml, expected)) in cases.iter().enumerate() {
        let path = write_temp(&format!("invalid-profile-{index}.toml"), toml);
        let result = CollectorConfig::load_with_path(Some(path.clone())).and_then(|config| {
            config.validate()?;
            Ok(config)
        });
        let _ = std::fs::remove_file(path);
        let err = result.expect_err(toml);
        assert!(
            format!("{err:#}").contains(expected),
            "{toml}\nexpected {expected:?} in: {err:#}"
        );
    }
}

/// Every TOML and JSON example in these sections of docs/configuration.md must load
/// and validate, so the documentation cannot drift from the code.
#[test]
fn documented_config_examples_are_valid() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let docs = std::fs::read_to_string(repo_path("docs/configuration.md")).expect("read docs");
    let mut checked = 0;
    for section in ["## Polling profiles", "## Complete examples"] {
        let start = docs.find(section).expect(section);
        let rest = &docs[start + section.len()..];
        let body = &rest[..rest.find("\n## ").unwrap_or(rest.len())];
        for (index, block) in body.split("```").enumerate().skip(1).step_by(2) {
            let Some((lang, content)) = block.split_once('\n') else {
                continue;
            };
            if lang != "toml" && lang != "json" {
                continue;
            }
            let path = write_temp(&format!("doc-example-{checked}-{index}.{lang}"), content);
            let result = CollectorConfig::load_with_path(Some(path.clone()))
                .and_then(|config| config.validate());
            let _ = std::fs::remove_file(path);
            result.unwrap_or_else(|err| panic!("{section} example:\n{content}\n{err:#}"));
            checked += 1;
        }
    }
    assert!(checked >= 4, "found only {checked} examples");
}

fn repo_path(relative: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../..");
    path.push(relative);
    path
}

/// The R3 acceptance test: a config file with slow models and per-device overrides,
/// run end to end, reads the slow models less often (counted by the simulator).
#[tokio::test]
async fn a_profile_from_a_config_file_reads_slow_models_less_often() {
    let gateway = vec![
        UnitSpec::with_models(1, parse_models("1,103,120").unwrap()),
        // Behind the same address, at a base the collector would not probe by itself.
        UnitSpec {
            unit_id: 2,
            base: 1_000,
            models: parse_models("1,103,160:48").unwrap(),
        },
    ];
    let sim = sunspec_sim::start(
        SimConfig {
            units: gateway,
            unmapped: Unmapped::Exception,
        },
        "127.0.0.1:0".parse().unwrap(),
    )
    .await
    .unwrap();
    let buffer_path = temp_db_path("profile");
    let config_path = write_temp(
        "profile-e2e.toml",
        &format!(
            r#"
[discovery]
port = {port}

[poller]
poll_interval_ms = 100
request_timeout_ms = 300

[poller.model_intervals_s]
1 = 1
120 = 1

[[discovery.static_devices]]
ip = "127.0.0.1"
unit_id = 1

[[discovery.static_devices]]
ip = "127.0.0.1"
unit_id = 2
base_address = 1000
poll_interval_ms = 200
include_models = [1, 103]

[discovery.static_devices.model_intervals_s]
1 = 60

[modbus]
retry_count = 0

[buffer]
path = "{buffer_path}"
drain_interval_ms = 3600000
shutdown_drain_timeout_ms = 0

[metrics]
port = 0

[runtime]
respawn_delay_ms = 100
"#,
            port = sim.port(),
        ),
    );
    let config = {
        // Other tests set SUNSPEC_* variables, which loading applies.
        let _guard = ENV_LOCK.lock().expect("env lock");
        CollectorConfig::load_with_path(Some(config_path.clone())).expect("load")
    };
    config.validate().expect("valid");
    let _ = std::fs::remove_file(config_path);

    collector_app::run(
        config,
        None,
        tokio::time::sleep(Duration::from_millis(2_500)),
    )
    .await
    .expect("collector run");

    // Unit 1 polls every 100 ms; models 1 and 120 only at about 0, 1 and 2 s.
    let fast = poll_reads(&sim, 1, 103);
    assert!(fast >= 15, "unit 1 model 103 read only {fast} times");
    for slow in [1, 120] {
        let reads = poll_reads(&sim, 1, slow);
        assert!(
            (2..=4).contains(&reads),
            "unit 1 model {slow} read {reads} times"
        );
    }
    // Unit 2: found at its own base, polled every 200 ms, model 1 once a minute,
    // model 160 left out by include_models.
    let fast = poll_reads(&sim, 2, 103);
    assert!(
        (7..=14).contains(&fast),
        "unit 2 model 103 read {fast} times"
    );
    assert_eq!(poll_reads(&sim, 2, 1), 1);
    assert_eq!(poll_reads(&sim, 2, 160), 0);

    let samples = buffered_samples(&buffer_path).await;
    assert!(samples
        .iter()
        .any(|sample| sample.device.unit_id == 2 && sample.model_id == 103));
    assert!(!samples
        .iter()
        .any(|sample| sample.device.unit_id == 2 && sample.model_id == 160));
    cleanup(&buffer_path);
}

/// Poll reads of a model, not counting the discovery read of its header.
fn poll_reads(sim: &SimHandle, unit_id: u8, model_id: u16) -> u64 {
    let start = sim.model_start(unit_id, model_id).expect("model in map");
    sim.reads_at(unit_id, start).saturating_sub(1)
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

fn write_temp(name: &str, content: &str) -> String {
    let path = env::temp_dir().join(format!("{}-{name}", std::process::id()));
    std::fs::write(&path, content).expect("write temp config");
    path.display().to_string()
}

fn temp_db_path(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    env::temp_dir()
        .join(format!(
            "profile-{prefix}-{}-{nanos}.sqlite",
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
