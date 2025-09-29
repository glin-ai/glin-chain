#!/bin/bash

# Build script for GLIN Substrate node

set -e

echo "🔨 Building GLIN Substrate Node..."
echo "================================"

# Build in release mode
cargo build --release

echo ""
echo "✅ Build complete!"
echo ""
echo "Binary location: ./target/release/glin-node"
echo ""
echo "To run the node:"
echo "  Development mode:  ./run-dev.sh"
echo "  Custom chain:      ./target/release/glin-node --chain local"
echo ""