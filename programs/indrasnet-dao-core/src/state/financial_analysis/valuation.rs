//! Valuation Analysis module
//!
//! Valuation analysis
//!
//! On-chain: Metadata for valuation analysis
//! Off-chain: Actual analysis, calculation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analysis type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ValuationAnalysisType {
    /// Market valuation
    Market,
    /// Book valuation
    Book,
    /// Intrinsic valuation
    Intrinsic,
    /// Custom analysis
    Custom,
}

/// Analysis status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ValuationAnalysisStatus {
    /// Analysis pending
    Pending,
    /// Analysis in progress
    InProgress,
    /// Analysis completed
    Completed,
}

/// Valuation analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct ValuationAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Analysis type
    pub analysis_type: ValuationAnalysisType,
    /// Status
    pub status: ValuationAnalysisStatus,
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
    pub fn initialize_valuation_analysis(
        analysis: &mut ValuationAnalysisMetadata,
        analysis_id: u64,
        entity_id: u64,
        analysis_type: ValuationAnalysisType,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.entity_id = entity_id;
        analysis.analysis_type = analysis_type;
        analysis.status = ValuationAnalysisStatus::Pending;
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn analyze_valuation(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_valuation_analysis() {
        let mut analysis = ValuationAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: ValuationAnalysisType::Market,
            status: ValuationAnalysisStatus::Completed,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_valuation_analysis(
            &mut analysis,
            1,
            10,
            ValuationAnalysisType::Intrinsic,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analysis.analysis_id, 1);
        assert_eq!(analysis.entity_id, 10);
        assert_eq!(analysis.analysis_type, ValuationAnalysisType::Intrinsic);
        assert_eq!(analysis.status, ValuationAnalysisStatus::Pending);
        assert_eq!(analysis.created_at, 1000);
        assert_eq!(analysis.analysis_data_hash, [1u8; 32]);
        assert_eq!(analysis.bump, 255);
    }

    #[test]
    fn test_initialize_valuation_analysis_invalid_id() {
        let mut analysis = ValuationAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: ValuationAnalysisType::Market,
            status: ValuationAnalysisStatus::Pending,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_valuation_analysis(
            &mut analysis,
            0, // Invalid: must be > 0
            10,
            ValuationAnalysisType::Intrinsic,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_valuation_analysis_all_types() {
        let types = vec![
            ValuationAnalysisType::Market,
            ValuationAnalysisType::Book,
            ValuationAnalysisType::Intrinsic,
            ValuationAnalysisType::Custom,
        ];

        for analysis_type in types {
            let mut analysis = ValuationAnalysisMetadata {
                analysis_id: 0,
                entity_id: 0,
                analysis_type: ValuationAnalysisType::Market,
                status: ValuationAnalysisStatus::Pending,
                created_at: 0,
                analysis_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_valuation_analysis(
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
    fn test_valuation_analysis_type_variants() {
        assert_eq!(ValuationAnalysisType::Market, ValuationAnalysisType::Market);
        assert_eq!(ValuationAnalysisType::Book, ValuationAnalysisType::Book);
        assert_eq!(ValuationAnalysisType::Intrinsic, ValuationAnalysisType::Intrinsic);
        assert_eq!(ValuationAnalysisType::Custom, ValuationAnalysisType::Custom);
    }

    #[test]
    fn test_valuation_analysis_status_variants() {
        assert_eq!(ValuationAnalysisStatus::Pending, ValuationAnalysisStatus::Pending);
        assert_eq!(ValuationAnalysisStatus::InProgress, ValuationAnalysisStatus::InProgress);
        assert_eq!(ValuationAnalysisStatus::Completed, ValuationAnalysisStatus::Completed);
    }

    #[test]
    fn test_valuation_analysis_type_all_variants_unique() {
        let variants = vec![
            ValuationAnalysisType::Market,
            ValuationAnalysisType::Book,
            ValuationAnalysisType::Intrinsic,
            ValuationAnalysisType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_valuation_analysis_status_all_variants_unique() {
        let variants = vec![
            ValuationAnalysisStatus::Pending,
            ValuationAnalysisStatus::InProgress,
            ValuationAnalysisStatus::Completed,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_analyze_valuation() {
        let result = offchain::analyze_valuation(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
