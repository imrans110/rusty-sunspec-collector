# Build Stage
FROM rust:1.80-slim-bullseye as builder

# Install build dependencies
# cmake is required for rdkafka-sys
# build-essential, pkg-config, libssl-dev are standard requirements
RUN apt-get update && apt-get install -y \
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
FROM debian:bullseye-slim

# Install runtime dependencies
# ca-certificates for SSL, libssl for Kafka encryption
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/collector-app /app/collector-app

# Expose metrics port
EXPOSE 9090

# Default configuration via env vars is expected, but we can look for config file
CMD ["/app/collector-app"]
