# Resource Optimization Guide

## Overview

This guide documents the resource optimization improvements applied to the GLIN testnet to reduce RAM and disk usage by 80-90%.

## Problem Summary

The testnet was experiencing rapid resource growth:
- **Disk**: Growing 2GB per day
- **RAM**: Increasing from 1.4GB to 2GB+ over hours
- **Root Cause**: Excessive block retention (1000 blocks), unfiltered logging, and high runtime instance count

## Solution Implemented

### 1. Optimized Pruning Settings

**Validators:**
- `--state-pruning 256` (reduced from 1000)
- `--blocks-pruning 256` (reduced from 1000)
- `--db-cache 512` (MB, explicit limit)
- `--state-cache-size 256` (MB, explicit limit)

**RPC Node:**
- `--state-pruning 256`
- `--blocks-pruning 256`
- `--db-cache 1024` (MB, slightly higher for query performance)
- `--state-cache-size 512` (MB)
- `--max-runtime-instances 64` (reduced from 256, saves ~10GB RAM)
- `--runtime-cache-size 4` (reduced from 8)

### 2. Log Filtering

All nodes now use reduced logging verbosity:

```bash
export RUST_LOG="info,sc_consensus_slots=warn,aura=warn,grandpa=warn,sc_network=warn,sync=warn"
export RUST_BACKTRACE="0"
```

This reduces log output by ~90% while maintaining critical error visibility.

### 3. Database Maintenance Script

Created `scripts/db_maintenance.sh` for periodic cleanup:

```bash
# Run on any node (while stopped)
./scripts/db_maintenance.sh validator-1
./scripts/db_maintenance.sh rpc
```

## Environment Variables

### Railway Configuration

Set these environment variables for all node services:

```bash
# Logging
RUST_LOG=info,sc_consensus_slots=warn,aura=warn,grandpa=warn,sc_network=warn
RUST_BACKTRACE=0

# Pruning (optional overrides)
PRUNING=256
BLOCKS_PRUNING=256

# Database (optional overrides)
DB_CACHE=512           # 1024 for RPC
STATE_CACHE=256        # 512 for RPC

# RPC-specific
MAX_RUNTIME_INSTANCES=64
RUNTIME_CACHE=4
```

## Expected Results

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| **Disk Growth** | 2 GB/day | 200-400 MB/day | 80-90% |
| **RAM Usage** | 2GB+ (growing) | 1.2-1.5 GB (stable) | 40% |
| **Log Volume** | ~500 MB/day | ~50 MB/day | 90% |
| **Boot Time** | Slower | Faster | Less data to load |

## Deployment Instructions

### Option 1: Rolling Update (Recommended)

1. **Test on one validator first:**
   ```bash
   # On Railway, redeploy validator-1 service
   # It will pick up the new script settings
   ```

2. **Monitor for 30 minutes:**
   - Check RAM usage stabilizes
   - Verify block production continues
   - Check logs for errors

3. **Roll out to remaining validators:**
   - Deploy validator-2
   - Deploy validator-3
   - Deploy RPC node

4. **Expected downtime per node:** 5-10 minutes (re-sync time)

### Option 2: Full Restart (Faster)

1. **Stop all nodes**
2. **Deploy all services simultaneously**
3. **Network resumes when 2/3 validators are online**
4. **Total downtime:** ~10-15 minutes

## Monitoring

After deployment, monitor these metrics:

- **RAM usage** should stabilize at 1.2-1.5 GB
- **Disk growth** should slow to 200-400 MB/day
- **Block production** should continue every 6 seconds
- **No consensus errors** in logs

## Maintenance Schedule

### Weekly
- Check disk usage trends via Railway dashboard
- Verify RAM remains stable

### Monthly
- Run `db_maintenance.sh` on each node (optional, only if needed)

### Quarterly
- Consider testnet reset if data accumulation becomes problematic
- Review and adjust pruning settings based on usage patterns

## Troubleshooting

### Node won't start after update
- Check logs for "database version mismatch"
- Solution: Clear database and re-sync from genesis
  ```bash
  ./scripts/purge-chain.sh
  ```

### RAM still growing
- Check `RUST_LOG` is properly set in environment
- Verify `--max-runtime-instances` is 64 (not 256) on RPC
- Review Railway metrics for memory leaks

### Disk still growing fast
- Verify `--state-pruning 256` is active (check node startup logs)
- Run database maintenance script
- Check for log accumulation in Railway volumes

### Sync issues after update
- Normal: nodes will re-sync with new pruning settings
- Should complete in 5-10 minutes for current testnet size
- If stuck, check bootnodes are accessible

## Technical Details

### Why 256 blocks?

- Substrate's GRANDPA finality typically finalizes within 100-200 blocks
- 256 blocks provides safety margin while keeping storage minimal
- At 6-second block time: 256 blocks = ~25 minutes of history

### Why reduce runtime instances?

- Each runtime instance loads entire WASM runtime into memory (~50-100 MB)
- 256 instances = 12-25 GB RAM (excessive for testnet)
- 64 instances provides sufficient parallel query capacity for testnet load

### Database backend

- Using RocksDB (Substrate default)
- Explicit cache limits prevent unbounded growth
- Periodic compaction recommended but not required

## Rollback Instructions

If issues arise, revert by setting environment variables:

```bash
PRUNING=1000
BLOCKS_PRUNING=1000
MAX_RUNTIME_INSTANCES=256
RUST_LOG=info
```

Then redeploy services. Note: This will restore original behavior but also restore high resource usage.

## Future Improvements

### Potential optimizations:
1. **Switch to ParityDB** - More efficient for Substrate
2. **Separate archive node** - Keep validators lean, run archive separately
3. **Log aggregation** - Send logs to external service, clear local logs
4. **Automated maintenance** - Cron job for periodic DB compaction

### When to consider:
- Testnet scales to 10+ validators
- Daily transaction volume exceeds 10,000
- Community requires longer historical data access