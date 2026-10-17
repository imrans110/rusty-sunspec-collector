use modbus_client::{ClientConfig, ClientError, ExceptionCode, ModbusClient};
use sunspec_sim::{ModelSpec, SimConfig, SimHandle, UnitSpec};

const BASE: u16 = 40_000;

async fn simulator() -> SimHandle {
    let models = vec![
        ModelSpec { id: 1, len: 66 },
        ModelSpec { id: 103, len: 50 },
        ModelSpec { id: 120, len: 26 },
    ];
    sunspec_sim::start(
        SimConfig::single(UnitSpec::with_models(1, models)),
        "127.0.0.1:0".parse().unwrap(),
    )
    .await
    .expect("start simulator")
}

fn config(sim: &SimHandle) -> ClientConfig {
    ClientConfig {
        host: "127.0.0.1".to_string(),
        port: sim.port(),
        timeout_ms: 300,
        retry_count: 1,
        retry_backoff_ms: 10,
        retry_max_backoff_ms: 50,
        ..Default::default()
    }
}

#[tokio::test]
async fn reads_beyond_protocol_limit_in_batches() {
    let sim = simulator().await;
    let client = ModbusClient::connect(config(&sim)).await.expect("connect");

    // 2 sentinel + (2 + 66) + (2 + 50) + (2 + 26) + 2 end marker = 152 registers.
    let values = client.read_range(1, BASE, 152).await.expect("read");

    assert_eq!(values.len(), 152);
    assert_eq!(&values[..2], &[0x5375, 0x6e53]);
    assert_eq!(values[150], 0xFFFF);
    assert_eq!(sim.requests_served(), 2, "split into 125 + 27");
}

#[tokio::test]
async fn oversized_batch_config_is_clamped_to_protocol_limit() {
    let sim = simulator().await;
    let mut config = config(&sim);
    config.max_batch_size = Some(200);
    let client = ModbusClient::connect(config).await.expect("connect");

    let values = client.read_range(1, BASE, 152).await.expect("read");

    assert_eq!(values.len(), 152);
}

#[tokio::test]
async fn modbus_exception_keeps_the_connection() {
    let sim = simulator().await;
    let client = ModbusClient::connect(config(&sim)).await.expect("connect");

    let err = client
        .read_range(1, BASE + 1_000, 4)
        .await
        .expect_err("unmapped address");
    assert!(
        matches!(
            err,
            ClientError::Exception(ExceptionCode::IllegalDataAddress)
        ),
        "got {err:?}"
    );
    assert!(!err.is_connection_error());

    client
        .read_range(1, BASE, 2)
        .await
        .expect("read after exception");
    assert_eq!(sim.connections_accepted(), 1);
}

#[tokio::test]
async fn reconnects_after_the_connection_drops() {
    let sim = simulator().await;
    let client = ModbusClient::connect(config(&sim)).await.expect("connect");
    client.read_range(1, BASE, 2).await.expect("first read");

    sim.drop_connections();
    let values = client
        .read_range(1, BASE, 2)
        .await
        .expect("read after drop");

    assert_eq!(values, vec![0x5375, 0x6e53]);
    assert_eq!(sim.connections_accepted(), 2);
}

#[tokio::test]
async fn timeout_drops_the_connection_and_recovers() {
    let sim = simulator().await;
    let mut config = config(&sim);
    config.retry_count = 0;
    let client = ModbusClient::connect(config).await.expect("connect");

    sim.set_stalled(true);
    let err = client.read_range(1, BASE, 2).await.expect_err("stalled");
    assert!(matches!(err, ClientError::Timeout { .. }), "got {err:?}");

    sim.set_stalled(false);
    client
        .read_range(1, BASE, 2)
        .await
        .expect("read after stall");
    assert_eq!(sim.connections_accepted(), 2, "reconnected after timeout");
}

#[tokio::test]
async fn offline_device_fails_to_connect_then_recovers() {
    let sim = simulator().await;
    sim.go_offline();
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let err = ModbusClient::connect(config(&sim))
        .await
        .expect_err("device offline");
    assert!(err.is_connection_error(), "got {err:?}");

    sim.go_online();
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    let client = ModbusClient::connect(config(&sim))
        .await
        .expect("connect after device comes back");
    client.read_range(1, BASE, 2).await.expect("read");
}

#[tokio::test]
async fn resolves_hostnames() {
    let sim = simulator().await;
    let mut config = config(&sim);
    config.host = "localhost".to_string();
    let client = ModbusClient::connect(config)
        .await
        .expect("connect by name");

    client.read_range(1, BASE, 2).await.expect("read");
}
