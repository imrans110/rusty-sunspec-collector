//! The read-only MCP server (R16): an MCP client talks to `collector_app::mcp` while a
//! collector polls the simulator, and every tool is called against live data.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use collector_app::mcp::{self, CollectorUrl};
use collector_app::CollectorConfig;
use rmcp::model::{CallToolRequestParams, CallToolResult};
use rmcp::service::{RoleClient, RunningService};
use rmcp::ServiceExt;
use serde_json::{json, Map, Value};
use sunspec_sim::{SimConfig, UnitSpec, Unmapped};
use types::DeviceIdentity;

/// The R16 acceptance test: an assistant connected to a collector that polls the simulator
/// lists devices, reads live values with their labels, explains a point from its
/// definition and checks the collector's health.
#[tokio::test]
async fn every_tool_answers_from_a_live_collector() {
    let sim = sunspec_sim::start(
        SimConfig {
            units: vec![UnitSpec::inverter(1), UnitSpec::inverter(2)],
            unmapped: Unmapped::Exception,
        },
        "127.0.0.1:0".parse().unwrap(),
    )
    .await
    .unwrap();
    sim.set_point(1, 103, "W", &[4300]);
    sim.set_point(1, 103, "St", &[4]);
    // Inverter 2 tripped on a ground fault: status FAULT, Evt1 bit 0 set.
    sim.set_point(2, 103, "W", &[0]);
    sim.set_point(2, 103, "St", &[7]);
    sim.set_point(2, 103, "Evt1", &[0, 1]);

    let port = free_port();
    let buffer_path = std::env::temp_dir()
        .join(format!("mcp-{}-{}.sqlite", std::process::id(), nanos()))
        .display()
        .to_string();
    let mut config = CollectorConfig::default();
    config.discovery.static_devices = [1, 2]
        .into_iter()
        .map(|unit_id| DeviceIdentity {
            ip: "127.0.0.1".to_string(),
            unit_id,
        })
        .collect();
    config.modbus.port = sim.port();
    config.modbus.retry_count = 0;
    config.poller.poll_interval = Duration::from_millis(100);
    config.poller.request_timeout = Duration::from_millis(200);
    config.buffer_path = buffer_path.clone();
    config.metrics_port = port;
    let app = tokio::spawn(collector_app::run(
        config,
        None,
        tokio::time::sleep(Duration::from_secs(6)),
    ));

    let client = connect(&format!("http://127.0.0.1:{port}")).await;
    let server = client.peer_info().expect("server info");
    let implementation = server.server_info.as_ref().expect("server implementation");
    assert_eq!(implementation.name, "rusty-sunspec-collector");
    let instructions = server.instructions.as_deref().unwrap_or_default();
    assert!(
        instructions.contains("Nothing here can change a device setting"),
        "{instructions}"
    );

    let mut names: Vec<String> = client
        .list_all_tools()
        .await
        .expect("tools/list")
        .into_iter()
        .map(|tool| tool.name.to_string())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            "collector_health",
            "explain_point",
            "get_readings",
            "list_devices"
        ]
    );

    // list_devices, once both inverters have been read.
    let mut devices = Value::Null;
    for _ in 0..40 {
        devices = call_ok(&client, "list_devices", json!({})).await;
        let up = devices["devices"]
            .as_array()
            .map(|ds| ds.iter().filter(|d| d["up"] == true).count())
            .unwrap_or(0);
        if up == 2 {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let list = devices["devices"].as_array().expect("devices");
    assert_eq!(list.len(), 2, "{devices}");
    assert_eq!(list[1]["device"], "127.0.0.1/2");
    assert_eq!(list[1]["up"], true);
    assert_eq!(list[1]["models"], json!([1, 103]));
    assert!(list[1]["seconds_since_last_success"].as_f64().unwrap() < 2.0);

    // get_readings: scaled values and labels, and the model filter.
    let two = call_ok(&client, "get_readings", json!({ "device": "127.0.0.1/2" })).await;
    let inverter = two["readings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["model_id"] == 103)
        .expect("model 103 reading");
    assert_eq!(inverter["points"]["W"], 0.0);
    assert_eq!(inverter["labels"]["St"], "FAULT");
    assert_eq!(inverter["labels"]["Evt1"], "GROUND_FAULT");
    let one = call_ok(
        &client,
        "get_readings",
        json!({ "device": "127.0.0.1/1", "model": 103 }),
    )
    .await;
    let readings = one["readings"].as_array().unwrap();
    assert_eq!(readings.len(), 1);
    assert_eq!(readings[0]["points"]["W"], 4300.0);
    assert_eq!(readings[0]["labels"]["St"], "MPPT");

    let missing = call_err(&client, "get_readings", json!({ "device": "127.0.0.1/9" })).await;
    assert!(
        missing.contains("known devices: 127.0.0.1/1, 127.0.0.1/2"),
        "{missing}"
    );
    let bad = call_err(&client, "get_readings", json!({ "device": "inverter two" })).await;
    assert!(bad.contains("host/unit"), "{bad}");
    let no_model = call_err(
        &client,
        "get_readings",
        json!({ "device": "127.0.0.1/1", "model": 160 }),
    )
    .await;
    assert!(no_model.contains("no reading of model 160"), "{no_model}");

    // explain_point: by symbol, by name, a repeating-group point, and the model's index.
    let fault = call_ok(
        &client,
        "explain_point",
        json!({ "model": 103, "point": "GROUND_FAULT" }),
    )
    .await;
    assert_eq!(fault["point"]["name"], "Evt1");
    assert_eq!(fault["matched_symbol"]["name"], "GROUND_FAULT");
    assert_eq!(fault["matched_symbol"]["value"], 0);
    let watts = call_ok(
        &client,
        "explain_point",
        json!({ "model": 103, "point": "w" }),
    )
    .await;
    assert_eq!(watts["point"]["name"], "W");
    assert_eq!(watts["point"]["units"], "W");
    let module_amps = call_ok(
        &client,
        "explain_point",
        json!({ "model": 160, "point": "module[3].DCA" }),
    )
    .await;
    assert_eq!(module_amps["point"]["name"], "module[*].DCA");
    let index = call_ok(&client, "explain_point", json!({ "model": 103 })).await;
    assert!(index["points"].as_array().unwrap().contains(&json!("St")));
    let unknown = call_err(&client, "explain_point", json!({ "model": 9999 })).await;
    assert!(
        unknown.contains("no bundled definition for model 9999"),
        "{unknown}"
    );
    let no_point = call_err(
        &client,
        "explain_point",
        json!({ "model": 103, "point": "NOPE" }),
    )
    .await;
    assert!(no_point.contains("no point or symbol named"), "{no_point}");

    // collector_health.
    let health = call_ok(&client, "collector_health", json!({})).await;
    assert_eq!(health["healthy"], true);
    assert_eq!(health["devices_up"], 2);
    assert_eq!(health["devices_total"], 2);

    client.cancel().await.expect("close the MCP session");
    app.await.unwrap().expect("collector run");
    for suffix in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{buffer_path}{suffix}"));
    }
}

/// Without a collector to talk to, every tool reports it instead of failing the session.
#[tokio::test]
async fn an_unreachable_collector_is_reported_by_every_tool() {
    let port = free_port();
    let client = connect(&format!("127.0.0.1:{port}")).await;
    for (tool, args) in [
        ("list_devices", json!({})),
        ("get_readings", json!({ "device": "127.0.0.1/1" })),
        ("explain_point", json!({ "model": 103 })),
        ("collector_health", json!({})),
    ] {
        let message = call_err(&client, tool, args).await;
        assert!(
            message.contains("could not reach the collector"),
            "{tool}: {message}"
        );
    }
    client.cancel().await.expect("close the MCP session");
}

async fn connect(url: &str) -> RunningService<RoleClient, ()> {
    let url = CollectorUrl::parse(url).expect("collector url");
    let (client_io, server_io) = tokio::io::duplex(256 * 1024);
    let (server_read, server_write) = tokio::io::split(server_io);
    tokio::spawn(mcp::serve(url, server_read, server_write));
    ().serve(tokio::io::split(client_io))
        .await
        .expect("MCP initialize")
}

async fn call(client: &RunningService<RoleClient, ()>, tool: &str, args: Value) -> CallToolResult {
    let arguments: Map<String, Value> = args.as_object().cloned().unwrap_or_default();
    client
        .call_tool(CallToolRequestParams::new(tool.to_string()).with_arguments(arguments))
        .await
        .unwrap_or_else(|err| panic!("{tool}: {err}"))
}

fn text(result: &CallToolResult) -> String {
    result
        .content
        .iter()
        .filter_map(|c| c.as_text().map(|t| t.text.clone()))
        .collect::<Vec<_>>()
        .join("\n")
}

async fn call_ok(client: &RunningService<RoleClient, ()>, tool: &str, args: Value) -> Value {
    let result = call(client, tool, args).await;
    assert_ne!(
        result.is_error,
        Some(true),
        "{tool} failed: {}",
        text(&result)
    );
    serde_json::from_str(&text(&result)).unwrap_or_else(|err| panic!("{tool}: {err}"))
}

async fn call_err(client: &RunningService<RoleClient, ()>, tool: &str, args: Value) -> String {
    let result = call(client, tool, args).await;
    assert_eq!(
        result.is_error,
        Some(true),
        "{tool} succeeded: {}",
        text(&result)
    );
    text(&result)
}

fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

fn nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}
