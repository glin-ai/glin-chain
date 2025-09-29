#!/bin/bash

# GLIN Testnet RPC Node Startup Script
# Public RPC endpoint for testnet users

set -e

BASE_DIR=${BASE_DIR:-/data}
CHAIN_SPEC=${CHAIN_SPEC:-glin_testnet}

echo "Starting GLIN RPC Node"

# Port configuration
P2P_PORT=${P2P_PORT:-30340}
RPC_PORT=${RPC_PORT:-9944}
WS_PORT=${WS_PORT:-9945}

# Node name
NODE_NAME=${NODE_NAME:-"GLIN-RPC"}

# Base directory for RPC node
RPC_DIR="$BASE_DIR/rpc"
mkdir -p "$RPC_DIR"

# Bootnodes configuration
BOOTNODES=""
BOOTNODE_ID=${VALIDATOR_1_PEER_ID:-""}
if [ ! -z "$BOOTNODE_ID" ]; then
    BOOTNODES="--bootnodes /dns/validator-1.railway.internal/tcp/30333/p2p/$BOOTNODE_ID"
fi

# Additional bootnodes if available
if [ ! -z "$VALIDATOR_2_PEER_ID" ]; then
    BOOTNODES="$BOOTNODES --bootnodes /dns/validator-2.railway.internal/tcp/30334/p2p/$VALIDATOR_2_PEER_ID"
fi

if [ ! -z "$VALIDATOR_3_PEER_ID" ]; then
    BOOTNODES="$BOOTNODES --bootnodes /dns/validator-3.railway.internal/tcp/30335/p2p/$VALIDATOR_3_PEER_ID"
fi

# Start the RPC node (not a validator)
exec ./target/release/glin-node \
    --base-path "$RPC_DIR" \
    --chain "$CHAIN_SPEC" \
    --name "$NODE_NAME" \
    --port $P2P_PORT \
    --rpc-port $RPC_PORT \
    --ws-port $WS_PORT \
    --rpc-external \
    --ws-external \
    --rpc-cors all \
    --rpc-methods safe \
    --ws-max-connections 1000 \
    --telemetry-url "${TELEMETRY_URL:-wss://telemetry.polkadot.io/submit/} 0" \
    --prometheus-external \
    --prometheus-port 9615 \
    $BOOTNODES \
    --execution wasm \
    --wasm-execution compiled \
    --state-pruning ${PRUNING:-1000} \
    --blocks-pruning ${BLOCKS_PRUNING:-1000} \
    --max-runtime-instances 256 \
    --runtime-cache-size 8