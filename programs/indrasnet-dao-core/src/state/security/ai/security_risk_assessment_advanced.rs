//! Security Advanced Risk Assessment module
//!
//! Advanced risk assessment
//!
//! On-chain: Metadata for risk assessments
//! Off-chain: Actual assessment, risk calculation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Risk assessment methodology
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum RiskAssessmentMethodology {
    /// FAIR
    FAIR,
    /// OCTAVE
    OCTAVE,
    /// NIST
    NIST,
    /// Custom methodology
    Custom,
}

/// Risk assessment status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AdvancedRiskAssessmentStatus {
    /// Assessment scheduled
    Scheduled,
    /// Assessment in progress
    InProgress,
    /// Assessment completed
    Completed,
}

/// Security advanced risk assessment metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct SecurityAdvancedRiskAssessmentMetadata {
    /// Assessment ID
    pub assessment_id: u64,
    /// Asset ID
    pub asset_id: u64,
    /// Methodology
    pub methodology: RiskAssessmentMethodology,
    /// Status
    pub status: AdvancedRiskAssessmentStatus,
    /// Created at
    pub created_at: i64,
    /// Completed at
    pub completed_at: Option<i64>,
    /// Assessment data hash
    pub assessment_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_security_advanced_risk_assessment(
        assessment: &mut SecurityAdvancedRiskAssessmentMetadata,
        assessment_id: u64,
        asset_id: u64,
        methodology: RiskAssessmentMethodology,
        assessment_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(assessment_id > 0, IndrasError::InvalidInput);
        assessment.assessment_id = assessment_id;
        assessment.asset_id = asset_id;
        assessment.methodology = methodology;
        assessment.status = AdvancedRiskAssessmentStatus::Scheduled;
        assessment.created_at = current_time;
        assessment.completed_at = None;
        assessment.assessment_data_hash = assessment_data_hash;
        assessment.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn assess_risk(_assessment_id: u64) -> Vec<u8> {
        vec![]
    }
}
