//! Financial Climate Risk module
//!
//! Financial climate risk management
//!
//! On-chain: Metadata for climate risk
//! Off-chain: Actual risk, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Climate risk type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialClimateRiskType {
    /// Physical risk
    Physical,
    /// Transition risk
    Transition,
    /// Liability risk
    Liability,
    /// Custom risk
    Custom,
}

/// Climate risk status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialClimateRiskStatus {
    /// Risk active
    Active,
    /// Risk paused
    Paused,
    /// Risk mitigated
    Mitigated,
}

/// Financial climate risk metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialClimateRiskMetadata {
    /// Risk ID
    pub risk_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Climate risk type
    pub climate_risk_type: FinancialClimateRiskType,
    /// Status
    pub status: FinancialClimateRiskStatus,
    /// Created at
    pub created_at: i64,
    /// Risk config hash
    pub risk_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_climate_risk(
        risk: &mut FinancialClimateRiskMetadata,
        risk_id: u64,
        entity_id: u64,
        climate_risk_type: FinancialClimateRiskType,
        risk_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(risk_id > 0, IndrasError::InvalidInput);
        risk.risk_id = risk_id;
        risk.entity_id = entity_id;
        risk.climate_risk_type = climate_risk_type;
        risk.status = FinancialClimateRiskStatus::Active;
        risk.created_at = current_time;
        risk.risk_config_hash = risk_config_hash;
        risk.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_climate_risk(_risk_id: u64) -> Vec<u8> {
        vec![]
    }
}
