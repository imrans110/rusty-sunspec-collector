# Architecture Guide

This document describes the internal architecture of Rusty SunSpec Collector: how the components fit together, the data flow through the system, and the design rationale behind key decisions.

For build/test commands see the [Developer Guide](DEVELOPER_GUIDE.md). For deployment and monitoring see [Operations](ops.md). For the original design plan see [Design Plan](plan.md).

---

## System Overview

Rusty SunSpec Collector is an edge service that continuously reads telemetry from SunSpec-compliant Modbus TCP devices (solar inverters, meters, batteries) and publishes the data to Apache Kafka. It is designed for reliability, low resource consumption, and graceful degradation.

```
 +-----------+    Modbus TCP    +------------+    mpsc    +---------+
 |  Inverter | <--------------  |   Poller   | --------> | Buffer  |
 |  (device) |    registers     |   Actor    |  samples  | (SQLite)|
 +-----------+                  +------------+           +---------+
                                      |                       |
                                      | metrics               | drain
                                      v                       v
                                +------------+         +----------+
                                | Prometheus |         |  Uplink  |
                                |  :9090     |         |  (Avro)  |
                                +------------+         +----------+
                                                            |
                                                            | Kafka produce
                                                            v
                                                       +----------+
                                                       |  Kafka   |
                                                       |  Broker  |
                                                       +----------+
```

---

## Workspace Structure

The project is organized as a Cargo workspace with 8 crates, each owning a single responsibility:

```
crates/
  types/            Shared data types (PointValue, DeviceIdentity)
  discovery/        Network scanning and device enumeration
  modbus-client/    Modbus TCP communication with retry logic
  sunspec-parser/   SunSpec model parsing (JSON, XML, registers)
  poller-actor/     Per-device async polling loop
  buffer/           SQLite-backed durable message queue
  avro-kafka/       Avro serialization and Kafka producer
  collector-app/    Main binary — orchestrates all above
```

### Dependency Graph

```
collector-app
  |-- discovery       (device enumeration)
  |-- modbus-client   (Modbus TCP reads)
  |-- sunspec-parser  (model parsing)
  |-- poller-actor    (polling loops)
  |     |-- modbus-client
  |     |-- sunspec-parser
  |     |-- types
  |-- buffer          (SQLite queue)
  |-- avro-kafka      (Kafka producer)
  |-- types           (shared DTOs)
```

The `types` crate is intentionally kept lightweight (only `serde`) so it can be depended on by every other crate without pulling in heavy transitive dependencies.

---

## Data Flow

### 1. Startup

```
main() -> load config -> validate -> discover devices -> build poller specs -> spawn tasks
```

1. Configuration is loaded from a TOML/JSON file or environment variables (`config.rs`)
2. Validation checks all ranges, CIDR format, Kafka topic rules
3. Device discovery runs: either static device list or CIDR subnet scan
4. For each discovered device, a Modbus connection is opened to read the SunSpec model registry
5. Poller actors are spawned into a `JoinSet` for supervision

### 2. Polling Loop (per device)

Each `PollerActor` runs an independent async loop:

```
connect -> loop {
    for model in models:
        read_range(model.start, model.length)
        send PollSample to channel
    sleep(poll_interval + jitter)
}
```

- Reads are batched according to `max_batch_size` with optional `inter_read_delay_ms`
- Failed reads increment a consecutive error counter; after 10 consecutive failures the actor exits
- The supervisor (main loop) respawns exited pollers with exponential backoff (base `respawn_delay_ms`, capped at 60s); successful exits reset the backoff counter
- Per-device jitter is salted by `hash(ip, unit_id)` to prevent correlated polling across devices

### 3. Buffer Layer

The `buffer_task` receives `PollSample` from the mpsc channel and writes JSON to SQLite:

```
recv PollSample -> JSON serialize -> SQLite INSERT (WAL mode)
```

SQLite with WAL mode provides:
- ACID durability (survives process crashes)
- Concurrent read/write without blocking
- No external infrastructure dependency

### 4. Uplink Layer

The `uplink_task` periodically drains the buffer and publishes to Kafka:

```
loop {
    dequeue_batch(N)
    separate corrupt messages from valid messages
    delete corrupt messages immediately (prevents head-of-line blocking)
    serialize valid samples -> Avro OCF -> Kafka produce
    if success: delete_batch(valid_ids)
    if failure: valid messages stay in buffer for retry next cycle
}
```

- Avro OCF with Deflate compression reduces message size by ~90%
- Exponential backoff on publish failures (1s base, 30s max)
- Corrupt messages are discarded immediately to prevent blocking
- Valid messages that fail to publish remain in SQLite for automatic retry

### 5. Metrics

An Axum HTTP server exposes Prometheus metrics on the configured port:

| Metric | Type | Description |
|--------|------|-------------|
| `poller_success` | Counter | Successful model reads (per IP) |
| `poller_error` | Counter | Failed reads (per IP, per error type) |
| `buffer_enqueue_success` | Counter | Successful buffer writes |
| `buffer_enqueue_error` | Counter | Failed buffer writes |
| `buffer_size` | Gauge | Current queue depth |
| `uplink_messages_sent` | Counter | Messages published to Kafka |
| `uplink_publish_error` | Counter | Failed Kafka publishes |
| `uplink_publish_latency` | Histogram | Batch publish duration |

