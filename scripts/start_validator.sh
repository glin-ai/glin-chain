#!/bin/bash

# GLIN Testnet Validator Startup Script
# Usage: ./start_validator.sh <validator_number>

set -e

VALIDATOR_NUM=${1:-1}
BASE_DIR=${BASE_DIR:-/data}
CHAIN_SPEC=${CHAIN_SPEC:-glin_testnet}

echo "Starting GLIN Validator $VALIDATOR_NUM"

# Load validator keys from environment
AURA_SEED_VAR="VALIDATOR_${VALIDATOR_NUM}_AURA_SEED"
GRANDPA_SEED_VAR="VALIDATOR_${VALIDATOR_NUM}_GRANDPA_SEED"

AURA_SEED=${!AURA_SEED_VAR}
GRANDPA_SEED=${!GRANDPA_SEED_VAR}

if [ -z "$AURA_SEED" ] || [ -z "$GRANDPA_SEED" ]; then
    echo "Error: Validator keys not found in environment"
    echo "Please set $AURA_SEED_VAR and $GRANDPA_SEED_VAR"
    exit 1
fi

# Port configuration
P2P_PORT=$((30333 + $VALIDATOR_NUM - 1))
RPC_PORT=$((9944 + ($VALIDATOR_NUM - 1) * 2))
WS_PORT=$((9945 + ($VALIDATOR_NUM - 1) * 2))

# Node name
NODE_NAME="GLIN-Validator-$VALIDATOR_NUM"

# Base directory for this validator
VALIDATOR_DIR="$BASE_DIR/validator-$VALIDATOR_NUM"
mkdir -p "$VALIDATOR_DIR"

# Insert keys into keystore
echo "Inserting Aura key..."
./target/release/glin-node key insert \
    --base-path "$VALIDATOR_DIR" \
    --chain "$CHAIN_SPEC" \
    --scheme Sr25519 \
    --suri "$AURA_SEED" \
    --key-type aura

echo "Inserting GRANDPA key..."
./target/release/glin-node key insert \
    --base-path "$VALIDATOR_DIR" \
    --chain "$CHAIN_SPEC" \
    --scheme Ed25519 \
    --suri "$GRANDPA_SEED" \
    --key-type gran

# Bootnodes configuration
BOOTNODES=""
if [ "$VALIDATOR_NUM" != "1" ]; then
    # Connect to validator 1 as bootnode
    BOOTNODE_ID=${VALIDATOR_1_PEER_ID:-""}
    if [ ! -z "$BOOTNODE_ID" ]; then
        BOOTNODES="--bootnodes /dns/validator-1.railway.internal/tcp/30333/p2p/$BOOTNODE_ID"
    fi
fi

# Start the validator node
exec ./target/release/glin-node \
    --base-path "$VALIDATOR_DIR" \
    --chain "$CHAIN_SPEC" \
    --name "$NODE_NAME" \
    --validator \
    --port $P2P_PORT \
    --rpc-port $RPC_PORT \
    --ws-port $WS_PORT \
    --rpc-cors all \
    --rpc-methods unsafe \
    --telemetry-url "${TELEMETRY_URL:-wss://telemetry.polkadot.io/submit/} 0" \
    --prometheus-external \
    --prometheus-port $((9615 + $VALIDATOR_NUM)) \
    $BOOTNODES \
    --execution wasm \
    --wasm-execution compiled \
    --state-pruning ${PRUNING:-1000} \
    --blocks-pruning ${BLOCKS_PRUNING:-1000}