//! Compliance module
//!
//! Partnership compliance management
//!
//! On-chain: Metadata for compliance checks
//! Off-chain: Actual compliance checking, reporting

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

/// Partnership compliance metadata (on-chain)
///
/// Stores metadata for partnership compliance
#[account]
#[derive(InitSpace)]
pub struct PartnershipComplianceMetadata {
    /// Compliance ID
    pub compliance_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Status
    pub status: ComplianceStatus,
    /// Created at
    pub created_at: i64,
    /// Last checked at
    pub last_checked_at: Option<i64>,
    /// Compliance data hash
    pub compliance_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for compliance
pub mod onchain {
    use super::*;

    /// Initialize partnership compliance
    pub fn initialize_partnership_compliance(
        compliance: &mut PartnershipComplianceMetadata,
        compliance_id: u64,
        partnership_id: u64,
        compliance_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(compliance_id > 0, IndrasError::InvalidInput);
        
        compliance.compliance_id = compliance_id;
        compliance.partnership_id = partnership_id;
        compliance.status = ComplianceStatus::UnderReview;
        compliance.created_at = current_time;
        compliance.last_checked_at = None;
        compliance.compliance_data_hash = compliance_data_hash;
        compliance.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for compliance
pub mod offchain {
    /// Check compliance
    pub fn check_compliance(_compliance_id: u64) -> super::ComplianceStatus {
        // Implementation in off-chain service
        super::ComplianceStatus::UnderReview
    }
}
