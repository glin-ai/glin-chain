# Build stage with cargo-chef for dependency caching
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Plan stage - prepare dependency list
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build stage - cache dependencies then build binary
FROM chef AS builder

# Install build dependencies for Substrate
RUN apt-get update && apt-get install -y \
    clang \
    libssl-dev \
    llvm \
    libudev-dev \
    libclang-dev \
    protobuf-compiler \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Install wasm32-unknown-unknown target and rust-src for Substrate runtime compilation
RUN rustup target add wasm32-unknown-unknown && \
    rustup component add rust-src

# Copy and build dependencies (cached if unchanged)
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source and build the actual binary
COPY . .
RUN cargo build --release --bin glin-node

# Runtime stage - minimal image for running node
FROM ubuntu:22.04

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/glin-node /usr/local/bin/glin-node

# Copy chain specification
COPY chain-specs/glin-testnet-raw.json /chain-specs/glin-testnet-raw.json

# Create non-root user for security
RUN useradd -m -u 1000 -U -s /bin/sh -d /glin glin && \
    mkdir -p /data /glin/.local/share && \
    chown -R glin:glin /glin/.local/share /usr/local/bin/glin-node && \
    chmod -R 755 /data

# Don't switch to glin user - run as root for Railway volumes
# Railway volumes require root access initially
USER root

# Expose P2P, RPC, WebSocket, and Prometheus ports
EXPOSE 30333 9933 9944 9615

# Health check for monitoring
HEALTHCHECK --interval=30s --timeout=3s --start-period=30s --retries=3 \
    CMD ["/usr/local/bin/glin-node", "--version"]

ENTRYPOINT ["/usr/local/bin/glin-node"]
# Default command for testnet - Railway will override this
CMD ["--chain", "/chain-specs/glin-testnet-raw.json", "--rpc-external", "--rpc-cors", "all"]