# Developer Guide

Everything needed to build, test, and contribute to Rusty SunSpec Collector.

For configuration options see [Configuration](configuration.md). For deployment see [Operations](ops.md). For system design see [Architecture](ARCHITECTURE.md).

---

## Prerequisites

### Required

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | 1.80+ (MSRV) | Compiler and cargo |
| cmake | 3.x+ | Building librdkafka (rdkafka-sys) |
| pkg-config | any | Locating system libraries |
| libssl-dev | 3.x+ | TLS support for Kafka |
| build-essential | any | C compiler for native deps |

### Optional

| Tool | Version | Purpose |
|------|---------|---------|
| Docker | 20+ | Container builds |
| cross | 0.2+ | ARM64 cross-compilation |
| mdbook | 0.4.40 | Documentation site |
| diagslave | any | Modbus TCP simulator for integration tests |

### macOS

```bash
brew install cmake openssl pkg-config
```

### Ubuntu/Debian

```bash
sudo apt-get install cmake build-essential libssl-dev pkg-config python3
```

---

## Build and Run

```bash
# Debug build
cargo build --workspace

# Release build
cargo build --workspace --release

# Run with example config
cargo run -p collector-app -- --config docs/config.example.toml

# Run with env vars (mock mode — no Kafka needed)
SUNSPEC_STATIC_DEVICES=192.168.1.20:1 cargo run -p collector-app
```

### ARM64 Cross-Compilation

```bash
docker build -t sunspec-cross-arm64 -f docker/Dockerfile.arm64 .
cross build --release --target aarch64-unknown-linux-gnu
```

Output: `target/aarch64-unknown-linux-gnu/release/collector-app`

---

## Testing

```bash
# All tests
cargo test --workspace

# Single crate
cargo test -p sunspec-parser
cargo test -p buffer

# Config validation (uses fixture files in crates/collector-app/tests/fixtures/)
cargo test -p collector-app --test config_validation_tests

# E2E harness (no external services needed)
cargo test -p collector-app --test e2e_harness_tests
```

### Integration tests (require external services)

```bash
# Modbus simulator
cargo test -p modbus-client --test diagslave_tests

# Kafka broker on localhost:9092
SUNSPEC_KAFKA_BROKERS=localhost:9092 cargo test -p avro-kafka --test kafka_integration_tests
```

---

## CI/CD

### CI Pipeline

GitHub Actions (`.github/workflows/ci.yml`) runs on every push/PR touching `crates/`, `Cargo.toml`, or `Cargo.lock`:

| Step | Command |
|------|---------|
| Format | `cargo fmt --all -- --check` |
| Lint | `cargo clippy --all-targets --workspace` (`RUSTFLAGS="-Dwarnings"`) |
| Build | `cargo build --workspace` |
| Test | `cargo test --workspace` |

### Run CI locally

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --workspace -- -D warnings
cargo build --workspace
cargo test --workspace
```

### Documentation deployment

Docs auto-deploy to GitHub Pages via `.github/workflows/deploy-docs.yml` on pushes to `main` that touch `docs/` or `README.md`. Uses mdBook v0.4.40 (pinned).

---

## Project Conventions

### Crate Design

- **`types`** stays lightweight — only `serde`, no heavy crates
- Each domain crate owns its error type via `thiserror`
- `collector-app` is the only binary crate; everything else is a library
- No `#![allow(dead_code)]` — address warnings before merging

### Error Handling

- Library crates: `Result<T, CrateError>` with `thiserror`
- Application crate: `anyhow::Result` with `.context()`
- Never `unwrap()` in production paths
- Prefer `warn!()` + continue over `panic!()`

### Logging

- Use `tracing` macros with structured fields: `info!(ip = %device.ip, model_id, "message")`
- `debug!` for high-frequency events, `info!` for per-cycle, `warn!` for recoverable, `error!` for fatal

### Code Formatting

Defined in `rustfmt.toml`: max width 100, auto-reordered imports/modules, edition 2021.

---

## Workspace Layout

```
rusty-sunspec-collector/
  Cargo.toml              Workspace root (MSRV 1.80)
  Cargo.lock              Dependency lock (committed)
  Dockerfile              Multi-stage production build
  Cross.toml              ARM64 cross-compilation config
  rustfmt.toml            Formatting rules
  .github/workflows/
    ci.yml                CI: fmt, clippy, build, test
    deploy-docs.yml       mdBook -> GitHub Pages
  crates/
    types/                Shared data types
    discovery/            Network device scanning
    modbus-client/        Modbus TCP client + retry
    sunspec-parser/       SunSpec model parsing
    poller-actor/         Per-device polling loop
    buffer/               SQLite message queue
    avro-kafka/           Avro serialization + Kafka
    collector-app/        Main binary
      src/
        main.rs           Entry point, orchestration
        config.rs         Config loading + validation
      tests/
        config_validation_tests.rs
        e2e_harness_tests.rs
        fixtures/
  docker/
    Dockerfile.arm64      Cross-compilation image
  docs/                   mdBook documentation site
```

---

## Adding a New Crate

```bash
cargo init --lib crates/my-new-crate
```

Then add to `[workspace] members` in root `Cargo.toml`, use workspace dependencies where possible, define a `thiserror` error type, and ensure `cargo clippy` + `cargo fmt` pass.

---

## Documentation

```bash
# Local preview with hot-reload
cargo install mdbook
mdbook serve docs --open

# Build static site
mdbook build docs
```

---

## Debugging

### Verbose logging

```bash
RUST_LOG=debug cargo run -p collector-app -- --config docs/config.example.toml

# Per-crate levels
RUST_LOG=collector_app=debug,modbus_client=trace,poller_actor=info cargo run -p collector-app
```

### Modbus simulator

For testing without real hardware, use [diagslave](https://www.modbusdriver.com/diagslave.html):

```bash
diagslave -m tcp -p 5020
```

Then point the collector at `localhost:5020`.

### Common build issues

| Issue | Fix |
|-------|-----|
| `cmake not found` | `brew install cmake` (macOS) or `apt install cmake` (Linux) |
| `failed to install metrics recorder` | Ensure `install_recorder()` is called only once |
| SQLite `database is locked` | One collector per buffer file. Check: `fuser <path>` |
| Modbus timeout | Verify `nc -zv <ip> 502`, increase `SUNSPEC_MODBUS_TIMEOUT_MS` |
