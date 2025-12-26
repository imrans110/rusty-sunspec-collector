# rusty-sunspec-collector

[![Rust 2021](https://img.shields.io/badge/rust-2021-1b1b1f?logo=rust)](https://www.rust-lang.org/) [![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-0f766e)](LICENSE) [![Docs](https://img.shields.io/badge/docs-GitHub%20Pages-3b82f6)](https://imrans110.github.io/rusty-sunspec-collector/) [![crates.io planned](https://img.shields.io/badge/crates.io-planned-6b7280)](https://crates.io/)

High-concurrency, memory-safe edge service that polls Modbus TCP inverters, parses SunSpec models, and publishes serialized data to Kafka. Built as a Cargo workspace with clear domain boundaries to keep build times and dependencies under control.

![Collector architecture diagram](docs/assets/arch.svg)

## Features

- **Reliable Polling**: Per-device supervision with automatic "zombie poller" detection and respawn.
- **Efficient Uplink**: Avro OCF batching reduces Kafka message overhead by 90%+.
- **Flexible Discovery**: Scans subnets and supports Gateway devices (multi-Unit ID).
- **Durability**: SQLite-backed buffer ensures zero data loss during network outages.
- **Observability**: Prometheus metrics (`:9090/metrics`) for monitoring throughput, latency, and errors.
- **Production Ready**: Docker containerized, systemd-ready, and configurable via config file or env vars.

## Workspace layout

- `crates/collector-app` — binary orchestrator.
- `crates/modbus-client` — Tokio Modbus wrapper and quirks config.
- `crates/sunspec-parser` — model registry, scaling, sentinel handling.
- `crates/poller-actor` — per-inverter polling loop and supervision hooks.
- `crates/avro-kafka` — Avro schemas and Kafka producer wrapper.
- `crates/buffer` — SQLite-backed store-and-forward buffer.
- `crates/discovery` — subnet scanning and static device list support.
- `crates/types` — shared DTOs/traits (kept lightweight).
- `docs/` — design notes and backlog (`docs/plan.md`).

## Roadmap (high level)

- [x] Foundation: cross-compile setup, Modbus client.
- [x] Core logic: SunSpec discovery, poller actors.
- [x] Reliability: Zombie poller detection, supervisor respawn.
- [x] Persistence: SQLite buffer with JSON serialization.
- [x] Uplink: Avro OCF batching for high-throughput Kafka publishing.
- [x] Ops: Prometheus metrics, Docker containerization.

## Getting started

### Docker (Recommended)

```bash
# Build the container (handles internal deps like librdkafka)
docker build -t sunspec-collector .

# Run with environment variables
docker run -p 9090:9090 --env-file .env sunspec-collector
```

### Local Development (Rust)

1.  **Prerequisites**: Install `cmake` (for librdkafka) and Rust.
2.  **Build**: `cargo build --workspace`
3.  **Test**: `cargo test --workspace`
4.  **Run**: `cargo run -p collector-app -- --config docs/config.example.toml`

## Configuration (env)

To avoid long env lists, you can point to a TOML or JSON file with `SUNSPEC_CONFIG=/path/to/config.toml` or pass `--config /path/to/config.toml` to the binary. Env vars override file values when set. See `docs/config.example.toml` for a starter config. Validation enforces IPv4 CIDR for discovery subnets and Kafka topic format rules.

### Discovery

- `SUNSPEC_SUBNET`: CIDR subnet for discovery (default `192.168.1.0/24`).
- `SUNSPEC_PORT`: Modbus TCP port (default `502`).
- `SUNSPEC_STATIC_DEVICES`: comma-separated `ip[:unit_id]` list to bypass subnet scans (example: `192.168.1.20:1,192.168.1.21`).
- `SUNSPEC_DISCOVERY_UNIT_IDS`: comma-separated list of Modbus Unit IDs to scan for each IP (default `1`). Useful for gateways (example: `1,2,3`).

### Polling

- `SUNSPEC_POLL_INTERVAL_MS`: poll interval in milliseconds (default `1000`).
- `SUNSPEC_REQUEST_TIMEOUT_MS`: per-request timeout in milliseconds (default `1000`).
- `SUNSPEC_JITTER_MS`: jitter added to poll interval (default `0`).

### Modbus client

- `SUNSPEC_MAX_BATCH_SIZE`: max registers per read batch.
- `SUNSPEC_MODBUS_TIMEOUT_MS`: Modbus request timeout override.

### SunSpec discovery

- `SUNSPEC_BASE_ADDRESS`: base address for the SunSpec sentinel (default `40000`).
- `SUNSPEC_DISCOVERY_REG_COUNT`: number of registers to read for model discovery (default `200`).

### Buffer + uplink

- `SUNSPEC_BUFFER_PATH`: SQLite path for buffered payloads (default `sunspec-buffer.sqlite`).
- `SUNSPEC_BUFFER_BATCH_SIZE`: number of buffered messages to drain per cycle (default `100`).
- `SUNSPEC_BUFFER_DRAIN_MS`: drain interval in milliseconds (default `500`).

### Kafka

- `SUNSPEC_KAFKA_BROKERS`: Kafka bootstrap servers (example: `localhost:9092`).
- `SUNSPEC_KAFKA_TOPIC`: topic name for telemetry (default `sunspec.telemetry`).
- `SUNSPEC_KAFKA_CLIENT_ID`: Kafka client id (default `sunspec-collector`).
- `SUNSPEC_KAFKA_ACKS`: producer acks (default `all`).
- `SUNSPEC_KAFKA_COMPRESSION`: compression type (default `zstd`).
- `SUNSPEC_KAFKA_TIMEOUT_MS`: producer message timeout in ms (default `5000`).
- `SUNSPEC_KAFKA_IDEMPOTENCE`: `true`/`false` toggle for idempotent producer.

### Observability

- `SUNSPEC_METRICS_PORT`: Port to expose Prometheus metrics (default: `9090`).
- Metrics endpoint: `http://localhost:9090/metrics`

## Deployment

- systemd unit template: `docs/sunspec-collector.service`
- Example environment file: `docs/sunspec-collector.env`
- Build/test/run workflows: `docs/build.md`
- Runtime operations: `docs/ops.md`
- Buffer maintenance: `docs/buffer_maintenance.md`
- GitHub Pages site: `https://imrans110.github.io/rusty-sunspec-collector/`



## Cross-compilation (ARM64)

1. Build the custom cross image: `docker build -t sunspec-cross-arm64 -f docker/Dockerfile.arm64 .`
2. Build with cross: `cross build --release --target aarch64-unknown-linux-gnu`
3. Binary output: `target/aarch64-unknown-linux-gnu/release/collector-app`

## Contributing

Issues and PRs are welcome. Please keep the `types` crate slim and avoid adding heavy deps to shared crates. See `docs/plan.md` for current priorities.

## License

Dual-licensed under MIT or Apache-2.0 (see workspace `Cargo.toml`).
