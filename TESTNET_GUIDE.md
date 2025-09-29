# GLIN Incentivized Testnet Guide

## Welcome to GLIN Testnet! 🚀

The GLIN Incentivized Testnet is your opportunity to earn mainnet tokens by helping us build and test the next generation federated learning marketplace. This is a **real** testnet with real rewards - not a simulation.

## Quick Start

### 1. Get a Wallet

Install a Substrate-compatible wallet:
- **Polkadot.js Extension** (Recommended): https://polkadot.js.org/extension/
- **Talisman**: https://talisman.xyz/
- **SubWallet**: https://subwallet.app/

### 2. Connect to GLIN Testnet

1. Open Polkadot.js Apps: https://polkadot.js.org/apps/
2. Click on the network dropdown (top-left)
3. Select "Development" → "Custom"
4. Enter our RPC endpoint: `wss://rpc.glin-testnet.railway.app`
5. Click "Switch"

### 3. Get Test Tokens

Visit our faucet to claim your initial tGLIN tokens:

1. Go to: https://faucet.glin-testnet.railway.app
2. Verify your account with Twitter or GitHub
3. Enter your wallet address
4. Claim 1000 tGLIN tokens

**Note**: Each social account can only claim once. Tokens are for testing only and have no monetary value.

## Earning Points for Mainnet Airdrop 🎯

Points determine your share of the mainnet airdrop. Here's how to earn:

### Activity Points Table

| Activity | Base Points | Description |
|----------|-------------|-------------|
| 🚰 **Faucet Claim** | 10 | One-time bonus for joining |
| 📝 **Create ML Task** | 100 | Submit training jobs |
| ✅ **Complete Task** | 500 | Successfully train models |
| 💻 **Provide GPU** | 50/hour | Share computing power |
| 🔍 **Validate Results** | 200 | Verify training outputs |
| 🗳️ **Governance Vote** | 50 | Participate in decisions |
| 🐛 **Report Bugs** | 1000 | Find and report issues |
| 👥 **Successful Referral** | 100 | Bring new users |
| 💧 **Provide Liquidity** | 300 | Add to DEX pools (coming soon) |
| 🧪 **Test Features** | 150 | Participate in specific tests |

### Bonus Multipliers

Earn additional multipliers on ALL activities:

- 🏃 **Early Bird** (First 1000 users): +50% bonus
- 💻 **GPU Provider**: +20% bonus on all activities
- 🐛 **Bug Hunter**: +30% bonus after first confirmed bug
- ✅ **Verified User**: +10% bonus (Twitter/GitHub verification)

### Leaderboard

Track your ranking: https://leaderboard.glin-testnet.railway.app

Top 100 users receive additional rewards:
- 🥇 Top 10: 5x multiplier on final points
- 🥈 Top 11-50: 3x multiplier
- 🥉 Top 51-100: 2x multiplier

## Core Activities

### 1. Create ML Training Tasks

Submit federated learning tasks to the network:

```javascript
// Example using our SDK (coming soon)
const task = await glin.createTask({
  model: "ResNet50",
  dataset: "CIFAR-10",
  epochs: 10,
  reward: 100 // tGLIN tokens
});
```

### 2. Provide GPU Resources

Share your GPU for training:

```bash
# Install GLIN GPU Provider
curl -sSL https://get.glin.ai | bash

# Start providing GPU
glin-provider start --wallet YOUR_ADDRESS
```

Requirements:
- NVIDIA GPU with 8GB+ VRAM
- CUDA 11.8+
- Stable internet connection

### 3. Validate Training Results

Become a validator to verify model training:

1. Stake at least 10,000 tGLIN
2. Run validation software
3. Earn rewards for accurate validations

### 4. Participate in Governance

Vote on protocol upgrades and parameter changes:

1. Go to Polkadot.js Apps → Democracy
2. Review active proposals
3. Vote with your tGLIN tokens
4. Earn points for each vote

### 5. Bug Bounty Program

Find bugs and earn massive points:

- **Critical**: 5000 points + 10,000 tGLIN bonus
- **High**: 2000 points + 5,000 tGLIN bonus
- **Medium**: 1000 points + 2,000 tGLIN bonus
- **Low**: 500 points + 1,000 tGLIN bonus

Report at: https://github.com/glin-ai/glin-chain/issues

## Technical Details

### Network Information

- **Chain**: GLIN Incentivized Testnet
- **Token**: tGLIN (test GLIN)
- **Decimals**: 18
- **Block Time**: ~6 seconds
- **Consensus**: Aura + GRANDPA

### RPC Endpoints

- **WebSocket**: `wss://rpc.glin-testnet.railway.app`
- **HTTPS**: `https://rpc.glin-testnet.railway.app`

### Block Explorer

Explore transactions and blocks:
https://explorer.glin-testnet.railway.app

### Smart Contracts (Coming Soon)

We're adding ink! smart contract support. Stay tuned for:
- Custom DeFi protocols
- NFT marketplaces
- Advanced ML orchestration

## Development Tools

### SDK Installation

```bash
npm install @glin/sdk
# or
yarn add @glin/sdk
```

### Example Integration

```typescript
import { GlinClient } from '@glin/sdk';

const client = new GlinClient({
  network: 'testnet',
  rpcUrl: 'wss://rpc.glin-testnet.railway.app'
});

// Check balance
const balance = await client.getBalance(address);

// Submit task
const task = await client.submitTask({
  type: 'training',
  model: 'gpt-2',
  bounty: 1000
});

// Track points
const points = await client.getUserPoints(address);
```

## Roadmap

### Phase 1: Foundation (Current)
- ✅ Launch testnet
- ✅ Faucet service
- ✅ Points tracking
- 🔄 Basic ML task submission

### Phase 2: GPU Network
- GPU provider software
- Task matching algorithm
- Validation framework
- Performance benchmarks

### Phase 3: Advanced Features
- DEX integration
- Governance module
- Smart contracts
- Cross-chain bridges

### Phase 4: Mainnet Preparation
- Security audits
- Load testing
- Economic modeling
- Airdrop snapshot

## Support & Community

### Discord
Join our community: https://discord.gg/glin-ai

### Twitter
Follow updates: https://twitter.com/glin_ai

### Telegram
Announcements: https://t.me/glin_announcements

### GitHub
Contribute: https://github.com/glin-ai/glin-chain

## FAQ

**Q: When is the mainnet launch?**
A: Q2 2025, pending successful testnet completion.

**Q: How many points do I need for the airdrop?**
A: No minimum, but more points = larger allocation.

**Q: Can I run multiple validators?**
A: Yes, but rewards are per-account, not per-validator.

**Q: Are testnet tokens valuable?**
A: No, tGLIN has no monetary value. Only for testing.

**Q: When will the airdrop happen?**
A: At mainnet launch. Snapshot date TBA.

## Security

- **Never share your seed phrase**
- **Use a separate wallet for testnet**
- **Verify all URLs carefully**
- **Report suspicious activity**

## Terms & Conditions

By participating in the GLIN Incentivized Testnet, you agree to:

1. Use the network for testing purposes only
2. Report bugs and issues responsibly
3. Not exploit vulnerabilities for gain
4. Comply with all applicable laws
5. Accept that testnet tokens have no value

Points and airdrop allocations are subject to:
- Anti-sybil verification
- KYC requirements (for large allocations)
- Final team discretion

---

**Ready to earn your share of GLIN?** 🚀

Start now: https://faucet.glin-testnet.railway.app

*Building the future of federated learning, together.*