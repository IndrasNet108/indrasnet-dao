//! Partnership Analytics Funnel Analysis module
//!
//! Partnership analytics funnel analysis
//!
//! On-chain: Metadata for funnel analysis
//! Off-chain: Actual analysis, calculation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Funnel stage
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipFunnelStage {
    /// Awareness stage
    Awareness,
    /// Interest stage
    Interest,
    /// Consideration stage
    Consideration,
    /// Conversion stage
    Conversion,
}

/// Analysis status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipFunnelAnalysisStatus {
    /// Analysis pending
    Pending,
    /// Analysis in progress
    InProgress,
    /// Analysis completed
    Completed,
}

/// Partnership analytics funnel analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsFunnelAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Funnel stage
    pub funnel_stage: PartnershipFunnelStage,
    /// Status
    pub status: PartnershipFunnelAnalysisStatus,
    /// Created at
    pub created_at: i64,
    /// Analysis data hash
    pub analysis_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_partnership_analytics_funnel_analysis(
        analysis: &mut PartnershipAnalyticsFunnelAnalysisMetadata,
        analysis_id: u64,
        partnership_id: u64,
        funnel_stage: PartnershipFunnelStage,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.partnership_id = partnership_id;
        analysis.funnel_stage = funnel_stage;
        analysis.status = PartnershipFunnelAnalysisStatus::Pending;
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn analyze_funnel(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }
}
