//! Financial Segment Reporting module
//!
//! Financial segment reporting
//!
//! On-chain: Metadata for segment reporting
//! Off-chain: Actual reporting, generation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Segment type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialSegmentType {
    /// Business segment
    Business,
    /// Geographic segment
    Geographic,
    /// Product segment
    Product,
    /// Custom segment
    Custom,
}

/// Reporting status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialSegmentReportingStatus {
    /// Reporting active
    Active,
    /// Reporting paused
    Paused,
    /// Reporting disabled
    Disabled,
}

/// Financial segment reporting metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialSegmentReportingMetadata {
    /// Reporting ID
    pub reporting_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Segment type
    pub segment_type: FinancialSegmentType,
    /// Status
    pub status: FinancialSegmentReportingStatus,
    /// Created at
    pub created_at: i64,
    /// Reporting config hash
    pub reporting_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_segment_reporting(
        reporting: &mut FinancialSegmentReportingMetadata,
        reporting_id: u64,
        entity_id: u64,
        segment_type: FinancialSegmentType,
        reporting_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(reporting_id > 0, IndrasError::InvalidInput);
        reporting.reporting_id = reporting_id;
        reporting.entity_id = entity_id;
        reporting.segment_type = segment_type;
        reporting.status = FinancialSegmentReportingStatus::Active;
        reporting.created_at = current_time;
        reporting.reporting_config_hash = reporting_config_hash;
        reporting.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_segment_report(_reporting_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_segment_reporting() {
        let mut reporting = FinancialSegmentReportingMetadata {
            reporting_id: 0,
            entity_id: 0,
            segment_type: FinancialSegmentType::Business,
            status: FinancialSegmentReportingStatus::Disabled,
            created_at: 0,
            reporting_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_segment_reporting(
            &mut reporting,
            1,
            10,
            FinancialSegmentType::Geographic,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(reporting.reporting_id, 1);
        assert_eq!(reporting.entity_id, 10);
        assert_eq!(reporting.segment_type, FinancialSegmentType::Geographic);
        assert_eq!(reporting.status, FinancialSegmentReportingStatus::Active);
        assert_eq!(reporting.created_at, 1000);
        assert_eq!(reporting.reporting_config_hash, [1u8; 32]);
        assert_eq!(reporting.bump, 255);
    }

    #[test]
    fn test_initialize_financial_segment_reporting_invalid_id() {
        let mut reporting = FinancialSegmentReportingMetadata {
            reporting_id: 0,
            entity_id: 0,
            segment_type: FinancialSegmentType::Business,
            status: FinancialSegmentReportingStatus::Active,
            created_at: 0,
            reporting_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_segment_reporting(
            &mut reporting,
            0, // Invalid: must be > 0
            10,
            FinancialSegmentType::Geographic,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_segment_reporting_all_types() {
        let types = vec![
            FinancialSegmentType::Business,
            FinancialSegmentType::Geographic,
            FinancialSegmentType::Product,
            FinancialSegmentType::Custom,
        ];

        for segment_type in types {
            let mut reporting = FinancialSegmentReportingMetadata {
                reporting_id: 0,
                entity_id: 0,
                segment_type: FinancialSegmentType::Business,
                status: FinancialSegmentReportingStatus::Active,
                created_at: 0,
                reporting_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_segment_reporting(
                &mut reporting,
                1,
                10,
                segment_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(reporting.segment_type, segment_type);
        }
    }

    #[test]
    fn test_financial_segment_type_variants() {
        assert_eq!(FinancialSegmentType::Business, FinancialSegmentType::Business);
        assert_eq!(FinancialSegmentType::Geographic, FinancialSegmentType::Geographic);
        assert_eq!(FinancialSegmentType::Product, FinancialSegmentType::Product);
        assert_eq!(FinancialSegmentType::Custom, FinancialSegmentType::Custom);
    }

    #[test]
    fn test_financial_segment_reporting_status_variants() {
        assert_eq!(FinancialSegmentReportingStatus::Active, FinancialSegmentReportingStatus::Active);
        assert_eq!(FinancialSegmentReportingStatus::Paused, FinancialSegmentReportingStatus::Paused);
        assert_eq!(FinancialSegmentReportingStatus::Disabled, FinancialSegmentReportingStatus::Disabled);
    }

    #[test]
    fn test_financial_segment_type_all_variants_unique() {
        let variants = vec![
            FinancialSegmentType::Business,
            FinancialSegmentType::Geographic,
            FinancialSegmentType::Product,
            FinancialSegmentType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_segment_reporting_status_all_variants_unique() {
        let variants = vec![
            FinancialSegmentReportingStatus::Active,
            FinancialSegmentReportingStatus::Paused,
            FinancialSegmentReportingStatus::Disabled,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_generate_segment_report() {
        let result = offchain::generate_segment_report(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
