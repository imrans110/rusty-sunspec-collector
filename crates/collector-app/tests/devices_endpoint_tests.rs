//! `/devices`: live decoded readings, served on the metrics port.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use collector_app::CollectorConfig;
use serde_json::Value;
use sunspec_sim::{SimConfig, UnitSpec, Unmapped};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use types::DeviceIdentity;

/// The R14 acceptance test: a collector polling the simulator shows each unit's latest
/// scaled values within a few poll intervals.
#[tokio::test]
async fn devices_shows_the_latest_decoded_readings() {
    let sim = sunspec_sim::start(
        SimConfig {
            units: vec![UnitSpec::inverter(1)],
            unmapped: Unmapped::Exception,
        },
        "127.0.0.1:0".parse().unwrap(),
    )
    .await
    .unwrap();
    sim.set_point(1, 103, "W", &[4321]);
    sim.set_text(1, 1, "Mn", "Rusty Sun");
    let port = free_port();
    let buffer_path = std::env::temp_dir()
        .join(format!("devices-{}-{}.sqlite", std::process::id(), nanos()))
        .display()
        .to_string();
    let mut config = CollectorConfig::default();
    // Unit 7 does not exist: it is listed, as down and without readings.
    config.discovery.static_devices = [1, 7]
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
        tokio::time::sleep(Duration::from_secs(3)),
    ));
    tokio::time::sleep(Duration::from_millis(1_000)).await;

    let (status, body) = get_json(port, "/devices").await;
    assert_eq!(status, 200, "{body}");
    let devices = body["devices"].as_array().expect("devices array");
    let keys: Vec<&str> = devices
        .iter()
        .map(|d| d["device"].as_str().unwrap())
        .collect();
    assert_eq!(keys, vec!["127.0.0.1/1", "127.0.0.1/7"]);

    let unit1 = &devices[0];
    assert_eq!(unit1["up"], true);
    assert_eq!(unit1["base_address"], 40_000);
    assert_eq!(unit1["models"], serde_json::json!([1, 103]));
    assert_eq!(unit1["polled_models"], serde_json::json!([1, 103]));
    let readings = unit1["readings"].as_array().unwrap();
    assert_eq!(readings[0]["model_id"], 1);
    assert_eq!(readings[0]["points"]["Mn"], "Rusty Sun");
    assert_eq!(readings[1]["model_name"], "inverter_three_phase");
    assert_eq!(readings[1]["points"]["W"], 4321.0);
    assert!(readings[1]["age_s"].as_f64().unwrap() < 1.0);
    let names: Vec<&String> = readings[1]["points"].as_object().unwrap().keys().collect();
    assert_eq!(names[0], "A", "points keep definition order");

    let unit7 = &devices[1];
    assert_eq!(unit7["up"], false);
    assert_eq!(unit7["readings"], serde_json::json!([]));
    assert_eq!(unit7["base_address"], Value::Null);

    // A new value shows up within a few poll intervals.
    sim.set_point(1, 103, "W", &[1234]);
    tokio::time::sleep(Duration::from_millis(400)).await;
    let (status, one) = get_json(port, "/devices/127.0.0.1/1").await;
    assert_eq!(status, 200, "{one}");
    assert_eq!(one["readings"][1]["points"]["W"], 1234.0);

    let (status, missing) = get_json(port, "/devices/127.0.0.1/9").await;
    assert_eq!(status, 404);
    assert_eq!(missing["error"], "unknown device 127.0.0.1/9");

    app.await.unwrap().expect("collector run");
    for suffix in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{buffer_path}{suffix}"));
    }
}

async fn get_json(port: u16, path: &str) -> (u16, Value) {
    let mut stream = TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect to metrics server");
    let request = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).await.unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).await.unwrap();
    let status = response[9..12].parse().expect("status code");
    let body = response.split_once("\r\n\r\n").expect("headers end").1;
    (status, serde_json::from_str(body).unwrap_or(Value::Null))
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
