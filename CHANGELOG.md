# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Upgrade notes
- A wrong setting now exits with status 78 instead of 1. Rerun `install.sh` to get the systemd unit with `RestartPreventExitStatus=78` (an edited unit is saved as `.service.bak`). Scripts or supervisors that check for exit status 1 after a configuration error should accept 78

### Added
- The documentation site lists every supported SunSpec model with its points (type, units, label, description and enum and bitfield names), generated from the bundled definitions: Reference → Supported SunSpec models

### Changed
- Configuration errors (settings that do not load or validate, an invalid `SUNSPEC_LOG_FORMAT`, Kafka settings the producer rejects) exit with status 78 (`EX_CONFIG`), and the systemd unit does not restart on it (`RestartPreventExitStatus=78`): the service stops once and shows as failed with the error in the journal, instead of restarting every 5 s with the same error. Other startup errors still exit with 1 and are restarted
- Quieter Kafka outages: librdkafka reported every reconnect attempt to a broker that was down, three error lines about every second. A failed broker connection is now logged at warn at most once per 5 minutes (`Kafka broker connection failed`), with the repeats at debug; librdkafka's other log lines keep their level under the `librdkafka` target, and other client errors are logged as `Kafka client error`. The uplink logs `records delivered again` (info) when an outage ends
- After a Kafka outage the backlog starts draining within about 2 s of the brokers answering, instead of after up to 30 s of backoff: when a batch timed out while broker connections were failing, the uplink asks the brokers for the topic's metadata every 2 s and ends the backoff when they answer. Rejected records keep the full backoff
- Discovery no longer warns on every reconnect about base addresses that hold no SunSpec map (for example 40000 on a device whose map is at 50000); those reads, and each failed Modbus attempt before a retry, are at debug. The poller still warns about a read that finally failed, and when no base has a map the error gives the device's last answer, e.g. `(last answer: Gateway target device failed to respond)`
- The docs and the shipped environment file say how to run without Kafka (comment out `SUNSPEC_KAFKA_BROKERS`; an empty value is a configuration error) and suggest `RUST_LOG=info,collector_app=debug,poller_actor=debug` over a bare `debug`, which also turns on the libraries' debug output
- The ignored Kafka decode test reads the device host from `SUNSPEC_E2E_DEVICE_HOST` (default `simulator`), so it also runs against a collector polling a simulator on the host
- The documentation site is reorganized by task: Get started (quick start, deployment), Guides (Kafka security, serial devices, polling profiles, probing a device, verifying a release, monitoring and alerts, buffer maintenance, troubleshooting, upgrade notes), Reference (configuration, command-line options, HTTP endpoints, metrics, record format, models), How it works, Contributing and Project (with the changelog). The Operations page was split up; its old URL redirects to Deployment, and moved sections leave a pointer at their old place. The site now follows the system light or dark setting, and each page links to its source for editing

### Fixed
- A device closing its Modbus TCP connection is reported as `connection closed by the device`. tokio-modbus reports it as whatever OS error the thread saw last, so logs showed messages such as `Operation now in progress (os error 36)` or `Resource temporarily unavailable (os error 35)`
- The deployment guide listed a `modbus reconnected` log line; the collector logs `modbus connected`

## [0.5.0] - 2026-10-18

### Upgrade notes
- If you run the collector with the shipped systemd unit (`install.sh` installs it), upgrade: in 0.2.0–0.4.0 its watchdog pings never reached systemd, so `WatchdogSec=30` killed and restarted the service about every 30 seconds (see Fixed). Rerun `install.sh`, then `sudo systemctl restart sunspec-collector`.
- Records use Avro schema 2.1, which adds a `labels` map with an empty default. Readers resolve old and new records with either schema, but the schema fingerprint changes: consumers that look up the writer schema by fingerprint need the new schema (`collector-app --print-schema`) first.

