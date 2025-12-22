//! Financial Risk Reporting module
//!
//! Financial risk reporting
//!
//! On-chain: Metadata for risk reporting
//! Off-chain: Actual reporting, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Risk report type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialRiskReportType {
    /// Market risk report
    MarketRisk,
    /// Credit risk report
    CreditRisk,
    /// Operational risk report
    OperationalRisk,
    /// Custom report
    Custom,
}

/// Report status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialRiskReportStatus {
    /// Report generating
    Generating,
    /// Report ready
    Ready,
    /// Report published
    Published,
}

/// Financial risk reporting metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialRiskReportingMetadata {
    /// Report ID
    pub report_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Risk report type
    pub risk_report_type: FinancialRiskReportType,
    /// Status
    pub status: FinancialRiskReportStatus,
    /// Created at
    pub created_at: i64,
    /// Report data hash
    pub report_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_risk_reporting(
        report: &mut FinancialRiskReportingMetadata,
        report_id: u64,
        entity_id: u64,
        risk_report_type: FinancialRiskReportType,
        report_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(report_id > 0, IndrasError::InvalidInput);
        report.report_id = report_id;
        report.entity_id = entity_id;
        report.risk_report_type = risk_report_type;
        report.status = FinancialRiskReportStatus::Generating;
        report.created_at = current_time;
        report.report_data_hash = report_data_hash;
        report.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_risk_report(_report_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_risk_reporting() {
        let mut report = FinancialRiskReportingMetadata {
            report_id: 0,
            entity_id: 0,
            risk_report_type: FinancialRiskReportType::MarketRisk,
            status: FinancialRiskReportStatus::Published,
            created_at: 0,
            report_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_risk_reporting(
            &mut report,
            1,
            10,
            FinancialRiskReportType::CreditRisk,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(report.report_id, 1);
        assert_eq!(report.entity_id, 10);
        assert_eq!(report.risk_report_type, FinancialRiskReportType::CreditRisk);
        assert_eq!(report.status, FinancialRiskReportStatus::Generating);
        assert_eq!(report.created_at, 1000);
        assert_eq!(report.report_data_hash, [1u8; 32]);
        assert_eq!(report.bump, 255);
    }

    #[test]
    fn test_initialize_financial_risk_reporting_invalid_id() {
        let mut report = FinancialRiskReportingMetadata {
            report_id: 0,
            entity_id: 0,
            risk_report_type: FinancialRiskReportType::MarketRisk,
            status: FinancialRiskReportStatus::Generating,
            created_at: 0,
            report_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_risk_reporting(
            &mut report,
            0, // Invalid: must be > 0
            10,
            FinancialRiskReportType::CreditRisk,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_risk_reporting_all_types() {
        let types = vec![
            FinancialRiskReportType::MarketRisk,
            FinancialRiskReportType::CreditRisk,
            FinancialRiskReportType::OperationalRisk,
            FinancialRiskReportType::Custom,
        ];

        for risk_report_type in types {
            let mut report = FinancialRiskReportingMetadata {
                report_id: 0,
                entity_id: 0,
                risk_report_type: FinancialRiskReportType::MarketRisk,
                status: FinancialRiskReportStatus::Generating,
                created_at: 0,
                report_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_risk_reporting(
                &mut report,
                1,
                10,
                risk_report_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(report.risk_report_type, risk_report_type);
        }
    }

    #[test]
    fn test_financial_risk_report_type_variants() {
        assert_eq!(FinancialRiskReportType::MarketRisk, FinancialRiskReportType::MarketRisk);
        assert_eq!(FinancialRiskReportType::CreditRisk, FinancialRiskReportType::CreditRisk);
        assert_eq!(FinancialRiskReportType::OperationalRisk, FinancialRiskReportType::OperationalRisk);
        assert_eq!(FinancialRiskReportType::Custom, FinancialRiskReportType::Custom);
    }

    #[test]
    fn test_financial_risk_report_status_variants() {
        assert_eq!(FinancialRiskReportStatus::Generating, FinancialRiskReportStatus::Generating);
        assert_eq!(FinancialRiskReportStatus::Ready, FinancialRiskReportStatus::Ready);
        assert_eq!(FinancialRiskReportStatus::Published, FinancialRiskReportStatus::Published);
    }

    #[test]
    fn test_financial_risk_report_type_all_variants_unique() {
        let variants = vec![
            FinancialRiskReportType::MarketRisk,
            FinancialRiskReportType::CreditRisk,
            FinancialRiskReportType::OperationalRisk,
            FinancialRiskReportType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_risk_report_status_all_variants_unique() {
        let variants = vec![
            FinancialRiskReportStatus::Generating,
            FinancialRiskReportStatus::Ready,
            FinancialRiskReportStatus::Published,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_generate_risk_report() {
        let result = offchain::generate_risk_report(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
