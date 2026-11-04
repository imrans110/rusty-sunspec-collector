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

#[test]
fn print_models_prints_the_catalog() {
    let catalog: serde_json::Value =
        serde_json::from_str(&run(&["--print-models"])).expect("catalog is JSON");
    let models = catalog.as_array().expect("array of models");
    assert!(models.len() >= 112);
    let inverter = models.iter().find(|model| model["id"] == 103).unwrap();
    assert_eq!(inverter["name"], "inverter_three_phase");
    let state = inverter["points"]
        .as_array()
        .unwrap()
        .iter()
        .find(|point| point["name"] == "St")
        .unwrap();
    assert_eq!(state["type"], "enum16");
    assert!(state["symbols"]
        .as_array()
        .unwrap()
        .iter()
        .any(|symbol| symbol["name"] == "MPPT" && symbol["value"] == 4));
}

// The systemd unit sets RestartPreventExitStatus=78, so a bad setting stops the service
// instead of restarting it every few seconds.
#[test]
fn a_bad_setting_exits_with_ex_config() {
    let output = Command::new(env!("CARGO_BIN_EXE_collector-app"))
        .env_clear()
        .env("SUNSPEC_KAFKA_BROKERS", "")
        .output()
        .expect("run collector-app");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(78), "{stderr}");
    assert!(stderr.contains("config validation failed"), "{stderr}");
    assert!(
        stderr.contains("kafka.brokers must be non-empty"),
        "{stderr}"
    );
}

#[test]
fn an_unparsable_env_value_exits_with_ex_config() {
    let output = Command::new(env!("CARGO_BIN_EXE_collector-app"))
        .env_clear()
        .env("SUNSPEC_PORT", "five-oh-two")
        .output()
        .expect("run collector-app");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(78), "{stderr}");
    assert!(stderr.contains("SUNSPEC_PORT"), "{stderr}");
}

// A metrics port in use may come free, so systemd should keep restarting.
#[test]
fn a_startup_failure_that_can_pass_exits_with_1() {
    let taken = std::net::TcpListener::bind("0.0.0.0:0").expect("bind a port");
    let port = taken.local_addr().expect("local addr").port();
    let output = Command::new(env!("CARGO_BIN_EXE_collector-app"))
        .env_clear()
        .env("SUNSPEC_METRICS_PORT", port.to_string())
        .env("SUNSPEC_STATIC_DEVICES", "127.0.0.1:1")
        .output()
        .expect("run collector-app");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(stderr.contains("bind metrics port"), "{stderr}");
}

// An assistant starts `collector-app --mcp URL` and speaks JSON-RPC on its stdin and stdout.
// Every stdout line must be protocol JSON: logs go to stderr.
#[test]
fn mcp_mode_speaks_json_rpc_on_stdio() {
    use std::io::{BufRead, BufReader, Write};
    use std::process::Stdio;

    // Nothing listens on this port, so the tool call reports an unreachable collector.
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let mut child = Command::new(env!("CARGO_BIN_EXE_collector-app"))
        .args(["--mcp", &format!("http://127.0.0.1:{port}")])
        .env_clear()
        .env("RUST_LOG", "debug")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("start collector-app --mcp");
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    let mut next = || {
        let mut line = String::new();
        stdout.read_line(&mut line).expect("read a response");
        serde_json::from_str::<serde_json::Value>(&line)
            .unwrap_or_else(|err| panic!("stdout line is not JSON ({err}): {line:?}"))
    };
    for request in [
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"cli-test","version":"0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
        r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"collector_health","arguments":{}}}"#,
    ] {
        writeln!(stdin, "{request}").unwrap();
    }
    stdin.flush().unwrap();

    let mut responses = std::collections::BTreeMap::new();
    while responses.len() < 3 {
        let message = next();
        if let Some(id) = message["id"].as_u64() {
            responses.insert(id, message);
        }
    }
    assert_eq!(
        responses[&1]["result"]["serverInfo"]["name"],
        "rusty-sunspec-collector"
    );
    assert_eq!(
        responses[&2]["result"]["tools"].as_array().unwrap().len(),
        4
    );
    assert_eq!(responses[&3]["result"]["isError"], true);
    let message = responses[&3]["result"]["content"][0]["text"]
        .as_str()
        .unwrap();
    assert!(
        message.contains("could not reach the collector"),
        "{message}"
    );

    // Closing stdin ends the session.
    drop(stdin);
    let status = child.wait().expect("collector-app --mcp exits");
    assert!(status.success(), "{status}");
}

#[test]
fn mcp_needs_a_collector_url() {
    let output = Command::new(env!("CARGO_BIN_EXE_collector-app"))
        .arg("--mcp")
        .env_clear()
        .output()
        .expect("run collector-app");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("--mcp needs the collector's URL"));
}
