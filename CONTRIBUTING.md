# Contributing

Thank you for your interest in contributing to Rusty SunSpec Collector. Everyone taking part is
expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## Issues

Use the issue forms: **Bug report**, **Feature request**, or **Device report**. A device report
says how the collector works with your inverter, meter or battery, and is useful whether or not it
works: it builds the list of tested devices and shows which vendor quirks need handling. Report
security problems privately as described in [SECURITY.md](SECURITY.md).

## Getting Started

1. Fork the repository and clone it locally.
2. Install the prerequisites:
   - Rust 1.88 or newer (the workspace MSRV)
   - cmake, a C toolchain and pkg-config (librdkafka is built from source)
   - OpenSSL and libcurl development headers; librdkafka 2.12 needs both to compile

   Debian/Ubuntu:

   ```sh
   sudo apt-get install cmake build-essential pkg-config libssl-dev libcurl4-openssl-dev
   ```

   macOS: `brew install cmake` (the OpenSSL and curl headers come with the SDK or Homebrew).

   SQLite and zstd are bundled; no system packages are needed for them.
3. Build: `cargo build --workspace`
4. Test: `cargo test --workspace`

See the [Developer Guide](docs/DEVELOPER_GUIDE.md) for full setup instructions.

## Tests

`cargo test --workspace` runs the whole suite. It needs no hardware and no Kafka: the Modbus,
poller and end-to-end collector tests start the `sunspec-sim` simulator on a local port, and the
collector tests use an in-memory or log-only publisher instead of Kafka.

To run one crate's tests:

```sh
cargo test -p poller-actor       # device lifecycle against the simulator
cargo test -p modbus-client      # Modbus client against the simulator
cargo test -p collector-app      # config, metrics, signals, full pipeline
```

The Kafka integration tests in `crates/avro-kafka/tests/kafka_integration_tests.rs` are
`#[ignore]`d because they need a broker, and one of them expects records published by a running
collector. Start the Compose stack, then run them against its host listener:

```sh
docker compose up --build -d kafka simulator collector
SUNSPEC_KAFKA_BROKERS=localhost:29092 \
  cargo test -p avro-kafka --test kafka_integration_tests -- --ignored
docker compose down -v
```

To try the collector by hand against simulated devices:

```sh
cargo run -p sunspec-sim -- --bind 127.0.0.1:1502 --units 1,2
SUNSPEC_STATIC_DEVICES=127.0.0.1:1,127.0.0.1:2 SUNSPEC_PORT=1502 cargo run -p collector-app
```

## Before Submitting a PR

Run the same checks that CI enforces:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --workspace -- -D warnings
RUSTFLAGS=-Dwarnings cargo build --workspace
RUSTFLAGS=-Dwarnings cargo test --workspace
cargo audit                      # cargo install cargo-audit --locked
```

CI also runs a Docker end-to-end job (`.github/workflows/ci.yml`): it builds the `simulator` and
`collector` images, starts Kafka, the simulator and the collector with Docker Compose, waits for
`/healthz`, checks that records reach Kafka keyed per device and decode, runs the ignored Kafka
tests, and checks that the collector exits 0 on SIGTERM. If your change touches the Dockerfile,
`docker-compose.yml`, the record format or shutdown, run the Compose steps above locally as well.
Changes to the Dockerfiles, `Cross.toml` or `scripts/` also trigger a dry run of the release
workflow, which cross-compiles both release targets, tests `install.sh` under systemd and builds
the multi-arch images without publishing them.

## Commits and Pull Requests

- Branch from `main` and keep each PR to one topic.
- Write commit summaries as short imperative sentences that say what changed.
- Add or update tests for behaviour you change; the simulator can inject faults (stalls, dropped
  connections, offline devices) for failure-path tests.
- Update the documentation in `docs/` and the `[Unreleased]` section of `CHANGELOG.md` when you
  change behaviour, configuration, metrics or the Kafka record format. Mark breaking changes.
- In workflows, pin every action to a full commit SHA with its release as a comment
  (`uses: owner/action@<sha> # v1.2.3`), keep the top-level `permissions` at `contents: read`
  and grant more per job; pin Docker base images by digest. Dependabot keeps the pins current.
- If you change the decoder, the model-chain walk or config parsing, run the matching fuzz target
  for a few minutes (see [Fuzzing](docs/DEVELOPER_GUIDE.md#fuzzing)); CI fuzzes code PRs too.
- All CI checks must pass before a PR is merged. A PR that changes only documentation (other
  than the docs files the tests read) skips the build, test and Docker jobs; see the CI section
  of the [Developer Guide](docs/DEVELOPER_GUIDE.md#ci).

## Cutting a release

Releases are published by pushing a `v*` tag; the steps (version bump, CHANGELOG, tag) are in
[docs/build.md](docs/build.md#cutting-a-release).

## Conventions

- **Error handling**: Library crates use `thiserror`; the application crate uses `anyhow`.
- **Logging**: Use `tracing` macros with structured fields. Per-cycle and per-batch messages go
  at debug level so steady-state logs stay quiet.
- **Metrics**: Names start with `sunspec_`, counters end in `_total`, durations are histograms in
  seconds ending in `_seconds`. Register help text for new metrics in
  `crates/collector-app/src/metrics_server.rs`.
- **Configuration**: Config is strict. New settings need a file key, an env var where it makes
  sense, validation, and an entry in `docs/configuration.md` and the example files.
- **No `#![allow(dead_code)]`**: Address warnings before merging.
- **Keep `types` lightweight**: Only `serde` as a dependency.
- **Formatting**: `rustfmt.toml` enforces max width 100 and auto-reordered imports.
- **MSRV**: 1.88 (`rust-version` in the workspace `Cargo.toml`; new crates add
  `rust-version.workspace = true`). The workspace uses resolver 3, so `cargo update` picks
  dependency versions that still build on 1.88.

## What to Work On

Check the backlog at the end of [docs/plan.md](docs/plan.md) and the limitations listed in the
[README](README.md#limitations), or open a GitHub Issue to discuss your idea before starting large
changes.

## License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache-2.0.
