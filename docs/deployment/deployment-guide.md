# Deployment Guide

**Created:** 2024-12-19  
**Last Updated:** 2025-12-10  
**Status:** Current  
**Version:** 1.0-MVP (Modular Architecture)

---

## 🎯 Overview

This guide covers deployment of IndrasNet DAO Core (MVP) modular architecture with 5 programs (Core, Orchestrator, AI, Security, Partnerships) to Solana networks (devnet, testnet, mainnet).

---

## 📋 Prerequisites

### Required Tools

1. **Solana CLI** (v1.17.0 or later)
2. **Anchor Framework** (v0.32.1 or later)
3. **Rust** (1.83.0 or later)
4. **Keypair** for deployment authority

### Network Configuration

```bash
# Devnet
solana config set --url devnet

# Testnet
solana config set --url testnet

# Mainnet
solana config set --url mainnet-beta
```

---

## 🚀 Deployment Steps

### 1. Build Programs

```bash
# Build all programs
anchor build

# Verify build
ls -la target/deploy/
```

### 2. Configure Program IDs

```bash
# Update Anchor.toml with program IDs
# Update declare_id! in each program's lib.rs
```

### 3. Deploy Programs

**Current Status:** ✅ All 5 programs deployed on DevNet

#### For Core Program (Already Deployed - Use Upgrade):
```bash
# ✅ Upgrade Core program (already deployed)
anchor upgrade target/deploy/indrasnet_dao_core.so \
  --program-id 73ZMGGaPJQz5cNbSfGB5KHS5k5cBBU5NTjMuXJgDJzu8 \
  --provider.cluster devnet
```

#### For Other Programs (Already Deployed):
```bash
# All programs are already deployed, use upgrade for updates:
anchor deploy --program-name indrasnet_dao_security
anchor deploy --program-name indrasnet_dao_ai
anchor deploy --program-name indrasnet_dao_partnerships
anchor deploy --program-name indrasnet_dao_orchestrator
```

**Note:** See [Program IDs](./program-ids.md) and [Deployment Status](../status/deployment/deployment_status.md) for current status.

### 4. Verify Deployment

```bash
# Verify all programs (all deployed ✅)
solana program show 73ZMGGaPJQz5cNbSfGB5KHS5k5cBBU5NTjMuXJgDJzu8 --url devnet  # Core ✅
solana program show Dfrv6QnAEaUfkoSFg92DeuhbPqbhDXK7J1FFMXMZFwPh --url devnet  # Security ✅
solana program show GwDihBiWfAmej9enpYRrie4D2XoPsfD44ML3sqDQvcXe --url devnet  # AI ✅
solana program show EN97gsvMDVLB9xbkaAUCC93aYY38Lv5VPSswcXf76q8F --url devnet  # Partnerships ✅
solana program show FB3hzRqwtJzomie2CjBFQWFJad6TwXC4uSfrUJm8X4Yo --url devnet  # Orchestrator ✅

# Verify programs on explorer
# See [Program IDs](./program-ids.md) for all Explorer links
```

---

## 🔐 Security Considerations

### Key Management

1. **Use Hardware Wallets:** For mainnet deployments
2. **Secure Key Storage:** Never commit keys to repository
3. **Multi-Signature:** Use multisig for authority accounts
4. **Key Rotation:** Regular key rotation schedule

### Program Upgrades

1. **Test Upgrades:** Test upgrades on devnet first
2. **Backup State:** Backup state before upgrades
3. **Upgrade Authority:** Secure upgrade authority
4. **Rollback Plan:** Have rollback plan ready

---

## 📊 Network-Specific Configuration

### Devnet

**Purpose:** Development and testing

**Configuration:**
```bash
solana config set --url devnet
solana airdrop 2  # Get test SOL
```

**Deployment:**
```bash
anchor deploy
```

### Testnet

**Purpose:** Pre-production testing

**Configuration:**
```bash
solana config set --url testnet
# Request testnet SOL from faucet
```

**Deployment:**
```bash
anchor deploy
```

### Mainnet

**Purpose:** Production deployment

