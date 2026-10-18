# Project Instructions

Instructions for coding agents and contributors working in this repository.

## Project Overview

Rusty SunSpec Collector is a Rust edge service. It finds SunSpec devices on Modbus TCP, walks each
device's model chain, polls every model on a fixed schedule, decodes points with the bundled
official SunSpec definitions, buffers samples in SQLite and publishes them to Kafka as Avro
single-object records keyed by `host/unit_id`. Delivery is at-least-once.

## Workspace Layout

```
crates/
  types/           – Shared DTOs, serde only: DeviceIdentity (key() = "host/unit"), Sample, PointValue
  discovery/       – CIDR subnet scan (TCP probe, bounded concurrency, per-host timeout) or static list
  modbus-client/   – Modbus TCP (tokio-modbus 0.17): reads split at 125 registers, retry with backoff,
                     lazy reconnect after transport errors/timeouts, hostname resolution
  sunspec-parser/  – Every official SunSpec JSON model (models/, embedded by build.rs), decode_model(),
                     model_name()
  poller-actor/    – Per-device lifecycle: run_device() supervision, discover_models(), polling,
                     stall detection, per-device metrics
  buffer/          – SQLite (sqlx, WAL) queue: enqueue_batch, dequeue_batch, delete_batch, retention
  avro-kafka/      – schemas/sunspec_sample.avsc, encode/decode, Publisher (Kafka, log-only, memory)
  collector-app/   – Binary + library: config, devices, ingest, uplink, retention, health,
                     metrics_server, systemd; entry points collector_app::run / run_with_publisher
  sunspec-sim/     – SunSpec Modbus TCP simulator (library + sunspec-sim binary) for tests and compose
fuzz/              – cargo-fuzz targets (own workspace, nightly): decode_model, model_chain, config;
                     seeds.py; run in CI by ClusterFuzzLite (.clusterfuzzlite/, fuzz.yml)
```

## Build & Test

```sh
# Prerequisites: Rust 1.88+, cmake, a C toolchain, pkg-config, libssl-dev, libcurl4-openssl-dev
# (librdkafka is built from source; SQLite and zstd are bundled)
cargo build --workspace
cargo test --workspace     # simulator-backed; no hardware or Kafka needed

# Ignored Kafka tests, with the compose stack running:
SUNSPEC_KAFKA_BROKERS=localhost:29092 cargo test -p avro-kafka --test kafka_integration_tests -- --ignored

# Run locally against the simulator (no brokers set = records only logged at debug)
cargo run -p sunspec-sim -- --bind 127.0.0.1:1502 --units 1,2
SUNSPEC_STATIC_DEVICES=127.0.0.1:1,127.0.0.1:2 SUNSPEC_PORT=1502 cargo run -p collector-app
```

## CI Checks (must pass before merging)

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --workspace -- -D warnings
RUSTFLAGS=-Dwarnings cargo build --workspace
RUSTFLAGS=-Dwarnings cargo test --workspace
cargo audit
```

Plus the **Docker E2E** job in `.github/workflows/ci.yml`: builds the `simulator` and `collector`
images, runs kafka + simulator + collector with compose, waits for `/healthz`, checks records are
keyed `simulator/1` and `simulator/2`, runs the ignored Kafka tests, and checks a clean SIGTERM
exit (code 0, "collector stopped" logged). `release.yml` cross-compiles x86_64 and aarch64 archives, tests `scripts/install.sh` under
systemd and builds multi-arch images; it publishes only on a `v*` tag (never push tags without
the owner's approval). On a tag (and on manual runs) it attests the release files and images with
`actions/attest` and publishes the bundle as `sunspec-collector-<version>.intoto.jsonl`.

On pull requests, a `Changes` job gates the jobs: Check & Lint, Test and Docker E2E run only
when code changed (crates, Cargo files, Docker files, compose, `fixtures/`, `ci.yml`, or the docs
files tests read: `docs/configuration.md`, `docs/guides/polling-profiles.md`, `docs/SUMMARY.md`,
`docs/reference/models/`, `docs/config.example.toml`, `docs/sunspec-collector.env`); Security Audit only when a `Cargo.toml`, `Cargo.lock` or `ci.yml`
changed. Skipped required jobs count as passing. Keep the list in `ci.yml` in step when tests or
the image start reading another file. `audit.yml` audits `main` daily. `scorecard.yml` runs
OpenSSF Scorecard (README badge, Security tab); it must stay minimal (no top-level env or
defaults, only checkout, upload-artifact, upload-sarif and scorecard-action). `codeql.yml` (Rust
and Actions, `build-mode: none`) runs on PRs that change code or workflows; `fuzz.yml`
(ClusterFuzzLite) fuzzes for 2 minutes on PRs that change sunspec-parser, poller-actor,
collector-app's `config.rs` or the fuzz setup, and for 30 minutes weekly; neither is required. Fixed fuzz crashes become
regression tests in the crate's tests.

Workflow hygiene: pin actions to a full commit SHA with the release as a comment
(`@<sha> # v6.1.0`), Docker base images by digest (the release matrix passes each target's
pinned cross-rs image as `BASE_IMAGE` to `docker/Dockerfile.cross`); top-level `permissions:
contents: read`, with write scopes granted per job. Dependabot updates actions, Cargo and the
Dockerfiles' digests.

