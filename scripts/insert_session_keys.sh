#!/bin/bash

# Script to insert session keys into a running validator node
# Usage: ./insert_session_keys.sh <validator_number> <rpc_url>

set -e

VALIDATOR_NUM="${1:-1}"
RPC_URL="${2:-http://localhost:9933}"
KEY_DIR="./validator-keys"

echo "Inserting session keys for Validator $VALIDATOR_NUM at $RPC_URL"

# Read the keys from the generated files
AURA_SEED=$(jq -r .secretSeed "$KEY_DIR/validator_${VALIDATOR_NUM}_aura.json")
GRANDPA_SEED=$(jq -r .secretSeed "$KEY_DIR/validator_${VALIDATOR_NUM}_grandpa.json")

if [ -z "$AURA_SEED" ] || [ -z "$GRANDPA_SEED" ]; then
    echo "Error: Could not read keys for validator $VALIDATOR_NUM"
    exit 1
fi

echo "Inserting AURA key..."
curl -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"author_insertKey\",\"params\":[\"aura\",\"$AURA_SEED\",null],\"id\":1}" \
  $RPC_URL

echo ""
echo "Inserting GRANDPA key..."
curl -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"author_insertKey\",\"params\":[\"gran\",\"$GRANDPA_SEED\",null],\"id\":2}" \
  $RPC_URL

echo ""
echo "Keys inserted successfully for Validator $VALIDATOR_NUM"
echo ""
echo "Verifying keys are loaded..."
curl -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"author_hasKey","params":[],"id":3}' \
  $RPC_URL

echo ""