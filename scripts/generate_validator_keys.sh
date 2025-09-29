#!/bin/bash

# GLIN Incentivized Testnet - Secure Validator Key Generation Script
# This script generates secure validator keys for production testnet deployment

set -e

echo "==========================================="
echo "GLIN Incentivized Testnet Key Generator"
echo "==========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
CHAIN_SPEC="${1:-testnet}"
OUTPUT_DIR="./validator-keys"
NUM_VALIDATORS="${2:-3}"

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo -e "${YELLOW}Generating keys for $NUM_VALIDATORS validators...${NC}"
echo ""

# Function to generate a secure mnemonic
generate_secure_account() {
    local name=$1
    local index=$2

    echo -e "${GREEN}Generating keys for $name...${NC}"

    # Generate SR25519 key for Aura (block production)
    echo "Generating Aura key (SR25519)..."
    ./target/release/glin-node key generate \
        --scheme Sr25519 \
        --output-type json \
        > "$OUTPUT_DIR/${name}_aura.json" 2>/dev/null || ./target/debug/glin-node key generate \
        --scheme Sr25519 \
        --output-type json \
        > "$OUTPUT_DIR/${name}_aura.json" 2>/dev/null

    # Generate ED25519 key for GRANDPA (finality)
    echo "Generating GRANDPA key (ED25519)..."
    ./target/release/glin-node key generate \
        --scheme Ed25519 \
        --output-type json \
        > "$OUTPUT_DIR/${name}_grandpa.json" 2>/dev/null || ./target/debug/glin-node key generate \
        --scheme Ed25519 \
        --output-type json \
        > "$OUTPUT_DIR/${name}_grandpa.json" 2>/dev/null

    # Extract public keys and addresses
    AURA_SEED=$(cat "$OUTPUT_DIR/${name}_aura.json" | grep -o '"secretSeed":"[^"]*"' | cut -d'"' -f4)
    AURA_PUBLIC=$(cat "$OUTPUT_DIR/${name}_aura.json" | grep -o '"publicKey":"[^"]*"' | cut -d'"' -f4)
    AURA_ADDRESS=$(cat "$OUTPUT_DIR/${name}_aura.json" | grep -o '"ss58Address":"[^"]*"' | cut -d'"' -f4)

    GRANDPA_SEED=$(cat "$OUTPUT_DIR/${name}_grandpa.json" | grep -o '"secretSeed":"[^"]*"' | cut -d'"' -f4)
    GRANDPA_PUBLIC=$(cat "$OUTPUT_DIR/${name}_grandpa.json" | grep -o '"publicKey":"[^"]*"' | cut -d'"' -f4)
    GRANDPA_ADDRESS=$(cat "$OUTPUT_DIR/${name}_grandpa.json" | grep -o '"ss58Address":"[^"]*"' | cut -d'"' -f4)

    # Save to environment file for Railway
    cat >> "$OUTPUT_DIR/railway_env_${name}.txt" <<EOF
# $name Validator Keys
VALIDATOR_${index}_AURA_SEED="$AURA_SEED"
VALIDATOR_${index}_GRANDPA_SEED="$GRANDPA_SEED"
VALIDATOR_${index}_ADDRESS="$AURA_ADDRESS"
EOF

    # Save public keys for chain spec
    cat >> "$OUTPUT_DIR/validator_public_keys.txt" <<EOF
Validator $index ($name):
  Aura Public Key: $AURA_PUBLIC
  Aura Address: $AURA_ADDRESS
  GRANDPA Public Key: $GRANDPA_PUBLIC
  GRANDPA Address: $GRANDPA_ADDRESS

EOF

    echo -e "${GREEN}✓ Keys generated for $name${NC}"
    echo ""
}

