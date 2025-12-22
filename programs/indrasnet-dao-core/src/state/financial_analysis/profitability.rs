//! Profitability Analysis module
//!
//! Profitability analysis
//!
//! On-chain: Metadata for profitability analysis
//! Off-chain: Actual analysis, calculation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analysis type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ProfitabilityAnalysisType {
    /// Gross margin analysis
    GrossMargin,
    /// Net margin analysis
    NetMargin,
    /// ROI analysis
    ROI,
    /// Custom analysis
    Custom,
}

/// Analysis status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ProfitabilityAnalysisStatus {
    /// Analysis pending
    Pending,
    /// Analysis in progress
    InProgress,
    /// Analysis completed
    Completed,
}

/// Profitability analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct ProfitabilityAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Analysis type
    pub analysis_type: ProfitabilityAnalysisType,
    /// Status
    pub status: ProfitabilityAnalysisStatus,
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
    pub fn initialize_profitability_analysis(
        analysis: &mut ProfitabilityAnalysisMetadata,
        analysis_id: u64,
        entity_id: u64,
        analysis_type: ProfitabilityAnalysisType,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.entity_id = entity_id;
        analysis.analysis_type = analysis_type;
        analysis.status = ProfitabilityAnalysisStatus::Pending;
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn analyze_profitability(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_profitability_analysis() {
        let mut analysis = ProfitabilityAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: ProfitabilityAnalysisType::GrossMargin,
            status: ProfitabilityAnalysisStatus::Completed,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_profitability_analysis(
            &mut analysis,
            1,
            10,
            ProfitabilityAnalysisType::ROI,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analysis.analysis_id, 1);
        assert_eq!(analysis.entity_id, 10);
        assert_eq!(analysis.analysis_type, ProfitabilityAnalysisType::ROI);
        assert_eq!(analysis.status, ProfitabilityAnalysisStatus::Pending);
        assert_eq!(analysis.created_at, 1000);
        assert_eq!(analysis.analysis_data_hash, [1u8; 32]);
        assert_eq!(analysis.bump, 255);
    }

    #[test]
    fn test_initialize_profitability_analysis_invalid_id() {
        let mut analysis = ProfitabilityAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: ProfitabilityAnalysisType::GrossMargin,
            status: ProfitabilityAnalysisStatus::Pending,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_profitability_analysis(
            &mut analysis,
            0, // Invalid: must be > 0
            10,
            ProfitabilityAnalysisType::ROI,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_profitability_analysis_all_types() {
        let types = vec![
            ProfitabilityAnalysisType::GrossMargin,
            ProfitabilityAnalysisType::NetMargin,
            ProfitabilityAnalysisType::ROI,
            ProfitabilityAnalysisType::Custom,
        ];

        for analysis_type in types {
            let mut analysis = ProfitabilityAnalysisMetadata {
                analysis_id: 0,
                entity_id: 0,
                analysis_type: ProfitabilityAnalysisType::GrossMargin,
                status: ProfitabilityAnalysisStatus::Pending,
                created_at: 0,
                analysis_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_profitability_analysis(
                &mut analysis,
                1,
                10,
                analysis_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(analysis.analysis_type, analysis_type);
        }
    }

    #[test]
    fn test_profitability_analysis_type_variants() {
        assert_eq!(ProfitabilityAnalysisType::GrossMargin, ProfitabilityAnalysisType::GrossMargin);
        assert_eq!(ProfitabilityAnalysisType::NetMargin, ProfitabilityAnalysisType::NetMargin);
        assert_eq!(ProfitabilityAnalysisType::ROI, ProfitabilityAnalysisType::ROI);
        assert_eq!(ProfitabilityAnalysisType::Custom, ProfitabilityAnalysisType::Custom);
    }

    #[test]
    fn test_profitability_analysis_status_variants() {
        assert_eq!(ProfitabilityAnalysisStatus::Pending, ProfitabilityAnalysisStatus::Pending);
        assert_eq!(ProfitabilityAnalysisStatus::InProgress, ProfitabilityAnalysisStatus::InProgress);
        assert_eq!(ProfitabilityAnalysisStatus::Completed, ProfitabilityAnalysisStatus::Completed);
    }

    #[test]
    fn test_profitability_analysis_type_all_variants_unique() {
        let variants = vec![
            ProfitabilityAnalysisType::GrossMargin,
            ProfitabilityAnalysisType::NetMargin,
            ProfitabilityAnalysisType::ROI,
            ProfitabilityAnalysisType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_profitability_analysis_status_all_variants_unique() {
        let variants = vec![
            ProfitabilityAnalysisStatus::Pending,
            ProfitabilityAnalysisStatus::InProgress,
            ProfitabilityAnalysisStatus::Completed,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_analyze_profitability() {
        let result = offchain::analyze_profitability(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
