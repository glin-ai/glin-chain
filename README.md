# GLIN Chain - Substrate Blockchain for Federated Learning

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Substrate](https://img.shields.io/badge/Substrate-4.0.0-green)](https://substrate.io/)
[![Discord](https://img.shields.io/discord/YOUR_DISCORD_ID?label=Discord&logo=discord)](https://discord.gg/YOUR_INVITE)
[![Twitter Follow](https://img.shields.io/twitter/follow/glin_ai?style=social)](https://twitter.com/glin_ai)

## 🌐 Overview

GLIN Chain is the blockchain infrastructure powering the GLIN federated learning marketplace. Built with Substrate, it provides trustless task management, provider staking, and reward distribution for decentralized AI training.

### Key Features

- 🔒 **Trustless Task Escrow**: Secure bounty locking for ML training tasks
- 💰 **Provider Staking**: Economic security through stake-based participation
- 🎯 **Batch Rewards**: Efficient merkle-tree based reward distribution
- 🛡️ **Slashing Mechanism**: Automated penalties for malicious behavior
- 🔍 **ZKP Validation**: On-chain verification for high-value computations

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Substrate dependencies
sudo apt update && sudo apt install -y git clang curl libssl-dev llvm libudev-dev make protobuf-compiler

# Clone the repository
git clone https://github.com/glin-ai/glin-chain.git
cd glin-chain
```

### Build & Run

```bash
# Build the node
cargo build --release

# Run development node
./target/release/glin-node --dev

# Run with custom chain spec
./target/release/glin-node --chain=local
```

### Docker

```bash
# Build Docker image
docker build -t glin-chain .

# Run container
docker run -p 9944:9944 -p 9933:9933 glin-chain
```

## 🏗️ Architecture

### Pallets

| Pallet | Purpose | Key Functions |
|--------|---------|---------------|
| **task-registry** | Task lifecycle management | `create_task`, `cancel_task`, `complete_task` |
| **provider-staking** | Provider registration & staking | `register_provider`, `slash_provider`, `withdraw_stake` |
| **reward-distribution** | Batch reward processing | `create_batch`, `submit_rewards`, `settle_batch` |
| **validation-zkp** | Zero-knowledge proof verification | `submit_proof`, `verify_gradient` |
| **reputation** | Provider reputation tracking | `update_score`, `get_reputation` |

### Economic Model

- **Token**: GLIN
- **Total Supply**: 1,000,000,000 GLIN
- **Minimum Provider Stake**: 1,000 GLIN
- **Minimum Task Bounty**: 10 GLIN
- **Platform Fee**: 2% of rewards
- **Slashing Rate**: 10% for violations

## 🔧 Configuration

### Chain Specification

```bash
# Generate chain spec
./target/release/glin-node build-spec --chain=local > customSpec.json

# Generate raw chain spec
./target/release/glin-node build-spec --chain=customSpec.json --raw > customSpecRaw.json
```

### Network Ports

- **P2P**: 30333
- **RPC**: 9933
- **WebSocket**: 9944
- **Prometheus**: 9615

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run pallet tests
cargo test -p pallet-task-registry
cargo test -p pallet-provider-staking
cargo test -p pallet-reward-distribution

# Run integration tests
cargo test --test '*'

# Run benchmarks
cargo test --features runtime-benchmarks
```

## 🔗 Integration

### JavaScript/TypeScript

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';

const provider = new WsProvider('ws://localhost:9944');
const api = await ApiPromise.create({ provider });

// Create a task
const tx = api.tx.taskRegistry.createTask(
  'Image Classification',
  'ResNet',
  1000000000000, // 1 GLIN bounty
  3, // min providers
  10, // max providers
  'QmHash...', // IPFS hash
  { minVramGb: 8, minComputeCapability: 75, minBandwidthMbps: 100 }
);

await tx.signAndSend(alice);
```

### Python

```python
from substrateinterface import SubstrateInterface

substrate = SubstrateInterface(
    url="ws://localhost:9944",
    type_registry_preset='substrate-node'
)

# Query provider info
result = substrate.query(
    module='ProviderStaking',
    storage_function='Providers',
    params=['5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY']
)
```

## 📊 Monitoring

### Prometheus Metrics

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'glin-node'
    static_configs:
      - targets: ['localhost:9615']
```

### Grafana Dashboard

Import our [Grafana dashboard](monitoring/grafana-dashboard.json) for real-time metrics.

## 🛠️ Development

### Adding a New Pallet

```bash
# Generate pallet template
substrate-node-new-pallet my-pallet

# Add to runtime
# Edit runtime/src/lib.rs
```

### Upgrading Runtime

```bash
# Build new runtime
cargo build --release -p glin-runtime

# Submit upgrade transaction
# Via polkadot.js or CLI
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings
```

## 📚 Documentation

- [Pallet Documentation](docs/pallets/)
- [RPC Documentation](docs/rpc.md)
- [Chain Specification](docs/chain-spec.md)
- [Integration Guide](docs/integration.md)

## 🔐 Security

### Audits

- [ ] Pallet logic audit (Planned Q1 2025)
- [ ] Runtime audit (Planned Q2 2025)

### Bug Bounty

Report security vulnerabilities to security@glin.ai. See [SECURITY.md](SECURITY.md) for our bug bounty program.

## 📜 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Website**: [https://glin.ai](https://glin.ai)
- **Documentation**: [https://docs.glin.ai](https://docs.glin.ai)
- **Discord**: [Join our community](https://discord.gg/YOUR_INVITE)
- **Twitter**: [@glin_ai](https://twitter.com/glin_ai)

## 🙏 Acknowledgments

Built with [Substrate](https://substrate.io/) by [Parity Technologies](https://parity.io/)

---

<p align="center">
  Made with ❤️ by the GLIN AI team
</p>