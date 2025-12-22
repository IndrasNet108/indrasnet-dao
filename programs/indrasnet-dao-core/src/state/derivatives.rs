//! Derivatives module
//!
//! Derivatives management
//!
//! On-chain: Metadata for derivatives
//! Off-chain: Actual derivatives calculations, settlements

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Derivative type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DerivativeType {
    /// Futures
    Futures,
    /// Options
    Options,
    /// Swaps
    Swaps,
}

/// Derivative status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DerivativeStatus {
    /// Derivative active
    Active,
    /// Derivative settled
    Settled,
    /// Derivative expired
    Expired,
}

/// Derivative metadata (on-chain)
///
/// Stores metadata for derivatives
#[account]
#[derive(InitSpace)]
pub struct DerivativeMetadata {
    /// Derivative ID
    pub derivative_id: u64,
    /// Holder pubkey
    pub holder_pubkey: Pubkey,
    /// Derivative type
    pub derivative_type: DerivativeType,
    /// Notional amount (in smallest unit)
    pub notional_amount: u64,
    /// Status
    pub status: DerivativeStatus,
    /// Created at
    pub created_at: i64,
    /// Expiry date
    pub expiry_date: i64,
    /// Derivative data hash
    pub derivative_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for derivatives
pub mod onchain {
    use super::*;

    /// Initialize derivative
    pub fn initialize_derivative(
        derivative: &mut DerivativeMetadata,
        derivative_id: u64,
        holder_pubkey: Pubkey,
        derivative_type: DerivativeType,
        notional_amount: u64,
        derivative_data_hash: [u8; 32],
        expiry_date: i64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(derivative_id > 0, IndrasError::InvalidInput);
        require!(notional_amount > 0, IndrasError::InvalidInput);
        require!(expiry_date > current_time, IndrasError::InvalidInput);
        
        derivative.derivative_id = derivative_id;
        derivative.holder_pubkey = holder_pubkey;
        derivative.derivative_type = derivative_type;
        derivative.notional_amount = notional_amount;
        derivative.status = DerivativeStatus::Active;
        derivative.created_at = current_time;
        derivative.expiry_date = expiry_date;
        derivative.derivative_data_hash = derivative_data_hash;
        derivative.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for derivatives
pub mod offchain {
    /// Calculate derivative value
    pub fn calculate_derivative_value(_derivative_id: u64, _current_time: i64) -> u64 {
        // Implementation in off-chain service
        0
    }
}