### Added
- Enum and bitfield names: records carry `labels`, e.g. `St: MPPT` and `Evt1: GROUND_FAULT,AC_DISCONNECT` (set bits comma-separated, `bit<n>` for unnamed bits, empty when no bit is set), decoded with the symbol tables in the bundled definitions. `/devices` shows them too
- Model catalog: `collector-app --print-models` prints, and `GET /models` and `/models/{id}` serve, every bundled model's points with type, units, label, description and enum/bitfield symbols; point names match records (`module[*].DCA` for repeating groups)
- `/devices` and `/devices/{host}/{unit}` on the metrics port: each device's state, discovered and polled models, and the latest decoded reading of each polled model, as JSON. Only the latest reading per model is kept
- Serial devices through serial-to-Ethernet converters: `modbus.framing = "rtu"` (also `SUNSPEC_MODBUS_FRAMING` and a per-device `framing`) speaks Modbus RTU frames over TCP. Units on the same converter share one connection, so their requests never overlap on the RS-485 bus; units sharing a converter must use the same `request_timeout_ms` and `max_batch_size`. `--probe` works through converters too
- Signed build provenance for releases: the archives, `install.sh`, `SHA256SUMS` and both GHCR images are attested by the release workflow (SLSA provenance signed with Sigstore, verifiable with `gh attestation verify`), and each release includes the signed bundle `sunspec-collector-<version>.intoto.jsonl`
- `sunspec-sim --rtu` and `sunspec_sim::start_rtu_over_tcp` simulate a converter (RTU framing with CRC; absent units stay silent); `ModbusClient::new` creates a client that connects on its first read

### Changed
- Every crate now declares the workspace MSRV (`rust-version = "1.88"`), and the workspace uses Cargo's resolver 3, so dependency updates keep to versions that build on Rust 1.88
- The help text of `sunspec_poll_duration_seconds` now says it times the models due in one poll cycle (it has done so since polling profiles)

### Fixed
- The systemd watchdog pings now reach systemd. The readiness notification unset `NOTIFY_SOCKET`, so every watchdog ping after it went nowhere, and with the shipped unit's `WatchdogSec=30` systemd killed and restarted the service about every 30 seconds. sd-notify is updated to 0.5, whose `notify` leaves the environment alone; the release workflow's install test now checks the service outlives its watchdog timeout
- A scale factor far outside SunSpec's -10 to 10 no longer produces infinite or NaN values (for example `WH_SF = 23901` made `WH` infinite); points whose value is not a finite number, including infinite float readings, are now null. Found by fuzzing

### Security
- The `openssl` crate (0.10.75, with eight advisories such as CVE-2026-41678) is no longer a dependency. It came in on Linux through the Prometheus exporter's push-gateway client (`hyper-tls`, `native-tls`), which the collector never used: `/metrics` is served by the collector's own HTTP server. The exporter's HTTP listener and push gateway are now disabled, removing about 30 crates; the advisories were in APIs the collector did not call. librdkafka's TLS (`openssl-sys`) is unaffected
- The Docker images and release binaries are built from base images pinned by digest (`rust`, `debian` and the cross-rs images), and every GitHub Actions step is pinned to a commit SHA, so a re-pushed tag upstream cannot change what a release is built with. OpenSSF Scorecard runs weekly; its badge is in the README

## [0.4.0] - 2026-10-17

### Added
- `collector-app --probe HOST[:UNIT]` reads one device's SunSpec map once and prints it as JSON (base address, model chain, registers, decoded points), using the service's configuration. Serial numbers and network addresses are blanked unless `--keep-identifiers` is given
- `sunspec-sim --fixture dump.json` replays a probe dump; the library gains `read_dump`, `UnitSpec::from_dump` and `SimHandle::load_dump`, and `types` gains `DeviceDump`
- `fixtures/devices/`: probe dumps that CI replays through discovery and decoding (`fixture_replay_tests.rs`); starts with a synthetic dump of the simulator's demo inverter
- The device report form asks for a probe dump

## [0.3.0] - 2026-10-17

### Upgrade notes
- A static device listed twice (same host and unit ID) now fails config validation; remove the duplicate entry.
- Records for models outside the 31 bundled in 0.2.0 (for example the float meters 211–214) now carry decoded `points` instead of an empty map. The schema is unchanged.
- The example config (`docs/config.example.toml`) now reads models 1 and 120 hourly and 121 every 10 minutes; remove `[poller.model_intervals_s]` from a copied config to read them every cycle.

