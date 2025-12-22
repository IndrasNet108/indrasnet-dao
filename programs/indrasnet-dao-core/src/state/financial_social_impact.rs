//! Financial Social Impact module
//!
//! Financial social impact management
//!
//! On-chain: Metadata for social impact
//! Off-chain: Actual impact, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Social impact area
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialSocialImpactArea {
    /// Poverty alleviation
    PovertyAlleviation,
    /// Education access
    EducationAccess,
    /// Healthcare access
    HealthcareAccess,
    /// Custom area
    Custom,
}

/// Impact status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialSocialImpactStatus {
    /// Impact active
    Active,
    /// Impact paused
    Paused,
    /// Impact achieved
    Achieved,
}

/// Financial social impact metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialSocialImpactMetadata {
    /// Impact ID
    pub impact_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Social impact area
    pub social_impact_area: FinancialSocialImpactArea,
    /// Status
    pub status: FinancialSocialImpactStatus,
    /// Created at
    pub created_at: i64,
    /// Impact data hash
    pub impact_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_social_impact(
        impact: &mut FinancialSocialImpactMetadata,
        impact_id: u64,
        entity_id: u64,
        social_impact_area: FinancialSocialImpactArea,
        impact_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(impact_id > 0, IndrasError::InvalidInput);
        impact.impact_id = impact_id;
        impact.entity_id = entity_id;
        impact.social_impact_area = social_impact_area;
        impact.status = FinancialSocialImpactStatus::Active;
        impact.created_at = current_time;
        impact.impact_data_hash = impact_data_hash;
        impact.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_social_impact(_impact_id: u64) -> Vec<u8> {
        vec![]
    }
}
