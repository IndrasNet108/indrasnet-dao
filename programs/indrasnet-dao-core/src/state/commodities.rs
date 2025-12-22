//! Commodities module
//!
//! Commodities trading management
//!
//! On-chain: Metadata for commodities
//! Off-chain: Actual commodities pricing, trading

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Commodity type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum CommodityType {
    /// Gold
    Gold,
    /// Silver
    Silver,
    /// Oil
    Oil,
    /// Custom
    Custom,
}

/// Commodity status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum CommodityStatus {
    /// Commodity active
    Active,
    /// Commodity inactive
    Inactive,
}

/// Commodity metadata (on-chain)
///
/// Stores metadata for commodities
#[account]
#[derive(InitSpace)]
pub struct CommodityMetadata {
    /// Commodity ID
    pub commodity_id: u64,
    /// Commodity type
    pub commodity_type: CommodityType,
    /// Current price (in smallest unit)
    pub current_price: u64,
    /// Status
    pub status: CommodityStatus,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Commodity data hash
    pub commodity_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for commodities
pub mod onchain {
    use super::*;

    /// Initialize commodity
    pub fn initialize_commodity(
        commodity: &mut CommodityMetadata,
        commodity_id: u64,
        commodity_type: CommodityType,
        current_price: u64,
        commodity_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(commodity_id > 0, IndrasError::InvalidInput);
        require!(current_price > 0, IndrasError::InvalidInput);
        
        commodity.commodity_id = commodity_id;
        commodity.commodity_type = commodity_type;
        commodity.current_price = current_price;
        commodity.status = CommodityStatus::Active;
        commodity.created_at = current_time;
        commodity.updated_at = current_time;
        commodity.commodity_data_hash = commodity_data_hash;
        commodity.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for commodities
pub mod offchain {
    /// Update commodity price
    pub fn update_commodity_price(_commodity_id: u64, _new_price: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}
