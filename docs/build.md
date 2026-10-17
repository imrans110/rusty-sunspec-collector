# Building and Packaging

How to build the collector natively, as Docker images and for 64-bit ARM, and how to run the
resulting binary. For the development workflow (tests, CI, the simulator) see the
[Developer Guide](DEVELOPER_GUIDE.md); for deploying with systemd see [Operations](ops.md).

---

## What gets built

The workspace produces two binaries:

- `collector-app`, the collector (crate `crates/collector-app`);
- `sunspec-sim`, the SunSpec Modbus TCP simulator (crate `crates/sunspec-sim`), for local
  development and tests.

| Build | Command | Output |
|-------|---------|--------|
| Debug | `cargo build --workspace` | `target/debug/collector-app`, `target/debug/sunspec-sim` |
| Release | `cargo build --release --locked -p collector-app -p sunspec-sim` | `target/release/collector-app`, `target/release/sunspec-sim` |
| aarch64 release | `cross build --release --locked --target aarch64-unknown-linux-gnu -p collector-app --features vendored-openssl` | `target/aarch64-unknown-linux-gnu/release/collector-app` |
| Collector image | `docker build -t sunspec-collector .` | `/app/collector-app` in the image |
| Simulator image | `docker build --target simulator -t sunspec-simulator .` | `/app/sunspec-sim` in the image |

`--locked` builds with the committed `Cargo.lock`, as CI and the Dockerfile do. The workspace uses
Cargo's default `release` profile.

---

## Toolchain and MSRV

The minimum supported Rust version is **1.88**, set as `rust-version` in the workspace
`Cargo.toml`. The Docker builder stage uses the same version (`rust:1.88-slim-bookworm`), so the
Docker build checks that the MSRV still works; the other CI jobs use the latest stable Rust. When
the MSRV changes, change both places.

---

## Prerequisites

### Debian / Ubuntu

```sh
sudo apt-get update
sudo apt-get install -y build-essential cmake pkg-config libssl-dev libcurl4-openssl-dev
```

These are the packages CI installs (`build-essential` is preinstalled on the CI runners).

### macOS

```sh
xcode-select --install       # compiler, linker and SDK (the SDK provides the curl headers)
brew install cmake openssl@3
```

openssl-sys finds Homebrew's OpenSSL without further configuration. Native macOS binaries link
`libssl` and `libcrypto` from Homebrew dynamically, so `openssl@3` must stay installed on the
machine that runs them.

---

## Why cmake, OpenSSL and curl

The Kafka client is rdkafka 0.39 with the features `cmake-build`, `ssl`, `tokio` and `zstd`:

- **cmake and a C/C++ toolchain.** `cmake-build` compiles librdkafka 2.12 (bundled in
  `rdkafka-sys 4.10.0+2.12.1`) from source with CMake. No system librdkafka is used, so the
  version is the same on every platform.
- **OpenSSL.** `ssl` builds librdkafka with TLS support against the system OpenSSL, which needs
  the headers at build time (`libssl-dev`) and the library at run time (`libssl3`), unless the
  `vendored-openssl` feature is used (see below).
- **curl headers.** librdkafka is built with curl disabled and the binary does not link libcurl.
  However, librdkafka 2.12's `rdkafka_conf.c` includes `<curl/curl.h>` under
  `#ifdef WITH_OAUTHBEARER_OIDC`, and the generated `config.h` always defines that macro (as 0 or
  1), so the header has to be present for the build to compile.
- **zstd and SQLite** need nothing from the system: `zstd-sys` compiles its bundled zstd, and
  sqlx's `sqlite` feature compiles a bundled SQLite.

The first build compiles librdkafka and therefore takes noticeably longer than later builds.

### Statically linked OpenSSL

`collector-app` has an opt-in feature `vendored-openssl` (forwarded to `avro-kafka` and rdkafka's
`ssl-vendored`). With it, OpenSSL is compiled from source and linked statically, so the binary
does not depend on the OpenSSL of the machine it runs on. Building OpenSSL needs `perl` and
`make`.

```sh
cargo build --release --locked -p collector-app --features vendored-openssl
```

The aarch64 cross build uses it; native and Docker builds do not.

---

## Native builds

```sh
cargo build --workspace                                       # debug, all crates
cargo build --release --locked -p collector-app               # release collector
cargo build --release --locked -p collector-app -p sunspec-sim
```

A native Linux release build links OpenSSL 3 and glibc dynamically; check the full list for your
system with `ldd target/release/collector-app`. To install it, copy the binary, for example to
`/usr/local/bin/collector-app` for the systemd unit described in [Operations](ops.md).

---

## Docker images

The `Dockerfile` has four stages:

