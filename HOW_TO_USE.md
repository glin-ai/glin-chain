# GLIN Blockchain Usage Guide

## 🚀 Quick Start

### 1. Build the Node
```bash
./build.sh
```

### 2. Run in Development Mode
```bash
./run-dev.sh
```

This starts a development blockchain with:
- Instant block production
- Pre-funded test accounts
- Single node (no network needed)

## 📊 Blockchain Features

### Block Production
- **Consensus**: Aura (Authority Round)
- **Block Time**: 6 seconds (configurable)
- **Finality**: GRANDPA (instant finality)

In development mode:
- Blocks are created instantly when transactions arrive
- No need to wait for the 6-second interval

### Pre-funded Accounts

| Account | Address | Balance | Seed |
|---------|---------|---------|------|
| **Alice** (sudo) | 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY | 100,000 GLIN | //Alice |
| **Bob** | 5FHneW46xGXgs3mDvNbLXCmqM7KogTFrDnXBpA9wkLSHTfU | 100,000 GLIN | //Bob |
| **Charlie** | 5FLSigC9HGRrYXOppRwrTDDEpRs4vAsFOD4BgkDgQMOD | 100,000 GLIN | //Charlie |
| **Dave** | 5DAAnrj7VHTznn2AWBemMuyBwZWs6FEdFrYNPns7TRhCHt | 100,000 GLIN | //Dave |
| **Eve** | 5HGjWAe5TkPN9dritt1NL6BhfWjWZyeFzpqV7VGtqvGNfFc | 100,000 GLIN | //Eve |

## 🔗 Connecting to Your Node

### 1. Polkadot.js Apps (Web UI)

Open in browser:
```
https://polkadot.js.org/apps/?rpc=ws://localhost:9944
```

Then:
1. Go to Settings → Developer
2. Paste this custom types:
```json
{
  "ModelType": {
    "_enum": ["ResNet", "Bert", "Gpt", "Custom", "LoraFineTune"]
  },
  "TaskStatus": {
    "_enum": ["Pending", "Recruiting", "Running", "Validating", "Completed", "Failed", "Cancelled"]
  },
  "HardwareRequirements": {
    "min_vram_gb": "u32",
    "min_compute_capability": "u32",
    "min_bandwidth_mbps": "u32"
  }
}
```

### 2. WebSocket Connection
```javascript
// Using @polkadot/api
const { ApiPromise, WsProvider } = require('@polkadot/api');

const provider = new WsProvider('ws://localhost:9944');
const api = await ApiPromise.create({ provider });

// Check chain info
const chain = await api.rpc.system.chain();
console.log('Connected to:', chain.toString());
```

### 3. RPC Endpoints
- **WebSocket**: ws://localhost:9944
- **HTTP JSON-RPC**: http://localhost:9933

## 💰 Using Wallets

### Create a New Account
```javascript
const { Keyring } = require('@polkadot/keyring');
const keyring = new Keyring({ type: 'sr25519' });

// Create from mnemonic
const mnemonic = 'entire material egg meadow latin bargain dutch coral blood melt acoustic thought';
const account = keyring.addFromUri(mnemonic);
console.log('Address:', account.address);
```

### Transfer GLIN Tokens
```javascript
// Transfer 10 GLIN from Alice to Bob
const transfer = api.tx.balances
  .transfer('5FHneW46xGXgs3mDvNbLXCmqM7KogTFrDnXBpA9wkLSHTfU', 10_000_000_000_000_000_000n);

// Sign and send
const hash = await transfer.signAndSend(alice);
console.log('Transfer sent with hash:', hash.toHex());
```

## 🎯 Using GLIN Custom Features

### 1. Create a Federated Learning Task

Using Polkadot.js Apps:
1. Go to Developer → Extrinsics
2. Select `taskRegistry` → `createTask`
3. Fill in:
   - name: "Image Classification"
   - modelType: ResNet
   - bounty: 10000000000000000000000 (10 GLIN)
   - minProviders: 3
   - maxProviders: 10
   - ipfsHash: 0x (empty for now)
   - hardwareRequirements: {minVramGb: 8, minComputeCapability: 75, minBandwidthMbps: 100}

