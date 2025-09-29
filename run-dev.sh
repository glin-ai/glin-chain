#!/bin/bash

# Run GLIN node in development mode

echo "🚀 Starting GLIN Node in Development Mode"
echo "========================================="
echo ""
echo "This will:"
echo "  • Start a single-node development chain"
echo "  • Create blocks instantly when transactions arrive"
echo "  • Use temporary storage (chain data deleted on restart)"
echo "  • Fund Alice, Bob, Charlie, Dave, Eve accounts"
echo ""
echo "Accounts with funds (100,000 GLIN each):"
echo "  • Alice (sudo): 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
echo "  • Bob:          5FHneW46xGXJLOITO0U2MkBMT9KBYexhjfXqQCzwJYHoUK"
echo "  • Charlie:      5FLSigC9HGRrYXOppRwrTDDEpRs4vAsFOD4BgkDgQMOD"
echo ""
echo "WebSocket RPC: ws://localhost:9944"
echo "HTTP RPC:      http://localhost:9933"
echo ""
echo "Connect with Polkadot.js Apps:"
echo "https://polkadot.js.org/apps/?rpc=ws://localhost:9944"
echo ""
echo "Press Ctrl+C to stop the node"
echo "========================================="
echo ""

# Run the node
./target/release/glin-node \
    --dev \
    --rpc-external \
    --rpc-cors all \
    --rpc-methods Unsafe \
    --log info \
    --name "GLIN-Dev-Node"