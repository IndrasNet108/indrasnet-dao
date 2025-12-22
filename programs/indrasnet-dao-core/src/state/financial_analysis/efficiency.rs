//! Efficiency Analysis module
//!
//! Efficiency analysis
//!
//! On-chain: Metadata for efficiency analysis
//! Off-chain: Actual analysis, calculation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analysis type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum EfficiencyAnalysisType {
    /// Asset turnover
    AssetTurnover,
    /// Inventory turnover
    InventoryTurnover,
    /// Receivables turnover
    ReceivablesTurnover,
    /// Custom analysis
    Custom,
}

/// Analysis status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum EfficiencyAnalysisStatus {
    /// Analysis pending
    Pending,
    /// Analysis in progress
    InProgress,
    /// Analysis completed
    Completed,
}

/// Efficiency analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct EfficiencyAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Analysis type
    pub analysis_type: EfficiencyAnalysisType,
    /// Status
    pub status: EfficiencyAnalysisStatus,
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
    pub fn initialize_efficiency_analysis(
        analysis: &mut EfficiencyAnalysisMetadata,
        analysis_id: u64,
        entity_id: u64,
        analysis_type: EfficiencyAnalysisType,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.entity_id = entity_id;
        analysis.analysis_type = analysis_type;
        analysis.status = EfficiencyAnalysisStatus::Pending;
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn analyze_efficiency(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_efficiency_analysis() {
        let mut analysis = EfficiencyAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: EfficiencyAnalysisType::AssetTurnover,
            status: EfficiencyAnalysisStatus::Completed,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_efficiency_analysis(
            &mut analysis,
            1,
            10,
            EfficiencyAnalysisType::InventoryTurnover,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analysis.analysis_id, 1);
        assert_eq!(analysis.entity_id, 10);
        assert_eq!(analysis.analysis_type, EfficiencyAnalysisType::InventoryTurnover);
        assert_eq!(analysis.status, EfficiencyAnalysisStatus::Pending);
        assert_eq!(analysis.created_at, 1000);
        assert_eq!(analysis.analysis_data_hash, [1u8; 32]);
        assert_eq!(analysis.bump, 255);
    }

    #[test]
    fn test_initialize_efficiency_analysis_invalid_id() {
        let mut analysis = EfficiencyAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: EfficiencyAnalysisType::AssetTurnover,
            status: EfficiencyAnalysisStatus::Pending,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_efficiency_analysis(
            &mut analysis,
            0, // Invalid: must be > 0
            10,
            EfficiencyAnalysisType::InventoryTurnover,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_efficiency_analysis_all_types() {
        let types = vec![
            EfficiencyAnalysisType::AssetTurnover,
            EfficiencyAnalysisType::InventoryTurnover,
            EfficiencyAnalysisType::ReceivablesTurnover,
            EfficiencyAnalysisType::Custom,
        ];

        for analysis_type in types {
            let mut analysis = EfficiencyAnalysisMetadata {
                analysis_id: 0,
                entity_id: 0,
                analysis_type: EfficiencyAnalysisType::AssetTurnover,
                status: EfficiencyAnalysisStatus::Pending,
                created_at: 0,
                analysis_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_efficiency_analysis(
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
    fn test_efficiency_analysis_type_variants() {
        assert_eq!(EfficiencyAnalysisType::AssetTurnover, EfficiencyAnalysisType::AssetTurnover);
        assert_eq!(EfficiencyAnalysisType::InventoryTurnover, EfficiencyAnalysisType::InventoryTurnover);
        assert_eq!(EfficiencyAnalysisType::ReceivablesTurnover, EfficiencyAnalysisType::ReceivablesTurnover);
        assert_eq!(EfficiencyAnalysisType::Custom, EfficiencyAnalysisType::Custom);
    }

    #[test]
    fn test_efficiency_analysis_status_variants() {
        assert_eq!(EfficiencyAnalysisStatus::Pending, EfficiencyAnalysisStatus::Pending);
        assert_eq!(EfficiencyAnalysisStatus::InProgress, EfficiencyAnalysisStatus::InProgress);
        assert_eq!(EfficiencyAnalysisStatus::Completed, EfficiencyAnalysisStatus::Completed);
    }

    #[test]
    fn test_efficiency_analysis_type_all_variants_unique() {
        let variants = vec![
            EfficiencyAnalysisType::AssetTurnover,
            EfficiencyAnalysisType::InventoryTurnover,
            EfficiencyAnalysisType::ReceivablesTurnover,
            EfficiencyAnalysisType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_efficiency_analysis_status_all_variants_unique() {
        let variants = vec![
            EfficiencyAnalysisStatus::Pending,
            EfficiencyAnalysisStatus::InProgress,
            EfficiencyAnalysisStatus::Completed,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_analyze_efficiency() {
        let result = offchain::analyze_efficiency(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
