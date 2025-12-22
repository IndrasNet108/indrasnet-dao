//! Tax Compliance module
//!
//! Tax compliance management
//!
//! On-chain: Metadata for tax compliance
//! Off-chain: Actual compliance, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Compliance requirement
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TaxComplianceRequirement {
    /// Filing requirement
    Filing,
    /// Payment requirement
    Payment,
    /// Reporting requirement
    Reporting,
    /// Custom requirement
    Custom,
}

/// Compliance status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TaxComplianceStatus {
    /// Compliance active
    Active,
    /// Compliance paused
    Paused,
    /// Compliance non-compliant
    NonCompliant,
}

/// Tax compliance metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct TaxComplianceMetadata {
    /// Compliance ID
    pub compliance_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Compliance requirement
    pub compliance_requirement: TaxComplianceRequirement,
    /// Status
    pub status: TaxComplianceStatus,
    /// Created at
    pub created_at: i64,
    /// Compliance config hash
    pub compliance_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_tax_compliance(
        compliance: &mut TaxComplianceMetadata,
        compliance_id: u64,
        entity_id: u64,
        compliance_requirement: TaxComplianceRequirement,
        compliance_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(compliance_id > 0, IndrasError::InvalidInput);
        compliance.compliance_id = compliance_id;
        compliance.entity_id = entity_id;
        compliance.compliance_requirement = compliance_requirement;
        compliance.status = TaxComplianceStatus::Active;
        compliance.created_at = current_time;
        compliance.compliance_config_hash = compliance_config_hash;
        compliance.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn ensure_tax_compliance(_compliance_id: u64) -> bool {
        false
    }
}
