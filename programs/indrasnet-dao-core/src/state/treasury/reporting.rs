//! Treasury Reporting
//!
//! Treasury reporting and analytics
//!
//! On-chain: Metadata for reports
//! Off-chain: Actual report generation, analytics

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Report type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TreasuryReportType {
    /// Balance report
    Balance,
    /// Transaction report
    Transaction,
    /// Allocation report
    Allocation,
    /// Performance report
    Performance,
}

/// Treasury report metadata (on-chain)
///
/// Stores metadata for treasury reports
#[account]
#[derive(InitSpace)]
pub struct TreasuryReportMetadata {
    /// Report ID
    pub report_id: u64,
    /// Treasury ID
    pub treasury_id: u64,
    /// Report type
    pub report_type: TreasuryReportType,
    /// Created at
    pub created_at: i64,
    /// Report data hash
    pub report_data_hash: [u8; 32],
    /// Report URI (for off-chain data)
    #[max_len(200)]
    pub report_uri: String,
    /// Bump seed
    pub bump: u8,
}

/// Parameters for initializing report metadata
pub struct InitializeReportMetadataParams {
    pub report_id: u64,
    pub treasury_id: u64,
    pub report_type: TreasuryReportType,
    pub report_data_hash: [u8; 32],
    pub report_uri: String,
    pub current_time: i64,
    pub bump: u8,
}

/// On-chain functions for reporting
pub mod onchain {
    use super::*;

    /// Initialize report metadata
    #[allow(clippy::too_many_arguments)]
    pub fn initialize_report_metadata(
        report: &mut TreasuryReportMetadata,
        report_id: u64,
        treasury_id: u64,
        report_type: TreasuryReportType,
        report_data_hash: [u8; 32],
        report_uri: String,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        let params = InitializeReportMetadataParams {
            report_id,
            treasury_id,
            report_type,
            report_data_hash,
            report_uri,
            current_time,
            bump,
        };
        initialize_report_metadata_with_params(report, params)
    }
    
    /// Initialize report metadata with parameters struct
    pub fn initialize_report_metadata_with_params(
        report: &mut TreasuryReportMetadata,
        params: InitializeReportMetadataParams,
    ) -> Result<()> {
        require!(params.report_id > 0, IndrasError::InvalidInput);
        require!(params.report_uri.len() <= 200, IndrasError::InvalidInput);
        
        report.report_id = params.report_id;
        report.treasury_id = params.treasury_id;
        report.report_type = params.report_type;
        report.created_at = params.current_time;
        report.report_data_hash = params.report_data_hash;
        report.report_uri = params.report_uri;
        report.bump = params.bump;
        
        Ok(())
    }
}

/// Off-chain functions for reporting
///
/// These functions should be implemented in off-chain service
/// for actual report generation.
pub mod offchain {
    // Off-chain functions will be implemented in separate service
    
    /// Generate treasury report
    pub fn generate_report(_treasury_id: u64, _report_type: super::TreasuryReportType) -> Vec<u8> {
        // Implementation in off-chain service
        // Generates detailed treasury report
        vec![]
    }

