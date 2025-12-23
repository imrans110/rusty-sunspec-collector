use modbus_client::{ClientConfig, ModbusClient};

#[tokio::test]
async fn diagslave_integration_read() {
    let host = match std::env::var("MODBUS_TEST_HOST") {
        Ok(value) => value,
        Err(_) => return,
    };

    let port = env_u16("MODBUS_TEST_PORT").unwrap_or(1502);
    let unit_id = env_u16("MODBUS_TEST_UNIT_ID").unwrap_or(1) as u8;
    let start = env_u16("MODBUS_TEST_START").unwrap_or(0);
    let count = env_u16("MODBUS_TEST_COUNT").unwrap_or(8);
    let max_batch = env_u16("MODBUS_TEST_MAX_BATCH").unwrap_or(2);

    let mut config = ClientConfig::default();
    config.host = host;
    config.port = port;
    config.max_batch_size = Some(max_batch);
    config.timeout_ms = env_u64("MODBUS_TEST_TIMEOUT_MS").unwrap_or(1_000);
    config.retry_count = env_usize("MODBUS_TEST_RETRY_COUNT").unwrap_or(1);
    config.retry_backoff_ms = env_u64("MODBUS_TEST_RETRY_BACKOFF_MS").unwrap_or(100);
    config.retry_max_backoff_ms = env_u64("MODBUS_TEST_RETRY_MAX_BACKOFF_MS").unwrap_or(500);

    let client = ModbusClient::connect(config).await.expect("connect");
    let values = client
        .read_range(unit_id, start, count)
        .await
        .expect("read");

    assert_eq!(values.len() as u16, count);
}

fn env_u16(key: &str) -> Option<u16> {
    std::env::var(key).ok().and_then(|value| value.parse().ok())
}

fn env_u64(key: &str) -> Option<u64> {
    std::env::var(key).ok().and_then(|value| value.parse().ok())
}

fn env_usize(key: &str) -> Option<usize> {
    std::env::var(key).ok().and_then(|value| value.parse().ok())
}
