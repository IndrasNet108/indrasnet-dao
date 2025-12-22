//! Reporting module
//!
//! Financial reporting management
//!
//! On-chain: Metadata for reports
//! Off-chain: Actual report generation, delivery

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Report type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ReportType {
    /// Financial report
    Financial,
    /// Performance report
    Performance,
    /// Compliance report
    Compliance,
    /// Custom report
    Custom,
}

/// Report status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ReportStatus {
    /// Report generating
    Generating,
    /// Report ready
    Ready,
    /// Report delivered
    Delivered,
}

/// Report metadata (on-chain)
///
/// Stores metadata for reports
#[account]
#[derive(InitSpace)]
pub struct ReportMetadata {
    /// Report ID
    pub report_id: u64,
    /// Report type
    pub report_type: ReportType,
    /// Status
    pub status: ReportStatus,
    /// Created at
    pub created_at: i64,
    /// Delivered at
    pub delivered_at: Option<i64>,
    /// Report data hash
    pub report_data_hash: [u8; 32],
    /// Report URI
    #[max_len(200)]
    pub report_uri: String,
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for reporting
pub mod onchain {
    use super::*;

    /// Initialize report
    pub fn initialize_report(
        report: &mut ReportMetadata,
        report_id: u64,
        report_type: ReportType,
        report_data_hash: [u8; 32],
        report_uri: String,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(report_id > 0, IndrasError::InvalidInput);
        require!(report_uri.len() <= 200, IndrasError::InvalidInput);
        
        report.report_id = report_id;
        report.report_type = report_type;
        report.status = ReportStatus::Generating;
        report.created_at = current_time;
        report.delivered_at = None;
        report.report_data_hash = report_data_hash;
        report.report_uri = report_uri;
        report.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for reporting
pub mod offchain {
    /// Generate report
    pub fn generate_report(_report_id: u64) -> Vec<u8> {
        // Implementation in off-chain service
        vec![]
    }
}
