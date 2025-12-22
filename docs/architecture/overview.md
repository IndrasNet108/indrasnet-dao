# Architecture Overview

IndrasNet DAO Core is a modular Solana/Anchor system composed of five on-chain
programs:

- Core: primary DAO state and governance logic
- AI: AI analysis registration and related accounts
- Security: security workflows and compliance-related state
- Partnerships: partnerships lifecycle and related state
- Orchestrator: cross-program coordination

Design goals:

- Clear separation of responsibilities between programs
- Deterministic PDA usage across on-chain and off-chain code
- Upgradeable programs with explicit deployment tracking
