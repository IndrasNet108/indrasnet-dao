//! Financial Brand Management module
//!
//! Financial brand management
//!
//! On-chain: Metadata for brand
//! Off-chain: Actual brand, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Brand asset type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialBrandAssetType {
    /// Brand identity
    BrandIdentity,
    /// Brand positioning
    BrandPositioning,
    /// Brand equity
    BrandEquity,
    /// Custom asset
    Custom,
}

/// Brand status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialBrandStatus {
    /// Brand active
    Active,
    /// Brand paused
    Paused,
    /// Brand optimized
    Optimized,
}

/// Financial brand management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialBrandManagementMetadata {
    /// Brand ID
    pub brand_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Brand asset type
    pub brand_asset_type: FinancialBrandAssetType,
    /// Status
    pub status: FinancialBrandStatus,
    /// Created at
    pub created_at: i64,
    /// Brand data hash
    pub brand_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_brand_management(
        brand: &mut FinancialBrandManagementMetadata,
        brand_id: u64,
        entity_id: u64,
        brand_asset_type: FinancialBrandAssetType,
        brand_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(brand_id > 0, IndrasError::InvalidInput);
        brand.brand_id = brand_id;
        brand.entity_id = entity_id;
        brand.brand_asset_type = brand_asset_type;
        brand.status = FinancialBrandStatus::Active;
        brand.created_at = current_time;
        brand.brand_data_hash = brand_data_hash;
        brand.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_brand(_brand_id: u64) -> Vec<u8> {
        vec![]
    }
}