### Added
- Every official SunSpec model definition is bundled: 112 models from github.com/sunspec/models (commit `90b4a331`), up from 31. Newly decoded: network and security models (2–19), IEEE 1547-2003 era controls (125–145), float meters (211–214) and secure meter readings (220), environmental sensors (302–308), string combiners (401–404), solar modules (501–502), tracker (601), DER DC measurement and control (714–715), storage and lithium-ion/flow batteries (801, 803–809), and the test and vendor models in the official repository. The release binary grows by about 1 MB
- Community files: `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1), `SECURITY.md`, issue forms (bug report, feature request, device report) and a pull request template; a social preview image (`docs/assets/social-preview.svg` and `.png`)
- Polling profiles: `poller.model_intervals_s` reads slow-changing models (for example common, nameplate and settings) at most once per interval instead of every cycle, and `poller.include_models` / `poller.exclude_models` choose which discovered models are polled (env `SUNSPEC_MODEL_INTERVALS_S`, `SUNSPEC_INCLUDE_MODELS`, `SUNSPEC_EXCLUDE_MODELS`). Cycles with nothing due do not contact the device and do not count as stalls
- Per-device settings on `[[discovery.static_devices]]` entries: `poll_interval_ms`, `request_timeout_ms`, `base_address`, `max_batch_size`, `include_models`, `exclude_models` and `model_intervals_s` (merged model by model with the global intervals)
- `SimHandle::reads_at(unit, addr)` in `sunspec-sim`, counting read requests per start address

### Changed
- `sunspec-parser` embeds every `models/model_<id>.json` through `build.rs` instead of a hand-maintained list
- **Breaking:** a static device listed twice (same host and unit ID) is now a config error instead of being polled once
- `sunspec_poll_duration_seconds` is recorded only for cycles that read at least one model

## [0.2.0] - 2026-10-17

### Upgrade notes
- Kafka consumers must switch to the v2 record format (one Avro single-object record per sample, keyed by device); see `crates/avro-kafka/schemas/sunspec_sample.avsc` and `docs/ops.md`. Rows buffered by an older version are published with empty `points`, so let the old version drain its buffer before upgrading if possible.
- Docker: the default buffer file is now `/app/data/sunspec-buffer.sqlite` (previously `/app/data/buffer.sqlite` via the image's `SUNSPEC_BUFFER_PATH`). To keep using an existing volume's unsent rows, set `SUNSPEC_BUFFER_PATH=/app/data/buffer.sqlite` (or `buffer.path`).
- Config is strict and several metric names changed; see `docs/ops.md` for the rename table and `docs/configuration.md` for deprecated keys.
- Docker: the collector image's `/app/collector-app` is now the `ENTRYPOINT`, so arguments after the image name go to the collector (`docker run sunspec-collector --config /app/collector.toml`) instead of replacing the command.

### Added
- Release pipeline (`.github/workflows/release.yml`): portable x86_64 and aarch64 Linux archives (glibc 2.18+, OpenSSL linked statically) with the binaries, systemd unit, environment file, example config and licenses; `SHA256SUMS`; an `install.sh` that verifies and installs a release (`curl -fsSL …/releases/latest/download/install.sh | sudo sh`) and keeps an existing `/etc/sunspec-collector.env`; multi-arch `ghcr.io/imrans110/sunspec-collector` and `sunspec-simulator` images built from the same binaries; GitHub Release notes from this file. Every push to `main` runs it as a dry run, which installs the archive under systemd and smoke-tests both image platforms; a `v*` tag publishes
- `collector-app --version` and `--help`, and `sunspec-sim --version`
- Kafka connection security: TLS (`ssl`), mutual TLS, and SASL `PLAIN` / `SCRAM-SHA-256` / `SCRAM-SHA-512` (`sasl_plaintext`, `sasl_ssl`) via new `kafka.security_protocol`, `kafka.ssl_*` and `kafka.sasl_*` settings (and `SUNSPEC_KAFKA_*` variables). Passwords can be read from files (`*_password_file`) for Docker secrets and systemd credentials, are redacted in debug output, and mismatched settings (e.g. `sasl_*` with `plaintext`) fail validation. The compose stack now publishes over SASL/PLAIN with a Docker secret, which the CI Docker E2E job exercises
- IEEE 1547-2018 DER models 701–713 (AC measurements, capacity, enter service, AC controls, volt-var, volt-watt, voltage/frequency trip curves, frequency droop, watt-var, storage capacity) bundled from the same pinned `sunspec/models` commit
- `vendored-openssl` cargo feature (collector-app, avro-kafka): builds OpenSSL from source and links it statically; used by the aarch64 cross build so the binary does not depend on the target's OpenSSL
- Docs: `docs/build.md` (building, Docker images, cross-compiling) and `docs/buffer_maintenance.md` (buffer semantics, retention, sizing, inspection); the October 2026 audit is recorded in `docs/CODE_REVIEW.md`
- CI `Docker E2E` job: builds the images, runs Kafka + simulator + collector with docker compose, waits for `/healthz`, checks records are keyed per device, decodes the published records with the Rust consumer test, and verifies a clean SIGTERM stop
- `simulator` Docker target and compose service (two units behind one gateway), so `docker compose up --build` produces data without hardware
- `/healthz` endpoint: 200 while ingest and the uplink are checking in, 503 otherwise, with JSON detail (`devices_up`, `devices_total`, heartbeat ages); the systemd watchdog is pinged only while healthy
- `SUNSPEC_LOG_FORMAT=json` for one JSON object per log line; `RUST_LOG` filtering is explicit
- `[metrics] port` and `[runtime] channel_capacity` / `respawn_delay_ms` config file sections
- Prometheus help text for every metric; `*_seconds` metrics are real histograms with buckets
- Edge decoding with the official SunSpec model definitions (models 1, 101–103, 111–113, 120–124, 160, 201–204, 802, bundled from github.com/sunspec/models under Apache-2.0): scale factors applied, "not implemented" sentinels mapped to null, strings trimmed, repeating groups named `group[i].Point`
- `collector-app --print-schema` prints the Avro schema of published records (`crates/avro-kafka/schemas/sunspec_sample.avsc`)
- `sunspec-sim` writes named points and runs a live demo (changing power, current, voltage, frequency and energy) in the binary
- Buffer retention: `buffer.max_rows` (default 1,000,000) and `buffer.max_age_s` (default 7 days) bound the buffer during long uplink outages; dropped rows are counted in `sunspec_buffer_dropped_total{reason}`, with `sunspec_buffer_rows` and `sunspec_buffer_oldest_age_seconds` gauges
- `buffer.shutdown_drain_timeout_ms` (default 5 s): on stop, the uplink keeps draining to Kafka for up to this long after ingest has persisted everything
- Per-device supervision: each device runs connect → discover → poll and is retried with exponential backoff (capped at 60 s) until shutdown, so devices that are offline at startup are picked up when they appear
- Stall detection: a device session with no successful read for `poller.stall_timeout_ms` (default max(10 × poll interval, 30 s)) is restarted
- Subnet mode rescans every `discovery.rescan_interval_s` (default 600) and starts pollers for new hosts; with no devices the collector stays up
- Per-device metrics labelled `device="host/unit"`: `sunspec_poll_success_total`, `sunspec_poll_errors_total{kind}`, `sunspec_poll_duration_seconds`, `sunspec_device_up`, `sunspec_device_last_success_timestamp_seconds`, `sunspec_poller_restarts_total`, `sunspec_poller_stalls_total`
- `sunspec-sim` crate: a SunSpec Modbus TCP simulator (library and `sunspec-sim` binary) with per-unit register maps and fault injection, used by the test suite
- CI pipeline with fmt, clippy, build, and test (`.github/workflows/ci.yml`)
- Dependabot configuration for Cargo and GitHub Actions
- Docker healthcheck and non-root user
- systemd service hardening (`NoNewPrivileges`, `ProtectSystem`, etc.)
- `BufferStore::with_config()` to respect `max_connections` setting
- Per-device respawn backoff (exponential, capped at 60s)
- Device-salted jitter to prevent correlated polling
- `debug!()` log for zero-length model skip
- `SemaphoreClosed` error variant in discovery
- `parse_unit_id_list()` and `discovery_unit_ids` config field
- Comprehensive documentation: Architecture, Developer Guide, Configuration, Operations
- License files (MIT + Apache-2.0)
- `CONTRIBUTING.md`

### Fixed
- Repeating groups whose count is a point in an enclosing group (e.g. the curve models' top-level `NPt` sizing `Crv[i].Pt`) decoded with no repetitions; counts are now resolved through enclosing groups like scale factors, and a not-implemented count (`0xFFFF`) means no repetitions
- `sunspec_device_up` is published as 0 as soon as a device's poller starts, so devices that never answer are visible (previously they had no series)
- `sunspec_poll_errors_total` also counts connect failures and Modbus errors during model discovery at session start
- A metrics port that cannot be bound now stops the collector at startup (`bind metrics port <port>`) instead of running without `/metrics` and `/healthz`
- The Docker image no longer sets `SUNSPEC_BUFFER_PATH` (which overrode `buffer.path` from a config file); the collector runs in `/app/data`, so the default buffer is `/app/data/sunspec-buffer.sqlite`. The `HEALTHCHECK` follows `SUNSPEC_METRICS_PORT`
- The example env file had an inline comment that systemd and `docker --env-file` pass through as part of the value, which strict config rejected; tests now load the shipped example config and env file
- docker-compose used `bitnami/kafka:3.7`, which no longer exists on Docker Hub; it now uses the official `apache/kafka:3.9.1` (KRaft) with a host listener on `localhost:29092`
- Uplink throughput was capped at `batch_size / drain_interval` (200 samples/s by default) because it paused between every batch; it now drains back to back while a backlog remains
- Gateways: devices are keyed by host and unit ID, so every unit behind one IP is polled (previously only the last one)
- Model discovery walks the model chain header by header instead of reading a fixed 200-register window, so models beyond register 200 (e.g. status, controls, MPPT) are no longer dropped; the standard bases 40000, 0 and 50000 are probed after the configured one
- `sunspec.base_address = 0` is accepted; 0 is a valid SunSpec base address
- Samples still queued at shutdown are written to the buffer before exit
- Kafka publishing failed at startup with the default `zstd` compression because librdkafka was built without zstd; rdkafka's `zstd` feature is now enabled
- SIGTERM (sent by systemd and Docker on stop) now takes the graceful shutdown path instead of killing the process
- Modbus client re-establishes the connection after transport errors and timeouts; Modbus exceptions no longer count as connection failures
- Device hosts may be hostnames; every resolved address is tried in turn
- Duplicate `PrometheusBuilder::new().install_recorder()` call (runtime crash)
- Duplicate `kafka_topic` and `kafka_enable_idempotence` in `Default` impl (compile error)
- Missing `discovery_unit_ids` field on `CollectorConfig` (compile error)
- Uplink task: separated corrupt vs valid messages to prevent head-of-line blocking
- Modbus backoff shift overflow (clamped to 63)
- Removed unused `retry_count` column from buffer schema

### Changed
- All crates take their version from `[workspace.package]` in `Cargo.toml`, so a release bumps one line
- Config validation rejects `kafka.acks` `0`/`1` while `kafka.enable_idempotence` is true (librdkafka refused to start) and `modbus.max_batch_size` above 125 (previously clamped silently)
- `modbus.timeout_ms` / `SUNSPEC_MODBUS_TIMEOUT_MS` are deprecated and ignored with a warning: device sessions always used `poller.request_timeout_ms`, so the setting had no effect
- README and docs rewritten against the code: removed claims that were not true ("zero data loss", "90%+" overhead reduction, "zombie poller detection", graceful SIGTERM before it existed) and dead links
- CI runs on every push to main and every pull request (no path filters), so its checks can be required for merging
- The systemd unit runs as a dynamic unprivileged user with `StateDirectory=sunspec-collector` (it ran as root), allows 30 s for the graceful stop, and adds further sandboxing
- Docker `HEALTHCHECK` uses `/healthz`; images run as a non-root user; the cross-compilation image (now `docker/Dockerfile.cross`) drops the unused `librdkafka-dev` and `libsqlite3-dev` packages
- **Breaking:** config is strict. Unknown keys in the config file, unparsable `SUNSPEC_*` values (the error names the variable), invalid static devices (e.g. unit 300) and unsupported `kafka.compression` / `kafka.acks` values fail at startup instead of silently falling back to defaults
- **Breaking:** metrics renamed to the `sunspec_` prefix with base units and `_total` counters: `sunspec_buffer_writes_total`, `sunspec_uplink_records_total`, `sunspec_uplink_errors_total`, `sunspec_uplink_publish_seconds`; the high-cardinality `batch_size` label is gone
- Logs use ANSI colours only on a terminal; per-host discovery timeouts log at debug instead of warn
- Kafka topic validation now matches Kafka's own rules
- **Breaking:** Kafka records are now one Avro single-object message per sample (schema v2, `com.rusty.sunspec.v2.SunspecSample`), keyed by `host/unit_id`, with `content-type` and `schema-fingerprint` headers. Previously each message was an Avro container file mixing devices, with the schema embedded and no key. Records carry decoded `points` alongside the raw `registers`
- **Breaking:** `model_name` uses the official SunSpec names (e.g. `inverter_three_phase` instead of `three_phase_inverter`)
- Only records Kafka acknowledges are deleted from the buffer; undelivered records stay buffered and are retried first, and records that cannot be encoded are discarded and counted
- The producer lingers 50 ms so records to the same partition are batched and compressed together
- Samples are written to SQLite in transactions of up to 256 rows (one fsync per batch instead of per sample), reducing flash wear
- Delivery is documented as at-least-once: rows are deleted only after Kafka acknowledges them, so a crash between publish and delete can produce duplicates
- `sunspec.discovery_register_count` / `SUNSPEC_DISCOVERY_REG_COUNT` are deprecated and ignored (a warning is logged)
- `poller.jitter_ms` now offsets each device's fixed polling schedule instead of stretching every interval
- Orchestration moved from `main.rs` into `collector_app::run` and library modules so it can be tested end to end
- tokio-modbus 0.9 → 0.17
- The Kafka integration test is `#[ignore]`d with instructions instead of silently passing without a broker; the `diagslave` test is replaced by simulator-backed client tests
- Docker base images upgraded from `bullseye` to `bookworm`
- Tokio features narrowed from `"full"` to specific features
- Removed `#![allow(dead_code)]` from all 7 library crates
- `Cargo.lock` is now committed (removed from `.gitignore` and `.dockerignore`)
- mdBook version pinned to 0.4.40 in deploy workflow
- MSRV raised to 1.88 in workspace `Cargo.toml` and the Docker builder image (required by quick-xml 0.42 and apache-avro 0.21)
- `rustfmt.toml` expanded with `max_width`, `reorder_imports`, `reorder_modules`
- sqlx 0.7 → 0.8, built with only the `sqlite` and `runtime-tokio` features (no TLS stack or unused MySQL/Postgres drivers)
- quick-xml 0.31 → 0.42

