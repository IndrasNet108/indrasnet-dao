//! Financial Enterprise Risk Management module
//!
//! Financial ERM
//!
//! On-chain: Metadata for ERM
//! Off-chain: Actual ERM, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Risk category
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialERMRiskCategory {
    /// Strategic risk
    Strategic,
    /// Operational risk
    Operational,
    /// Financial risk
    Financial,
    /// Compliance risk
    Compliance,
}

/// ERM status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialERMStatus {
    /// ERM active
    Active,
    /// ERM paused
    Paused,
    /// ERM optimized
    Optimized,
}

/// Financial ERM metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialEnterpriseRiskManagementMetadata {
    /// ERM ID
    pub erm_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Risk category
    pub risk_category: FinancialERMRiskCategory,
    /// Status
    pub status: FinancialERMStatus,
    /// Created at
    pub created_at: i64,
    /// ERM config hash
    pub erm_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_enterprise_risk_management(
        erm: &mut FinancialEnterpriseRiskManagementMetadata,
        erm_id: u64,
        entity_id: u64,
        risk_category: FinancialERMRiskCategory,
        erm_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(erm_id > 0, IndrasError::InvalidInput);
        erm.erm_id = erm_id;
        erm.entity_id = entity_id;
        erm.risk_category = risk_category;
        erm.status = FinancialERMStatus::Active;
        erm.created_at = current_time;
        erm.erm_config_hash = erm_config_hash;
        erm.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_erm(_erm_id: u64) -> Vec<u8> {
        vec![]
    }
}
