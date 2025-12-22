//! DeFi Governance module
//!
//! DeFi protocol governance
//!
//! On-chain: Metadata for governance
//! Off-chain: Actual governance, voting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Governance type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DeFiGovernanceType {
    /// Token-based governance
    TokenBased,
    /// Multi-sig governance
    MultiSig,
    /// Time-locked governance
    TimeLocked,
    /// Custom governance
    Custom,
}

/// Governance status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DeFiGovernanceStatus {
    /// Governance active
    Active,
    /// Governance paused
    Paused,
    /// Governance disabled
    Disabled,
}

/// DeFi governance metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct DeFiGovernanceMetadata {
    /// Governance ID
    pub governance_id: u64,
    /// Protocol ID
    pub protocol_id: u64,
    /// Governance type
    pub governance_type: DeFiGovernanceType,
    /// Status
    pub status: DeFiGovernanceStatus,
    /// Created at
    pub created_at: i64,
    /// Governance config hash
    pub governance_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_defi_governance(
        governance: &mut DeFiGovernanceMetadata,
        governance_id: u64,
        protocol_id: u64,
        governance_type: DeFiGovernanceType,
        governance_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(governance_id > 0, IndrasError::InvalidInput);
        governance.governance_id = governance_id;
        governance.protocol_id = protocol_id;
        governance.governance_type = governance_type;
        governance.status = DeFiGovernanceStatus::Active;
        governance.created_at = current_time;
        governance.governance_config_hash = governance_config_hash;
        governance.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn execute_defi_governance(_governance_id: u64) -> Vec<u8> {
        vec![]
    }
}