---

## Key Design Decisions

### Actor Model for Polling

Each device gets its own `PollerActor` task. This provides:
- **Fault isolation**: One device timing out doesn't block others
- **Independent lifecycle**: Actors can be spawned, stopped, and respawned independently
- **Natural concurrency**: Tokio schedules actors across the thread pool

### SQLite as a Buffer (not in-memory)

The buffer uses SQLite instead of an in-memory queue because:
- **Durability**: Data survives process restarts and crashes
- **Backpressure**: Disk-backed queue can absorb bursts without OOM
- **Simplicity**: No external service (Redis, RabbitMQ) required at the edge

### JSON in Buffer, Avro on Wire

Samples are stored as JSON in SQLite and re-serialized to Avro OCF at publish time:
- JSON is self-describing and debuggable (can inspect buffer with `sqlite3` CLI)
- Avro OCF provides compact wire format with schema and compression
- Decouples internal format from wire format

### Lenient SunSpec Parsing

`parse_models_from_registers_lenient()` gracefully handles truncated model lists:
- Real-world inverters often have firmware bugs that truncate the model table
- Strict parsing (`parse_models_from_registers`) is available for conformance testing
- Lenient mode collects as many valid models as possible before the truncation

---

## Configuration Hierarchy

Configuration values are resolved in this order (later wins):

```
1. Compiled defaults (Default::default())
2. Config file (TOML or JSON, via --config or SUNSPEC_CONFIG env var)
3. Environment variables (SUNSPEC_*)
```

This allows a base config file with environment-specific overrides, which is the standard pattern for containerized deployments.

See [Configuration](configuration.md) for the full reference including TOML template, environment variables, and validation rules.

---

## Error Handling Strategy

### Crate-level errors

Each crate defines its own error enum using `thiserror`:
- `ClientError` (modbus-client)
- `ParserError` (sunspec-parser)
- `DiscoveryError` (discovery)
- `PollerError` (poller-actor)
- `PublishError` (avro-kafka)
- `BufferError` (buffer)

### Application-level errors

`collector-app` uses `anyhow::Result` for the main function, wrapping crate errors with `.context()` for human-readable messages.

### Recovery Patterns

| Component | Failure Mode | Recovery |
|-----------|-------------|----------|
| Modbus read | Timeout/transport | Exponential backoff retry (per request) |
| Poller actor | 10 consecutive errors | Actor exits, supervisor respawns |
| Buffer enqueue | SQLite error | Log and drop sample |
| Kafka publish | Broker unavailable | Exponential backoff; messages stay in buffer |
| Discovery | Host unreachable | Skip host, continue scan |

---

## Concurrency Model

The application runs on the Tokio multi-threaded runtime:

| Task | Lifetime | Concurrency |
|------|----------|-------------|
| Poller actors | Supervised (respawn on exit) | One per device, all concurrent |
| Buffer task | Application lifetime | Single consumer from mpsc channel |
| Uplink task | Application lifetime | Single drainer with configurable interval |
| Metrics server | Application lifetime | Axum handles concurrent HTTP requests |
| Watchdog | Application lifetime (Linux only) | Periodic sd-notify ping |

All tasks respect a shared `watch::channel<bool>` shutdown signal, enabling graceful shutdown on SIGINT/SIGTERM.

---

## Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Linux x86_64 | Full support | Including systemd integration |
| Linux ARM64 | Cross-compilation | Via `cross` tool with custom Docker image |
| macOS | Development only | No systemd (watchdog/notify are no-ops) |
| Windows | Not tested | Should compile but not officially supported |

---

## Technology Choices

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Language | Rust | Memory safety, zero-GC, async/await |
| Async runtime | Tokio | Industry standard, mature ecosystem |
| Modbus | tokio-modbus | Native async Modbus TCP |
| Serialization | Avro (wire), JSON (buffer) | Compact wire format, debuggable buffer |
| Message broker | Kafka (rdkafka) | High-throughput, durable, industry standard |
| Local storage | SQLite (sqlx) | Zero-config, ACID, WAL mode |
| Metrics | Prometheus (metrics crate) | Pull-based, Kubernetes-native |
| HTTP | Axum | Lightweight, Tokio-native |
| Logging | tracing | Structured, async-aware |

---

## Future Architecture Considerations

These are architectural areas that may need attention as the system scales:

1. **Scale factor normalization**: Currently raw registers are forwarded; scale factors should be applied before Kafka
2. **Schema Registry**: Hardcoded Avro schema should evolve via Confluent Schema Registry
3. **Repeating groups**: SunSpec models like storage/BESS have variable-length blocks
4. **Multi-collector coordination**: Multiple instances would duplicate work without leader election
5. **Backpressure**: Buffer depth should throttle polling rate when Kafka is slow
6. **Config reload**: SIGHUP-based config reload without restart
7. **Per-device profiles**: Different poll intervals or model sets per device