**Configuration:**
```bash
solana config set --url mainnet-beta
# Ensure sufficient SOL for deployment
```

**Deployment:**
```bash
# Double-check all configurations
anchor build
anchor deploy --provider.cluster mainnet-beta
```

---

## 🔄 Upgrade Process

### 1. Build New Version

```bash
anchor build
```

### 2. Deploy Upgrade

```bash
# Upgrade program
anchor upgrade target/deploy/<PROGRAM_NAME>.so \
    --program-id <PROGRAM_ID> \
    --provider.cluster <NETWORK>
```

### 3. Verify Upgrade

```bash
# Check program version
solana program show <PROGRAM_ID>

# Test upgraded functionality
anchor test
```

---

## 🧪 Post-Deployment Testing

### 1. Initialize DAO

```bash
# Initialize DAO configuration
anchor run initialize-dao
```

### 2. Test Core Functionality

```bash
# Test idea creation
anchor run create-idea

# Test grant creation
anchor run create-grant

# Test voting
anchor run cast-vote
```

### 3. Monitor System

```bash
# Monitor logs
solana logs

# Check account states
solana account <ACCOUNT_ADDRESS>
```

---

## 📈 Monitoring

### Metrics to Monitor

1. **Transaction Success Rate:** Percentage of successful transactions
2. **Compute Unit Usage:** Compute unit consumption
3. **Account Growth:** Account creation rate
4. **Error Rate:** Error frequency and types

### Tools

1. **Solana Explorer:** Transaction and account inspection
2. **Solana Beach:** Advanced analytics
3. **Custom Dashboards:** Build custom monitoring dashboards

---

## ✅ Pre-Deployment Checklist

### Environment Setup
- [ ] Solana CLI installed (`solana --version`)
- [ ] Anchor CLI installed (`anchor --version`)
- [ ] Rust toolchain installed (`rustc --version`)
- [ ] Wallet configured (`solana address`)
- [ ] Devnet balance sufficient (2+ SOL for first deploy, 0.5+ SOL for upgrade)

### Code Quality
- [ ] `anchor build` passes successfully
- [ ] `cargo clippy` passes (warnings acceptable)
- [ ] Program compiles without errors
- [ ] No critical warnings

### Configuration
- [ ] `Anchor.toml` configured for target network
- [ ] Program ID set correctly in `lib.rs` and `Anchor.toml`
- [ ] RPC endpoint configured
- [ ] Wallet path correct

### Testing
- [ ] Basic tests pass (`anchor test`)
- [ ] Critical functions tested
- [ ] No blocking test failures

## 📋 Deployment Checklist

### Before Deployment
- [ ] All pre-deployment checks completed
- [ ] Program IDs verified
- [ ] Balance sufficient for deployment
- [ ] Backup of current state (if upgrading)

### During Deployment
- [ ] Build successful
- [ ] Deployment transaction confirmed
- [ ] Program ID matches expected value

### After Deployment
- [ ] Program verified on Solana Explorer
- [ ] Program IDs updated in all configurations
- [ ] Basic functionality tested
- [ ] Monitoring configured

## 🔧 Troubleshooting

### Common Issues

1. **Deployment Failures:**
   ```bash
   # Check SOL balance
   solana balance
   
   # Check program size
   ls -lh target/deploy/*.so
   ```

2. **Program ID Mismatches:**
   ```bash
   # Verify program IDs
   anchor keys list
   
   # Update if needed
   anchor keys sync
   ```

3. **Account Rent Issues:**
   ```bash
   # Check rent exemption
   solana account <ACCOUNT>
   
   # Add rent if needed
   solana transfer <ACCOUNT> <AMOUNT>
   ```

---

## 📚 Related Documentation

- [Deployment Status](../status/deployment/deployment_status.md) - Current deployment status
- [Program IDs](program-ids.md) - All program IDs with Explorer links
- [Build Status](../status/build/build_status.md) - Build information
- [Architecture Overview](../architecture/programs-overview.md) - System architecture

---

## 🚀 Next Steps

- Set up monitoring and alerting
- Configure backup procedures
- Establish upgrade procedures
- Document runbooks for operations
