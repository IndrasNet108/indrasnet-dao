//! Financial Corporate Social Responsibility module
//!
//! Financial CSR management
//!
//! On-chain: Metadata for CSR
//! Off-chain: Actual CSR, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// CSR initiative type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCSRInitiativeType {
    /// Environmental initiative
    Environmental,
    /// Social initiative
    Social,
    /// Community initiative
    Community,
    /// Custom initiative
    Custom,
}

/// CSR status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCSRStatus {
    /// CSR active
    Active,
    /// CSR paused
    Paused,
    /// CSR implemented
    Implemented,
}

/// Financial CSR metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialCorporateSocialResponsibilityMetadata {
    /// CSR ID
    pub csr_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// CSR initiative type
    pub csr_initiative_type: FinancialCSRInitiativeType,
    /// Status
    pub status: FinancialCSRStatus,
    /// Created at
    pub created_at: i64,
    /// CSR data hash
    pub csr_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_corporate_social_responsibility(
        csr: &mut FinancialCorporateSocialResponsibilityMetadata,
        csr_id: u64,
        entity_id: u64,
        csr_initiative_type: FinancialCSRInitiativeType,
        csr_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(csr_id > 0, IndrasError::InvalidInput);
        csr.csr_id = csr_id;
        csr.entity_id = entity_id;
        csr.csr_initiative_type = csr_initiative_type;
        csr.status = FinancialCSRStatus::Active;
        csr.created_at = current_time;
        csr.csr_data_hash = csr_data_hash;
        csr.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_csr(_csr_id: u64) -> Vec<u8> {
        vec![]
    }
}
