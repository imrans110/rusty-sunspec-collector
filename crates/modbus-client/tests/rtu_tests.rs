//! RTU frames over TCP, against the simulator's serial-converter mode.

use std::sync::Arc;

use modbus_client::{ClientConfig, ClientError, ExceptionCode, Framing, ModbusClient};
use sunspec_sim::{parse_models, SimConfig, SimHandle, UnitSpec, Unmapped};

const BASE: u16 = 40_000;

/// A converter with units 1 and 2 on its bus, each with models 1, 103 and 120:
/// 2 sentinel + (2 + 66) + (2 + 50) + (2 + 26) + 2 end marker = 152 registers.
async fn converter() -> SimHandle {
    let chain = parse_models("1,103,120").unwrap();
    sunspec_sim::start_rtu_over_tcp(
        SimConfig {
            units: vec![
                UnitSpec::with_models(1, chain.clone()),
                UnitSpec::with_models(2, chain),
            ],
            unmapped: Unmapped::Exception,
        },
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
        retry_count: 0,
        framing: Framing::Rtu,
        ..Default::default()
    }
}

#[tokio::test]
async fn reads_every_unit_over_one_connection_in_batches() {
    let sim = converter().await;
    let client = ModbusClient::new(config(&sim));

    let unit1 = client.read_range(1, BASE, 152).await.expect("unit 1");
    let unit2 = client.read_range(2, BASE, 2).await.expect("unit 2");

    assert_eq!(&unit1[..2], &[0x5375, 0x6e53]);
    assert_eq!(unit1[2], 1, "model 1 header");
    assert_eq!(unit1[150], 0xFFFF, "end marker");
    assert_eq!(unit2, vec![0x5375, 0x6e53]);
    assert_eq!(sim.requests_served(), 3, "125 + 27 for unit 1, then unit 2");
    assert_eq!(
        sim.connections_accepted(),
        1,
        "units share the converter connection"
    );
}

#[tokio::test]
async fn exceptions_come_back_as_exceptions() {
    let sim = converter().await;
    let client = ModbusClient::new(config(&sim));

    let err = client.read_range(1, 1_000, 2).await.expect_err("unmapped");

    assert!(
        matches!(
            err,
            ClientError::Exception(ExceptionCode::IllegalDataAddress)
        ),
        "{err:?}"
    );
    client
        .read_range(1, BASE, 2)
        .await
        .expect("connection kept");
    assert_eq!(sim.connections_accepted(), 1);
}

#[tokio::test]
async fn an_absent_unit_times_out_and_the_bus_recovers() {
    let sim = converter().await;
    let client = ModbusClient::new(config(&sim));

    let err = client
        .read_range(9, BASE, 2)
        .await
        .expect_err("nobody answers");
    assert!(matches!(err, ClientError::Timeout { .. }), "{err:?}");

    // The timed-out connection is dropped (a late reply would desynchronise the
    // stream) and the next read reconnects.
    assert_eq!(
        client.read_range(1, BASE, 2).await.unwrap(),
        vec![0x5375, 0x6e53]
    );
    assert_eq!(sim.connections_accepted(), 2);
}

#[tokio::test]
async fn concurrent_readers_share_the_bus_one_request_at_a_time() {
    let sim = converter().await;
    let client = Arc::new(ModbusClient::new(config(&sim)));

    let readers: Vec<_> = (0..8u8)
        .map(|index| {
            let client = client.clone();
            tokio::spawn(async move { client.read_range(index % 2 + 1, BASE, 152).await })
        })
        .collect();
    for reader in readers {
        let values = reader.await.unwrap().expect("read");
        assert_eq!(values.len(), 152);
        assert_eq!(&values[..2], &[0x5375, 0x6e53]);
    }
    assert_eq!(sim.connections_accepted(), 1);
}
