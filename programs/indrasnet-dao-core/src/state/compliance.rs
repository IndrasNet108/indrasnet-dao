//! Compliance module
//!
//! Compliance management
//!
//! On-chain: Metadata for compliance checks
//! Off-chain: Actual compliance verification, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Compliance status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ComplianceStatus {
    /// Compliance compliant
    Compliant,
    /// Compliance non-compliant
    NonCompliant,
    /// Compliance under review
    UnderReview,
}

/// Compliance check metadata (on-chain)
///
/// Stores metadata for compliance checks
#[account]
#[derive(InitSpace)]
pub struct ComplianceCheckMetadata {
    /// Check ID
    pub check_id: u64,
    /// Compliance standard
    #[max_len(100)]
    pub compliance_standard: String,
    /// Status
    pub status: ComplianceStatus,
    /// Created at
    pub created_at: i64,
    /// Last checked at
    pub last_checked_at: Option<i64>,
    /// Check data hash
    pub check_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for compliance
pub mod onchain {
    use super::*;

    /// Initialize compliance check
    pub fn initialize_compliance_check(
        check: &mut ComplianceCheckMetadata,
        check_id: u64,
        compliance_standard: String,
        check_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(check_id > 0, IndrasError::InvalidInput);
        require!(!compliance_standard.is_empty(), IndrasError::InvalidInput);
        require!(compliance_standard.len() <= 100, IndrasError::InvalidInput);
        
        check.check_id = check_id;
        check.compliance_standard = compliance_standard;
        check.status = ComplianceStatus::UnderReview;
        check.created_at = current_time;
        check.last_checked_at = None;
        check.check_data_hash = check_data_hash;
        check.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for compliance
pub mod offchain {
    /// Verify compliance
    pub fn verify_compliance(_check_id: u64) -> super::ComplianceStatus {
        // Implementation in off-chain service
        super::ComplianceStatus::UnderReview
    }
}
