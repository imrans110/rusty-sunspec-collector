# rusty-sunspec-collector

High-concurrency, memory-safe edge service that polls Modbus TCP inverters, parses SunSpec models, and publishes serialized data to Kafka. Built as a Cargo workspace with clear domain boundaries to keep build times and dependencies under control.

## Features (planned)
- Async Modbus TCP client with timeouts, retries, and device-quirk controls (`max_batch_size`, optional inter-read delay).
- SunSpec model parser (JSON/XML) with scale-factor handling and “Not Implemented” sentinel detection.
- Per-device poller actors using Tokio, supervised via `JoinSet` to auto-respawn failures.
- Avro/Kafka publisher with mockable producer for local testing.
- Discovery service with subnet scan and static-config mode for sensitive networks.

## Workspace layout
- `crates/collector-app` — binary orchestrator.
- `crates/modbus-client` — Tokio Modbus wrapper and quirks config.
- `crates/sunspec-parser` — model registry, scaling, sentinel handling.
- `crates/poller-actor` — per-inverter polling loop and supervision hooks.
- `crates/avro-kafka` — Avro schemas and Kafka producer wrapper.
- `crates/discovery` — subnet scanning and static device list support.
- `crates/types` — shared DTOs/traits (kept lightweight).
- `docs/` — design notes and backlog (`docs/plan.md`).

## Roadmap (weeks)
- Week 0: finalize deps, fmt/clippy/test aliases, sample configs.
- Week 1: Modbus client with retries/backoff and quirks; diagslave integration test.
- Week 2: SunSpec parser (JSON/XML load, scaling, NI sentinels, lenient lengths); golden tests.
- Week 3: Poller actors with `JoinSet` supervision, jittered loop, bounded channels.
- Week 4: Avro/Kafka producer (mockable), end-to-end harness from simulator to payload; discovery scan/static config.

## Getting started
1) Install Rust (latest stable recommended).  
2) Clone the repo and build: `cargo build --workspace`.  
3) Run lint/tests (once implemented): `cargo fmt && cargo clippy && cargo test --workspace`.  
4) For Modbus simulation during development, use a local simulator such as `diagslave` and point the client to it.

## Contributing
Issues and PRs are welcome. Please keep the `types` crate slim and avoid adding heavy deps to shared crates. See `docs/plan.md` for current priorities.

## License
Dual-licensed under MIT or Apache-2.0 (see workspace `Cargo.toml`).
