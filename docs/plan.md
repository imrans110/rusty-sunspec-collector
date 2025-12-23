# Rusty SunSpec Collector plan

## 1. Executive summary and strategic imperative
The renewable energy sector is moving toward decentralized generation and real-time telemetry. Solar PV, BESS, and smart inverters at the grid edge require reliable, low-latency data collection. A Rust-based SunSpec collector targets high-assurance, low-overhead edge compute by avoiding GC pauses, reducing memory usage, and enforcing memory safety at compile time. The design focuses on a production-grade collector that discovers SunSpec models, normalizes scale factors, buffers data durably, and forwards telemetry securely.

The architecture centers on a collect-buffer-forward pipeline using Rust for reliability and performance, tokio for async execution, SQLite for durable buffering, and Kafka for high-throughput uplink. This plan is a detailed execution blueprint for engineering and operations.

## 2. Architectural philosophy and design principles

### 2.1 The case for Rust at the edge
Edge gateways operate with tight CPU and memory budgets. Rust's zero-cost abstractions and memory safety provide strong correctness guarantees without a garbage collector. These properties reduce the risk of runtime failures and data corruption, which are costly in remote deployments.

### 2.2 System architecture: collect-buffer-forward

#### 2.2.1 Ingestion layer (SunSpec poller)
- Modbus TCP connections per device.
- SunSpec model discovery based on the sentinel and model list.
- Async actor model so timeouts on one device do not block others.

#### 2.2.2 Persistence layer (resilient buffer)
- Durable, on-disk buffer to handle backhaul interruptions.
- SQLite with WAL mode for ACID durability and operational tooling.

#### 2.2.3 Uplink layer (Kafka producer)
- Reads normalized data from SQLite and sends to Kafka.
- Handles batching, compression, and backpressure.

### 2.3 Feature matrix and technology selection

| Component | Selected technology | Justification | Alternatives |
| --- | --- | --- | --- |
| Language | Rust (2021) | Memory safety, zero-GC latency, high performance | Python, C++, Go |
| Async runtime | Tokio | Mature ecosystem, robust async patterns | async-std, smol |
| Modbus stack | sunspec + tokio-modbus | Type-safe model discovery and scale factors | raw tokio-modbus |
| Local buffer | SQLite (sqlx) | ACID, file-based, strong tooling | Sled, RocksDB |
| Messaging | Kafka (rust-rdkafka) | High throughput, decoupled producer/consumer | MQTT, HTTP |
| Serialization | Avro | Compact binary with schema evolution | JSON, Protobuf |
| Resilience | sd-notify (systemd) | Native watchdog integration | custom timers |

## 3. Detailed component implementation

### 3.1 SunSpec polling engine

#### 3.1.1 Device model discovery
- Connect via Modbus TCP.
- Read the SunSpec sentinel and iterate models.
- Build a device model catalog (e.g., model 103, 160, 201).
- Use generated Rust structs from SunSpec JSON definitions.

#### 3.1.2 Scale factor normalization
- Raw register values represent fixed-point data.
- Apply scale factors: Value * 10^ScaleFactor.
- Treat sentinel values (e.g., 0x8000, NaN) as None.
- Convert into normalized domain types (e.g., `NormalizedInverterData`).

#### 3.1.3 Concurrency and actor model
- One ConnectionActor per device.
- Bounded mpsc channels for messaging.
- Failures isolated per device with supervision and restart.

### 3.2 Persistence subsystem: SQLite vs. Sled

#### 3.2.1 Technology evaluation
- SQLite provides ACID durability and inspection tooling.
- Sled offers higher raw throughput but less operational tooling.

#### 3.2.2 Implementation strategy
- Use sqlx for SQLite with compile-time query checking.
- WAL mode and `synchronous=NORMAL` for performance on flash media.

Schema:
```sql
CREATE TABLE IF NOT EXISTS telemetry_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    topic TEXT NOT NULL,
    payload BLOB NOT NULL,
    created_at INTEGER NOT NULL
);
```

### 3.3 Uplink subsystem: Kafka and Avro

#### 3.3.1 Kafka producer configuration
- Compression: zstd.
- Micro-batching with short buffering windows.
- Backpressure handling to stop draining SQLite when queue is full.

#### 3.3.2 Serialization with Apache Avro
- Schema-first format with evolution support.
- Use `serde` + `apache-avro` derive.
- Ensure Option maps to Avro unions.

## 4. Implementation and execution guide

### 4.1 Phase 1: environment setup and cross-compilation

#### 4.1.1 The cross-compilation challenge
Cross-compiling with C dependencies (librdkafka, libsqlite3) requires a target sysroot. Direct `cargo build --target aarch64-unknown-linux-gnu` is insufficient without target libs.

#### 4.1.2 Dockerized solution
Use `cross` with a custom Docker image.

Dockerfile (`docker/Dockerfile.arm64`):
```dockerfile
FROM ghcr.io/cross-rs/aarch64-unknown-linux-gnu:latest

RUN dpkg --add-architecture arm64
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
    libssl-dev:arm64 \
    libsqlite3-dev:arm64 \
    librdkafka-dev:arm64 \
    pkg-config

ENV PKG_CONFIG_LIBDIR=/usr/lib/aarch64-linux-gnu/pkgconfig
ENV PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig
```

Cross config (`Cross.toml`):
```toml
[target.aarch64-unknown-linux-gnu]
image = "my-sunspec-builder"
```

