//! Solvency Analysis module
//!
//! Solvency analysis
//!
//! On-chain: Metadata for solvency analysis
//! Off-chain: Actual analysis, calculation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analysis type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum SolvencyAnalysisType {
    /// Debt-to-equity ratio
    DebtToEquity,
    /// Debt ratio
    DebtRatio,
    /// Equity ratio
    EquityRatio,
    /// Custom analysis
    Custom,
}

/// Analysis status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum SolvencyAnalysisStatus {
    /// Analysis pending
    Pending,
    /// Analysis in progress
    InProgress,
    /// Analysis completed
    Completed,
}

/// Solvency analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct SolvencyAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Analysis type
    pub analysis_type: SolvencyAnalysisType,
    /// Status
    pub status: SolvencyAnalysisStatus,
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
    pub fn initialize_solvency_analysis(
        analysis: &mut SolvencyAnalysisMetadata,
        analysis_id: u64,
        entity_id: u64,
        analysis_type: SolvencyAnalysisType,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.entity_id = entity_id;
        analysis.analysis_type = analysis_type;
        analysis.status = SolvencyAnalysisStatus::Pending;
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn analyze_solvency(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_solvency_analysis() {
        let mut analysis = SolvencyAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: SolvencyAnalysisType::DebtToEquity,
            status: SolvencyAnalysisStatus::Completed,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_solvency_analysis(
            &mut analysis,
            1,
            10,
            SolvencyAnalysisType::DebtRatio,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analysis.analysis_id, 1);
        assert_eq!(analysis.entity_id, 10);
        assert_eq!(analysis.analysis_type, SolvencyAnalysisType::DebtRatio);
        assert_eq!(analysis.status, SolvencyAnalysisStatus::Pending);
        assert_eq!(analysis.created_at, 1000);
        assert_eq!(analysis.analysis_data_hash, [1u8; 32]);
        assert_eq!(analysis.bump, 255);
    }

    #[test]
    fn test_initialize_solvency_analysis_invalid_id() {
        let mut analysis = SolvencyAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: SolvencyAnalysisType::DebtToEquity,
            status: SolvencyAnalysisStatus::Pending,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_solvency_analysis(
            &mut analysis,
            0, // Invalid: must be > 0
            10,
            SolvencyAnalysisType::DebtRatio,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_solvency_analysis_all_types() {
        let types = vec![
            SolvencyAnalysisType::DebtToEquity,
            SolvencyAnalysisType::DebtRatio,
            SolvencyAnalysisType::EquityRatio,
            SolvencyAnalysisType::Custom,
        ];

        for analysis_type in types {
            let mut analysis = SolvencyAnalysisMetadata {
                analysis_id: 0,
                entity_id: 0,
                analysis_type: SolvencyAnalysisType::DebtToEquity,
                status: SolvencyAnalysisStatus::Pending,
                created_at: 0,
                analysis_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_solvency_analysis(
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
    fn test_solvency_analysis_type_variants() {
        assert_eq!(SolvencyAnalysisType::DebtToEquity, SolvencyAnalysisType::DebtToEquity);
        assert_eq!(SolvencyAnalysisType::DebtRatio, SolvencyAnalysisType::DebtRatio);
        assert_eq!(SolvencyAnalysisType::EquityRatio, SolvencyAnalysisType::EquityRatio);
        assert_eq!(SolvencyAnalysisType::Custom, SolvencyAnalysisType::Custom);
    }

    #[test]
    fn test_solvency_analysis_status_variants() {
        assert_eq!(SolvencyAnalysisStatus::Pending, SolvencyAnalysisStatus::Pending);
        assert_eq!(SolvencyAnalysisStatus::InProgress, SolvencyAnalysisStatus::InProgress);
        assert_eq!(SolvencyAnalysisStatus::Completed, SolvencyAnalysisStatus::Completed);
    }

    #[test]
    fn test_solvency_analysis_type_all_variants_unique() {
        let variants = vec![
            SolvencyAnalysisType::DebtToEquity,
            SolvencyAnalysisType::DebtRatio,
            SolvencyAnalysisType::EquityRatio,
            SolvencyAnalysisType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_solvency_analysis_status_all_variants_unique() {
        let variants = vec![
            SolvencyAnalysisStatus::Pending,
            SolvencyAnalysisStatus::InProgress,
            SolvencyAnalysisStatus::Completed,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_analyze_solvency() {
        let result = offchain::analyze_solvency(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
