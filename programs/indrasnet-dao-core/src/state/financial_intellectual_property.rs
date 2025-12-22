//! Financial Intellectual Property module
//!
//! Financial IP management
//!
//! On-chain: Metadata for IP
//! Off-chain: Actual IP, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// IP type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialIPType {
    /// Patents
    Patents,
    /// Trademarks
    Trademarks,
    /// Copyrights
    Copyrights,
    /// Trade secrets
    TradeSecrets,
}

/// IP status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialIPStatus {
    /// IP active
    Active,
    /// IP pending
    Pending,
    /// IP expired
    Expired,
}

/// Financial IP metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialIntellectualPropertyMetadata {
    /// IP ID
    pub ip_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// IP type
    pub ip_type: FinancialIPType,
    /// Status
    pub status: FinancialIPStatus,
    /// Created at
    pub created_at: i64,
    /// IP data hash
    pub ip_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_intellectual_property(
        ip: &mut FinancialIntellectualPropertyMetadata,
        ip_id: u64,
        entity_id: u64,
        ip_type: FinancialIPType,
        ip_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(ip_id > 0, IndrasError::InvalidInput);
        ip.ip_id = ip_id;
        ip.entity_id = entity_id;
        ip.ip_type = ip_type;
        ip.status = FinancialIPStatus::Active;
        ip.created_at = current_time;
        ip.ip_data_hash = ip_data_hash;
        ip.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_ip(_ip_id: u64) -> Vec<u8> {
        vec![]
    }
}
