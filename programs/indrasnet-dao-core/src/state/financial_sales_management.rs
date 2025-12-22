//! Financial Sales Management module
//!
//! Financial sales management
//!
//! On-chain: Metadata for sales
//! Off-chain: Actual sales, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Sales channel
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialSalesChannel {
    /// Direct sales
    Direct,
    /// Online sales
    Online,
    /// Retail sales
    Retail,
    /// Custom channel
    Custom,
}

/// Sales status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialSalesStatus {
    /// Sales active
    Active,
    /// Sales paused
    Paused,
    /// Sales optimized
    Optimized,
}

/// Financial sales management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialSalesManagementMetadata {
    /// Sales ID
    pub sales_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Sales channel
    pub sales_channel: FinancialSalesChannel,
    /// Status
    pub status: FinancialSalesStatus,
    /// Created at
    pub created_at: i64,
    /// Sales data hash
    pub sales_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_sales_management(
        sales: &mut FinancialSalesManagementMetadata,
        sales_id: u64,
        entity_id: u64,
        sales_channel: FinancialSalesChannel,
        sales_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(sales_id > 0, IndrasError::InvalidInput);
        sales.sales_id = sales_id;
        sales.entity_id = entity_id;
        sales.sales_channel = sales_channel;
        sales.status = FinancialSalesStatus::Active;
        sales.created_at = current_time;
        sales.sales_data_hash = sales_data_hash;
        sales.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_sales(_sales_id: u64) -> Vec<u8> {
        vec![]
    }
}
