# Architecture Guide

This document describes how Rusty SunSpec Collector is built: the crates and what each one owns,
the life of a device, how registers become Kafka records, what is durable and what is not, and
why the main design choices were made.

For building and testing see the [Developer Guide](DEVELOPER_GUIDE.md) and
[Building & Packaging](build.md). Every setting mentioned here is listed in
[Configuration](configuration.md); deployment, metrics and alerts are covered in
[Operations](ops.md).

---

## Overview

The collector is a single Rust binary (`collector-app`) that runs at the edge, next to the
devices. It:

1. finds SunSpec devices (inverters, meters, batteries) on Modbus TCP, either from a static list or
   by scanning a subnet;
2. walks each device's SunSpec model chain and then reads every model on a fixed schedule;
3. decodes each model's registers into named points with the official SunSpec model definitions
   (scale factors applied, "not implemented" values mapped to null);
4. buffers the samples in a local SQLite database;
5. publishes each sample to Kafka as one Avro single-object record keyed by device.

Delivery is at-least-once: a buffered sample is deleted only after Kafka has acknowledged it.

```
  Modbus TCP                    one supervised task per device (poller-actor)
 +------------+  read models  +-------------------+
 | 10.0.0.5/1 | <-----------> | device supervisor |---+
 +------------+               +-------------------+   |
 +------------+               +-------------------+   |   Sample
 | 10.0.0.5/2 | <-----------> | device supervisor |---+-------------+
 +------------+               +-------------------+   bounded mpsc  |
                                                      (256)         v
                                                              +-----------+
                                    batches of up to 256 rows |  ingest   |
                                    in one transaction        +-----------+
                                                                    |
                                                                    v
                     every 10 s: drop rows past        +-------------------------+
                     max_age / beyond max_rows ------> |  SQLite buffer (WAL)    |
                     (retention)                       |  table telemetry_queue  |
                                                       +-------------------------+
                                                                    | oldest rows first,
                                                                    | buffer.batch_size per batch
                                                                    v
 +-------------+   one record per sample, key host/unit       +-----------+
 |    Kafka    | <------------------------------------------- |  uplink   |
 +-------------+   delete only the rows Kafka acknowledged    +-----------+

 HTTP on metrics.port: /metrics (Prometheus) and /healthz; systemd watchdog on Linux.
```

---

## Workspace and crate responsibilities

The workspace has nine crates under `crates/`. Only `collector-app` and `sunspec-sim` build
binaries; everything else is a library.

| Crate | Responsibility |
|-------|----------------|
| `types` | Shared data types, depends only on `serde`: `DeviceIdentity { ip, unit_id }` with `key()` = `"host/unit"`, `Sample` (one model read from one device: device, model ID and name, start address, raw registers, read time, decoded points) and `PointValue` (`Null`, `Int(i64)`, `Float(f64)`, `Text(String)`). |
| `discovery` | IPv4 CIDR subnet scan: a TCP connect probe on the Modbus port for every host, with bounded concurrency and a per-host timeout. Each responding host yields one device per configured unit ID. |
| `modbus-client` | Modbus TCP client on tokio-modbus 0.17: splits reads at 125 registers, retries with exponential backoff, reconnects after connection errors, resolves hostnames. |
| `sunspec-parser` | The official SunSpec JSON model definitions, embedded at compile time, and `decode_model()`, which turns a model's registers into named points. Also `model_name()`, `SUNSPEC_ID` and `SUNSPEC_END_ID`. |
| `poller-actor` | The per-device lifecycle: `run_device()` supervises sessions (connect, discover the model chain, poll) with backoff and stall detection, and records per-device metrics. `discover_models()` walks the chain. |
| `buffer` | `BufferStore`, a SQLite queue (sqlx 0.8): batched enqueue, dequeue of the oldest rows, delete by ID, row-count and age retention. |
| `avro-kafka` | The record schema (`schemas/sunspec_sample.avsc`), single-object `encode`/`decode`, and `Publisher`, which sends to Kafka through rdkafka 0.39, only logs (no brokers configured), or keeps records in memory (tests). |
| `collector-app` | The binary and a library (`collector_app::run`, `run_with_publisher`) with the orchestration modules: `config`, `devices`, `ingest`, `uplink`, `retention`, `health`, `metrics_server`, `systemd`. |
| `sunspec-sim` | A SunSpec Modbus TCP simulator (library and `sunspec-sim` binary) with fault injection, used by the tests and the Docker Compose stack. |

### Dependency direction

Dependencies point from the application towards the shared types; no library depends on
`collector-app`, and `types` depends on nothing in the workspace.

```
collector-app
  |-- poller-actor --+-- modbus-client
  |                  +-- sunspec-parser -- types
  |                  +-- types
  |-- discovery ------- types
  |-- avro-kafka ------ types
  |-- buffer            (stores opaque bytes; no workspace dependencies)
  |-- modbus-client, sunspec-parser, types

sunspec-sim -- sunspec-parser
  (a dev-dependency of modbus-client, poller-actor and collector-app only)
```