## Conventions

- **Error handling**: Library crates use `thiserror`; `collector-app` uses `anyhow`.
- **Logging**: Use `tracing` macros with structured fields; per-cycle/per-drain messages at debug.
- **No `#![allow(dead_code)]`** — address warnings before merging.
- **`types` crate stays lightweight** — only `serde` as a dependency.
- **Formatting**: `rustfmt.toml` enforces max width 100 and auto-reordered imports.
- **Dual license**: MIT OR Apache-2.0.
- **MSRV**: 1.88 (`rust-version` in workspace `Cargo.toml`, inherited by every crate with
  `rust-version.workspace = true`). Resolver 3 makes `cargo update` and Dependabot prefer
  dependency versions that support it; check a lockfile change with `cargo +1.88 check --locked`.

## Key Patterns

- **Per-device supervision**: one task per device (`poller_actor::run_device`). Session = connect →
  discover → poll. On failure or stall, back off `respawn_delay_ms` × 2^(n−1), capped at 60 s, plus
  up to 10 % device-salted jitter; reset after a session that polled. Exits only on shutdown.
  Devices are keyed by host and unit ID.
- **RTU over TCP**: `modbus.framing = "rtu"` (or per device, or `SUNSPEC_MODBUS_FRAMING`) makes
  `ModbusClient` attach tokio-modbus's RTU client to a TCP stream. Units with RTU framing on one
  host:port share one lazily connecting `ModbusClient` (`DeviceConfig::shared_client`, built in
  `devices.rs`), so requests never overlap on the bus; validation requires them to share
  `request_timeout_ms` and `max_batch_size`. `sunspec_sim::start_rtu_over_tcp` / `sunspec-sim --rtu`
  simulate a converter.
- **Probe and fixtures**: `collector-app --probe host[:unit]` (`collector_app::probe::probe`)
  writes a `types::DeviceDump` (format 1: registers as hex words, points in definition order,
  top-level `SN`/MAC/IP blanked unless `--keep-identifiers`); `sunspec-sim --fixture` and
  `UnitSpec::from_dump` + `SimHandle::load_dump` replay it; `fixture_replay_tests.rs` replays
  every `fixtures/devices/*.json`.
- **Polling profiles**: `poller_actor::ModelSchedule` (include, exclude, per-model intervals) on
  `ActorConfig.models`, from `poller.include_models` / `exclude_models` / `model_intervals_s`
  (env `SUNSPEC_INCLUDE_MODELS` etc.). Static device entries override poll interval, request
  timeout, base address, max batch size and the model settings (`CollectorConfig::device_config`
  merges them; intervals merge per model, lists replace). A slow model is due again on the first
  tick ≥ interval after its last successful read; failed reads stay due. Idle cycles count as
  progress for stall detection. `SimHandle::reads_at` counts reads per address for tests.
- **Stall detection**: no successful read for `poller.stall_timeout_ms` (default
  max(10 × poll interval, 30 s)) aborts the session (`sunspec_poller_stalls_total`). 10 consecutive
  failed cycles also end a session.
- **Model chain walk**: read 2 registers for `SunS` (0x5375 0x6e53) at the configured base, then
  40000, 0, 50000; read each model header one at a time until 0xFFFF (max 256 models).
  Rediscovered on every reconnect.
- **Non-blocking polling**: a full channel drops the sample after one poll interval
  (`sunspec_samples_dropped_total{reason="channel_full"}`).
- **Ingest** batches samples arriving within 100 ms (max 256) into one SQLite transaction; buffer
  rows hold the `Sample` as JSON (Avro is produced at uplink time).
