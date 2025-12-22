//! Financial Legal Compliance module
//!
//! Financial legal compliance
//!
//! On-chain: Metadata for legal compliance
//! Off-chain: Actual compliance, monitoring

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Compliance requirement
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialLegalComplianceRequirement {
    /// Regulatory compliance
    Regulatory,
    /// Tax compliance
    Tax,
    /// Contract compliance
    Contract,
    /// Custom requirement
    Custom,
}

/// Compliance status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialLegalComplianceStatus {
    /// Compliance active
    Active,
    /// Compliance paused
    Paused,
    /// Compliance non-compliant
    NonCompliant,
}

/// Financial legal compliance metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialLegalComplianceMetadata {
    /// Compliance ID
    pub compliance_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Compliance requirement
    pub compliance_requirement: FinancialLegalComplianceRequirement,
    /// Status
    pub status: FinancialLegalComplianceStatus,
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
    pub fn initialize_financial_legal_compliance(
        compliance: &mut FinancialLegalComplianceMetadata,
        compliance_id: u64,
        entity_id: u64,
        compliance_requirement: FinancialLegalComplianceRequirement,
        compliance_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(compliance_id > 0, IndrasError::InvalidInput);
        compliance.compliance_id = compliance_id;
        compliance.entity_id = entity_id;
        compliance.compliance_requirement = compliance_requirement;
        compliance.status = FinancialLegalComplianceStatus::Active;
        compliance.created_at = current_time;
        compliance.compliance_config_hash = compliance_config_hash;
        compliance.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn monitor_compliance(_compliance_id: u64) -> Vec<u8> {
        vec![]
    }
}