### Removed
- `CollectorConfig::discovery_unit_ids` (duplicate of `discovery.unit_ids`)
- Unused model-catalog code in `sunspec-parser` (the id/name/length JSON and XML parsers, `ModelCatalog`, `apply_scale`); the decoder and bundled definitions replace them

### Security
- Resolved all `cargo audit` vulnerabilities: RUSTSEC-2024-0363 (sqlx), RUSTSEC-2023-0071 (rsa, via the unused sqlx MySQL driver), RUSTSEC-2026-0346/0349 (rustls 0.21), RUSTSEC-2026-0098/0099/0104 (rustls-webpki 0.101), RUSTSEC-2026-0194/0195 (quick-xml), RUSTSEC-2026-0007 (bytes), RUSTSEC-2026-0204 (crossbeam-epoch)

## [0.1.0] - 2024-12-23

### Added
- Initial implementation
- Modbus TCP client with retry and exponential backoff
- SunSpec model parsing (JSON, XML, register map)
- Per-device poller actors with supervision
- SQLite-backed durable message buffer (WAL mode)
- Avro OCF serialization with Kafka producer
- CIDR subnet discovery and static device lists
- Prometheus metrics endpoint via Axum
- systemd watchdog integration (Linux)
- Docker multi-stage build
- ARM64 cross-compilation via `cross`
- mdBook documentation site with GitHub Pages deployment
