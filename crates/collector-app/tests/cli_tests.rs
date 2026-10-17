use std::process::Command;

fn run(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_collector-app"))
        .args(args)
        .env_clear()
        .output()
        .expect("run collector-app");
    assert!(output.status.success(), "{args:?} failed: {output:?}");
    String::from_utf8(output.stdout).expect("utf-8 output")
}

// Release packaging and the install script rely on `--version` printing exactly this.
#[test]
fn version_prints_the_package_version() {
    let expected = format!("collector-app {}\n", env!("CARGO_PKG_VERSION"));
    assert_eq!(run(&["--version"]), expected);
    assert_eq!(run(&["-V"]), expected);
}

#[test]
fn help_lists_the_flags() {
    let help = run(&["--help"]);
    for flag in ["--config", "--print-schema", "--version"] {
        assert!(help.contains(flag), "{flag} missing from:\n{help}");
    }
}

#[test]
fn print_schema_prints_the_avro_schema() {
    let schema: serde_json::Value =
        serde_json::from_str(&run(&["--print-schema"])).expect("schema is JSON");
    assert_eq!(schema["type"], "record");
}

// `--probe` prints only JSON on stdout (so it can be redirected to a file) and its summary
// on stderr, with serial numbers blanked by default.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn probe_prints_a_dump_of_the_device() {
    let sim = sunspec_sim::start(
        sunspec_sim::SimConfig::single(sunspec_sim::UnitSpec::inverter(7)),
        "127.0.0.1:0".parse().unwrap(),
    )
    .await
    .expect("start simulator");
    sim.set_text(7, 1, "SN", "SERIAL-123");
    let port = sim.port().to_string();

    let output = tokio::task::spawn_blocking(move || {
        Command::new(env!("CARGO_BIN_EXE_collector-app"))
            .args(["--probe", "127.0.0.1:7"])
            .env_clear()
            .env("SUNSPEC_PORT", port)
            .output()
            .expect("run collector-app")
    })
    .await
    .unwrap();

    assert!(output.status.success(), "{output:?}");
    let dump: types::DeviceDump =
        serde_json::from_slice(&output.stdout).expect("stdout is only the JSON dump");
    assert_eq!(dump.unit_id, 7);
    assert_eq!(dump.base_address, 40_000);
    let models: Vec<u16> = dump.models.iter().map(|model| model.id).collect();
    assert_eq!(models, vec![1, 103]);
    assert_eq!(dump.redacted, vec!["1.SN".to_string()]);
    assert!(!String::from_utf8_lossy(&output.stdout).contains("SERIAL-123"));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("models 1, 103"), "{stderr}");
}

#[test]
fn probe_rejects_a_bad_target() {
    let output = Command::new(env!("CARGO_BIN_EXE_collector-app"))
        .args(["--probe", "10.0.0.5:300"])
        .env_clear()
        .output()
        .expect("run collector-app");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("--probe"));
}