| Stage | Base | Contents |
|-------|------|----------|
| `builder` | `rust:1.88-slim-bookworm` | Installs cmake, build-essential, libssl-dev, libcurl4-openssl-dev, pkg-config and python3, then runs `cargo build --release --locked -p collector-app -p sunspec-sim` and copies both binaries to `/out` |
| `runtime` | `debian:bookworm-slim` | Installs ca-certificates, libssl3 and curl (for the health check) and creates the system user and group `collector` |
| `simulator` | `runtime` | `/app/sunspec-sim`, runs as `collector`, exposes 1502, `ENTRYPOINT ["/app/sunspec-sim"]` with default arguments `--bind 0.0.0.0:1502 --units 1` |
| `collector` | `runtime` | `/app/collector-app`, runs as `collector` with working directory `/app/data` (owned by `collector`), exposes 9090, a `HEALTHCHECK` on `/healthz`; the default target |

```sh
docker build -t sunspec-collector .                         # collector (default target)
docker build --target simulator -t sunspec-simulator .      # simulator
```

### Build cache

The Dockerfile starts with `# syntax=docker/dockerfile:1` and uses BuildKit cache mounts for the
cargo registry (`/usr/local/cargo/registry`) and the target directory (`/app/target`), so
rebuilds after a source change reuse the compiled dependencies, librdkafka included. Cache mounts
need BuildKit, which is the default builder in current Docker Engine and Docker Desktop releases.
Because the target directory is a cache mount and not part of the image, the binaries are copied
to `/out` in the same step. Both images share the `builder` stage, which builds both binaries in
one cargo invocation.

`.dockerignore` keeps `target/`, `.git/`, `.github/`, `docs/`, `docker/` and SQLite files out of
the build context.

### Running the collector image

```sh
docker run -d --name sunspec-collector \
  -p 9090:9090 \
  -v sunspec-data:/app/data \
  -e SUNSPEC_STATIC_DEVICES=192.168.1.20:1 \
  -e SUNSPEC_KAFKA_BROKERS=kafka.example.internal:9092 \
  sunspec-collector
```

- **Buffer.** The image does not set `SUNSPEC_BUFFER_PATH`. The collector runs in `/app/data`, so
  the default relative `buffer.path` (`sunspec-buffer.sqlite`) is
  `/app/data/sunspec-buffer.sqlite`, and a `buffer.path` from a config file or
  `SUNSPEC_BUFFER_PATH` is used as given (relative paths resolve against `/app/data`). Keep
  `/app/data` on a volume so buffered samples survive container replacement. A new named volume
  takes its ownership from the image, so it is writable by `collector`; a bind-mounted host
  directory must be made writable for that user yourself.
- **Upgrading from older images.** Images that set `SUNSPEC_BUFFER_PATH=/app/data/buffer.sqlite`
  stored the buffer under that name. To keep sending rows already buffered in an existing volume,
  set `SUNSPEC_BUFFER_PATH=/app/data/buffer.sqlite` (or move the file and its `-wal` and `-shm`
  files to `sunspec-buffer.sqlite` while the collector is stopped).
- **Config file.** Mount it and set `SUNSPEC_CONFIG`, or override the command, which is
  `/app/collector-app` (a `CMD`, not an `ENTRYPOINT`):

  ```sh
  docker run -d -v "$PWD/collector.toml:/app/collector.toml:ro" -v sunspec-data:/app/data \
    -p 9090:9090 sunspec-collector /app/collector-app --config /app/collector.toml
  ```

- **Environment files.** With `--env-file`, put comments on their own lines: Docker does not strip
  trailing `# ...` from values, and the collector rejects values it cannot parse.
- **Health check.** `curl -fsS "http://localhost:${SUNSPEC_METRICS_PORT:-9090}/healthz"` every
  30 s (timeout 3 s, start period 20 s, 3 retries), so it follows `SUNSPEC_METRICS_PORT`. If you set
  the port only in a config file (`[metrics] port`), also set `SUNSPEC_METRICS_PORT` or override the
  check with `--health-cmd`.
- **Stopping.** `docker stop` sends SIGTERM, which starts the graceful shutdown (pollers stop, queued
  samples are written, the uplink drains for up to `buffer.shutdown_drain_timeout_ms`). Docker sends
  SIGKILL after 10 s by default; `docker stop -t 30` matches the 30 s the systemd unit allows.
- **Schema.** `docker run --rm sunspec-collector /app/collector-app --print-schema` prints the Avro
  schema of the published records.

### Running the simulator image

```sh
docker run --rm -p 1502:1502 sunspec-simulator --units 1,2
```