### 4.2 Phase 2: core application logic

#### 4.2.1 Project structure
- `src/models/`: generated SunSpec structs.
- `src/poller/`: Modbus logic and actors.
- `src/buffer/`: SQLite buffer.
- `src/uplink/`: Kafka producer.
- `src/main.rs`: orchestrator.

#### 4.2.2 Main event loop
```rust
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let _ = sd_notify::notify(true, &);

    let (tx, rx) = mpsc::channel(100);
    tokio::spawn(buffer_task(rx));
    tokio::spawn(poller_task(tx));

    loop {
        sleep(Duration::from_secs(5)).await;
        let _ = sd_notify::notify(true, &);
    }
}
```

### 4.3 Phase 3: resilience and observability
- systemd watchdog integration (`READY=1`, `WATCHDOG=1`).
- structured logs via `tracing`.
- optional OpenTelemetry metrics.

### 4.4 Phase 4: testing and validation
- Use a Modbus simulator for CI (e.g., pymodbus-sim).
- Integration tests validate discovery and scale-factor math.
- Benchmarks with `criterion` for serialization and memory use.

## 5. Security considerations
- Rust prevents buffer overflows and data races.
- Kafka communication uses TLS (`security.protocol=SSL`).
- Prefer encrypted storage (LUKS) on edge devices.

## 6. Execution roadmap
- Foundation (Weeks 1-2): Cross-compilation setup, dependency selection, basic Modbus actor.
- Core logic (Weeks 3-4): SunSpec discovery, scale-factor normalization, unit tests.
- Persistence and uplink (Weeks 5-6): SQLite buffer, Kafka producer, Avro serialization.
- Hardening (Weeks 7-8): Watchdog, structured logging, load testing, deployment artifacts.

## 7. Conclusion
A Rust-based SunSpec collector provides a reliable, high-performance edge data pipeline. The architecture and execution plan emphasize resilience, operational safety, and data correctness for modern grid analytics.

## 8. Detailed technical implementation specification

### 8.1 Dependency management and crate selection
- `tokio` with `full` + `macros`.
- `sunspec` with `tokio` + `serde`.
- `tokio-modbus` compatible with `sunspec`.
- `sqlx` with `sqlite` + `runtime-tokio-rustls` + `macros`.
- `rdkafka` with `cmake-build` + `ssl`.
- `apache-avro` with `derive`.

### 8.2 Error handling
- `thiserror` for library errors.
- `anyhow` for top-level context.

### 8.3 Poller actor state machine
States: Disconnected -> Connected -> Discovering -> Polling -> Backoff.

```rust
pub struct DeviceActor {
    client: Option<AsyncClient>,
    config: DeviceConfig,
    sender: mpsc::Sender<TelemetryData>,
}

impl DeviceActor {
    pub async fn run(&mut self) {
        loop {
            if let Err(e) = self.cycle().await {
                error!("Device cycle failed: {:?}", e);
                self.client = None;
                sleep(self.backoff_duration()).await;
            }
        }
    }
}
```

### 8.4 Buffer schema and operations
```sql
CREATE TABLE IF NOT EXISTS telemetry_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    topic TEXT NOT NULL,
    payload BLOB NOT NULL,
    retry_count INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_created_at ON telemetry_queue(created_at);
```

Operational logic:
- Insert on poll.
- Drain ordered by id.
- Delete on successful Kafka send.

### 8.5 Uplink configuration
- `message.timeout.ms=5000`.
- `queue.buffering.max.messages=10000`.
- `compression.codec=zstd`.
- `acks=all`.
- Pause drains on `QueueFull` errors.

### 8.6 Cross-compilation workflow
- Build custom cross image.
- Configure `Cross.toml`.
- Build with `cross build --release --target aarch64-unknown-linux-gnu`.

### 8.7 Resilience engineering
Systemd service (`/etc/systemd/system/sunspec-collector.service`):
```ini
[Unit]
Description=Rust SunSpec Collector
After=network.target

[Service]
Type=notify
ExecStart=/usr/local/bin/sunspec-collector
Restart=always
RestartSec=5
WatchdogSec=10
LimitNOFILE=4096

[Install]
WantedBy=multi-user.target
```

Watchdog task:
```rust
tokio::spawn(async {
    loop {
        let _ = sd_notify::notify(true, &);
        sleep(Duration::from_secs(5)).await;
    }
});
```

## Implementation tracker
- [x] Modbus client connect/read with timeout, retry/backoff, batching, inter-read delay.
- [x] Modbus integration test against `diagslave`.
- [x] Poller actor loop reads model ranges and publishes payloads.
- [x] Orchestrator supervision (`JoinSet`), bounded channels, graceful shutdown, loop lag metrics.
- [x] SunSpec parser JSON model definitions + register map discovery helper.
- [x] SunSpec XML model definitions parsing.
- [x] Model metadata expanded with `start` address for polling.
- [x] Lenient model read handling + cached parsed models + golden tests.
- [x] Avro schemas + Kafka producer wrapper scaffold.
- [x] Kafka producer integration (rdkafka-backed publish).
- [x] End-to-end harness from synthetic payloads to serialized output.
- [x] Discovery subnet scanner with concurrency cap and per-host timeout.
- [x] `static_config` discovery mode.
- [x] SQLite buffer layer (enqueue, dequeue, delete) with WAL mode.
- [x] systemd READY/WATCHDOG integration.
- [x] systemd unit template + env file.

## Change log
