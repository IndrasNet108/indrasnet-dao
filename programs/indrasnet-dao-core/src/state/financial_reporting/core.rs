//! Financial Reporting module
//!
//! Financial reporting management
//!
//! On-chain: Metadata for financial reports
//! Off-chain: Actual report generation, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Report type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialReportType {
    /// Income statement
    IncomeStatement,
    /// Balance sheet
    BalanceSheet,
    /// Cash flow statement
    CashFlowStatement,
    /// Custom report
    Custom,
}

/// Report status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialReportStatus {
    /// Report generating
    Generating,
    /// Report ready
    Ready,
    /// Report published
    Published,
}

/// Financial report metadata (on-chain)
///
/// Stores metadata for financial reports
#[account]
#[derive(InitSpace)]
pub struct FinancialReportMetadata {
    /// Report ID
    pub report_id: u64,
    /// Report type
    pub report_type: FinancialReportType,
    /// Status
    pub status: FinancialReportStatus,
    /// Created at
    pub created_at: i64,
    /// Period start
    pub period_start: i64,
    /// Period end
    pub period_end: i64,
    /// Report data hash
    pub report_data_hash: [u8; 32],
    /// Report URI
    #[max_len(200)]
    pub report_uri: String,
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for financial reporting
pub mod onchain {
    use super::*;

