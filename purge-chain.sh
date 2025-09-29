#!/bin/bash

# Purge chain data

echo "⚠️  WARNING: This will delete all blockchain data!"
echo ""
read -p "Are you sure you want to purge the chain? (y/N): " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🗑️  Purging chain data..."
    ./target/release/glin-node purge-chain --dev -y
    echo "✅ Chain purged successfully!"
else
    echo "❌ Purge cancelled"
fi