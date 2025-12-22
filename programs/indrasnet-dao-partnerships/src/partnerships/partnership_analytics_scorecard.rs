//! Partnership Analytics Scorecard module
//!
//! Partnership analytics scorecard
//!
//! On-chain: Metadata for scorecard
//! Off-chain: Actual scorecard, generation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Scorecard type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipScorecardType {
    /// Balanced scorecard
    Balanced,
    /// Performance scorecard
    Performance,
    /// Strategic scorecard
    Strategic,
    /// Custom scorecard
    Custom,
}

/// Scorecard status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipScorecardStatus {
    /// Scorecard generating
    Generating,
    /// Scorecard ready
    Ready,
    /// Scorecard published
    Published,
}

/// Partnership analytics scorecard metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsScorecardMetadata {
    /// Scorecard ID
    pub scorecard_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Scorecard type
    pub scorecard_type: PartnershipScorecardType,
    /// Status
    pub status: PartnershipScorecardStatus,
    /// Created at
    pub created_at: i64,
    /// Scorecard data hash
    pub scorecard_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_partnership_analytics_scorecard(
        scorecard: &mut PartnershipAnalyticsScorecardMetadata,
        scorecard_id: u64,
        partnership_id: u64,
        scorecard_type: PartnershipScorecardType,
        scorecard_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(scorecard_id > 0, IndrasError::InvalidInput);
        scorecard.scorecard_id = scorecard_id;
        scorecard.partnership_id = partnership_id;
        scorecard.scorecard_type = scorecard_type;
        scorecard.status = PartnershipScorecardStatus::Generating;
        scorecard.created_at = current_time;
        scorecard.scorecard_data_hash = scorecard_data_hash;
        scorecard.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_scorecard(_scorecard_id: u64) -> Vec<u8> {
        vec![]
    }
}
