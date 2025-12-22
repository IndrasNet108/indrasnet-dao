//! Cryptocurrency module
//!
//! Cryptocurrency management
//!
//! On-chain: Metadata for cryptocurrencies
//! Off-chain: Actual cryptocurrency pricing, trading

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Cryptocurrency status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum CryptocurrencyStatus {
    /// Cryptocurrency active
    Active,
    /// Cryptocurrency inactive
    Inactive,
}

/// Cryptocurrency metadata (on-chain)
///
/// Stores metadata for cryptocurrencies
#[account]
#[derive(InitSpace)]
pub struct CryptocurrencyMetadata {
    /// Cryptocurrency ID
    pub cryptocurrency_id: u64,
    /// Token mint
    pub token_mint: Pubkey,
    /// Current price (in smallest unit)
    pub current_price: u64,
    /// Status
    pub status: CryptocurrencyStatus,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Cryptocurrency data hash
    pub cryptocurrency_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for cryptocurrency
pub mod onchain {
    use super::*;

    /// Initialize cryptocurrency
    pub fn initialize_cryptocurrency(
        cryptocurrency: &mut CryptocurrencyMetadata,
        cryptocurrency_id: u64,
        token_mint: Pubkey,
        current_price: u64,
        cryptocurrency_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(cryptocurrency_id > 0, IndrasError::InvalidInput);
        require!(current_price > 0, IndrasError::InvalidInput);
        
        cryptocurrency.cryptocurrency_id = cryptocurrency_id;
        cryptocurrency.token_mint = token_mint;
        cryptocurrency.current_price = current_price;
        cryptocurrency.status = CryptocurrencyStatus::Active;
        cryptocurrency.created_at = current_time;
        cryptocurrency.updated_at = current_time;
        cryptocurrency.cryptocurrency_data_hash = cryptocurrency_data_hash;
        cryptocurrency.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for cryptocurrency
pub mod offchain {
    /// Update cryptocurrency price
    pub fn update_cryptocurrency_price(_cryptocurrency_id: u64, _new_price: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}