- **At-least-once uplink**: one Kafka record per sample; delete only rows Kafka acknowledged;
  undelivered rows stay and are retried first; rows that can never be encoded are deleted and
  counted `reason="corrupt"`. Drains back to back while batches are full; backs off 1 s × 2^(n−1)
  up to 30 s on errors.
- **Retention** (every 10 s): `buffer.max_age_s` and `buffer.max_rows`, counted in
  `sunspec_buffer_dropped_total{reason}`.
- **Shutdown order** (SIGINT/SIGTERM via a `tokio::sync::watch` channel): device pollers (10 s
  grace, then abort) → ingest persists everything queued → uplink drains for up to
  `buffer.shutdown_drain_timeout_ms` → exit 0.
- **Strict config**: defaults → file (`--config` / `SUNSPEC_CONFIG`; `.json` = JSON, else TOML) →
  `SUNSPEC_*` env. Unknown file keys (`deny_unknown_fields`), unparsable env values, invalid static
  devices and unsupported `kafka.acks`/`kafka.compression` fail at startup; `validate()` rejects
  zero intervals and ports, `modbus.max_batch_size` above 125, and `kafka.acks` 0/1 while
  idempotence is on. An unbindable metrics port stops startup (`bind metrics port <port>`). New
  settings need validation and docs.
- **Live readings**: `DeviceHealth::snapshot()` holds the latest discovery and one `Sample` per
  polled model; `Health::devices()` serves them as `/devices` and `/devices/{host}/{unit}` on the
  metrics port (`health.rs`, `metrics_server.rs`).
- **Health**: `/healthz` is 200 while ingest and uplink heartbeats are under 120 s old; device
  outages do not affect it. The systemd watchdog is pinged only while healthy.
  `sunspec_device_up` is set to 0 when a device's poller starts, so never-reachable devices show 0.
- **Metrics naming**: `sunspec_*` prefix, counters end in `_total`, durations are `_seconds`
  histograms; per-device metrics use `device="host/unit"`. Add help text in
  `collector-app/src/metrics_server.rs`.
- **Record format** is a public contract (schema `com.rusty.sunspec.v2.SunspecSample`, version
  2.1): changes to the `.avsc`, key or headers must be noted in `CHANGELOG.md`. Fields that old
  readers can resolve (a new field with a default, like 2.1's `labels`) stay in `v2`; anything
  else needs `v3` and a breaking entry. Any schema change changes the fingerprint.
- **Labels and catalog**: `sunspec_parser::decode_model_labeled` returns points plus `labels`
  (enum symbol names, set bitfield bits as `A,B` or `bit<n>`); `Sample.labels` carries them to
  records and `/devices`. `catalog()` / `model_info(id)` serve `--print-models`, `/models` and
  `/models/{id}` (point names with `[*]` for repeating groups).

## Docker

```sh
docker compose up --build        # kafka (KRaft, host listener localhost:29092), simulator (units 1,2),
                                 # collector (:9090, polls simulator:1,simulator:2), prometheus (:9091)
docker compose down -v           # Tear down with volumes
```

`Dockerfile` targets: `collector` (default) and `simulator`. Images run as non-root user
`collector`. The collector stage has `WORKDIR /app/data` and does not set `SUNSPEC_BUFFER_PATH`, so
the default buffer is `/app/data/sunspec-buffer.sqlite` (the `/app/data` volume) and a config-file
`buffer.path` is respected. `HEALTHCHECK` hits `http://localhost:${SUNSPEC_METRICS_PORT:-9090}/healthz`.

## Documentation

Docs live in `docs/` and are built with mdBook (`docs/SUMMARY.md`); links between pages are
relative. The site is organized by task: `getting-started/`, `guides/`, `reference/`,
`contributing/` and `project/`, with the larger pages (`configuration.md`, `ARCHITECTURE.md`,
`DEVELOPER_GUIDE.md`, `build.md`, `buffer_maintenance.md`) at the top of `docs/`. Keep `README.md`,
`docs/configuration.md`, `docs/guides/polling-profiles.md`, `docs/config.example.toml`,
`docs/sunspec-collector.env` and `docs/reference/metrics.md` in sync with the code, and record
user-visible changes in `CHANGELOG.md`. `docs/reference/models/` is generated from the bundled
definitions (`UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs`); a test fails
when it is stale. Deploy Docs builds the site with mdbook-linkcheck on docs PRs, so broken links
fail there.
