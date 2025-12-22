//! Futures module
//!
//! Futures trading management
//!
//! On-chain: Metadata for futures contracts
//! Off-chain: Actual futures pricing, settlements

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Futures status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FuturesStatus {
    /// Futures active
    Active,
    /// Futures settled
    Settled,
    /// Futures expired
    Expired,
}

/// Futures metadata (on-chain)
///
/// Stores metadata for futures contracts
#[account]
#[derive(InitSpace)]
pub struct FuturesMetadata {
    /// Futures ID
    pub futures_id: u64,
    /// Holder pubkey
    pub holder_pubkey: Pubkey,
    /// Contract size (in smallest unit)
    pub contract_size: u64,
    /// Entry price (in smallest unit)
    pub entry_price: u64,
    /// Status
    pub status: FuturesStatus,
    /// Created at
    pub created_at: i64,
    /// Expiry date
    pub expiry_date: i64,
    /// Futures data hash
    pub futures_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for futures
pub mod onchain {
    use super::*;

    /// Initialize futures
    pub fn initialize_futures(
        futures: &mut FuturesMetadata,
        futures_id: u64,
        holder_pubkey: Pubkey,
        contract_size: u64,
        entry_price: u64,
        futures_data_hash: [u8; 32],
        expiry_date: i64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(futures_id > 0, IndrasError::InvalidInput);
        require!(contract_size > 0, IndrasError::InvalidInput);
        require!(expiry_date > current_time, IndrasError::InvalidInput);
        
        futures.futures_id = futures_id;
        futures.holder_pubkey = holder_pubkey;
        futures.contract_size = contract_size;
        futures.entry_price = entry_price;
        futures.status = FuturesStatus::Active;
        futures.created_at = current_time;
        futures.expiry_date = expiry_date;
        futures.futures_data_hash = futures_data_hash;
        futures.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for futures
pub mod offchain {
    /// Calculate futures P&L
    pub fn calculate_futures_pnl(_futures_id: u64, _current_price: u64) -> i64 {
        // Implementation in off-chain service
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    #[test]
    fn test_initialize_futures() {
        let mut futures = FuturesMetadata {
            futures_id: 0,
            holder_pubkey: create_test_pubkey(1),
            contract_size: 0,
            entry_price: 0,
            status: FuturesStatus::Expired,
            created_at: 0,
            expiry_date: 0,
            futures_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let holder = create_test_pubkey(2);
        let result = onchain::initialize_futures(
            &mut futures,
            1,
            holder,
            1000,
            50000,
            [1u8; 32],
            2000,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(futures.futures_id, 1);
        assert_eq!(futures.holder_pubkey, holder);
        assert_eq!(futures.contract_size, 1000);
        assert_eq!(futures.entry_price, 50000);
        assert_eq!(futures.status, FuturesStatus::Active);
        assert_eq!(futures.created_at, 1000);
        assert_eq!(futures.expiry_date, 2000);
        assert_eq!(futures.futures_data_hash, [1u8; 32]);
        assert_eq!(futures.bump, 255);
    }

    #[test]
    fn test_initialize_futures_invalid_id() {
        let mut futures = FuturesMetadata {
            futures_id: 0,
            holder_pubkey: create_test_pubkey(1),
            contract_size: 0,
            entry_price: 0,
            status: FuturesStatus::Active,
            created_at: 0,
            expiry_date: 0,
            futures_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_futures(
            &mut futures,
            0, // Invalid: must be > 0
            create_test_pubkey(2),
            1000,
            50000,
            [1u8; 32],
            2000,
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_futures_invalid_contract_size() {
        let mut futures = FuturesMetadata {
            futures_id: 0,
            holder_pubkey: create_test_pubkey(1),
            contract_size: 0,
            entry_price: 0,
            status: FuturesStatus::Active,
            created_at: 0,
            expiry_date: 0,
            futures_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_futures(
            &mut futures,
            1,
            create_test_pubkey(2),
            0, // Invalid: must be > 0
            50000,
            [1u8; 32],
            2000,
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_futures_invalid_expiry_date() {
        let mut futures = FuturesMetadata {
            futures_id: 0,
            holder_pubkey: create_test_pubkey(1),
            contract_size: 0,
            entry_price: 0,
            status: FuturesStatus::Active,
            created_at: 0,
            expiry_date: 0,
            futures_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_futures(
            &mut futures,
            1,
            create_test_pubkey(2),
            1000,
            50000,
            [1u8; 32],
            1000, // Invalid: expiry_date <= current_time
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_futures_expiry_date_equal_current_time() {
        let mut futures = FuturesMetadata {
            futures_id: 0,
            holder_pubkey: create_test_pubkey(1),
            contract_size: 0,
            entry_price: 0,
            status: FuturesStatus::Active,
            created_at: 0,
            expiry_date: 0,
            futures_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_futures(
            &mut futures,
            1,
            create_test_pubkey(2),
            1000,
            50000,
            [1u8; 32],
            1000, // Invalid: expiry_date == current_time (must be >)
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_futures_always_active_on_init() {
        let mut futures = FuturesMetadata {
            futures_id: 0,
            holder_pubkey: create_test_pubkey(1),
            contract_size: 0,
            entry_price: 0,
            status: FuturesStatus::Expired, // Will be reset
            created_at: 0,
            expiry_date: 0,
            futures_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_futures(
            &mut futures,
            1,
            create_test_pubkey(2),
            1000,
            50000,
            [1u8; 32],
            2000,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(futures.status, FuturesStatus::Active);
    }

    #[test]
    fn test_futures_status_variants() {
        assert_eq!(FuturesStatus::Active, FuturesStatus::Active);
        assert_eq!(FuturesStatus::Settled, FuturesStatus::Settled);
        assert_eq!(FuturesStatus::Expired, FuturesStatus::Expired);
    }

    #[test]
    fn test_futures_status_all_variants_unique() {
        let variants = vec![
            FuturesStatus::Active,
            FuturesStatus::Settled,
            FuturesStatus::Expired,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_initialize_futures_large_values() {
        let mut futures = FuturesMetadata {
            futures_id: 0,
            holder_pubkey: create_test_pubkey(1),
            contract_size: 0,
            entry_price: 0,
            status: FuturesStatus::Active,
            created_at: 0,
            expiry_date: 0,
            futures_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_futures(
            &mut futures,
            u64::MAX,
            create_test_pubkey(2),
            u64::MAX,
            u64::MAX,
            [255u8; 32],
            i64::MAX,
            i64::MAX - 1,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(futures.futures_id, u64::MAX);
        assert_eq!(futures.contract_size, u64::MAX);
        assert_eq!(futures.entry_price, u64::MAX);
        assert_eq!(futures.expiry_date, i64::MAX);
        assert_eq!(futures.created_at, i64::MAX - 1);
    }

    #[test]
    fn test_futures_metadata_all_fields() {
        let holder = create_test_pubkey(5);
        let futures = FuturesMetadata {
            futures_id: 123,
            holder_pubkey: holder,
            contract_size: 5000,
            entry_price: 100000,
            status: FuturesStatus::Settled,
            created_at: 2000,
            expiry_date: 3000,
            futures_data_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(futures.futures_id, 123);
        assert_eq!(futures.holder_pubkey, holder);
        assert_eq!(futures.contract_size, 5000);
        assert_eq!(futures.entry_price, 100000);
        assert_eq!(futures.status, FuturesStatus::Settled);
        assert_eq!(futures.created_at, 2000);
        assert_eq!(futures.expiry_date, 3000);
        assert_eq!(futures.futures_data_hash, [42u8; 32]);
        assert_eq!(futures.bump, 128);
    }

    #[test]
    fn test_offchain_calculate_futures_pnl() {
        let result = offchain::calculate_futures_pnl(1, 60000);
        assert_eq!(result, 0);
    }
}