### 2. Register as a Provider

```javascript
// Stake 1000 GLIN to become a provider
const stake = api.tx.providerStaking.registerProvider(
  1000_000_000_000_000_000_000n, // 1000 GLIN
  {
    gpu_model: 'RTX 4090',
    gpu_tier: 'Prosumer',
    vram_gb: 24,
    compute_capability: 89,
    bandwidth_mbps: 1000,
    cpu_cores: 16,
    ram_gb: 32
  }
);

await stake.signAndSend(bob);
```

### 3. Join a Task as Provider

```javascript
const join = api.tx.taskRegistry.joinTask(taskId);
await join.signAndSend(provider);
```

### 4. Submit Rewards Batch

```javascript
const rewards = api.tx.rewardDistribution.submitRewards(
  batchId,
  [
    {
      provider: bobAddress,
      amount: 5_000_000_000_000_000_000n, // 5 GLIN
      gradients_contributed: 1000n,
      quality_score: 950,
      hardware_multiplier: 150
    }
  ]
);

await rewards.signAndSend(coordinator);
```

## 📈 Monitoring Your Node

### Check Block Production
```bash
# Watch logs
./target/release/glin-node --dev --log info

# You'll see:
# - "Prepared block for proposing"
# - "New block #X"
# - "Imported #X"
```

### Check Node Health
```bash
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
     http://localhost:9933
```

### Query Chain State
```javascript
// Get current block number
const blockNumber = await api.query.system.number();
console.log('Current block:', blockNumber.toString());

// Get account balance
const balance = await api.query.system.account(alice.address);
console.log('Alice balance:', balance.data.free.toString());

// Get all tasks
const tasks = await api.query.taskRegistry.tasks.entries();
console.log('Active tasks:', tasks.length);
```

## 🛠️ Advanced Usage

### Custom Chain Configuration
```bash
# Export chain spec
./target/release/glin-node build-spec --chain local > customSpec.json

# Modify customSpec.json as needed

# Convert to raw
./target/release/glin-node build-spec --chain customSpec.json --raw > customSpecRaw.json

# Run with custom spec
./target/release/glin-node --chain customSpecRaw.json
```

### Multi-Node Network
```bash
# Node 1 (Bootnode)
./target/release/glin-node \
  --base-path /tmp/node1 \
  --chain local \
  --alice \
  --port 30333 \
  --ws-port 9944 \
  --rpc-port 9933 \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001

# Get node identity
# Look for: Local node identity is: 12D3KooW...

# Node 2
./target/release/glin-node \
  --base-path /tmp/node2 \
  --chain local \
  --bob \
  --port 30334 \
  --ws-port 9945 \
  --rpc-port 9934 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/BOOTNODE_PEER_ID
```

### Backup & Restore
```bash
# Export blocks
./target/release/glin-node export-blocks --chain local > blocks.json

# Import blocks
./target/release/glin-node import-blocks --chain local < blocks.json
```

## 🐛 Troubleshooting

### Build Errors
```bash
# Clean build
cargo clean
cargo build --release

# Update dependencies
cargo update
```

### Node Won't Start
```bash
# Purge old chain data
./purge-chain.sh

# Check ports aren't in use
lsof -i :9944
lsof -i :9933
```

### Can't Connect to Node
- Ensure node is running: `ps aux | grep glin-node`
- Check firewall settings
- Try `--rpc-external --rpc-cors all` flags

## 📚 Resources

- **Polkadot.js Apps**: https://polkadot.js.org/apps/
- **Substrate Docs**: https://docs.substrate.io/
- **GLIN GitHub**: https://github.com/glin-ai/glin-chain

## 🎉 You're Ready!

Your GLIN blockchain is now running with:
- ✅ Block production every 6 seconds
- ✅ Pre-funded test accounts
- ✅ Task creation and management
- ✅ Provider staking system
- ✅ Reward distribution
- ✅ Web interface via Polkadot.js

Happy building! 🚀