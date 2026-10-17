//! Installs the global metrics recorder, so it lives in its own test binary.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use collector_app::CollectorConfig;
use sunspec_sim::{SimConfig, UnitSpec};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use types::DeviceIdentity;

#[tokio::test]
async fn metrics_and_health_report_a_polled_device() {
    let handle = collector_app::metrics_server::install_recorder().expect("install recorder");
    let sim = sunspec_sim::start(
        SimConfig::single(UnitSpec::inverter(1)),
        "127.0.0.1:0".parse().unwrap(),
    )
    .await
    .unwrap();
    let port = free_port();
    let buffer_path = std::env::temp_dir()
        .join(format!("metrics-{}-{}.sqlite", std::process::id(), nanos()))
        .display()
        .to_string();
    let mut config = CollectorConfig::default();
    // Unit 7 does not exist on the simulator: it must still show up, as down.
    config.discovery.static_devices = [1, 7]
        .into_iter()
        .map(|unit_id| DeviceIdentity {
            ip: "127.0.0.1".to_string(),
            unit_id,
        })
        .collect();
    config.modbus.port = sim.port();
    config.poller.poll_interval = Duration::from_millis(100);
    config.buffer_path = buffer_path.clone();
    config.buffer_drain_interval_ms = 100;
    config.metrics_port = port;
    let (publisher, _sink) = avro_kafka::Publisher::new_memory("sunspec.telemetry");

    let app = tokio::spawn(collector_app::run_with_publisher(
        config,
        Some(handle),
        publisher,
        tokio::time::sleep(Duration::from_secs(3)),
    ));
    tokio::time::sleep(Duration::from_millis(1_500)).await;

    let health = http_get(port, "/healthz").await;
    assert!(health.starts_with("HTTP/1.1 200"), "{health}");
    assert!(health.contains("\"healthy\":true"), "{health}");
    assert!(health.contains("\"devices_up\":1"), "{health}");
    assert!(health.contains("\"devices_total\":2"), "{health}");

    let metrics = http_get(port, "/metrics").await;
    for expected in [
        "# HELP sunspec_poll_success_total",
        "sunspec_poll_success_total{device=\"127.0.0.1/1\"}",
        "sunspec_device_up{device=\"127.0.0.1/1\"} 1",
        "sunspec_device_up{device=\"127.0.0.1/7\"} 0",
        "sunspec_poll_duration_seconds_bucket{device=\"127.0.0.1/1\",le=\"0.005\"}",
        "sunspec_buffer_writes_total",
        "sunspec_uplink_records_total",
        "sunspec_uplink_publish_seconds_bucket",
    ] {
        assert!(metrics.contains(expected), "missing {expected}\n{metrics}");
    }
    assert!(!metrics.contains("batch_size="), "no per-batch-size label");

    app.await.unwrap().expect("collector run");
    for suffix in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{buffer_path}{suffix}"));
    }
}

async fn http_get(port: u16, path: &str) -> String {
    let mut stream = TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect to metrics server");
    let request = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).await.unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).await.unwrap();
    response
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
