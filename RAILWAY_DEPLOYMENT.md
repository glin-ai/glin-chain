# Railway Deployment Guide for GLIN Testnet

## Overview
This guide explains how to deploy the GLIN incentivized testnet on Railway with 3 validators.

## Prerequisites
- Railway account with 3 services deployed
- Generated validator keys in `validator-keys/` directory
- Built Docker image with chain spec

## Deployment Steps

### 1. Configure Validator 1 (Bootnode)
**Service Name:** glin-validator-1

**Environment Variables:**
```
# From validator-keys/railway_env_validator_1.txt
VALIDATOR_1_AURA_SEED=<copy from file>
VALIDATOR_1_GRANDPA_SEED=<copy from file>
```

**Start Command:**
```
--chain /chain-specs/glin-testnet-raw.json \
--validator \
--name validator-1 \
--base-path /data \
--port 30333 \
--rpc-port 9933 \
--ws-port 9944 \
--rpc-external \
--ws-external \
--rpc-cors all \
--rpc-methods unsafe \
--public-addr /dns4/<YOUR-RAILWAY-TCP-PROXY-URL>/tcp/<PORT> \
--bootnodes ""
```

**TCP Proxy:** Enable on port 30333 for P2P

### 2. Get Validator 1 Peer ID
After Validator 1 starts, check logs for:
```
Local node identity is: 12D3KooW...
```
Copy this peer ID for other validators.

### 3. Configure Validator 2
**Service Name:** glin-validator-2

**Environment Variables:**
```
# From validator-keys/railway_env_validator_2.txt
VALIDATOR_2_AURA_SEED=<copy from file>
VALIDATOR_2_GRANDPA_SEED=<copy from file>
```

**Start Command:**
```
--chain /chain-specs/glin-testnet-raw.json \
--validator \
--name validator-2 \
--base-path /data \
--port 30333 \
--rpc-port 9933 \
--ws-port 9944 \
--bootnodes /dns4/glin-chain.railway.internal/tcp/30333/p2p/<VALIDATOR_1_PEER_ID>
```

### 4. Configure Validator 3
**Service Name:** glin-validator-3

**Environment Variables:**
```
# From validator-keys/railway_env_validator_3.txt
VALIDATOR_3_AURA_SEED=<copy from file>
VALIDATOR_3_GRANDPA_SEED=<copy from file>
```

**Start Command:**
```
--chain /chain-specs/glin-testnet-raw.json \
--validator \
--name validator-3 \
--base-path /data \
--port 30333 \
--rpc-port 9933 \
--ws-port 9944 \
--bootnodes /dns4/glin-chain.railway.internal/tcp/30333/p2p/<VALIDATOR_1_PEER_ID>
```

### 5. Insert Session Keys
Once validators are running, insert their session keys:

For each validator, run from your local machine:
```bash
# Validator 1
./scripts/insert_session_keys.sh 1 https://<YOUR-RAILWAY-URL>

# Validator 2 (if exposed)
./scripts/insert_session_keys.sh 2 <validator-2-rpc-url>

# Validator 3 (if exposed)
./scripts/insert_session_keys.sh 3 <validator-3-rpc-url>
```

### 6. Verify Network

1. **Check Peer Connections:**
   Each validator should show 2 peers in logs:
   ```
   💤 Idle (2 peers)
   ```

2. **Check Block Production:**
   Look for block production messages:
   ```
   🎁 Prepared block for proposing at #1
   ```

3. **Connect with Polkadot.js:**
   - Go to https://polkadot.js.org/apps
   - Custom endpoint: `wss://<YOUR-RAILWAY-URL>`
   - Should see blocks being produced

## Troubleshooting

### Validators Not Connecting
- Verify bootnode address is correct
- Check Railway internal DNS is working
- Ensure TCP proxy is enabled on validator-1

### No Block Production
- Verify session keys are inserted
- Check that validator addresses match chain spec
- Ensure all 3 validators are running

### Session Key Issues
- Keys must be inserted after node starts
- Use `--rpc-methods unsafe` to allow key insertion
- Verify with `author_hasKey` RPC call

## Security Notes
- Never commit validator keys to git
- Store seeds securely in Railway secrets
- Rotate keys periodically
- Monitor validator performance