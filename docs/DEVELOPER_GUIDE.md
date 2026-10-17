# Developer Guide

Everything needed to build, run, test and change Rusty SunSpec Collector.

For release builds, Docker images and cross-compiling see [Building & Packaging](build.md). For
configuration options see [Configuration](configuration.md), for the system design see
[Architecture](ARCHITECTURE.md), and for deployment see [Operations](ops.md).

---

## Prerequisites

### Required

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | 1.88+ (MSRV, `rust-version` in the workspace `Cargo.toml`) | Compiler and cargo |
| cmake | 3.x | Builds librdkafka from source (rdkafka's `cmake-build` feature) |
| C/C++ toolchain | any | Compiles librdkafka and the bundled SQLite and zstd |
| pkg-config | any | Locates OpenSSL on Linux |
| OpenSSL headers | 3.x | TLS support in librdkafka (`libssl-dev`) |
| libcurl headers | any | Needed to compile librdkafka 2.12, although curl support is disabled (`libcurl4-openssl-dev`); see [Building & Packaging](build.md#why-cmake-openssl-and-curl) |

SQLite (through sqlx) and zstd are bundled and need no system packages.

### Optional

| Tool | Purpose |
|------|---------|
| Docker with Compose v2 | Docker images and the local stack (Kafka, simulator, collector, Prometheus) |
| [`cross`](https://github.com/cross-rs/cross) | aarch64 cross-compilation |
| `cargo-audit` | The security audit CI runs (`cargo install cargo-audit --locked`) |
| mdBook 0.4.40 | Building this documentation |
| `sqlite3` CLI | Inspecting the buffer |

### Debian / Ubuntu

```sh
sudo apt-get update
sudo apt-get install -y build-essential cmake pkg-config libssl-dev libcurl4-openssl-dev
```

### macOS

```sh
xcode-select --install          # C/C++ toolchain and SDK (includes the curl headers)
brew install cmake openssl@3
```

The OpenSSL build scripts find Homebrew's OpenSSL on their own; the resulting debug and release
binaries link it dynamically from Homebrew.

---

## Repository layout

```
rusty-sunspec-collector/
  Cargo.toml              Workspace root (members, MSRV 1.88, shared dependencies)
  Cargo.lock              Committed; CI and Docker builds use --locked
  CHANGELOG.md            Keep a Changelog format
  CODE_OF_CONDUCT.md      Contributor Covenant 2.1
  SECURITY.md             How to report vulnerabilities privately
  Dockerfile              Multi-target build: builder, runtime, simulator, collector
  docker-compose.yml      Kafka + simulator + collector + Prometheus
  Cross.toml              Maps the release targets to the custom cross images
  rustfmt.toml            Formatting rules (max width 100)
  .github/
    workflows/ci.yml          fmt, clippy, build, test, cargo audit, Docker E2E
    workflows/release.yml     Release archives and images (dry run unless a v* tag)
    workflows/deploy-docs.yml mdBook -> GitHub Pages
    workflows/scorecard.yml   OpenSSF Scorecard (README badge, Security tab)
    workflows/codeql.yml      CodeQL static analysis of the Rust code and the workflows
    workflows/fuzz.yml        ClusterFuzzLite: runs the fuzz targets
    dependabot.yml            Weekly Cargo, GitHub Actions and Docker image updates
    ISSUE_TEMPLATE/           Issue forms: bug report, feature request, device report
    pull_request_template.md  PR checklist
  crates/
    types/                Shared DTOs (serde only)
    discovery/            CIDR subnet scan
    modbus-client/        Modbus TCP client
      tests/sim_tests.rs
    sunspec-parser/       SunSpec model definitions and decoder
      models/             Official SunSpec JSON definitions (Apache-2.0)
      build.rs            Embeds every models/model_<id>.json
      src/registry.rs     Model registry and the definition types
      src/decode.rs       decode_model()
      tests/decode_tests.rs
    poller-actor/         Per-device supervisor, model-chain walk, polling
      tests/device_tests.rs
    buffer/               SQLite queue
      tests/buffer_tests.rs
    avro-kafka/           Record schema, single-object codec, Kafka publisher
      schemas/sunspec_sample.avsc
      tests/              codec, producer config and (ignored) Kafka integration tests
    collector-app/        The collector binary and its library
      src/main.rs         CLI, logging, signals
      src/app.rs          run(), run_with_publisher(), shutdown order
      src/config.rs       Config loading and validation
      src/devices.rs      Device set and per-device tasks
      src/ingest.rs       Channel -> SQLite batches
      src/uplink.rs       SQLite -> Kafka
      src/retention.rs    Row and age limits, buffer gauges
      src/health.rs       Heartbeats and /healthz report
      src/metrics_server.rs  Prometheus recorder, /metrics, /healthz
      src/systemd.rs      READY and watchdog notifications (Linux)
      tests/              End-to-end, CLI, config, metrics and signal tests
    sunspec-sim/          SunSpec Modbus TCP simulator (library + binary)
  fuzz/                   cargo-fuzz targets (own workspace, nightly); seeds.py writes seed corpora
  .clusterfuzzlite/       Build image and script for running the fuzz targets in CI
  docker/
    Dockerfile.cross      Cross-compilation image for the release targets
    prometheus.yml        Scrape config for the compose stack
  scripts/                Release packaging, install.sh, CHANGELOG section extraction
  fixtures/devices/       collector-app --probe dumps replayed by the fixture test
  docs/                   This documentation (mdBook), example config, systemd unit
```

---

## Building

```sh
cargo build --workspace                  # debug build of every crate
cargo build --release -p collector-app   # optimised collector binary
```

Binaries land in `target/debug/` or `target/release/` (`collector-app`, `sunspec-sim`). The first
build compiles librdkafka from source and takes noticeably longer than later ones. See
[Building & Packaging](build.md) for release builds, Docker images and cross-compiling.

---

## Running locally with the simulator

No hardware or Kafka is needed. Start two simulated inverters behind one address:

```sh
# Terminal 1
cargo run -p sunspec-sim -- --bind 127.0.0.1:1502 --units 1,2
```

Then point the collector at them:

```sh
# Terminal 2
SUNSPEC_STATIC_DEVICES=127.0.0.1:1,127.0.0.1:2 SUNSPEC_PORT=1502 cargo run -p collector-app
```

Without `SUNSPEC_KAFKA_BROKERS` the collector encodes each record and only logs it, at debug level,
then removes it from the buffer. To see one log line per record:

```sh
SUNSPEC_STATIC_DEVICES=127.0.0.1:1,127.0.0.1:2 SUNSPEC_PORT=1502 \
  RUST_LOG=info,avro_kafka=debug cargo run -p collector-app
```

`RUST_LOG=debug` shows everything, including every Modbus read and poll cycle. The buffer is
`sunspec-buffer.sqlite` in the current directory. Check the collector from a third terminal:

```sh
curl localhost:9090/healthz
curl -s localhost:9090/metrics | grep '^sunspec_'
```

### With Kafka

Start only the broker from the compose stack and publish to its host listener:

```sh
docker compose up -d kafka
SUNSPEC_STATIC_DEVICES=127.0.0.1:1,127.0.0.1:2 SUNSPEC_PORT=1502 \
  SUNSPEC_KAFKA_BROKERS=localhost:29092 cargo run -p collector-app

# Record keys (values are binary Avro)
docker compose exec kafka /opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 --topic sunspec.telemetry --from-beginning \
  --property print.key=true --property print.value=false
```

`docker compose up --build` runs the whole stack instead, with the collector and the simulator in
containers; see [Building & Packaging](build.md#docker-compose-stack).

### Other ways to run

```sh
cargo run -p collector-app -- --config docs/config.example.toml   # edit devices and brokers first
cargo run -p collector-app -- --print-schema                      # print the Avro schema and exit
```

---

## The simulator

`crates/sunspec-sim` is a SunSpec Modbus TCP device simulator, used by the test suite, the compose
stack and local development.

### What it serves

- Read holding registers (function 0x03) for one or more unit IDs on one listener. Each unit has
  its own register map: the `SunS` marker at the base address, each model's header (ID, length)
  and a zero-filled body, then the end marker `0xFFFF 0x0000`.
- Reads of more than 125 registers (or 0) are rejected with exception 0x03, like a real device.
- Other function codes get exception 0x01; addresses outside the map get exception 0x02 (or zeros,
  see `Unmapped`); unknown unit IDs get exception 0x0B (gateway target device failed to respond).

### Binary

```sh
cargo run -p sunspec-sim -- [--bind ADDR:PORT] [--units 1,2] [--models 1,103,160:48] [--base 40000]
cargo run -p sunspec-sim -- [--bind ADDR:PORT] --fixture fixtures/devices/<device>.json
```

| Flag | Default | Meaning |
|------|---------|---------|
| `--bind ADDR:PORT` | `0.0.0.0:1502` | Listen address |
| `--units LIST` | `1` | Unit IDs to serve, each with the same model chain |
| `--models LIST` | `1,103,120,160:48` | Model chain. A bare ID uses the body length of the bundled definition; `id:len` sets the body length explicitly, which models with repeating groups need (`160:48` is model 160 with two MPPT modules). |
| `--base ADDR` | `40000` | Base address of the `SunS` marker |
| `--rtu` | off | Speak Modbus RTU frames (unit, PDU, CRC) over TCP, like a transparent serial converter: absent units stay silent and frames with a bad CRC are ignored. Only function 0x03 is understood. |
| `--fixture FILE` | none | Replay a `collector-app --probe` dump: its unit ID, base address, model chain and registers. Cannot be combined with `--units`, `--models` or `--base`; the demo does not run. |

Invalid arguments print the usage and exit with code 2. The binary also runs a live demo for every
unit: it writes nameplate strings into model 1 (`Mn`, `Md`, `SN`, `DA`), sets the scale factors of
model 103, and once per second updates `W`, `A`, `PhVphA`, `Hz`, `St` and `WH`, so dashboards show
changing values. Points of models that are not in the chain are skipped.

### Library

Tests start a simulator on an ephemeral port and drive it through `SimHandle`. Add it as a
dev-dependency (`sunspec-sim = { path = "../sunspec-sim" }`).

| Item | Purpose |
|------|---------|
| `sunspec_sim::start(config, bind)` | Starts a simulator (async, needs a Tokio runtime); returns `io::Result<SimHandle>`. Bind `127.0.0.1:0` for an ephemeral port. |
| `sunspec_sim::start_rtu_over_tcp(config, bind)` | The same, speaking RTU frames over TCP like a serial converter |
| `SimConfig { units, unmapped }`, `SimConfig::single(unit)` | Units to serve and how unmapped addresses are answered |
| `UnitSpec::inverter(unit_id)` | Models 1 and 103 (body lengths 66 and 50) at base 40000 |
| `UnitSpec::with_models(unit_id, models)`, `UnitSpec { unit_id, base, models }` | Custom chains |
| `ModelSpec { id, len }` | One model; `len` excludes the 2 header registers |
| `parse_models("1,103,160:48")`, `default_body_len(id)` | Build chains the way the binary does |
| `Unmapped::Exception` / `Unmapped::Zeros` | Exception 0x02 or zeros outside the map |
| `read_dump(path)`, `UnitSpec::from_dump(&dump)`, `handle.load_dump(unit_id, &dump)` | Replay a `collector-app --probe` dump (`types::DeviceDump`); `from_dump` rejects a chain whose model addresses do not line up |
| `handle.reads_at(unit_id, addr)` | Read requests that started at an address (includes discovery's header read), for polling-schedule tests |
| `run_demo(handle, unit_ids)` | The binary's live demo |

`SimHandle` methods (cloning a handle shares the same simulator):

| Method | Effect |
|--------|--------|
| `local_addr()`, `port()` | Where the simulator listens |
| `model_start(unit, model_id)` | Address of a model's header, if the unit has the model |
| `set_register(unit, addr, value)`, `set_registers(unit, addr, &values)` | Write raw registers |
| `set_point(unit, model_id, "W", &values)` | Write a top-level point by name (raw big-endian words); returns `false` if the unit, model or point does not exist |
| `set_text(unit, model_id, "Mn", "text")` | Write a string point, NUL-padded to its size |
| `set_stalled(true)` | Read requests but never answer (timeouts, stall detection) |
| `drop_connections()` | Close every open connection; new connections are still accepted |
| `go_offline()` / `go_online()` | Refuse connections (and drop open ones) / listen again on the same port |
| `connections_accepted()`, `requests_served()` | Counters for assertions |

Example:

```rust
use sunspec_sim::{SimConfig, UnitSpec};

#[tokio::test]
async fn reads_scaled_power() {
    let sim = sunspec_sim::start(
        SimConfig::single(UnitSpec::inverter(1)),
        "127.0.0.1:0".parse().unwrap(),
    )
    .await
    .expect("start simulator");
    sim.set_point(1, 103, "W", &[4321]);
    sim.set_point(1, 103, "W_SF", &[(-1i16) as u16]); // W = 432.1
    sim.set_text(1, 1, "Mn", "Example");

    // Point a ModbusClient or a CollectorConfig at 127.0.0.1:sim.port() ...

    sim.set_stalled(true); // requests now time out
    sim.go_offline(); // connections are refused
    sim.go_online(); // and accepted again
}
```

---

## Tests

```sh
cargo test --workspace                                # everything except the Kafka tests
cargo test -p sunspec-parser                          # one crate
cargo test -p collector-app --test app_tests          # one test file
cargo test -p poller-actor stalled_session_is_restarted  # one test
```

The suite has about 60 tests and needs no hardware, network access or Kafka: device behaviour is
tested against the simulator on ephemeral ports, and the collector end to end with an in-memory
publisher. Tests use short real intervals (100 ms polls), so they take seconds once built.

### Integration test files

| File | Covers |
|------|--------|
| `crates/sunspec-parser/tests/decode_tests.rs` | Enum and bitfield labels (named, unnamed bits, unknown and unimplemented values, repeating groups); the catalog covers every point name records carry, for every model, with units, labels and symbols; every file in `models/` is bundled; every model decodes a zeroed map with each top-level point once and unique point names, and short or long maps never panic; float meters 211–214; model names; scaling (model 103); "not implemented" values become null; header, scale factors and padding are omitted; model 1 strings; model 160 repeating modules using top-level scale factors; float models and NaN; truncated registers |
| `crates/modbus-client/tests/sim_tests.rs` | Reads over 125 registers are split; an oversized `max_batch_size` passed to the client library is clamped to 125; an exception keeps the connection; reconnect after a dropped connection; a timeout drops the connection and recovers; offline device fails to connect, then recovers; hostnames are resolved |
| `crates/modbus-client/tests/rtu_tests.rs` | RTU over TCP against the simulator's converter mode: split reads and two units over one connection, exceptions, an absent unit timing out and the client reconnecting, concurrent readers sharing one client |
| `crates/poller-actor/tests/device_tests.rs` | Discovery of a 290-register, 7-model chain; map found at another standard base; device without a SunSpec map; device offline at startup is polled once it appears; a stalled session is restarted |
| `crates/buffer/tests/buffer_tests.rs` | Enqueue, dequeue and delete; deleting nothing is a no-op; batch enqueue keeps order; trimming keeps the newest rows; age-based retention |
| `crates/avro-kafka/tests/codec_tests.rs` | Single-object round trip (points and labels); `labels` was added with a default; `C3 01` and fingerprint header; a foreign fingerprint is rejected; one record per sample keyed by device |
| `crates/avro-kafka/tests/kafka_config_tests.rs` | The producer accepts every supported compression codec (zstd is compiled in), the default config and each security protocol (TLS with the `tests/fixtures/test-ca.pem` CA, SASL PLAIN and SCRAM); an unreadable CA is rejected; secrets are redacted in debug output. No broker needed |
| `crates/avro-kafka/tests/kafka_integration_tests.rs` | Ignored by default; see below |
| `crates/collector-app/tests/app_tests.rs` | The whole collector against the simulator: every unit behind a gateway is polled; every model of a long chain is polled; subnet mode keeps running and finds a device on rescan; a 5,000-row backlog drains within 2 s; published records are decoded and keyed by device; a metrics port that is already taken fails startup (`a_taken_metrics_port_fails_startup`) |
| `crates/collector-app/tests/config_validation_tests.rs` | TOML and JSON fixtures (`tests/fixtures/`); invalid values fail validation; unknown keys are rejected; unparsable environment values name the variable; static device parsing (unit range, hostnames, default unit); Kafka compression and acks values; `acks` below `all` only with idempotence off (`acks_below_all_require_idempotence_off`); `modbus.max_batch_size` at most 125 (`modbus_batch_size_is_limited_to_the_protocol_maximum`); `[metrics]` and `[runtime]` sections; the shipped `docs/config.example.toml` and `docs/sunspec-collector.env` load and validate (`shipped_example_config_file_is_valid`, `shipped_example_env_file_is_valid`; the env file is read the way systemd and Docker read it, without trailing comments); Kafka security: SASL requires mechanism, username and password, `ssl_*`/`sasl_*` settings must match the protocol, certificate files must exist, passwords load from `*_password_file` and never appear in debug output |
| `crates/collector-app/tests/polling_profile_tests.rs` | Polling profiles from TOML, JSON and environment variables; per-device merge rules; `SUNSPEC_STATIC_DEVICES` dropping file overrides; every validation error; every TOML and JSON example in the Polling profiles and Complete examples sections of `docs/configuration.md`; an end-to-end run where the simulator's read counts show slow models are read less often |
| `crates/collector-app/tests/devices_endpoint_tests.rs` | `/devices` against a collector polling the simulator: devices sorted, a down unit listed without readings, scaled values in definition order, a changed register visible within a few poll intervals, `/devices/{host}/{unit}` and its 404 |
| `crates/collector-app/tests/rtu_tests.rs` | Framing from the file, the environment and device entries; units on one RTU converter must share timeout and batch size; two units behind one simulated converter are polled end to end over a single connection; `--probe` through a converter |
| `crates/collector-app/tests/cli_tests.rs` | `--version`, `--help`, `--print-schema`; `--probe` against the simulator prints only JSON on stdout with the serial number blanked; a bad `--probe` target fails |
| `crates/collector-app/tests/fixture_replay_tests.rs` | Every dump in `fixtures/devices/` is served by the simulator, probed again, and must give the same model chain, registers and decoded points; a dump with a broken chain is rejected |
| `crates/collector-app/tests/metrics_tests.rs` | `/healthz` and `/metrics` of a collector polling unit 1 and a unit 7 that does not exist: `devices_up` 1 of `devices_total` 2, HELP text, per-device series, `sunspec_device_up` 1 for unit 1 and 0 for the unreachable unit 7, histogram buckets, no `batch_size` label. It installs the global metrics recorder, so it is its own test binary. |
| `crates/collector-app/tests/signal_tests.rs` | Unix only: spawns the built `collector-app` binary, sends SIGTERM and expects a graceful stop with exit code 0 |

### Unit tests inside the crates

| Module | Covers |
|--------|--------|
| `poller-actor/src/lib.rs` | Base address order; respawn backoff doubles and caps at 60 s; default stall timeout |
| `collector-app/src/health.rs` | Healthy only while both heartbeats are fresh; device counts do not affect health |
| `collector-app/src/ingest.rs` | Everything queued before the channel closes is persisted |
| `collector-app/src/uplink.rs` | No pause while a backlog remains; failure backoff up to 30 s |
| `collector-app/src/retention.rs` | Row and age limits |

### Kafka integration tests

`crates/avro-kafka/tests/kafka_integration_tests.rs` has two `#[ignore]`d tests that need a broker:

- `publishes_keyed_records_to_kafka` publishes two records to `SUNSPEC_KAFKA_BROKERS`.
- `published_records_decode_to_samples` consumes the topic and checks that the records published by
  a running collector decode, carry the `content-type` header, are keyed by their device, and
  include a decoded model 103 power value. It expects records from the compose stack's simulator
  (keys `simulator/1`, `simulator/2`).

Run them against the compose stack, once the collector is healthy and publishing:

```sh
docker compose up -d --build
curl -fsS localhost:9090/healthz                       # wait until this returns 200
SUNSPEC_KAFKA_BROKERS=localhost:29092 \
  cargo test -p avro-kafka --test kafka_integration_tests -- --ignored --nocapture
docker compose down -v
```

The CI **Docker E2E** job does the same: it builds the `simulator` and `collector` images, starts
Kafka, the simulator and the collector with `docker compose`, waits for `/healthz` to return 200 and
for `sunspec_uplink_records_total` to be non-zero, checks with the console consumer that records
are keyed `simulator/1` and `simulator/2`, runs the ignored tests above with
`SUNSPEC_KAFKA_BROKERS=localhost:29092`, and finally stops the collector with SIGTERM and checks that
it logged `collector stopped` and exited with code 0.

### Writing tests

- Use the simulator rather than mocks, and bind it to `127.0.0.1:0`.
- A test that runs the collector sets `metrics_port = 0` so it gets a free port: a taken port fails
  startup, and tests run in parallel.
- Put SQLite buffers in the system temp directory with a unique name and remove the file and its
  `-wal` and `-shm` files afterwards.
- A test that installs the global metrics recorder must live in its own test file (the recorder
  can be installed once per process).
- Tests that set environment variables hold the file's `ENV_LOCK` mutex.
- Tests that need external services are `#[ignore = "reason"]`, with the reason saying what they
  need.

---

## CI

`.github/workflows/ci.yml` runs on every push to `main` and on every pull request, with
`RUSTFLAGS=-Dwarnings`:

| Job | What it runs | On a pull request, runs when it changes |
|-----|--------------|------------------------------------------|
| Changes | Classifies the PR's changed files | Always |
| Check & Lint | `cargo fmt --all -- --check`, `cargo clippy --all-targets --workspace`, `cargo build --workspace` | Code (see below) |
| Test | `cargo test --workspace` | Code |
| Security Audit | `cargo audit` (prebuilt `cargo-audit`) | `Cargo.lock`, a `Cargo.toml` or `ci.yml` |
| Docker E2E | The compose stack end to end (see [Kafka integration tests](#kafka-integration-tests)) | Code |

"Code" is anything compiled, tested or built into an image: `crates/`, the Cargo files,
`rustfmt.toml`, `Cross.toml`, the Dockerfiles and `docker/`, `docker-compose.yml`, `fixtures/`,
`ci.yml`, and the three docs files the tests read (`docs/configuration.md`,
`docs/config.example.toml`, `docs/sunspec-collector.env`). A PR that changes only other files
(README, other docs, images, issue templates, the release or docs workflows) skips the jobs it
does not need. The four jobs stay required for merging: a job skipped by its `if:` reports as
skipped, which GitHub counts as passing. If you add a file that tests or the image build read, add
it to the Changes job's list. Pushes to `main` always run every job.

A new push to a pull request cancels that PR's run in progress. `audit.yml` runs `cargo audit`
on `main` every day as well, because advisories are published for dependencies that have not
changed.

Other workflows: `audit.yml` is the daily audit above, `release.yml` builds and tests the
release archives and images, and publishes them on a `v*` tag (see
[Building & Packaging](build.md#release-workflow)), `deploy-docs.yml` publishes this book to
GitHub Pages on pushes to `main` that touch `docs/` or `README.md` (mdBook 0.4.40), and
`scorecard.yml` runs [OpenSSF Scorecard](https://scorecard.dev/) weekly and on pushes to `main`,
publishing the score behind the README badge and the findings to the repository's Security tab.
`codeql.yml` runs CodeQL on the Rust code and on the workflows (pull requests that change code or
workflows, every push to `main`, weekly), and `fuzz.yml` runs the fuzz targets (see
[Fuzzing](#fuzzing)). Neither is required for merging; CodeQL's check on a pull request lists the
alerts it adds, and the Security tab has all of them.

Workflow hygiene, which Scorecard checks:

- Every action is pinned to a full commit SHA, with the release it is in as a comment
  (`uses: actions/checkout@<sha> # v6.1.0`); `dtolnay/rust-toolchain` is pinned to a commit on
  its `master` branch and given `toolchain: stable`. Base images in the Dockerfiles are pinned by
  digest, with the tag kept for readers, and the release workflow passes each target's pinned
  cross-rs image to `docker/Dockerfile.cross`. Dependabot proposes new SHAs and digests weekly.
- Each workflow's top-level `permissions` is `contents: read`; a job that needs more (publishing
  images, releases or Pages) asks for it in its own `permissions`.

The CI jobs use the latest stable Rust; the Docker images are built with Rust 1.88, so the Docker
E2E job is the one that exercises the MSRV. Every crate inherits `rust-version` from the workspace,
and the workspace uses resolver 3, so `cargo update` and Dependabot prefer dependency versions that
support 1.88 (a crate whose newer releases need a newer Rust stays at its last compatible
version). To check a lockfile change locally: `rustup toolchain install 1.88` and
`cargo +1.88 check --locked --workspace --all-targets`.

### Before pushing

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --workspace -- -D warnings
RUSTFLAGS=-Dwarnings cargo build --workspace
RUSTFLAGS=-Dwarnings cargo test --workspace
cargo audit
```

Changing `RUSTFLAGS` rebuilds every dependency, librdkafka included. Setting
`CARGO_TARGET_DIR=target/ci` for the last two commands keeps that build separate from your normal
one.

---

## Fuzzing

`fuzz/` holds [cargo-fuzz](https://rust-fuzz.github.io/book/cargo-fuzz.html) targets for the
code that reads input the collector does not control:

| Target | Input | Checks |
|--------|-------|--------|
| `decode_model` | A bundled model ID and arbitrary registers | `decode_model_labeled` does not panic, agrees with `decode_model`, names each point once, labels only existing points, and returns only finite floats |
| `model_chain` | An arbitrary register map at the configured or a standard base | `discover_models` ends with a chain or an error; an accepted chain starts with `SunS`, has at most 256 contiguous models and ends with `0xFFFF`; each model it finds decodes without panicking |
| `config` | An arbitrary config file, loaded as TOML and as JSON | `CollectorConfig::from_file_contents`, `validate()` and `device_config()` return errors instead of panicking |

`discover_models` reads through the `ReadRegisters` trait, which the target implements with an
in-memory register map, so the chain walk runs without a network. The config target skips inputs
that could name a `*_file` setting, since loading reads those files.

`fuzz/` is its own workspace (it needs nightly Rust and sanitizers), so the workspace commands
above do not build it. It has no committed lockfile: builds copy the workspace's `Cargo.lock`
into `fuzz/` first, so the targets use the same dependency versions as the collector, and only
one lockfile needs updates. To run a target locally:

```sh
rustup toolchain install nightly
cargo install cargo-fuzz --locked
cp Cargo.lock fuzz/                          # build against the workspace's locked versions
fuzz/seeds.py fuzz/seeds                     # seed corpora from fixtures/ and the example config
cargo +nightly fuzz run -O decode_model fuzz/corpus/decode_model fuzz/seeds/decode_model \
  -- -max_total_time=300
```

A crash is saved under `fuzz/artifacts/<target>/`; `cargo +nightly fuzz run -O <target> <file>`
replays it and `cargo +nightly fuzz tmin -O <target> <file>` shrinks it. Turn a fixed crash into
a regression test in the crate's tests (see `values_that_are_not_finite_are_null` in
`crates/sunspec-parser/tests/decode_tests.rs`, found by `decode_model`).

In CI, `.github/workflows/fuzz.yml` uses [ClusterFuzzLite](https://google.github.io/clusterfuzzlite/):
it builds the targets in the OSS-Fuzz Rust image (`.clusterfuzzlite/`) with AddressSanitizer and
seed corpora from `fuzz/seeds.py`, fuzzes for five minutes on pull requests that change code, and
for half an hour every Sunday, keeping the corpus between runs as workflow artifacts. A crash fails
the run and is uploaded with its input. When you add a target, add a `[[bin]]` to
`fuzz/Cargo.toml`, add it to `targets` in `.clusterfuzzlite/build.sh`, and give it seeds in
`fuzz/seeds.py` if the fixtures have useful input.

---

## Adding a SunSpec model definition

Every model in the pinned commit of [github.com/sunspec/models](https://github.com/sunspec/models)
is already bundled. To add a model published later, or to move to a newer commit:

1. Copy `json/model_<id>.json` into `crates/sunspec-parser/models/`, unmodified, and update the
   commit recorded in `crates/sunspec-parser/models/README.md`.
2. Nothing else needs registering: `build.rs` embeds every `model_<id>.json` in that directory.
   `every_model_file_is_bundled` and `every_bundled_model_decodes_its_fixed_points`
   (`crates/sunspec-parser/tests/decode_tests.rs`) check that the file parses, decodes, and has
   unique point names.
3. If the model has repeating groups, count points or float points, add a value test for it.
   Point types the decoder does not know are published as null.
4. Update the model tables in `crates/sunspec-parser/models/README.md` and
   [Architecture](ARCHITECTURE.md#bundled-definitions), and add a CHANGELOG entry.

The simulator can then serve the model: `--models 1,103,<id>`, or `<id>:<len>` when the model
has repeating groups (the bundled length only counts fixed groups).

---

## Adding a crate

```sh
cargo new --lib crates/my-new-crate
```

Make sure it is listed in `[workspace] members` in the root `Cargo.toml`, inherit `edition`,
`rust-version`, `license` and `repository` from `[workspace.package]`
(`edition.workspace = true`, ...), use workspace dependencies where they exist, define a
`thiserror` error type, and make sure `cargo clippy` and `cargo fmt` pass.

---

## Coding conventions

### Crates and errors

- Library crates return `Result<T, CrateError>` with an error enum built with `thiserror`;
  `collector-app` uses `anyhow::Result` with `.context()`.
- `types` stays lightweight: `serde` is its only dependency.
- No `unwrap()` on runtime paths. `expect()` is used only for invariants such as the bundled schema
  and model JSON or a poisoned lock.
- After startup, failures are logged, counted and recovered rather than ending the process.
- No `#![allow(dead_code)]`; fix warnings instead (CI builds with `-Dwarnings`).

### Logging

- Use `tracing` macros with structured fields, not formatted strings:
  `warn!(device = %device, model_id = model.id, error = %err, "modbus read failed")`.
- Messages are short, lower case, without a trailing period.
- `debug!` for anything per read, per cycle or per batch; `info!` for lifecycle events (startup,
  models discovered, shutdown); `warn!` for recoverable failures (a session ended, a sample dropped,
  records not delivered). Steady-state operation should be quiet at `info`.

### Metrics

- Names start with `sunspec_`; counters end in `_total`; durations are histograms in seconds ending
  in `_seconds`.
- Register HELP text for every new metric in `describe_metrics()`
  (`crates/collector-app/src/metrics_server.rs`) and add it to the metrics table in
  [Operations](ops.md).
- Per-device metrics use the `device` label with `DeviceIdentity::key()` (`host/unit`). Do not add
  labels with unbounded values (batch sizes, row IDs, error messages).

### Formatting

`rustfmt.toml`: edition 2021, `max_width = 100`, `reorder_imports` and `reorder_modules`. Run
`cargo fmt --all` before committing.

### Record schema

Any change to `crates/avro-kafka/schemas/sunspec_sample.avsc` changes the schema fingerprint, so
consumers holding the old schema can no longer decode new records. Treat it as a breaking change:
bump the namespace (`com.rusty.sunspec.v2` to `v3`), update the codec and its tests, and describe
the change in the CHANGELOG.

---

## Changelog and releases

- `CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Every
  user-visible change gets an entry under `## [Unreleased]` in the same pull request, in `Added`,
  `Changed`, `Fixed`, `Removed` or `Security`.
- Changes that need action from users (record format, metric names, config keys and validation)
  start with **Breaking:**.
- All crates share one version, `version` under `[workspace.package]` in `Cargo.toml`. Releases are
  published by `.github/workflows/release.yml` when a `v*` tag is pushed; the steps are in
  [Cutting a release](build.md#cutting-a-release).
- Dependabot opens weekly pull requests for Cargo dependencies (`deps` prefix; patch updates
  grouped into one PR, minor and major updates one PR each, since a 0.x minor bump is
  breaking), GitHub Actions (`ci` prefix; minor and patch releases grouped) and the base image
  digests of the Dockerfiles and `.clusterfuzzlite/Dockerfile` (`docker` prefix, grouped). The Rust builder image
  stays on the MSRV, so Dependabot updates only its digest.

---

## Documentation

The documentation is an mdBook in `docs/` (`docs/SUMMARY.md` lists the pages). Links between pages
are relative (`[Architecture](ARCHITECTURE.md)`).

```sh
cargo install mdbook --version 0.4.40 --locked
mdbook serve docs --open    # local preview with reload
mdbook build docs           # static site in docs/book
```

---

## Debugging

### Verbose logging

```sh
RUST_LOG=debug cargo run -p collector-app
RUST_LOG=info,poller_actor=debug,modbus_client=debug cargo run -p collector-app
SUNSPEC_LOG_FORMAT=json cargo run -p collector-app    # one JSON object per line
```

### Common problems

| Problem | Fix |
|---------|-----|
| `cmake` not found while building `rdkafka-sys` | Install cmake (`apt install cmake`, `brew install cmake`) |
| `curl/curl.h: No such file or directory` | Install the curl headers (`libcurl4-openssl-dev`); see [Building & Packaging](build.md#why-cmake-openssl-and-curl) |
| openssl-sys cannot find OpenSSL | Install `libssl-dev` and `pkg-config` (Linux) or `brew install openssl@3` (macOS) |
| Startup fails with `load config failed` or `config validation failed` | Configuration is strict; the error names the key or variable |
| Startup fails with `bind metrics port 9090` | The port is taken (often by another collector); set `SUNSPEC_METRICS_PORT` to a free port (0 picks one) |
| `failed to install metrics recorder` | The recorder can be installed once per process; keep such tests in their own test file |
| SQLite `database is locked` | Use one collector per buffer file |
| Modbus timeouts | Check `nc -zv <ip> 502`; raise `SUNSPEC_REQUEST_TIMEOUT_MS` (`SUNSPEC_MODBUS_TIMEOUT_MS` is deprecated and ignored) |
| `no SunSpec register map at base addresses` | The device does not expose `SunS` at the configured base or at 40000, 0 or 50000; check the unit ID and `SUNSPEC_BASE_ADDRESS` |