Arguments replace the default `--bind 0.0.0.0:1502 --units 1`; the bind address defaults to
`0.0.0.0:1502` anyway. See the [Developer Guide](DEVELOPER_GUIDE.md#the-simulator) for all flags.

---

## Docker Compose stack

`docker-compose.yml` runs a complete local pipeline without hardware:

| Service | Image | Notes |
|---------|-------|-------|
| `kafka` | `apache/kafka:3.9.1` | Single node in KRaft mode (broker and controller). Listeners: `kafka:9092` inside the compose network, `localhost:29092` from the host. Topics are created automatically with 3 partitions; data in the `kafka-data` volume. |
| `simulator` | `sunspec-simulator:local` (built from the `simulator` target) | Units 1 and 2 on port 1502 |
| `collector` | `sunspec-collector:local` (built from the `collector` target) | Polls `simulator:1` and `simulator:2`, publishes to `kafka:9092`, topic `sunspec.telemetry`; port 9090 on the host; buffer in the `collector-data` volume; starts once Kafka is healthy; `restart: unless-stopped` |
| `prometheus` | `prom/prometheus:v3.5.0` | `localhost:9091` on the host; scrapes `collector:9090` every 15 s (`docker/prometheus.yml`) |

```sh
docker compose up --build            # build the images and start everything
curl localhost:9090/healthz          # 200 with a JSON body once the pipeline runs
curl -s localhost:9090/metrics | grep '^sunspec_uplink_records_total'

# Record keys on the topic (the values are binary Avro)
docker compose exec kafka /opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 --topic sunspec.telemetry --from-beginning \
  --max-messages 10 --property print.key=true --property print.value=false

docker compose down -v               # stop and remove the containers and volumes
```

`docker compose down` without `-v` keeps the volumes, and with them the collector's buffer and the
Kafka data. `docker compose stop` gives containers 10 s before killing them; use
`docker compose stop --timeout 30 collector` for a full graceful stop, as CI does.

---

## Cross-compiling for aarch64

64-bit ARM gateways (`aarch64-unknown-linux-gnu`) are built with
[`cross`](https://github.com/cross-rs/cross) and a custom image:

```sh
cargo install cross --locked
docker build -t sunspec-cross-arm64 -f docker/Dockerfile.arm64 .
cross build --release --locked --target aarch64-unknown-linux-gnu -p collector-app \
  --features vendored-openssl
file target/aarch64-unknown-linux-gnu/release/collector-app    # "ARM aarch64"
```

- `Cross.toml` maps the target to the image: `[target.aarch64-unknown-linux-gnu]`,
  `image = "sunspec-cross-arm64"`. The image must be built locally under that tag first.
- `docker/Dockerfile.arm64` starts from `ghcr.io/cross-rs/aarch64-unknown-linux-gnu:latest`, adds
  the `arm64` architecture, installs `cmake`, `make`, `perl`, `pkg-config` and
  `libcurl4-openssl-dev:arm64` (the curl headers librdkafka needs at compile time), and points
  `PKG_CONFIG_LIBDIR` and `PKG_CONFIG_PATH` at `/usr/lib/aarch64-linux-gnu/pkgconfig`.
- `--features vendored-openssl` is required: the image's arm64 OpenSSL is older than the version
  librdkafka 2.12 needs, and linking against it fails. With the feature, OpenSSL is compiled from
  source (hence `make` and `perl`) and linked statically, which also makes the binary independent
  of the target system's OpenSSL.
- The binary is still linked dynamically against the target's C library; check it with `ldd` on
  the device.

### CI workflow

`.github/workflows/cross-arm64.yml` runs the same steps on pushes to `main`, on pull requests that
change `docker/Dockerfile.arm64`, `Cross.toml` or the workflow, and on manual dispatch. It checks
with `file` that the result is an aarch64 binary and uploads it as the artifact
`collector-app-aarch64-unknown-linux-gnu`.

---

## Running the binary

```sh
collector-app [--config PATH] [--print-schema]
```

| Argument | Effect |
|----------|--------|
| `--config PATH` (or `--config=PATH`) | Load this TOML or JSON file (`.json` is parsed as JSON, anything else as TOML). `SUNSPEC_CONFIG` does the same. |
| `--print-schema` | Print the Avro schema of the published records to stdout and exit, without reading any configuration |

Other arguments are ignored. Without a config file the collector starts from the compiled defaults
and `SUNSPEC_*` environment variables, which override file values when both are used:

```sh
SUNSPEC_STATIC_DEVICES=192.168.1.20:1,192.168.1.21:1 \
SUNSPEC_KAFKA_BROKERS=kafka.example.internal:9092 \
  ./collector-app
```

Without `SUNSPEC_KAFKA_BROKERS` (or `kafka.brokers`) nothing is sent to Kafka: records are only
logged at debug level. Every setting is described in [Configuration](configuration.md).

- **Logging:** `RUST_LOG` sets the filter (default `info`, for example `info,poller_actor=debug`);
  `SUNSPEC_LOG_FORMAT=json` writes one JSON object per line (`text` is the default; any other value
  fails at startup). Colours are used only when stdout is a terminal.
- **Files:** the buffer is `sunspec-buffer.sqlite` in the working directory by default, with
  SQLite's `-wal` and `-shm` files next to it. In the Docker image the working directory is
  `/app/data`, so the default is `/app/data/sunspec-buffer.sqlite`; the example systemd
  environment file sets `/var/lib/sunspec-collector/buffer.sqlite`.
- **Ports:** `/metrics` and `/healthz` on `metrics.port` (default 9090). If the port cannot be
  bound, the collector exits at startup with an error containing `bind metrics port <port>`.
- **Signals:** SIGINT and SIGTERM start a graceful shutdown.
- **Exit status:** 0 after a graceful shutdown. Startup errors (configuration that does not load or
  validate, a metrics port that cannot be bound, a buffer that cannot be opened, a Kafka producer
  that cannot be created) are printed to stderr and the process exits with a non-zero status.
