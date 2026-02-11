# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added
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
- Duplicate `PrometheusBuilder::new().install_recorder()` call (runtime crash)
- Duplicate `kafka_topic` and `kafka_enable_idempotence` in `Default` impl (compile error)
- Missing `discovery_unit_ids` field on `CollectorConfig` (compile error)
- Uplink task: separated corrupt vs valid messages to prevent head-of-line blocking
- Modbus backoff shift overflow (clamped to 63)
- Removed unused `retry_count` column from buffer schema

### Changed
- Docker base images upgraded from `bullseye` to `bookworm`
- Tokio features narrowed from `"full"` to specific features
- Removed `#![allow(dead_code)]` from all 7 library crates
- `Cargo.lock` is now committed (removed from `.gitignore` and `.dockerignore`)
- mdBook version pinned to 0.4.40 in deploy workflow
- MSRV set to 1.80 in workspace `Cargo.toml`
- `rustfmt.toml` expanded with `max_width`, `reorder_imports`, `reorder_modules`

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
