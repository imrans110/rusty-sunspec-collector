# Quick start

## Docker Compose (no hardware needed)

```sh
docker compose up --build
```

This starts Kafka, a simulator with two units, the collector and Prometheus. Then:

```sh
curl localhost:9090/healthz          # 200 with a JSON body while the pipeline is running
curl -s localhost:9090/metrics | grep '^sunspec_'
```

Prometheus is on <http://localhost:9091>; Kafka is reachable from the host at `localhost:29092`
(topic `sunspec.telemetry`). `docker compose down -v` stops the stack and removes its volumes.

## Local build

Prerequisites: Rust 1.88+, cmake and a C toolchain, pkg-config, and the OpenSSL and libcurl
development headers (`libssl-dev` and `libcurl4-openssl-dev` on Debian and Ubuntu). librdkafka is
built from source; SQLite and zstd are bundled.

```sh
cargo build --workspace
cargo test --workspace

# Terminal 1: simulated devices
cargo run -p sunspec-sim -- --bind 127.0.0.1:1502 --units 1,2

# Terminal 2: the collector (no Kafka configured, so each record's key and size is logged
# at debug level instead of being published)
SUNSPEC_STATIC_DEVICES=127.0.0.1:1,127.0.0.1:2 SUNSPEC_PORT=1502 \
  RUST_LOG=info,avro_kafka=debug cargo run -p collector-app
```

Set `SUNSPEC_KAFKA_BROKERS` to publish to a broker, for example `localhost:29092` while the
Compose stack is running.
