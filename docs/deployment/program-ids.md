# Program IDs - DevNet Deployment

**Last Updated:** 2025-12-16  
**Network:** DevNet  
**Status:** ✅ All programs deployed

---

## 📋 Program IDs

| Program | Program ID | Status | Solana Explorer |
|---------|-----------|--------|-----------------|
| **Core** | `73ZMGGaPJQz5cNbSfGB5KHS5k5cBBU5NTjMuXJgDJzu8` | ✅ Deployed | [View on Explorer](https://explorer.solana.com/address/73ZMGGaPJQz5cNbSfGB5KHS5k5cBBU5NTjMuXJgDJzu8?cluster=devnet) |
| **Security** | `Dfrv6QnAEaUfkoSFg92DeuhbPqbhDXK7J1FFMXMZFwPh` | ✅ Deployed | [View on Explorer](https://explorer.solana.com/address/Dfrv6QnAEaUfkoSFg92DeuhbPqbhDXK7J1FFMXMZFwPh?cluster=devnet) |
| **AI** | `GwDihBiWfAmej9enpYRrie4D2XoPsfD44ML3sqDQvcXe` | ✅ Deployed | [View on Explorer](https://explorer.solana.com/address/GwDihBiWfAmej9enpYRrie4D2XoPsfD44ML3sqDQvcXe?cluster=devnet) |
| **Partnerships** | `EN97gsvMDVLB9xbkaAUCC93aYY38Lv5VPSswcXf76q8F` | ✅ Deployed | [View on Explorer](https://explorer.solana.com/address/EN97gsvMDVLB9xbkaAUCC93aYY38Lv5VPSswcXf76q8F?cluster=devnet) |
| **Orchestrator** | `FB3hzRqwtJzomie2CjBFQWFJad6TwXC4uSfrUJm8X4Yo` | ✅ Deployed | [View on Explorer](https://explorer.solana.com/address/FB3hzRqwtJzomie2CjBFQWFJad6TwXC4uSfrUJm8X4Yo?cluster=devnet) |

---

## 🔍 Verification Commands

### Check Program Deployment

```bash
# Core Program
solana program show 73ZMGGaPJQz5cNbSfGB5KHS5k5cBBU5NTjMuXJgDJzu8 --url devnet

# Security Program
solana program show Dfrv6QnAEaUfkoSFg92DeuhbPqbhDXK7J1FFMXMZFwPh --url devnet

# AI Program
solana program show GwDihBiWfAmej9enpYRrie4D2XoPsfD44ML3sqDQvcXe --url devnet

# Partnerships Program
solana program show EN97gsvMDVLB9xbkaAUCC93aYY38Lv5VPSswcXf76q8F --url devnet

# Orchestrator Program
solana program show FB3hzRqwtJzomie2CjBFQWFJad6TwXC4uSfrUJm8X4Yo --url devnet
```

### Verify Program Security Metadata

```bash
# Core Program
npx @solana-program/program-metadata@latest read security 73ZMGGaPJQz5cNbSfGB5KHS5k5cBBU5NTjMuXJgDJzu8 --cluster devnet

# Other programs (replace with respective Program ID)
npx @solana-program/program-metadata@latest read security <PROGRAM_ID> --cluster devnet
```

---

## 📝 Configuration

### Anchor.toml

Program IDs are configured in `Anchor.toml`:

```toml
[programs.devnet]
indrasnet_dao_core = { address = "73ZMGGaPJQz5cNbSfGB5KHS5k5cBBU5NTjMuXJgDJzu8", features = ["skip-ed25519-verify"] }
```

### Code Declaration

Program IDs are declared in each program's `lib.rs`:

- **Core**: `programs/indrasnet-dao-core/src/lib.rs`
- **Orchestrator**: `programs/indrasnet-dao-orchestrator/src/lib.rs`
- **AI**: `programs/indrasnet-dao-ai/src/lib.rs`
- **Security**: `programs/indrasnet-dao-security/src/lib.rs`
- **Partnerships**: `programs/indrasnet-dao-partnerships/src/lib.rs`

---

## 🔗 Related Documentation

- [Deployment Guide](./deployment-guide.md) - How to deploy programs
- [Program ID Management](./program-id-management.md) - Managing program IDs
- [Programs Overview](../architecture/programs-overview.md) - Program architecture

---

**Last Updated:** 2025-12-16
