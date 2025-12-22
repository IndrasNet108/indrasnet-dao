//! Financial Governance, Risk & Compliance module
//!
//! Financial GRC management
//!
//! On-chain: Metadata for GRC
//! Off-chain: Actual GRC, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// GRC component
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialGRCComponent {
    /// Governance
    Governance,
    /// Risk
    Risk,
    /// Compliance
    Compliance,
    /// Integrated
    Integrated,
}

/// GRC status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialGRCStatus {
    /// GRC active
    Active,
    /// GRC paused
    Paused,
    /// GRC optimized
    Optimized,
}

/// Financial GRC metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialGovernanceRiskComplianceMetadata {
    /// GRC ID
    pub grc_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// GRC component
    pub grc_component: FinancialGRCComponent,
    /// Status
    pub status: FinancialGRCStatus,
    /// Created at
    pub created_at: i64,
    /// GRC config hash
    pub grc_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_governance_risk_compliance(
        grc: &mut FinancialGovernanceRiskComplianceMetadata,
        grc_id: u64,
        entity_id: u64,
        grc_component: FinancialGRCComponent,
        grc_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(grc_id > 0, IndrasError::InvalidInput);
        grc.grc_id = grc_id;
        grc.entity_id = entity_id;
        grc.grc_component = grc_component;
        grc.status = FinancialGRCStatus::Active;
        grc.created_at = current_time;
        grc.grc_config_hash = grc_config_hash;
        grc.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_grc(_grc_id: u64) -> Vec<u8> {
        vec![]
    }
}