    /// Initialize financial report
    pub fn initialize_financial_report(
        report: &mut FinancialReportMetadata,
        report_id: u64,
        report_type: FinancialReportType,
        report_data_hash: [u8; 32],
        report_uri: String,
        period_start: i64,
        period_end: i64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(report_id > 0, IndrasError::InvalidInput);
        require!(period_end > period_start, IndrasError::InvalidInput);
        require!(report_uri.len() <= 200, IndrasError::InvalidInput);
        
        report.report_id = report_id;
        report.report_type = report_type;
        report.status = FinancialReportStatus::Generating;
        report.created_at = current_time;
        report.period_start = period_start;
        report.period_end = period_end;
        report.report_data_hash = report_data_hash;
        report.report_uri = report_uri;
        report.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for financial reporting
pub mod offchain {
    /// Generate financial report
    pub fn generate_financial_report(_report_id: u64) -> Vec<u8> {
        // Implementation in off-chain service
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_report() {
        let mut report = FinancialReportMetadata {
            report_id: 0,
            report_type: FinancialReportType::IncomeStatement,
            status: FinancialReportStatus::Published,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };
        
        let result = onchain::initialize_financial_report(
            &mut report,
            1,
            FinancialReportType::BalanceSheet,
            [1u8; 32],
            "https://example.com/report".to_string(),
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(report.report_id, 1);
        assert_eq!(report.report_type, FinancialReportType::BalanceSheet);
        assert_eq!(report.status, FinancialReportStatus::Generating);
        assert_eq!(report.created_at, 1500);
        assert_eq!(report.period_start, 1000);
        assert_eq!(report.period_end, 2000);
        assert_eq!(report.report_data_hash, [1u8; 32]);
        assert_eq!(report.report_uri, "https://example.com/report");
        assert_eq!(report.bump, 255);
    }

    #[test]
    fn test_initialize_financial_report_invalid_id() {
        let mut report = FinancialReportMetadata {
            report_id: 0,
            report_type: FinancialReportType::IncomeStatement,
            status: FinancialReportStatus::Generating,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };
        
        let result = onchain::initialize_financial_report(
            &mut report,
            0, // Invalid: must be > 0
            FinancialReportType::BalanceSheet,
            [1u8; 32],
            "https://example.com/report".to_string(),
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_report_invalid_period() {
        let mut report = FinancialReportMetadata {
            report_id: 0,
            report_type: FinancialReportType::IncomeStatement,
            status: FinancialReportStatus::Generating,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };
        
        let result = onchain::initialize_financial_report(
            &mut report,
            1,
            FinancialReportType::BalanceSheet,
            [1u8; 32],
            "https://example.com/report".to_string(),
            2000,
            1000, // Invalid: end <= start
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_report_uri_too_long() {
        let mut report = FinancialReportMetadata {
            report_id: 0,
            report_type: FinancialReportType::IncomeStatement,
            status: FinancialReportStatus::Generating,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };
        
        let long_uri = "a".repeat(201); // 201 chars, max is 200
        let result = onchain::initialize_financial_report(
            &mut report,
            1,
            FinancialReportType::BalanceSheet,
            [1u8; 32],
            long_uri,
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_report_max_uri_length() {
        let mut report = FinancialReportMetadata {
            report_id: 0,
            report_type: FinancialReportType::IncomeStatement,
            status: FinancialReportStatus::Generating,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };
        
        let max_uri = "a".repeat(200); // Exactly 200 chars
        let result = onchain::initialize_financial_report(
            &mut report,
            1,
            FinancialReportType::BalanceSheet,
            [1u8; 32],
            max_uri.clone(),
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(report.report_uri.len(), 200);
    }

    #[test]
    fn test_initialize_financial_report_all_types() {
        let types = vec![
            FinancialReportType::IncomeStatement,
            FinancialReportType::BalanceSheet,
            FinancialReportType::CashFlowStatement,
            FinancialReportType::Custom,
        ];

        for report_type in types {
            let mut report = FinancialReportMetadata {
                report_id: 0,
                report_type: FinancialReportType::IncomeStatement,
                status: FinancialReportStatus::Generating,
                created_at: 0,
                period_start: 0,
                period_end: 0,
                report_data_hash: [0u8; 32],
                report_uri: String::new(),
                bump: 0,
            };

            let result = onchain::initialize_financial_report(
                &mut report,
                1,
                report_type,
                [0u8; 32],
                "https://example.com/report".to_string(),
                1000,
                2000,
                1500,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(report.report_type, report_type);
        }
    }

    #[test]
    fn test_financial_report_type_variants() {
        assert_eq!(FinancialReportType::IncomeStatement, FinancialReportType::IncomeStatement);
        assert_eq!(FinancialReportType::BalanceSheet, FinancialReportType::BalanceSheet);
        assert_eq!(FinancialReportType::CashFlowStatement, FinancialReportType::CashFlowStatement);
        assert_eq!(FinancialReportType::Custom, FinancialReportType::Custom);
    }

    #[test]
    fn test_financial_report_status_variants() {
        assert_eq!(FinancialReportStatus::Generating, FinancialReportStatus::Generating);
        assert_eq!(FinancialReportStatus::Ready, FinancialReportStatus::Ready);
        assert_eq!(FinancialReportStatus::Published, FinancialReportStatus::Published);
    }

    #[test]
    fn test_financial_report_type_all_variants_unique() {
        let variants = vec![
            FinancialReportType::IncomeStatement,
            FinancialReportType::BalanceSheet,
            FinancialReportType::CashFlowStatement,
            FinancialReportType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_report_status_all_variants_unique() {
        let variants = vec![
            FinancialReportStatus::Generating,
            FinancialReportStatus::Ready,
            FinancialReportStatus::Published,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_initialize_financial_report_always_generating_on_init() {
        let mut report = FinancialReportMetadata {
            report_id: 0,
            report_type: FinancialReportType::IncomeStatement,
            status: FinancialReportStatus::Published, // Will be reset
            created_at: 0,
            period_start: 0,
            period_end: 0,
            report_data_hash: [0u8; 32],
            report_uri: String::new(),
            bump: 0,
        };
        
        let result = onchain::initialize_financial_report(
            &mut report,
            1,
            FinancialReportType::BalanceSheet,
            [1u8; 32],
            "https://example.com/report".to_string(),
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(report.status, FinancialReportStatus::Generating);
    }

    #[test]
    fn test_offchain_generate_financial_report() {
        let result = offchain::generate_financial_report(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