---

## Startup

`main()` in `crates/collector-app/src/main.rs`:

1. If `--print-schema` is given, prints the Avro schema and exits.
2. Initialises logging (`RUST_LOG`, `SUNSPEC_LOG_FORMAT`).
3. Installs the SIGINT/SIGTERM handler before loading the configuration, so a stop during startup
   still takes the graceful path.
4. Loads the configuration (defaults, then the config file, then `SUNSPEC_*` variables) and
   validates it. Any error ends the process with a non-zero exit code.
5. Installs the Prometheus recorder and calls `collector_app::run`.

`collector_app::run` builds the publisher (Kafka when `kafka.brokers` is set, otherwise a sink that
only logs) and calls `run_with_publisher`, which:

1. creates the shutdown `watch` channel and the shared `Health` state;
2. binds `metrics.port` and starts the HTTP server for `/metrics` and `/healthz`; if the port
   cannot be bound, startup fails with an error containing `bind metrics port <port>`;
3. opens the SQLite buffer (failure ends the process);
4. creates the bounded sample channel (`runtime.channel_capacity`, default 256);
5. starts the ingest, uplink, retention and device tasks;
6. tells systemd it is ready (`READY=1`, Linux only) and starts the watchdog task;
7. waits for the shutdown signal (see [Shutdown](#shutdown)).

---

## Device lifecycle

### The device set

`devices::run` owns the set of devices and starts one supervised task per device.

- **Static mode.** When `discovery.static_devices` (or `SUNSPEC_STATIC_DEVICES`) is non-empty, those
  devices are started once and no subnet scan runs. Hosts may be IP addresses or hostnames.
- **Subnet mode.** Otherwise `discovery.subnet` is scanned. Every host that accepts a TCP
  connection on the Modbus port yields one device per entry in `discovery.unit_ids`. The scan is
  repeated every `discovery.rescan_interval_s` (default 600 s) and pollers are started for devices
  not seen before. Devices are never removed: a device that disappears keeps being retried by its
  supervisor.
- With no devices at all the collector logs a warning and keeps running (and, in subnet mode, keeps
  rescanning).

A device is identified by host **and** unit ID (`DeviceIdentity`), so every unit behind a Modbus
gateway gets its own poller, metrics series and Kafka key (`192.168.1.20/1`, `192.168.1.20/2`).
`sunspec_devices_known` reports how many devices are supervised.

### Supervisor

`poller_actor::run_device` runs a device until shutdown. It repeatedly runs a *session*; when a
session ends for any reason other than shutdown, it marks the device down, counts a restart
(`sunspec_poller_restarts_total`, plus `sunspec_poller_stalls_total` for stalls), logs the reason
and waits before the next session:

```
delay = runtime.respawn_delay_ms x 2^(n - 1), capped at 60 s, plus 0-10 % jitter
```

`n` is the number of consecutive failed sessions. It is reset when a session read at least one
model successfully, so a device that worked and then failed is retried after the base delay. The
jitter is derived from a hash of the device identity, so devices that fail together (for example
behind one gateway) do not retry in lockstep. The supervisor only returns on shutdown; a device
that is offline at startup is picked up when it comes online.

### Session

A session is: connect, discover the model chain, then poll until something fails.

1. **Connect.** A `ModbusClient` is created for the device with `poller.request_timeout_ms` as
   both the connect timeout and the per-request timeout.
2. **Discover.** The base addresses are tried in order: `sunspec.base_address` (default 40000)
   first, then the other standard bases 40000, 0 and 50000. At each base two registers are read and
   compared with the `SunS` marker (`0x5375 0x6e53`). A mismatch or a Modbus exception moves on to
   the next base; a timeout or transport error ends the session. At the first base with the marker,
   the model chain is walked one header at a time: read the two-register header (model ID, body
   length) at the current address, record the model, advance by `length + 2`, and stop at model ID
   `0xFFFF`. A chain of more than 256 models, an address overflow or a chain with no models ends the
   session as malformed. Models are rediscovered on every new session, so firmware updates that
   change the chain are picked up after a reconnect.
3. **Poll.** Polling runs on a fixed schedule with period `poller.poll_interval_ms` (missed ticks
   are skipped, not bunched). The first tick is offset by up to `poller.jitter_ms`, derived from the
   device identity, so devices sharing a network do not all poll at the same instant. In each cycle
   every model is read in full, header included, and decoded (see
   [SunSpec decoding](#sunspec-decoding)); each successful read produces one `Sample`.

A session ends with one of these errors (`PollerError`):

| Error | Cause |
|-------|-------|
| `Modbus` | Connect failure, or a timeout/transport error while discovering the model chain. |
| `NoSunSpecMap` | No `SunS` marker at any candidate base address. |
| `MalformedChain` | Address or length overflow, more than 256 models, or no models before the end marker. |
| `TooManyFailedCycles` | 10 consecutive poll cycles in which at least one model read failed. |
| `Stalled` | No successful read for the stall timeout (see below). |
| `ChannelClosed` | The sample channel closed; only happens during shutdown. |

### Stall detection

Every read has a timeout, but a device can still make no progress for a long time: with several
models, retries and backoff, ten failed cycles can take minutes. A watchdog inside each session
checks the device's last successful read every quarter of the stall timeout. If no read has
succeeded for `poller.stall_timeout_ms` (default: the larger of 10 poll intervals and 30 s), the
session is aborted with `Stalled` and the supervisor starts a new one after the usual backoff.

### Full channel

A sample is handed to the ingest task through the bounded channel with a timeout of one poll
interval. If the channel is still full after that, the sample is dropped, a warning is logged and
`sunspec_samples_dropped_total{reason="channel_full"}` is incremented. Polling never blocks for
longer than that, so a slow disk delays samples rather than the polling schedule.

---

## Modbus client

`modbus_client::ModbusClient` wraps a tokio-modbus TCP context behind a mutex, so each device
session issues one request at a time.

- **Read splitting.** `read_range(unit, start, count)` splits a read into requests of at most
  `modbus.max_batch_size` registers. The value defaults to 125, the protocol limit for function
  0x03, and configuration validation rejects values outside 1-125. A model longer than the batch
  size is read in several requests.
  `modbus.inter_read_delay_ms` optionally pauses between the requests of one split read, for
  devices that cannot handle back-to-back requests.
- **Retries.** Each request is retried up to `modbus.retry_count` times (default 2) after the first
  attempt, waiting `modbus.retry_backoff_ms x 2^attempt` (default 100 ms, doubling) up to
  `modbus.retry_max_backoff_ms` (default 2000 ms). Modbus exceptions are not retried, except
  0x06 (Server Device Busy); register address overflow is not retried.
- **Reconnects.** A failed hostname lookup, a failed connect, a transport error or a timeout drops
  the connection; the next attempt opens a new one. After a timeout the old connection could still
  deliver the late response and have it matched to the next request, so it is not reused.
- **Exceptions keep the connection.** A Modbus exception is a valid response from the device, so
  the connection stays open.
- **Hostnames.** The host is resolved on every connect and each resolved address is tried in turn
  (for example `localhost` may resolve to `::1` before `127.0.0.1`).

Errors are counted in `sunspec_poll_errors_total{device, kind}` with `kind` one of `exception`,
`timeout`, `transport`, `connect` (connect or resolve failure) and `address`. Counted are failed
model reads in the polling loop, a failed connect at the start of a session (`connect`, or
`timeout` when the connect timed out), and Modbus errors while discovering the model chain. A
Modbus exception while probing a base address for the `SunS` marker is not counted, because it
only means the map is not at that base. A session that ends without a Modbus error
(`NoSunSpecMap`, `MalformedChain`, `Stalled`) shows up only in `sunspec_poller_restarts_total`.

---

## SunSpec decoding

### Bundled definitions

`crates/sunspec-parser/models/` holds the official SunSpec JSON model definitions from
[github.com/sunspec/models](https://github.com/sunspec/models) (commit `90b4a331`, Apache-2.0,
unmodified). They are embedded with `include_str!` (the `MODEL_SOURCES` list in
`src/registry.rs`) and parsed once on first use. Bundled models:

| Models | Description |
|--------|-------------|
| 1 | Common (manufacturer, model, serial number) |
| 101, 102, 103 | Inverter, single/split/three phase (integer + scale factor) |
| 111, 112, 113 | Inverter, single/split/three phase (float) |
| 120-124 | Nameplate, basic settings, status, controls, basic storage |
| 160 | Multiple MPPT (repeating modules) |
| 201-204 | AC meters |
| 802 | Battery |

`model_name(id)` returns the definition's name (for example `inverter_three_phase` for 103), or
`model_<id>` for a model without a definition. Models without a definition are still polled and
published, with raw registers and an empty `points` map.

### Decoding

`decode_model(model_id, registers)` takes a model's registers starting at the header and returns
the points in definition order.

- **Groups.** A definition is a tree of groups. Each group's points are sliced first and their
  values computed afterwards, because a scale factor may appear after the points that use it.
  Nested groups follow their parent's points:
  - `count` 1: a fixed sub-group, named `group.Point`;
  - `count` n > 1: n repetitions, named `group[i].Point`;
  - `count` 0: repeated to fill the rest of the model;
  - `count` naming a point: repeated as many times as that point's value.

  For example, model 160 publishes `module[0].DCA`, `module[1].DCA` and so on.
- **Omitted.** The top-level `ID` and `L` header points, `sunssf` (scale factor) points and `pad`
  points are not published.
- **Truncated input.** If the registers end early, the points that are present are decoded and the
  rest are omitted.

### Point types and "not implemented" values

SunSpec marks unimplemented points with a sentinel value. Sentinels become `Null`.

| Point type | Published as | Not implemented |
|------------|--------------|-----------------|
| `int16` | long (double if scaled) | `0x8000` |
| `uint16`, `enum16`, `bitfield16` | long (double if scaled) | `0xFFFF` |
| `count`, `acc16` | long (double if scaled) | `0` |
| `int32` | long (double if scaled) | `0x80000000` |
| `uint32`, `enum32`, `bitfield32` | long (double if scaled) | `0xFFFFFFFF` |
| `acc32`, `acc64`, `ipaddr` | long (double if scaled) | `0` |
| `int64` | long (double if scaled) | `0x8000000000000000` |
| `uint64` | long (double if scaled) | `0xFFFFFFFFFFFFFFFF` |
| `float32`, `float64` | double | NaN |
| `string` | string, trimmed of NULs and whitespace | empty after trimming |
| `eui48` | string of hex digits | all registers `0xFFFF` |
| `ipv6addr` | string of hex digits | all registers `0` |

Enums and bitfields are published as integers; their symbolic names are not expanded. An integer
that does not fit an `i64` (a large `uint64` or `acc64`) is published as a double.

### Scale factors

A point with a scale factor is published as `value x 10^sf`, always as a double (also when the
exponent is 0). For negative exponents the result is rounded to that many decimal places, so
`2301` with `V_SF = -1` is `230.1` rather than `230.10000000000002`.

- A scale factor is either a literal exponent in the definition or the name of a `sunssf` point.
- Names are resolved from the innermost group outwards, so points in a repeating group can use a
  scale factor defined at the top level of the model (model 160's `module[i].DCA` uses the
  top-level `DCA_SF`).
- If the scale factor point itself is not implemented (`0x8000`), the point is `Null`: the value
  cannot be interpreted.

---

## Data path and durability

### Ingest

`ingest::run` receives samples from the channel. When a sample arrives it waits up to 100 ms for
more (at most 256 in total), serialises them to JSON and writes them to the buffer in one
transaction with `BufferStore::enqueue_batch`. One transaction per batch means one fsync per batch
rather than per sample, which matters on flash storage. Written rows are counted in
`sunspec_buffer_writes_total`; if the write fails, the batch is dropped and counted in
`sunspec_buffer_dropped_total{reason="write_error"}`.

Ingest runs until every sender has been dropped, so at shutdown it writes everything still queued
before it returns.

### Buffer

The buffer is one SQLite file (`buffer.path`) with one table:

```sql
CREATE TABLE telemetry_queue (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    topic      TEXT NOT NULL,
    payload    BLOB NOT NULL,     -- the Sample as JSON
    created_at INTEGER NOT NULL   -- enqueue time, Unix ms
);
CREATE INDEX idx_created_at ON telemetry_queue(created_at);
```

The database is created if missing and opened in WAL mode with `synchronous=NORMAL`, applied to
every pooled connection (pool of up to 5). Rows are dequeued in `id` order, oldest first. The
payload is JSON so the buffer can be inspected with the `sqlite3` CLI; rows written before decoded
points existed (no `points` field) still deserialize. JSON has no infinity, so an infinite float
point is stored, and later published, as null.

### Uplink

`uplink::run` drains the buffer to the publisher:

1. Dequeue the oldest `buffer.batch_size` rows (default 100).
2. Parse each row. Rows that are not valid JSON are deleted immediately and counted in
   `sunspec_buffer_dropped_total{reason="corrupt"}`, so a bad row cannot block the queue.
3. Publish the samples, one Kafka record per sample. All records of the batch are handed to the
   producer before waiting for acknowledgements, so librdkafka can batch and compress them.
4. Delete only the rows Kafka acknowledged and count them in `sunspec_uplink_records_total`.
   Rows that could never be encoded are deleted and counted as `corrupt`. Rows that were not
   acknowledged stay in the buffer, are counted in `sunspec_uplink_errors_total` (once per failed
   attempt), and are retried first in the next batch because they are the oldest.

Scheduling:

- While batches come back full, the next batch is drained immediately (back to back), so a backlog
  drains at the speed of Kafka rather than one batch per interval.
- When the backlog is empty, the uplink waits `buffer.drain_interval_ms` (default 500 ms).
- When the dequeue fails or no record of a batch is acknowledged, it backs off 1 s x 2^(n-1), up
  to 30 s (never less than the drain interval).

`sunspec_uplink_publish_seconds` records the time to publish a batch and receive its
acknowledgements.

Without `kafka.brokers` the publisher only encodes each record and logs it at debug level
(target `avro_kafka`); the rows are then deleted as if delivered. This mode is for local
development.

### Retention

`retention::run` runs every 10 s. It deletes rows enqueued more than `buffer.max_age_s` ago
(default 604800 s, 7 days) and then the oldest rows beyond `buffer.max_rows` (default 1,000,000),
counting them in `sunspec_buffer_dropped_total{reason="max_age"}` and `{reason="max_rows"}`. It also
updates the `sunspec_buffer_rows` and `sunspec_buffer_oldest_age_seconds` gauges. While the
uplink keeps up, the buffer stays small and retention deletes nothing; its job is to bound the
buffer during long outages so it cannot fill the disk.

### Delivery semantics

Delivery from the buffer to Kafka is **at-least-once**:

- A row is deleted only after Kafka acknowledged its record. If the process dies between the
  acknowledgement and the delete, or the delete fails, the record is sent again after the restart.
  The producer's idempotence avoids duplicates from its own retries within one run, but not across
  a restart.
- Consumers that need exactly-once processing should deduplicate on
  `(device_key, model_id, collected_at_ms)`.

Samples can be lost before or after they reach the buffer, and each case is counted:

| Where | Counter |
|-------|---------|
| Channel full for a whole poll interval | `sunspec_samples_dropped_total{reason="channel_full"}` |
| SQLite write failed | `sunspec_buffer_dropped_total{reason="write_error"}` |
| Row older than `buffer.max_age_s` | `sunspec_buffer_dropped_total{reason="max_age"}` |
| More than `buffer.max_rows` rows | `sunspec_buffer_dropped_total{reason="max_rows"}` |
| Row cannot be parsed or encoded | `sunspec_buffer_dropped_total{reason="corrupt"}` |

Samples still in the channel or in an ingest batch that has not been committed are lost if the
process is killed; a graceful stop writes them first. With WAL and `synchronous=NORMAL`, committed
rows survive a process crash, but SQLite may roll back the most recent transactions after a power
loss or operating-system crash.

---

## Kafka record contract

Schema version 2. This format is not compatible with earlier versions of the collector (see
[Earlier format](#earlier-format)).

### Record

- **Topic:** `kafka.topic` (default `sunspec.telemetry`).
- **One record per sample**, that is per model read from one device.
- **Key:** `device_key`, `host/unit_id`, for example `192.168.1.20/1`. With librdkafka's default
  partitioner (CRC32 of the key) all records of a device go to the same partition.
- **Value:** Avro [single-object encoding](https://avro.apache.org/docs/current/specification/#single-object-encoding):

  ```
  C3 01 | 8-byte CRC-64-AVRO (Rabin) fingerprint of the schema, little-endian | Avro binary datum
  ```

- **Headers:**
  - `content-type: application/avro; encoding=single-object`
  - `schema-fingerprint: <16 hex characters>`: the 8 fingerprint bytes in the order they appear in
    the value.

Records are published in buffer order, but a record that failed delivery is retried in a later
batch, after records that were acknowledged. Order consumers by `collected_at_ms` when order
matters.

### Schema

`com.rusty.sunspec.v2.SunspecSample`, in `crates/avro-kafka/schemas/sunspec_sample.avsc`;
`collector-app --print-schema` prints it.

| Field | Avro type | Meaning |
|-------|-----------|---------|
| `device_key` | `string` | `host/unit_id`; also the record key |
| `ip` | `string` | Device host (IP address or hostname as configured) |
| `unit_id` | `int` | Modbus unit ID |
| `model_id` | `int` | SunSpec model ID, e.g. `103` |
| `model_name` | `string` | SunSpec model name, e.g. `inverter_three_phase`, or `model_<id>` |
| `start` | `int` | Register address of the model header |
| `collected_at_ms` | `long` (`timestamp-millis`) | When the registers were read |
| `points` | `map` of `["null", "double", "long", "string"]` | Decoded points by SunSpec name |
| `registers` | `array` of `int` | Raw registers, including the two header registers |

Point semantics, as described in [SunSpec decoding](#sunspec-decoding):

- names are the SunSpec point names; repeating groups are `group[i].Point` (e.g. `module[0].DCA`)
  and fixed sub-groups `group.Point`;
- scaled values are `double`; enums, bitfields and unscaled integers are `long`; strings are
  `string`, trimmed;
- "not implemented" values, and points whose scale factor is not implemented, are `null`;
- the model header (`ID`, `L`), scale-factor points and padding are omitted;
- models without a bundled definition have an empty `points` map; only `registers` is set.

`points` is an Avro map, so its order is not preserved. Point units and labels are not in the
record; look them up by model ID and point name in the bundled definitions.

A model 103 record, shown as JSON for readability (unions unwrapped, lists shortened):

```text
{
  "device_key": "192.168.1.20/1",
  "ip": "192.168.1.20",
  "unit_id": 1,
  "model_id": 103,
  "model_name": "inverter_three_phase",
  "start": 40070,
  "collected_at_ms": 1760000000123,
  "points": { "A": 12.34, "PhVphA": 230.1, "W": 4321.0, "Hz": 50.0, "St": 4, "VA": null, ... },
  "registers": [103, 50, 1234, ...]
}
```

### Reading records

- **Rust:** `avro_kafka::decode(bytes)` checks the header and fingerprint and returns a `Sample`
  (points sorted by name).
- **Other languages:** check that the value starts with `C3 01` and that the next 8 bytes match the
  fingerprint of the schema you hold, then decode the rest as an Avro binary datum with that schema.
  Most Avro libraries implement single-object decoding directly (for example `BinaryMessageDecoder`
  in Java).

### Producer settings

`acks=all`, `enable.idempotence=true`, `compression.type=zstd`, `linger.ms=50` (fixed) and
`message.timeout.ms=5000` by default. Every setting except `linger.ms` can be changed in the
`[kafka]` section. An idempotent producer requires `acks=all`, so configuration validation rejects
`kafka.acks` `0` or `1` while `kafka.enable_idempotence` is true (the default).

### Earlier format

Earlier versions sent one Avro Object Container File per Kafka message, mixing samples from several
devices, with the schema embedded in every message, no key, raw registers only, and model names
such as `three_phase_inverter`. Consumers of that format must be updated.

---

## Health and observability

### Heartbeats and `/healthz`

The ingest and uplink tasks record a heartbeat on every loop iteration: ingest at least every 5 s
while idle, the uplink at least every 30 s (its longest backoff) plus the time one drain takes.

`GET /healthz` on `metrics.port` returns 200 while both heartbeats are younger than 120 s, and 503
otherwise, with a JSON body:

```json
{"healthy":true,"ingest_age_s":0.7,"uplink_age_s":0.3,"devices_up":2,"devices_total":2}
```

Ages are `null` before the first heartbeat. `devices_up` counts devices whose current session has
read successfully. Device outages do not make the collector unhealthy: restarting the collector
would not bring a device back, and they are visible in the per-device metrics. The Docker
`HEALTHCHECK` uses this endpoint.

### systemd watchdog

On Linux, when systemd sets `WATCHDOG_USEC` (the unit uses `Type=notify` and `WatchdogSec=30`), a
task pings the watchdog every half watchdog interval, but only while `/healthz` would report
healthy. If ingest or the uplink stops checking in, the pings stop and systemd restarts the
service. On other platforms the watchdog and readiness notification are no-ops.

### Metrics

The HTTP server (axum) listens on `0.0.0.0:metrics.port` (default 9090; 0 picks a free port).
The port is bound before the buffer is opened and the tasks start; if it is taken, the collector exits with an error
rather than running without `/metrics` and `/healthz`.

Naming conventions: every metric has the `sunspec_` prefix, counters end in `_total`, durations
are in seconds and end in `_seconds`, and every metric has HELP text. `*_seconds` metrics are
histograms with buckets from 1 ms to 10 s. Per-device metrics carry a `device` label in the
`host/unit` form (`192.168.1.20/1`); there are no other high-cardinality labels.

| Area | Metrics |
|------|---------|
| Devices | `sunspec_poll_success_total`, `sunspec_poll_errors_total{kind}`, `sunspec_poll_duration_seconds`, `sunspec_device_up`, `sunspec_device_last_success_timestamp_seconds`, `sunspec_poller_restarts_total`, `sunspec_poller_stalls_total`, `sunspec_devices_known` |
| Buffer | `sunspec_samples_dropped_total{reason}`, `sunspec_buffer_writes_total`, `sunspec_buffer_dropped_total{reason}`, `sunspec_buffer_rows`, `sunspec_buffer_oldest_age_seconds` |
| Uplink | `sunspec_uplink_records_total`, `sunspec_uplink_errors_total`, `sunspec_uplink_publish_seconds` |

`sunspec_device_up` is set to 0 when a device's poller starts, to 1 on a successful read and back
to 0 when a session ends, so a device that has never answered has a series with value 0.
`sunspec_uplink_errors_total` counts delivery attempts: a record that stays buffered and fails
again is counted again. The buffer gauges are updated every 10 s by the retention task. The full
table and suggested alerts are in [Operations](ops.md).

### Logs

Logging uses `tracing` with structured fields (`device`, `model_id`, `error`, ...). `RUST_LOG`
sets the filter (default `info`) and `SUNSPEC_LOG_FORMAT=json` writes one JSON object per line.
Steady-state operation is quiet at `info`: per-cycle and per-batch messages are at `debug`;
session failures, dropped samples and undelivered records are warnings.

---

## Shutdown

SIGINT and SIGTERM (sent by systemd and Docker on stop) both start a graceful shutdown:

1. The shutdown flag is set on the `watch` channel. The HTTP server, the retention task and the
   watchdog stop.
2. **Device pollers stop.** Each supervisor sees the flag and ends its session, cancelling any
   request in flight. The device task waits up to 10 s for all pollers, then aborts the rest. The pollers own the only senders of the sample
   channel, so the channel closes.
3. **Ingest writes everything still queued** to the buffer and returns.
4. **The uplink stops**, then keeps draining batch after batch for up to
   `buffer.shutdown_drain_timeout_ms` (default 5000; 0 skips the final drain). It is stopped through
   its own `watch` channel, only after ingest has finished.
5. The process exits with code 0. Rows that were not published stay in the buffer and are sent
   after the next start.

The systemd unit allows 30 s for this (`TimeoutStopSec=30`).

---

## Concurrency model

The binary runs on Tokio's multi-threaded runtime. Every long-lived component is a Tokio task:

| Task | Count | Lifetime |
|------|-------|----------|
| Signal handler | 1 | Until SIGINT or SIGTERM |
| Device set (`devices::run`) | 1 | Until shutdown; runs subnet scans |
| Device supervisor (`run_device`) | 1 per device, in a `JoinSet` | Until shutdown |
| Ingest | 1 | Until the sample channel closes |
| Uplink | 1 | Until its stop signal, plus the final drain |
| Retention | 1 | Until shutdown |
| HTTP server (`/metrics`, `/healthz`) | 1 | Until shutdown |
| systemd watchdog | 0 or 1 (Linux, when enabled) | Until shutdown |

Coordination:

- **Shutdown:** a `tokio::sync::watch::channel(bool)` shared by all tasks; the uplink has a second
  `watch` channel so it stops after ingest.
- **Samples:** one bounded `tokio::sync::mpsc` channel (`runtime.channel_capacity`) from all pollers
  to the single ingest task. It is the only queue in memory; the SQLite buffer absorbs anything
  longer.
- **Shared state:** device and pipeline health are atomics behind `Arc` (`DeviceHealth`, `Health`),
  read by `/healthz` and the watchdog without locks on the polling path.
- **Modbus:** each session owns its client; requests to one device are serialised by the client's
  mutex, and different devices are polled concurrently.
- **SQLite:** ingest, uplink and retention share one sqlx pool; sqlx runs each SQLite connection on
  its own thread, so database I/O does not block the runtime's worker threads.
- **Kafka:** librdkafka runs its own threads; the uplink awaits delivery reports.
- **Discovery:** a subnet scan probes hosts concurrently, bounded by a semaphore of
  `discovery.max_concurrency` permits.

---

## Configuration

Values are resolved in this order, later sources winning:

1. compiled defaults;
2. the config file given with `--config PATH` or `SUNSPEC_CONFIG` (`.json` is parsed as JSON,
   anything else as TOML);
3. `SUNSPEC_*` environment variables.

Loading is strict: unknown keys in the file, unparsable environment values (the error names the
variable), invalid static device entries and unsupported `kafka.compression` / `kafka.acks` values
fail at startup. `validate()` also rejects zero intervals, ports and sizes, a
`modbus.max_batch_size` outside 1-125, and `kafka.acks` `0` or `1` while idempotence is enabled.
The full reference is in [Configuration](configuration.md).

---

## Error handling

Each library crate has its own error type built with `thiserror`: `ClientError` (modbus-client),
`DiscoveryError` (discovery), `PollerError` (poller-actor), `BufferError` (buffer) and
`PublishError` (avro-kafka). `sunspec-parser` has no error type: decoding is total and returns
`None` only for models without a definition. `collector-app` uses `anyhow` with `.context()` for
startup errors; once running, runtime failures are logged, counted and recovered rather than
propagated.

| Component | Failure | Recovery |
|-----------|---------|----------|
| Modbus request | Timeout, transport or connect error | Retry with backoff on a new connection |
| Modbus request | Exception | Not retried (except Server Device Busy); connection kept |
| Device session | Connect, discovery, 10 failed cycles, stall | Supervisor restarts it with exponential backoff |
| Sample channel | Full for a poll interval | Sample dropped and counted |
| Buffer write | SQLite error | Batch dropped and counted (`write_error`) |
| Buffer row | Not parsable or not encodable | Row deleted and counted (`corrupt`) |
| Kafka publish | Not acknowledged | Row kept and retried first; uplink backs off when a whole batch fails |
| Subnet scan | Host unreachable | Host skipped; found on a later rescan |
| Ingest or uplink | Stuck | `/healthz` returns 503; the systemd watchdog restarts the service |

---

## Design decisions

### One task per device

Each device runs in its own supervised task. A device that times out does not delay the others,
each device has its own backoff and health, and the Tokio runtime spreads the work over its worker
threads. The supervisor never gives up, because at the edge a device that is offline now (at night,
during maintenance) is expected to come back.

### Walk the model chain instead of reading a fixed window

Earlier versions read a fixed 200-register window at the base address and parsed the model list
from it, which silently dropped every model past the window (status, controls, MPPT). The chain is
now walked one header at a time: each read is two registers, never runs past the end of the map,
and works for chains of any length.

### Decode at the edge

The collector already needs the model definitions to know which registers form a model. Scale
factors are stored in other registers of the same model, so a value and its scale factor have to
come from the same read to be consistent; decoding at read time guarantees that. Consumers then
receive named, scaled values and need no SunSpec knowledge. The raw registers are still included,
so data can be re-decoded later or checked against the device. The costs are larger records and
that new model definitions need a collector update; models without a definition still pass through
as raw registers.

### Avro single-object encoding instead of container files or a Schema Registry

Earlier versions sent an Avro Object Container File per message. A container embeds the full schema
in every message and, because one message mixed devices, there was no meaningful key. Single-object
encoding adds a 10-byte header to each record and allows one record per sample, keyed by device, so
Kafka can partition and compact per device and consumers can process records one at a time.

A Schema Registry (Confluent wire format) was not used because it adds a service the edge device
must reach and register with before it can publish, and its schema IDs only mean something with
that registry. Single-object encoding is defined by the Avro specification itself: the fingerprint
is computed from the schema, so any consumer that has the schema file can check that a record was
written with it. The trade-off is that there is no central compatibility checking; a new schema
version gets a new fingerprint (and namespace), and consumers must be given the new schema.

### SQLite as the buffer

The buffer has to survive restarts, crashes and long uplink outages without another service on the
edge device. SQLite is a single file, needs no daemon, is crash-safe, and in WAL mode lets the
uplink read while ingest writes. Writes are batched into transactions to limit fsyncs and flash
wear. Retention bounds its size. One collector process should use a given buffer file.

### JSON in the buffer, Avro on the wire

Samples are stored as JSON and encoded to Avro when published. The buffer stays readable with the
`sqlite3` CLI, and the stored format does not have to change when the wire format does: rows
buffered by the previous version, which had no decoded points, are still published after an
upgrade, in the new format with an empty `points` map.

### At-least-once delivery

Deleting a row only after Kafka acknowledged it means a crash can produce a duplicate but never
silently loses a buffered sample. Exactly-once would need the Kafka acknowledgement and the SQLite
delete to commit atomically, which two independent systems cannot do without transactions on both
sides and a consumer that reads only committed data. Duplicates are cheap to remove downstream with
the `(device_key, model_id, collected_at_ms)` key.

### Health means the pipeline, not the devices

`/healthz` and the watchdog only consider ingest and the uplink. A restart fixes a stuck pipeline
but not an offline inverter, so device outages are reported through metrics instead of causing
restart loops.

---

## Technology choices

| Component | Choice |
|-----------|--------|
| Async runtime | Tokio (multi-threaded) |
| Modbus TCP | tokio-modbus 0.17 |
| SunSpec models | Official SunSpec JSON definitions, parsed with serde_json |
| Buffer | SQLite through sqlx 0.8 (bundled SQLite) |
| Serialization | JSON in the buffer, Avro single-object (apache-avro 0.21) on the wire |
| Kafka | rdkafka 0.39 (librdkafka 2.12, built from source with SSL and zstd) |
| HTTP | axum 0.8 |
| Metrics | metrics 0.22 with metrics-exporter-prometheus 0.13 |
| Logging | tracing with tracing-subscriber (text or JSON) |
| Errors | thiserror in libraries, anyhow in the application |

---

## Platform support

| Platform | Status | Notes |
|----------|--------|-------|
| Linux x86_64 | Supported | systemd readiness and watchdog; Docker images |
| Linux aarch64 | Cross-compiled | `cross` with `docker/Dockerfile.arm64`; built by CI (see [Building & Packaging](build.md)) |
| macOS | Development | systemd readiness and watchdog are no-ops |
| Windows | Not tested | Only SIGINT (Ctrl-C) triggers a graceful stop |

---

## Limitations and future work

- Kafka TLS and SASL settings are not exposed in the configuration yet (librdkafka is built with
  SSL).
- No Schema Registry integration; records use fingerprint-based single-object encoding.
- Point metadata (units, labels) is not in the payload; consumers look it up by model and point
  name in the bundled definitions.
- Enum and bitfield points are published as integers, not expanded to their names.
- No configuration reload (SIGHUP) and no per-device polling profiles (interval or model set).
- Delivery is at-least-once; duplicates are possible after a crash.
- Subnet scanning is IPv4 only, and devices found by a scan are never removed from the set.
- Running two collectors against the same devices duplicates polling and records; there is no
  coordination between instances.
- A slow uplink does not slow polling down; the buffer grows until retention drops the oldest rows.