# Generate special account keys
generate_special_accounts() {
    echo -e "${YELLOW}Generating special account keys...${NC}"
    echo ""

    # Faucet account
    echo "Generating Faucet account..."
    ./target/release/glin-node key generate \
        --scheme Sr25519 \
        --output-type json \
        > "$OUTPUT_DIR/faucet_account.json" 2>/dev/null || ./target/debug/glin-node key generate \
        --scheme Sr25519 \
        --output-type json \
        > "$OUTPUT_DIR/faucet_account.json" 2>/dev/null

    FAUCET_SEED=$(cat "$OUTPUT_DIR/faucet_account.json" | grep -o '"secretSeed":"[^"]*"' | cut -d'"' -f4)
    FAUCET_ADDRESS=$(cat "$OUTPUT_DIR/faucet_account.json" | grep -o '"ss58Address":"[^"]*"' | cut -d'"' -f4)

    # Treasury account
    echo "Generating Treasury account..."
    ./target/release/glin-node key generate \
        --scheme Sr25519 \
        --output-type json \
        > "$OUTPUT_DIR/treasury_account.json" 2>/dev/null || ./target/debug/glin-node key generate \
        --scheme Sr25519 \
        --output-type json \
        > "$OUTPUT_DIR/treasury_account.json" 2>/dev/null

    TREASURY_ADDRESS=$(cat "$OUTPUT_DIR/treasury_account.json" | grep -o '"ss58Address":"[^"]*"' | cut -d'"' -f4)

    # Team Ops account
    echo "Generating Team Ops account..."
    ./target/release/glin-node key generate \
        --scheme Sr25519 \
        --output-type json \
        > "$OUTPUT_DIR/team_ops_account.json" 2>/dev/null || ./target/debug/glin-node key generate \
        --scheme Sr25519 \
        --output-type json \
        > "$OUTPUT_DIR/team_ops_account.json" 2>/dev/null

    TEAM_OPS_ADDRESS=$(cat "$OUTPUT_DIR/team_ops_account.json" | grep -o '"ss58Address":"[^"]*"' | cut -d'"' -f4)

    # Ecosystem account
    echo "Generating Ecosystem account..."
    ./target/release/glin-node key generate \
        --scheme Sr25519 \
        --output-type json \
        > "$OUTPUT_DIR/ecosystem_account.json" 2>/dev/null || ./target/debug/glin-node key generate \
        --scheme Sr25519 \
        --output-type json \
        > "$OUTPUT_DIR/ecosystem_account.json" 2>/dev/null

    ECOSYSTEM_ADDRESS=$(cat "$OUTPUT_DIR/ecosystem_account.json" | grep -o '"ss58Address":"[^"]*"' | cut -d'"' -f4)

    # Save special accounts to file
    cat > "$OUTPUT_DIR/special_accounts.txt" <<EOF
Special Account Addresses:
==========================
Faucet: $FAUCET_ADDRESS
Treasury: $TREASURY_ADDRESS
Team Ops: $TEAM_OPS_ADDRESS
Ecosystem: $ECOSYSTEM_ADDRESS

IMPORTANT:
- Faucet seed is in railway_env_faucet.txt
- Other accounts should be controlled by multisig in production
EOF

    # Save faucet seed for Railway deployment
    cat > "$OUTPUT_DIR/railway_env_faucet.txt" <<EOF
# Faucet Service Configuration
FAUCET_SEED="$FAUCET_SEED"
FAUCET_ADDRESS="$FAUCET_ADDRESS"
EOF

    echo -e "${GREEN}✓ Special accounts generated${NC}"
    echo ""
}

# Generate validator keys
for i in $(seq 1 $NUM_VALIDATORS); do
    generate_secure_account "validator_$i" $i
done

# Generate special accounts
generate_special_accounts

# Create a summary file
cat > "$OUTPUT_DIR/DEPLOYMENT_SUMMARY.md" <<EOF
# GLIN Incentivized Testnet Deployment Summary

Generated on: $(date)

## Validators
- Number of validators: $NUM_VALIDATORS
- Keys stored in: $OUTPUT_DIR/

## Important Files
- \`validator_public_keys.txt\`: Public keys for chain spec
- \`railway_env_validator_*.txt\`: Environment variables for each validator (Railway deployment)
- \`railway_env_faucet.txt\`: Faucet service credentials
- \`special_accounts.txt\`: Addresses for special accounts

## Security Notes
1. **CRITICAL**: Store all seed phrases securely
2. Never commit these files to git
3. Use Railway secrets or environment variables for deployment
4. Back up all keys in a secure location
5. For production mainnet, use hardware security modules (HSM)

## Next Steps
1. Update chain spec with validator public keys
2. Configure Railway environment variables
3. Deploy validators with their respective keys
4. Configure faucet service with faucet seed
5. Set up monitoring and alerting

## Railway Deployment
Upload the contents of \`railway_env_*.txt\` files to Railway secrets:
- Each validator service gets its corresponding env file
- Faucet service gets the faucet env file
- Never expose these seeds publicly
EOF

echo ""
echo -e "${GREEN}==========================================="
echo -e "Key generation complete!"
echo -e "==========================================="
echo -e "${NC}"
echo -e "${YELLOW}Generated files in $OUTPUT_DIR/:${NC}"
echo "  - validator_*_aura.json: Aura keys for each validator"
echo "  - validator_*_grandpa.json: GRANDPA keys for each validator"
echo "  - railway_env_*.txt: Environment variables for Railway"
echo "  - special_accounts.txt: Special account addresses"
echo "  - DEPLOYMENT_SUMMARY.md: Complete deployment guide"
echo ""
echo -e "${RED}⚠️  SECURITY WARNING ⚠️${NC}"
echo "These keys control real value on the testnet!"
echo "1. Store them securely"
echo "2. Never commit to git"
echo "3. Use Railway secrets for deployment"
echo "4. Back up in multiple secure locations"
echo ""
echo -e "${GREEN}Ready for Railway deployment!${NC}"