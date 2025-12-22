//! Financial Insurance module
//!
//! Financial insurance management
//!
//! On-chain: Metadata for insurance
//! Off-chain: Actual insurance, policy management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Insurance type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialInsuranceType {
    /// Property insurance
    Property,
    /// Liability insurance
    Liability,
    /// Business insurance
    Business,
    /// Custom insurance
    Custom,
}

/// Insurance status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialInsuranceStatus {
    /// Insurance active
    Active,
    /// Insurance expired
    Expired,
    /// Insurance cancelled
    Cancelled,
}

/// Financial insurance metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialInsuranceMetadata {
    /// Insurance ID
    pub insurance_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Insurance type
    pub insurance_type: FinancialInsuranceType,
    /// Status
    pub status: FinancialInsuranceStatus,
    /// Created at
    pub created_at: i64,
    /// Insurance data hash
    pub insurance_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_insurance(
        insurance: &mut FinancialInsuranceMetadata,
        insurance_id: u64,
        entity_id: u64,
        insurance_type: FinancialInsuranceType,
        insurance_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(insurance_id > 0, IndrasError::InvalidInput);
        insurance.insurance_id = insurance_id;
        insurance.entity_id = entity_id;
        insurance.insurance_type = insurance_type;
        insurance.status = FinancialInsuranceStatus::Active;
        insurance.created_at = current_time;
        insurance.insurance_data_hash = insurance_data_hash;
        insurance.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_insurance(_insurance_id: u64) -> Vec<u8> {
        vec![]
    }
}
