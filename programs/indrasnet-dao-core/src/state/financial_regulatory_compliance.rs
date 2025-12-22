//! Financial Regulatory Compliance module
//!
//! Financial regulatory compliance
//!
//! On-chain: Metadata for regulatory compliance
//! Off-chain: Actual compliance, monitoring

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Regulatory framework
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialRegulatoryFramework {
    /// SEC regulations
    SEC,
    /// FINRA regulations
    FINRA,
    /// CFTC regulations
    CFTC,
    /// Custom framework
    Custom,
}

/// Compliance status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialRegulatoryComplianceStatus {
    /// Compliance active
    Active,
    /// Compliance paused
    Paused,
    /// Compliance non-compliant
    NonCompliant,
}

/// Financial regulatory compliance metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialRegulatoryComplianceMetadata {
    /// Compliance ID
    pub compliance_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Regulatory framework
    pub regulatory_framework: FinancialRegulatoryFramework,
    /// Status
    pub status: FinancialRegulatoryComplianceStatus,
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
    pub fn initialize_financial_regulatory_compliance(
        compliance: &mut FinancialRegulatoryComplianceMetadata,
        compliance_id: u64,
        entity_id: u64,
        regulatory_framework: FinancialRegulatoryFramework,
        compliance_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(compliance_id > 0, IndrasError::InvalidInput);
        compliance.compliance_id = compliance_id;
        compliance.entity_id = entity_id;
        compliance.regulatory_framework = regulatory_framework;
        compliance.status = FinancialRegulatoryComplianceStatus::Active;
        compliance.created_at = current_time;
        compliance.compliance_config_hash = compliance_config_hash;
        compliance.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn monitor_regulatory_compliance(_compliance_id: u64) -> Vec<u8> {
        vec![]
    }
}
