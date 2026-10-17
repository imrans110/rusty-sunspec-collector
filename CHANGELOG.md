# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added
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
