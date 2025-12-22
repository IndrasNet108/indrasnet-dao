//! AI-powered idea analyzer for off-chain analysis validation
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-ai/src/ai/idea_analyzer.rs

use anchor_lang::prelude::*;
use crate::ai::offchain_analysis::OffChainIdeaAnalysis;
use crate::error::IndrasError;

/// AI-powered idea analyzer for off-chain analysis validation
pub struct IdeaAnalyzer;

impl IdeaAnalyzer {
    /// Validates off-chain AI analysis results
    /// This is the only on-chain function - all AI computation is done off-chain
    pub fn validate_offchain_analysis(
        analysis: &OffChainIdeaAnalysis,
        expected_content_hash: [u8; 32],
    ) -> Result<()> {
        // Validate the off-chain analysis using the secure verification
        Self::validate_analysis_consistency(
            analysis,
            expected_content_hash,
        )?;

        // Additional on-chain validations
        Self::validate_analysis_basic_consistency(analysis)?;
        Self::validate_analysis_freshness(analysis)?;

        Ok(())
    }

    /// Validate analysis consistency with expected parameters
    fn validate_analysis_consistency(
        analysis: &OffChainIdeaAnalysis,
        expected_content_hash: [u8; 32],
    ) -> Result<()> {
        // Validate content hash
        // NOTE: Convert [u8; 32] to hex string for comparison
        let expected_hash_hex = expected_content_hash.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        
        require!(
            analysis.content_hash == expected_hash_hex,
            IndrasError::InvalidInput
        );

        // Validate timestamp is reasonable (not too old, not in future)
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            analysis.analysis_timestamp <= current_time,
            IndrasError::InvalidTimestamp
        );

        // Validate analysis is not too old (max 24 hours)
        require!(
            current_time - analysis.analysis_timestamp <= 86400,
            IndrasError::InvalidTimestamp
        );

        Ok(())
    }

    /// Validate analysis consistency (simplified version for internal use)
    fn validate_analysis_basic_consistency(analysis: &OffChainIdeaAnalysis) -> Result<()> {
        // Basic consistency checks
        require!(
            !analysis.idea_analysis.keywords.is_empty(),
            IndrasError::InvalidInput
        );

        require!(
            analysis.idea_analysis.keywords.len() <= 10,
            IndrasError::DataTooLarge
        );

        Ok(())
    }

    /// Validate analysis freshness
    fn validate_analysis_freshness(analysis: &OffChainIdeaAnalysis) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        
        // Analysis should not be older than 1 hour for real-time decisions
        require!(
            current_time - analysis.analysis_timestamp <= 3600,
            IndrasError::InvalidTimestamp
        );

        Ok(())
    }
}
