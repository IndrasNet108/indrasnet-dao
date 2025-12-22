//! Options module
//!
//! Options trading management
//!
//! On-chain: Metadata for options
//! Off-chain: Actual options pricing, settlements

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Option type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum OptionType {
    /// Call option
    Call,
    /// Put option
    Put,
}

/// Option status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum OptionStatus {
    /// Option active
    Active,
    /// Option exercised
    Exercised,
    /// Option expired
    Expired,
}

/// Option metadata (on-chain)
///
/// Stores metadata for options
#[account]
#[derive(InitSpace)]
pub struct OptionMetadata {
    /// Option ID
    pub option_id: u64,
    /// Holder pubkey
    pub holder_pubkey: Pubkey,
    /// Option type
    pub option_type: OptionType,
    /// Strike price (in smallest unit)
    pub strike_price: u64,
    /// Premium (in smallest unit)
    pub premium: u64,
    /// Status
    pub status: OptionStatus,
    /// Created at
    pub created_at: i64,
    /// Expiry date
    pub expiry_date: i64,
    /// Option data hash
    pub option_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for options
pub mod onchain {
    use super::*;

    /// Initialize option
    pub fn initialize_option(
        option: &mut OptionMetadata,
        option_id: u64,
        holder_pubkey: Pubkey,
        option_type: OptionType,
        strike_price: u64,
        premium: u64,
        option_data_hash: [u8; 32],
        expiry_date: i64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(option_id > 0, IndrasError::InvalidInput);
        require!(strike_price > 0, IndrasError::InvalidInput);
        require!(expiry_date > current_time, IndrasError::InvalidInput);
        
        option.option_id = option_id;
        option.holder_pubkey = holder_pubkey;
        option.option_type = option_type;
        option.strike_price = strike_price;
        option.premium = premium;
        option.status = OptionStatus::Active;
        option.created_at = current_time;
        option.expiry_date = expiry_date;
        option.option_data_hash = option_data_hash;
        option.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for options
pub mod offchain {
    /// Calculate option value
    pub fn calculate_option_value(_option_id: u64, _current_price: u64) -> u64 {
        // Implementation in off-chain service
        0
    }
}
