//! Financial Marketing Management module
//!
//! Financial marketing management
//!
//! On-chain: Metadata for marketing
//! Off-chain: Actual marketing, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Marketing channel
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialMarketingChannel {
    /// Digital marketing
    Digital,
    /// Traditional marketing
    Traditional,
    /// Social media
    SocialMedia,
    /// Custom channel
    Custom,
}

/// Marketing status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialMarketingStatus {
    /// Marketing active
    Active,
    /// Marketing paused
    Paused,
    /// Marketing optimized
    Optimized,
}

/// Financial marketing management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialMarketingManagementMetadata {
    /// Marketing ID
    pub marketing_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Marketing channel
    pub marketing_channel: FinancialMarketingChannel,
    /// Status
    pub status: FinancialMarketingStatus,
    /// Created at
    pub created_at: i64,
    /// Marketing data hash
    pub marketing_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_marketing_management(
        marketing: &mut FinancialMarketingManagementMetadata,
        marketing_id: u64,
        entity_id: u64,
        marketing_channel: FinancialMarketingChannel,
        marketing_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(marketing_id > 0, IndrasError::InvalidInput);
        marketing.marketing_id = marketing_id;
        marketing.entity_id = entity_id;
        marketing.marketing_channel = marketing_channel;
        marketing.status = FinancialMarketingStatus::Active;
        marketing.created_at = current_time;
        marketing.marketing_data_hash = marketing_data_hash;
        marketing.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_marketing(_marketing_id: u64) -> Vec<u8> {
        vec![]
    }
}