    /// Analyze treasury data
    pub fn analyze_treasury_data(_treasury_id: u64) -> Vec<String> {
        // Implementation in off-chain service
        // Analyzes treasury data and returns insights
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_report_metadata() {
        let mut report = TreasuryReportMetadata {
            report_id: 0,
            treasury_id: 0,
            report_type: TreasuryReportType::Balance,
            created_at: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };
        
        let result = onchain::initialize_report_metadata(
            &mut report,
            1,
            10,
            TreasuryReportType::Transaction,
            [1u8; 32],
            "ipfs://test".to_string(),
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(report.report_id, 1);
        assert_eq!(report.treasury_id, 10);
        assert_eq!(report.report_type, TreasuryReportType::Transaction);
        assert_eq!(report.created_at, 1000);
        assert_eq!(report.report_uri, "ipfs://test");
        assert_eq!(report.bump, 255);
    }

    #[test]
    fn test_initialize_report_metadata_invalid_id() {
        let mut report = TreasuryReportMetadata {
            report_id: 0,
            treasury_id: 0,
            report_type: TreasuryReportType::Balance,
            created_at: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };
        
        let result = onchain::initialize_report_metadata(
            &mut report,
            0, // Invalid: must be > 0
            10,
            TreasuryReportType::Transaction,
            [1u8; 32],
            "ipfs://test".to_string(),
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_report_metadata_with_params() {
        let mut report = TreasuryReportMetadata {
            report_id: 0,
            treasury_id: 0,
            report_type: TreasuryReportType::Balance,
            created_at: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };
        
        let params = InitializeReportMetadataParams {
            report_id: 1,
            treasury_id: 10,
            report_type: TreasuryReportType::Allocation,
            report_data_hash: [1u8; 32],
            report_uri: "ipfs://test".to_string(),
            current_time: 1000,
            bump: 255,
        };
        
        let result = onchain::initialize_report_metadata_with_params(&mut report, params);
        
        assert!(result.is_ok());
        assert_eq!(report.report_id, 1);
        assert_eq!(report.treasury_id, 10);
        assert_eq!(report.report_type, TreasuryReportType::Allocation);
    }

    #[test]
    fn test_treasury_report_type_variants() {
        assert_eq!(TreasuryReportType::Balance, TreasuryReportType::Balance);
        assert_eq!(TreasuryReportType::Transaction, TreasuryReportType::Transaction);
        assert_eq!(TreasuryReportType::Allocation, TreasuryReportType::Allocation);
        assert_eq!(TreasuryReportType::Performance, TreasuryReportType::Performance);
    }

    #[test]
    fn test_initialize_report_metadata_all_report_types() {
        let report_types = vec![
            TreasuryReportType::Balance,
            TreasuryReportType::Transaction,
            TreasuryReportType::Allocation,
            TreasuryReportType::Performance,
        ];

        for report_type in report_types {
            let mut report = TreasuryReportMetadata {
                report_id: 0,
                treasury_id: 0,
                report_type: TreasuryReportType::Balance,
                created_at: 0,
                report_data_hash: [0u8; 32],
                report_uri: String::new(),
                bump: 0,
            };

            let result = onchain::initialize_report_metadata(
                &mut report,
                1,
                10,
                report_type,
                [1u8; 32],
                "ipfs://test".to_string(),
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(report.report_type, report_type);
        }
    }

    #[test]
    fn test_initialize_report_metadata_uri_too_long() {
        let mut report = TreasuryReportMetadata {
            report_id: 0,
            treasury_id: 0,
            report_type: TreasuryReportType::Balance,
            created_at: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };

        let long_uri = "a".repeat(201); // > 200
        let result = onchain::initialize_report_metadata(
            &mut report,
            1,
            10,
            TreasuryReportType::Balance,
            [1u8; 32],
            long_uri,
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_report_metadata_uri_max_length() {
        let mut report = TreasuryReportMetadata {
            report_id: 0,
            treasury_id: 0,
            report_type: TreasuryReportType::Balance,
            created_at: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };

        let max_uri = "a".repeat(200); // Exactly 200
        let result = onchain::initialize_report_metadata(
            &mut report,
            1,
            10,
            TreasuryReportType::Balance,
            [1u8; 32],
            max_uri.clone(),
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(report.report_uri, max_uri);
    }

    #[test]
    fn test_initialize_report_metadata_empty_uri() {
        let mut report = TreasuryReportMetadata {
            report_id: 0,
            treasury_id: 0,
            report_type: TreasuryReportType::Balance,
            created_at: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };

        let result = onchain::initialize_report_metadata(
            &mut report,
            1,
            10,
            TreasuryReportType::Balance,
            [1u8; 32],
            String::new(),
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(report.report_uri, "");
    }

    #[test]
    fn test_initialize_report_metadata_all_fields() {
        let mut report = TreasuryReportMetadata {
            report_id: 999,
            treasury_id: 888,
            report_type: TreasuryReportType::Performance,
            created_at: 777,
            report_data_hash: [99u8; 32],
            report_uri: "old".to_string(),
            bump: 66,
        };

        let report_id = 1u64;
        let treasury_id = 10u64;
        let report_type = TreasuryReportType::Allocation;
        let report_data_hash = [1u8; 32];
        let report_uri = "ipfs://new".to_string();
        let current_time = 1000i64;
        let bump = 255u8;

        let result = onchain::initialize_report_metadata(
            &mut report,
            report_id,
            treasury_id,
            report_type,
            report_data_hash,
            report_uri.clone(),
            current_time,
            bump,
        );

        assert!(result.is_ok());
        assert_eq!(report.report_id, report_id);
        assert_eq!(report.treasury_id, treasury_id);
        assert_eq!(report.report_type, report_type);
        assert_eq!(report.created_at, current_time);
        assert_eq!(report.report_data_hash, report_data_hash);
        assert_eq!(report.report_uri, report_uri);
        assert_eq!(report.bump, bump);
    }

    #[test]
    fn test_initialize_report_metadata_with_params_uri_validation() {
        let mut report = TreasuryReportMetadata {
            report_id: 0,
            treasury_id: 0,
            report_type: TreasuryReportType::Balance,
            created_at: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };

        let params = InitializeReportMetadataParams {
            report_id: 1,
            treasury_id: 10,
            report_type: TreasuryReportType::Balance,
            report_data_hash: [1u8; 32],
            report_uri: "a".repeat(201), // Too long
            current_time: 1000,
            bump: 255,
        };

        let result = onchain::initialize_report_metadata_with_params(&mut report, params);
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_report_metadata_with_params_valid_uri() {
        let mut report = TreasuryReportMetadata {
            report_id: 0,
            treasury_id: 0,
            report_type: TreasuryReportType::Balance,
            created_at: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };

        let params = InitializeReportMetadataParams {
            report_id: 1,
            treasury_id: 10,
            report_type: TreasuryReportType::Performance,
            report_data_hash: [2u8; 32],
            report_uri: "ipfs://valid".to_string(),
            current_time: 2000,
            bump: 128,
        };

        let result = onchain::initialize_report_metadata_with_params(&mut report, params);
        assert!(result.is_ok());
        assert_eq!(report.report_type, TreasuryReportType::Performance);
        assert_eq!(report.report_data_hash, [2u8; 32]);
        assert_eq!(report.report_uri, "ipfs://valid");
        assert_eq!(report.created_at, 2000);
        assert_eq!(report.bump, 128);
    }

    #[test]
    fn test_initialize_report_metadata_report_id_edge_cases() {
        // Test report_id = 1 (minimum valid)
        let mut report = TreasuryReportMetadata {
            report_id: 0,
            treasury_id: 0,
            report_type: TreasuryReportType::Balance,
            created_at: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };

        let result = onchain::initialize_report_metadata(
            &mut report,
            1,
            10,
            TreasuryReportType::Balance,
            [1u8; 32],
            "ipfs://test".to_string(),
            1000,
            255,
        );
        assert!(result.is_ok());
        assert_eq!(report.report_id, 1);

        // Test report_id = u64::MAX
        let mut report2 = TreasuryReportMetadata {
            report_id: 0,
            treasury_id: 0,
            report_type: TreasuryReportType::Balance,
            created_at: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };

        let result2 = onchain::initialize_report_metadata(
            &mut report2,
            u64::MAX,
            10,
            TreasuryReportType::Balance,
            [1u8; 32],
            "ipfs://test".to_string(),
            1000,
            255,
        );
        assert!(result2.is_ok());
        assert_eq!(report2.report_id, u64::MAX);
    }

    #[test]
    fn test_initialize_report_metadata_treasury_id_variations() {
        let treasury_ids = vec![0u64, 1u64, 100u64, u64::MAX];

        for treasury_id in treasury_ids {
            let mut report = TreasuryReportMetadata {
                report_id: 0,
                treasury_id: 0,
                report_type: TreasuryReportType::Balance,
                created_at: 0,
                report_data_hash: [0u8; 32],
                report_uri: String::new(),
                bump: 0,
            };

            let result = onchain::initialize_report_metadata(
                &mut report,
                1,
                treasury_id,
                TreasuryReportType::Balance,
                [1u8; 32],
                "ipfs://test".to_string(),
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(report.treasury_id, treasury_id);
        }
    }

    #[test]
    fn test_initialize_report_metadata_data_hash_variations() {
        let hashes = vec![
            [0u8; 32],
            [1u8; 32],
            [255u8; 32],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
        ];

        for hash in hashes {
            let mut report = TreasuryReportMetadata {
                report_id: 0,
                treasury_id: 0,
                report_type: TreasuryReportType::Balance,
                created_at: 0,
                report_data_hash: [0u8; 32],
                report_uri: String::new(),
                bump: 0,
            };

            let result = onchain::initialize_report_metadata(
                &mut report,
                1,
                10,
                TreasuryReportType::Balance,
                hash,
                "ipfs://test".to_string(),
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(report.report_data_hash, hash);
        }
    }

    #[test]
    fn test_initialize_report_metadata_timestamp_variations() {
        let timestamps = vec![0i64, 1i64, 1234567890i64, i64::MAX, -1i64];

        for timestamp in timestamps {
            let mut report = TreasuryReportMetadata {
                report_id: 0,
                treasury_id: 0,
                report_type: TreasuryReportType::Balance,
                created_at: 0,
                report_data_hash: [0u8; 32],
                report_uri: String::new(),
                bump: 0,
            };

            let result = onchain::initialize_report_metadata(
                &mut report,
                1,
                10,
                TreasuryReportType::Balance,
                [1u8; 32],
                "ipfs://test".to_string(),
                timestamp,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(report.created_at, timestamp);
        }
    }

    #[test]
    fn test_initialize_report_metadata_bump_variations() {
        let bumps = vec![0u8, 1u8, 128u8, 255u8];

        for bump in bumps {
            let mut report = TreasuryReportMetadata {
                report_id: 0,
                treasury_id: 0,
                report_type: TreasuryReportType::Balance,
                created_at: 0,
                report_data_hash: [0u8; 32],
                report_uri: String::new(),
                bump: 0,
            };

            let result = onchain::initialize_report_metadata(
                &mut report,
                1,
                10,
                TreasuryReportType::Balance,
                [1u8; 32],
                "ipfs://test".to_string(),
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(report.bump, bump);
        }
    }

    #[test]
    fn test_treasury_report_type_all_variants_unique() {
        let variants = vec![
            TreasuryReportType::Balance,
            TreasuryReportType::Transaction,
            TreasuryReportType::Allocation,
            TreasuryReportType::Performance,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_generate_report() {
        let result = offchain::generate_report(1, TreasuryReportType::Balance);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_generate_report_all_types() {
        let report_types = vec![
            TreasuryReportType::Balance,
            TreasuryReportType::Transaction,
            TreasuryReportType::Allocation,
            TreasuryReportType::Performance,
        ];

        for report_type in report_types {
            let result = offchain::generate_report(1, report_type);
            assert_eq!(result, Vec::<u8>::new());
        }
    }

    #[test]
    fn test_offchain_analyze_treasury_data() {
        let result = offchain::analyze_treasury_data(1);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_offchain_analyze_treasury_data_different_ids() {
        let result1 = offchain::analyze_treasury_data(1);
        let result2 = offchain::analyze_treasury_data(999);
        let result3 = offchain::analyze_treasury_data(u64::MAX);

        assert_eq!(result1, Vec::<String>::new());
        assert_eq!(result2, Vec::<String>::new());
        assert_eq!(result3, Vec::<String>::new());
    }
}
