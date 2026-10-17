# syntax=docker/dockerfile:1

# Build stage: compiles the collector and the device simulator.
FROM rust:1.88-slim-bookworm AS builder

# cmake builds librdkafka; librdkafka 2.12 also needs the curl headers to compile.
RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    build-essential \
    libssl-dev \
    libcurl4-openssl-dev \
    pkg-config \
    python3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Cache mounts keep the registry and build artifacts between local builds; the
# binaries are copied out because the target directory is not part of the image.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked -p collector-app -p sunspec-sim \
    && mkdir -p /out \
    && cp target/release/collector-app target/release/sunspec-sim /out/

# Shared runtime base.
FROM debian:bookworm-slim AS runtime

# ca-certificates and libssl3 for Kafka TLS, curl for the health check.
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd -r collector \
    && useradd -r -g collector -s /bin/false collector

WORKDIR /app

# Simulated SunSpec inverters for local development and end-to-end tests.
FROM runtime AS simulator
COPY --from=builder /out/sunspec-sim /app/sunspec-sim
USER collector:collector
EXPOSE 1502
ENTRYPOINT ["/app/sunspec-sim"]
CMD ["--bind", "0.0.0.0:1502", "--units", "1"]

# The collector (default target). It runs in /app/data (a volume in compose) so the
# default relative buffer path lands there; buffer.path in a config file still applies.
FROM runtime AS collector
RUN mkdir -p /app/data && chown collector:collector /app/data
COPY --from=builder /out/collector-app /app/collector-app
USER collector:collector
WORKDIR /app/data
EXPOSE 9090
HEALTHCHECK --interval=30s --timeout=3s --start-period=20s --retries=3 \
    CMD curl -fsS "http://localhost:${SUNSPEC_METRICS_PORT:-9090}/healthz" || exit 1
CMD ["/app/collector-app"]
