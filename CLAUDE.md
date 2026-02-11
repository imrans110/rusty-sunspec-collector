# CLAUDE.md — Project Instructions

## Project Overview

Rusty SunSpec Collector is a Rust workspace that polls SunSpec-compatible solar inverters over Modbus TCP, buffers telemetry in SQLite, and forwards it to Kafka in Avro format.

## Workspace Layout

```
crates/
  types/           – Shared domain types (lightweight, serde-only)
  discovery/       – CIDR subnet scanning + static device lists
  modbus-client/   – Modbus TCP with retry & exponential backoff
  sunspec-parser/  – SunSpec model parser (JSON, XML, register maps)
  poller-actor/    – Per-device poller actors with supervision
  buffer/          – SQLite WAL-mode durable message buffer
  avro-kafka/      – Avro OCF serialization + Kafka producer
  collector-app/   – Application binary (config, orchestration, metrics)
```

## Build & Test

```sh
# Prerequisites: Rust 1.80+, cmake, libssl-dev, pkg-config
cargo build --workspace
cargo test --workspace
```

## CI Checks (must pass before merging)

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --workspace -- -D warnings
cargo build --workspace
cargo test --workspace
cargo audit
```

## Conventions

- **Error handling**: Library crates use `thiserror`; `collector-app` uses `anyhow`.
- **Logging**: Use `tracing` macros with structured fields.
- **No `#![allow(dead_code)]`** — address warnings before merging.
- **`types` crate stays lightweight** — only `serde` as a dependency.
- **Formatting**: `rustfmt.toml` enforces max width 100 and auto-reordered imports.
- **Dual license**: MIT OR Apache-2.0.
- **MSRV**: 1.80 (set in workspace `Cargo.toml`).

## Key Patterns

- Shutdown coordination via `tokio::sync::watch` channel.
- Per-device respawn with exponential backoff (capped at 60s) and device-salted jitter.
- Uplink task separates corrupt vs valid messages to prevent head-of-line blocking.
- Modbus retry backoff shift clamped to 63 to prevent overflow.

## Docker

```sh
docker compose up --build        # Starts Kafka + collector + Prometheus
docker compose down -v           # Tear down with volumes
```

## Documentation

Docs live in `docs/` and are built with mdBook. See `docs/SUMMARY.md` for structure.
