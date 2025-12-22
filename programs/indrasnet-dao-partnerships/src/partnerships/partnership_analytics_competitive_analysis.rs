//! Partnership Analytics Competitive Analysis module
//!
//! Partnership analytics competitive analysis
//!
//! On-chain: Metadata for competitive analysis
//! Off-chain: Actual analysis, comparison

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analysis scope
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipCompetitiveAnalysisScope {
    /// Direct competitors
    Direct,
    /// Indirect competitors
    Indirect,
    /// Market leaders
    MarketLeaders,
    /// Custom scope
    Custom,
}

/// Analysis status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipCompetitiveAnalysisStatus {
    /// Analysis pending
    Pending,
    /// Analysis in progress
    InProgress,
    /// Analysis completed
    Completed,
}

/// Partnership analytics competitive analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsCompetitiveAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Analysis scope
    pub analysis_scope: PartnershipCompetitiveAnalysisScope,
    /// Status
    pub status: PartnershipCompetitiveAnalysisStatus,
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
    pub fn initialize_partnership_analytics_competitive_analysis(
        analysis: &mut PartnershipAnalyticsCompetitiveAnalysisMetadata,
        analysis_id: u64,
        partnership_id: u64,
        analysis_scope: PartnershipCompetitiveAnalysisScope,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.partnership_id = partnership_id;
        analysis.analysis_scope = analysis_scope;
        analysis.status = PartnershipCompetitiveAnalysisStatus::Pending;
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn perform_competitive_analysis(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }
}
