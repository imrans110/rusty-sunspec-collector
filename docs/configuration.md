# Configuration

Rusty SunSpec Collector can be configured via a TOML/JSON file, environment variables, or both. When both are used, environment variables take precedence.

---

## Configuration Hierarchy

Values are resolved in this order (later wins):

```
1. Compiled defaults (Default::default())
2. Config file (TOML or JSON)
3. Environment variables (SUNSPEC_*)
```

This allows a base config file with environment-specific overrides — the standard pattern for containerized deployments.

### Specifying a config file

```sh
# Via CLI argument
collector-app --config /path/to/config.toml

# Via environment variable
SUNSPEC_CONFIG=/path/to/config.toml collector-app
```

If neither is set, the collector starts with compiled defaults and environment variable overrides only.

---

## Config File Reference

A complete example config file. All fields are optional — the collector has sensible defaults for everything.

```toml
[discovery]
subnet = "192.168.1.0/24"
port = 502
max_concurrency = 64
per_host_timeout_ms = 200
unit_ids = [1]                    # Modbus Unit IDs to scan per host (useful for gateways)

[[discovery.static_devices]]
ip = "192.168.1.20"
unit_id = 1

[poller]
poll_interval_ms = 1000
request_timeout_ms = 1000
jitter_ms = 0

[modbus]
max_batch_size = 64
timeout_ms = 1000
retry_count = 2
retry_backoff_ms = 100
retry_max_backoff_ms = 2000
inter_read_delay_ms = 5

[sunspec]
base_address = 40000
discovery_register_count = 200

[buffer]
path = "sunspec-buffer.sqlite"
batch_size = 100
drain_interval_ms = 500

[kafka]
brokers = "localhost:9092"
topic = "sunspec.telemetry"
client_id = "sunspec-collector"
acks = "all"
compression = "zstd"
timeout_ms = 5000
enable_idempotence = true
```

> The raw file is available at [`config.example.toml`](config.example.toml).

---

## Environment Variables

Every config file field can be overridden via environment variables. Variables that are commented out below show the default value.

### Discovery

| Variable | Default | Description |
|----------|---------|-------------|
| `SUNSPEC_SUBNET` | `192.168.1.0/24` | IPv4 CIDR subnet for device discovery |
| `SUNSPEC_PORT` | `502` | Modbus TCP port |
| `SUNSPEC_STATIC_DEVICES` | *(empty)* | Comma-separated `ip[:unit_id]` list to bypass subnet scan. Example: `192.168.1.20:1,192.168.1.21` |
| `SUNSPEC_DISCOVERY_UNIT_IDS` | `1` | Comma-separated Modbus Unit IDs to scan per host. Example: `1,2,3` |

### Polling

| Variable | Default | Description |
|----------|---------|-------------|
| `SUNSPEC_POLL_INTERVAL_MS` | `1000` | Milliseconds between poll cycles per device |
| `SUNSPEC_REQUEST_TIMEOUT_MS` | `1000` | Per-request Modbus timeout |
| `SUNSPEC_JITTER_MS` | `0` | Random jitter added to poll interval (per-device salted) |

### Modbus

| Variable | Default | Description |
|----------|---------|-------------|
| `SUNSPEC_MAX_BATCH_SIZE` | *(unlimited)* | Max registers per read batch |
| `SUNSPEC_MODBUS_TIMEOUT_MS` | `1000` | Modbus request timeout override |

### SunSpec

| Variable | Default | Description |
|----------|---------|-------------|
| `SUNSPEC_BASE_ADDRESS` | `40000` | Base address for the SunSpec sentinel |
| `SUNSPEC_DISCOVERY_REG_COUNT` | `200` | Number of registers to read for model discovery |

### Buffer

| Variable | Default | Description |
|----------|---------|-------------|
| `SUNSPEC_BUFFER_PATH` | `sunspec-buffer.sqlite` | SQLite database file path |
| `SUNSPEC_BUFFER_BATCH_SIZE` | `100` | Messages to drain per uplink cycle |
| `SUNSPEC_BUFFER_DRAIN_MS` | `500` | Drain interval in milliseconds |

### Kafka

| Variable | Default | Description |
|----------|---------|-------------|
| `SUNSPEC_KAFKA_BROKERS` | *(unset — mock mode)* | Kafka bootstrap servers. Example: `localhost:9092` |
| `SUNSPEC_KAFKA_TOPIC` | `sunspec.telemetry` | Kafka topic name |
| `SUNSPEC_KAFKA_CLIENT_ID` | `sunspec-collector` | Kafka client identifier |
| `SUNSPEC_KAFKA_ACKS` | `all` | Producer acknowledgment level |
| `SUNSPEC_KAFKA_COMPRESSION` | `zstd` | Compression codec |
| `SUNSPEC_KAFKA_TIMEOUT_MS` | `5000` | Producer message timeout |
| `SUNSPEC_KAFKA_IDEMPOTENCE` | *(unset)* | Enable idempotent producer (`true`/`false`) |

### Observability

| Variable | Default | Description |
|----------|---------|-------------|
| `SUNSPEC_METRICS_PORT` | `9090` | Prometheus metrics HTTP port |

### Internal

| Variable | Default | Description |
|----------|---------|-------------|
| `SUNSPEC_CHANNEL_CAPACITY` | `256` | Bounded mpsc channel size between pollers and buffer |
| `SUNSPEC_RESPAWN_DELAY_MS` | `1000` | Base delay before respawning a failed poller |

> A template env file is available at [`sunspec-collector.env`](sunspec-collector.env).

---

## Mock Mode

If `SUNSPEC_KAFKA_BROKERS` is **not set**, the publisher runs in mock mode — it logs publish events but doesn't connect to a real broker. This is useful for testing the polling and buffer pipeline without a Kafka cluster.

---

## Validation Rules

The collector validates configuration at startup and exits with a clear error if anything is invalid:

| Rule | Error message |
|------|---------------|
| Subnet must be IPv4 CIDR | `discovery.subnet must be CIDR (e.g. 192.168.1.0/24)` |
| Port must be 1–65535 | `discovery.port must be between 1 and 65535` |
| Kafka topic: alphanumeric + `.` `_` `-` | `kafka.topic contains invalid characters` |
| Kafka topic: must start/end alphanumeric | `kafka.topic must start and end with an alphanumeric character` |
| Kafka topic: max 249 chars | `kafka.topic must be <= 249 characters` |
| Brokers must be non-empty when set | `kafka.brokers must be non-empty when set` |
| All interval/timeout values must be >= 1 | Various `must be >= 1` messages |

---

## Example Deployments

### Minimal (mock mode, no Kafka)

```sh
SUNSPEC_STATIC_DEVICES=192.168.1.20:1 cargo run -p collector-app
```

### Subnet scan with Kafka

```sh
SUNSPEC_SUBNET=10.0.0.0/24 \
SUNSPEC_KAFKA_BROKERS=kafka.internal:9092 \
SUNSPEC_KAFKA_TOPIC=solar.telemetry \
cargo run -p collector-app
```

### Docker with config file

```sh
docker run -p 9090:9090 \
  -v $(pwd)/my-config.toml:/app/config.toml:ro \
  -v sunspec-data:/app/data \
  sunspec-collector --config /app/config.toml
```

### Gateway with multiple Unit IDs

```sh
SUNSPEC_STATIC_DEVICES=192.168.1.100:1 \
SUNSPEC_DISCOVERY_UNIT_IDS=1,2,3,4 \
SUNSPEC_KAFKA_BROKERS=localhost:9092 \
cargo run -p collector-app
```
