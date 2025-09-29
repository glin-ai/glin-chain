# Build stage
FROM rust:1.75 as builder

# Install dependencies
RUN apt-get update && \
    apt-get install -y \
    clang \
    libssl-dev \
    llvm \
    libudev-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /glin-chain

# Copy source files
COPY Cargo.toml Cargo.lock ./
COPY pallets ./pallets
COPY runtime ./runtime
COPY node ./node

# Build release binary
RUN cargo build --release

# Runtime stage
FROM ubuntu:22.04

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create user
RUN useradd -m -u 1000 -U -s /bin/sh -d /glin glin

# Copy binary from builder
COPY --from=builder /glin-chain/target/release/glin-node /usr/local/bin/glin-node

# Set ownership
RUN chown -R glin:glin /usr/local/bin/glin-node

USER glin

# Expose ports
# P2P
EXPOSE 30333
# RPC
EXPOSE 9933
# WebSocket
EXPOSE 9944
# Prometheus
EXPOSE 9615

# Volume for chain data
VOLUME ["/glin/.local/share/glin-node"]

# Default command
ENTRYPOINT ["glin-node"]
CMD ["--dev", "--ws-external", "--rpc-external", "--prometheus-external"]