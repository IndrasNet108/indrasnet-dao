//! Financial Data Governance module
//!
//! Financial data governance
//!
//! On-chain: Metadata for data governance
//! Off-chain: Actual governance, policy enforcement

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Governance policy type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataGovernancePolicyType {
    /// Data access policy
    DataAccess,
    /// Data retention policy
    DataRetention,
    /// Data privacy policy
    DataPrivacy,
    /// Custom policy
    Custom,
}

/// Governance status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataGovernanceStatus {
    /// Governance active
    Active,
    /// Governance paused
    Paused,
    /// Governance disabled
    Disabled,
}

/// Financial data governance metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDataGovernanceMetadata {
    /// Governance ID
    pub governance_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Governance policy type
    pub governance_policy_type: FinancialDataGovernancePolicyType,
    /// Status
    pub status: FinancialDataGovernanceStatus,
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
    pub fn initialize_financial_data_governance(
        governance: &mut FinancialDataGovernanceMetadata,
        governance_id: u64,
        entity_id: u64,
        governance_policy_type: FinancialDataGovernancePolicyType,
        governance_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(governance_id > 0, IndrasError::InvalidInput);
        governance.governance_id = governance_id;
        governance.entity_id = entity_id;
        governance.governance_policy_type = governance_policy_type;
        governance.status = FinancialDataGovernanceStatus::Active;
        governance.created_at = current_time;
        governance.governance_config_hash = governance_config_hash;
        governance.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn enforce_data_governance(_governance_id: u64) -> bool {
        false
    }
}
