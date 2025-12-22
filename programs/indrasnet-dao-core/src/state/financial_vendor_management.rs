//! Financial Vendor Management module
//!
//! Financial vendor management
//!
//! On-chain: Metadata for vendors
//! Off-chain: Actual vendors, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Vendor type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialVendorType {
    /// Supplier
    Supplier,
    /// Service provider
    ServiceProvider,
    /// Consultant
    Consultant,
    /// Custom vendor
    Custom,
}

/// Vendor status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialVendorStatus {
    /// Vendor active
    Active,
    /// Vendor suspended
    Suspended,
    /// Vendor terminated
    Terminated,
}

/// Financial vendor management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialVendorManagementMetadata {
    /// Vendor ID
    pub vendor_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Vendor type
    pub vendor_type: FinancialVendorType,
    /// Status
    pub status: FinancialVendorStatus,
    /// Created at
    pub created_at: i64,
    /// Vendor data hash
    pub vendor_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_vendor_management(
        vendor: &mut FinancialVendorManagementMetadata,
        vendor_id: u64,
        entity_id: u64,
        vendor_type: FinancialVendorType,
        vendor_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(vendor_id > 0, IndrasError::InvalidInput);
        vendor.vendor_id = vendor_id;
        vendor.entity_id = entity_id;
        vendor.vendor_type = vendor_type;
        vendor.status = FinancialVendorStatus::Active;
        vendor.created_at = current_time;
        vendor.vendor_data_hash = vendor_data_hash;
        vendor.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_vendor(_vendor_id: u64) -> Vec<u8> {
        vec![]
    }
}
