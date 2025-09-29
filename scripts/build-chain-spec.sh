#!/bin/bash
# Build chain spec without default bootnodes

echo "Building GLIN testnet chain spec..."
./target/release/glin-node build-spec --chain glin_testnet --disable-default-bootnode > chain-specs/glin-testnet.json

echo "Building raw chain spec..."
./target/release/glin-node build-spec --chain chain-specs/glin-testnet.json --raw --disable-default-bootnode > chain-specs/glin-testnet-raw.json

echo "Chain specs built successfully!"
