//! DeFi Compliance module
//!
//! DeFi compliance and regulation
//!
//! On-chain: Metadata for compliance
//! Off-chain: Actual compliance checking, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Compliance standard
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DeFiComplianceStandard {
    /// KYC/AML
    KYCAML,
    /// MiCA (EU)
    MiCA,
    /// SEC compliance
    SEC,
    /// Custom standard
    Custom,
}

/// Compliance status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DeFiComplianceStatus {
    /// Compliance active
    Active,
    /// Compliance paused
    Paused,
    /// Compliance non-compliant
    NonCompliant,
}

/// DeFi compliance metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct DeFiComplianceMetadata {
    /// Compliance ID
    pub compliance_id: u64,
    /// Protocol ID
    pub protocol_id: u64,
    /// Compliance standard
    pub compliance_standard: DeFiComplianceStandard,
    /// Status
    pub status: DeFiComplianceStatus,
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
    pub fn initialize_defi_compliance(
        compliance: &mut DeFiComplianceMetadata,
        compliance_id: u64,
        protocol_id: u64,
        compliance_standard: DeFiComplianceStandard,
        compliance_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(compliance_id > 0, IndrasError::InvalidInput);
        compliance.compliance_id = compliance_id;
        compliance.protocol_id = protocol_id;
        compliance.compliance_standard = compliance_standard;
        compliance.status = DeFiComplianceStatus::Active;
        compliance.created_at = current_time;
        compliance.compliance_config_hash = compliance_config_hash;
        compliance.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn check_defi_compliance(_compliance_id: u64) -> bool {
        false
    }
}
