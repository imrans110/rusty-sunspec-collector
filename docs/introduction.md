# Rusty SunSpec Collector

Rusty SunSpec Collector is an edge service written in Rust. It finds SunSpec devices (inverters,
meters, batteries) on Modbus TCP, walks each device's SunSpec model chain, polls every model on a
fixed schedule and decodes the points at the edge using the official SunSpec model definitions.
Samples are buffered in SQLite on local disk and published to Kafka as Avro single-object
records keyed by device.

Delivery is at-least-once: a buffered sample is deleted only after Kafka acknowledges it. The
buffer is bounded by row count and age, so during a very long outage the oldest samples are
dropped (and counted) instead of growing the buffer without limit.

The project is a Cargo workspace of nine crates, including `sunspec-sim`, a SunSpec Modbus TCP
simulator used by the tests and the Docker Compose stack.

![Architecture diagram](assets/arch.svg)

---

## How data flows

1. **Device set.** Either a static list of hosts (`discovery.static_devices`) or a subnet scan
   (`discovery.subnet`, rescanned every `discovery.rescan_interval_s`). Each device is identified
   by host and Modbus unit ID, so every unit behind a gateway is polled.
2. **Per-device poller.** Each device runs in its own supervised task. A session connects, finds
   the `SunS` marker (configured base address first, then 40000, 0 and 50000), reads the model
   headers one at a time until the end marker, and then reads and decodes every model once per
   poll interval. A failed or stalled session is restarted with exponential backoff, capped at
   60 s.
3. **Channel and ingest.** Samples go through a bounded channel to the ingest task, which writes
   them to SQLite in transactions of up to 256 rows. If the channel stays full for a poll
   interval, the sample is dropped and counted; polling never blocks.
4. **Buffer.** SQLite in WAL mode. A retention task runs every 10 s and drops rows older than
   `buffer.max_age_s` (7 days) or beyond `buffer.max_rows` (1,000,000).
5. **Uplink.** The uplink reads the oldest rows in batches, publishes one Kafka record per sample
   and deletes only the rows Kafka acknowledged. Undelivered rows stay and are retried first.
6. **Shutdown.** On SIGINT or SIGTERM the pollers stop, ingest writes everything still queued, and
   the uplink keeps draining for up to `buffer.shutdown_drain_timeout_ms` (5 s). Rows that are
   still buffered are sent after the next start.

Each Kafka record is keyed by `host/unit_id` and carries the device, model ID and name, the read
time, the decoded points and the raw registers. Consumers can deduplicate on
`(device_key, model_id, collected_at_ms)`. The record format is described in the
[README](https://github.com/imrans110/rusty-sunspec-collector#kafka-record-format) and the
[Avro schema](https://github.com/imrans110/rusty-sunspec-collector/blob/main/crates/avro-kafka/schemas/sunspec_sample.avsc);
`collector-app --print-schema` prints the schema.

---

## Quick start

### Docker Compose (no hardware needed)

```sh
docker compose up --build
```

This starts Kafka, a simulator with two units, the collector and Prometheus. Then:

```sh
curl localhost:9090/healthz          # 200 with a JSON body while the pipeline is running
curl -s localhost:9090/metrics | grep '^sunspec_'
```

Prometheus is on <http://localhost:9091>; Kafka is reachable from the host at `localhost:29092`
(topic `sunspec.telemetry`). `docker compose down -v` stops the stack and removes its volumes.

### Local build

Prerequisites: Rust 1.88+, cmake and a C toolchain, pkg-config, and the OpenSSL and libcurl
development headers (`libssl-dev` and `libcurl4-openssl-dev` on Debian and Ubuntu). librdkafka is
built from source; SQLite and zstd are bundled.

```sh
cargo build --workspace
cargo test --workspace

# Terminal 1: simulated devices
cargo run -p sunspec-sim -- --bind 127.0.0.1:1502 --units 1,2

# Terminal 2: the collector (no Kafka configured, so each record's key and size is logged
# at debug level instead of being published)
SUNSPEC_STATIC_DEVICES=127.0.0.1:1,127.0.0.1:2 SUNSPEC_PORT=1502 \
  RUST_LOG=info,avro_kafka=debug cargo run -p collector-app
```

Set `SUNSPEC_KAFKA_BROKERS` to publish to a broker, for example `localhost:29092` while the
Compose stack is running.

---

## Where to go next

| Page | Read it for |
|------|-------------|
| [Developer Guide](DEVELOPER_GUIDE.md) | Prerequisites, building, testing, the simulator, CI |
| [Building & Packaging](build.md) | Release builds, Docker images, aarch64 cross-compilation |
| [Configuration](configuration.md) | Every setting, its environment variable, default and validation rules |
| [Architecture](ARCHITECTURE.md) | Crates, tasks, data flow and design decisions |
| [Operations](ops.md) | Deployment with systemd or Docker, metrics, alerts, troubleshooting |
| [Buffer Maintenance](buffer_maintenance.md) | Inspecting and maintaining the SQLite buffer |

### Background

| Page | Contents |
|------|----------|
| [Design Plan](plan.md) | Original design notes and backlog |
| [Code Review & Audit](CODE_REVIEW.md) | Audit findings and how they were resolved |

---

## Known limitations

- Kafka TLS and SASL settings are not exposed in the configuration yet.
- No Schema Registry integration; records use fingerprint-based single-object encoding.
- Point units and labels are not in the payload; look them up by model and point name in the
  bundled SunSpec definitions.
- No configuration reload and no per-device polling profiles; enum and bitfield points are
  published as integers.
- Delivery is at-least-once, so duplicates are possible after a crash.

---

## License

Dual-licensed under MIT or Apache-2.0.
