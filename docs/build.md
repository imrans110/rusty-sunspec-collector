# Build, test, and run

## Build

- Build the full workspace:

```sh
cargo build --workspace
```

- Release build (local):

```sh
cargo build --workspace --release
```

## Run

- Run the collector with defaults:

```sh
cargo run -p collector-app
```

- Run with a config file:

```sh
SUNSPEC_CONFIG=docs/config.example.toml cargo run -p collector-app
```

- Run with environment overrides:

```sh
SUNSPEC_STATIC_DEVICES=192.168.1.20:1 SUNSPEC_KAFKA_BROKERS=localhost:9092 cargo run -p collector-app
```

## Tests

- All tests:

```sh
cargo test --workspace
```

- SunSpec parser tests:

```sh
cargo test -p sunspec-parser
```

- Buffer tests:

```sh
cargo test -p buffer
```

- Optional Modbus integration test (requires a running simulator):

```sh
MODBUS_TEST_HOST=127.0.0.1 MODBUS_TEST_PORT=1502 cargo test -p modbus-client --test diagslave_tests
```

- Optional synthetic end-to-end harness (no simulator required):

```sh
cargo test -p collector-app --test e2e_harness_tests
```

- Optional Kafka integration test (requires a running broker):

```sh
SUNSPEC_KAFKA_BROKERS=localhost:9092 cargo test -p avro-kafka --test kafka_integration_tests
```

- Config validation tests (TOML/JSON fixtures):

```sh
cargo test -p collector-app --test config_validation_tests
```

## Cross-compilation (ARM64)

- Build the custom cross image:

```sh
docker build -t sunspec-cross-arm64 -f docker/Dockerfile.arm64 .
```

- Build with cross:

```sh
cross build --release --target aarch64-unknown-linux-gnu
```

- Output binary:

```text
target/aarch64-unknown-linux-gnu/release/collector-app
```
