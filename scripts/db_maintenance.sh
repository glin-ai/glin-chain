#!/bin/bash

# GLIN Database Maintenance Script
# Purpose: Compact and optimize RocksDB to reduce disk usage
# Usage: ./db_maintenance.sh <validator_number|rpc>

set -e

NODE_TYPE=${1:-validator-1}
BASE_DIR=${BASE_DIR:-/data}
CHAIN_SPEC=${CHAIN_SPEC:-glin_testnet}

# Determine node directory
if [[ "$NODE_TYPE" == "rpc" ]]; then
    NODE_DIR="$BASE_DIR/rpc"
elif [[ "$NODE_TYPE" =~ ^validator-([0-9]+)$ ]]; then
    NODE_DIR="$BASE_DIR/validator-${BASH_REMATCH[1]}"
else
    VALIDATOR_NUM=$NODE_TYPE
    NODE_DIR="$BASE_DIR/validator-$VALIDATOR_NUM"
fi

echo "====================================="
echo "GLIN Database Maintenance"
echo "====================================="
echo "Node: $NODE_TYPE"
echo "Directory: $NODE_DIR"
echo "Chain: $CHAIN_SPEC"
echo ""

if [ ! -d "$NODE_DIR" ]; then
    echo "Error: Node directory does not exist: $NODE_DIR"
    exit 1
fi

DB_PATH="$NODE_DIR/chains/$CHAIN_SPEC/db/full"

if [ ! -d "$DB_PATH" ]; then
    echo "Error: Database path does not exist: $DB_PATH"
    exit 1
fi

echo "Checking database size before maintenance..."
BEFORE_SIZE=$(du -sh "$DB_PATH" | cut -f1)
echo "Current size: $BEFORE_SIZE"
echo ""

# Check if node is running
if pgrep -f "glin-node.*--base-path $NODE_DIR" > /dev/null; then
    echo "WARNING: Node appears to be running!"
    echo "Please stop the node before running maintenance."
    echo "Continue anyway? (y/N)"
    read -r response
    if [[ ! "$response" =~ ^[Yy]$ ]]; then
        echo "Maintenance cancelled."
        exit 0
    fi
fi

echo "Starting database compaction..."
echo "This may take several minutes..."
echo ""

# Method 1: Use Substrate's built-in purge with keep-blocks
# This removes old blocks while keeping recent state
if command -v ./target/release/glin-node &> /dev/null; then
    echo "Running Substrate purge-chain with block retention..."
    ./target/release/glin-node purge-chain \
        --base-path "$NODE_DIR" \
        --chain "$CHAIN_SPEC" \
        --keep-blocks 256 \
        -y || echo "Warning: purge-chain failed, continuing..."
fi

echo ""
echo "Database maintenance completed!"
echo ""

AFTER_SIZE=$(du -sh "$DB_PATH" | cut -f1)
echo "Size before: $BEFORE_SIZE"
echo "Size after:  $AFTER_SIZE"
echo ""
echo "Note: For best results, perform maintenance while node is stopped."
echo "Restart your node with the updated pruning settings for optimal performance."