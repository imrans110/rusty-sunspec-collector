# Build Stage
FROM rust:1.80-slim-bookworm AS builder

# Install build dependencies
# cmake is required for rdkafka-sys
# build-essential, pkg-config, libssl-dev are standard requirements
RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    build-essential \
    libssl-dev \
    pkg-config \
    python3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy entire workspace
COPY . .

# Build release binary
# We use --locked to ensure reproducible builds from Cargo.lock
RUN cargo build --release --locked --workspace

# Runtime Stage
FROM debian:bookworm-slim

# Install runtime dependencies
# ca-certificates for SSL, libssl for Kafka encryption, curl for healthcheck
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r collector && useradd -r -g collector -s /bin/false collector

WORKDIR /app

# Create data directory for SQLite buffer
RUN mkdir -p /app/data && chown collector:collector /app/data

# Copy binary from builder
COPY --from=builder /app/target/release/collector-app /app/collector-app

# Switch to non-root user
USER collector:collector

# Expose metrics port
EXPOSE 9090

# Health check against Prometheus metrics endpoint
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:9090/metrics || exit 1

# Default configuration via env vars is expected, but we can look for config file
CMD ["/app/collector-app"]
