//! Forex module
//!
//! Foreign exchange management
//!
//! On-chain: Metadata for forex pairs
//! Off-chain: Actual forex pricing, trading

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Forex pair status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ForexPairStatus {
    /// Pair active
    Active,
    /// Pair inactive
    Inactive,
}

/// Forex pair metadata (on-chain)
///
/// Stores metadata for forex pairs
#[account]
#[derive(InitSpace)]
pub struct ForexPairMetadata {
    /// Pair ID
    pub pair_id: u64,
    /// Base currency
    #[max_len(10)]
    pub base_currency: String,
    /// Quote currency
    #[max_len(10)]
    pub quote_currency: String,
    /// Exchange rate (scaled)
    pub exchange_rate: u64,
    /// Status
    pub status: ForexPairStatus,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Pair data hash
    pub pair_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for forex
pub mod onchain {
    use super::*;

    /// Initialize forex pair
    pub fn initialize_forex_pair(
        pair: &mut ForexPairMetadata,
        pair_id: u64,
        base_currency: String,
        quote_currency: String,
        exchange_rate: u64,
        pair_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(pair_id > 0, IndrasError::InvalidInput);
        require!(!base_currency.is_empty(), IndrasError::InvalidInput);
        require!(base_currency.len() <= 10, IndrasError::InvalidInput);
        require!(!quote_currency.is_empty(), IndrasError::InvalidInput);
        require!(quote_currency.len() <= 10, IndrasError::InvalidInput);
        require!(exchange_rate > 0, IndrasError::InvalidInput);
        
        pair.pair_id = pair_id;
        pair.base_currency = base_currency;
        pair.quote_currency = quote_currency;
        pair.exchange_rate = exchange_rate;
        pair.status = ForexPairStatus::Active;
        pair.created_at = current_time;
        pair.updated_at = current_time;
        pair.pair_data_hash = pair_data_hash;
        pair.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for forex
pub mod offchain {
    /// Update exchange rate
    pub fn update_exchange_rate(_pair_id: u64, _new_rate: u64) -> bool {
        // Implementation in off-chain service
        false
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
    fn test_initialize_forex_pair() {
        let mut pair = ForexPairMetadata {
            pair_id: 0,
            base_currency: String::new(),
            quote_currency: String::new(),
            exchange_rate: 0,
            status: ForexPairStatus::Inactive,
            created_at: 0,
            updated_at: 0,
            pair_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_forex_pair(
            &mut pair,
            1,
            "USD".to_string(),
            "EUR".to_string(),
            1000000, // 1.0 scaled
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(pair.pair_id, 1);
        assert_eq!(pair.base_currency, "USD");
        assert_eq!(pair.quote_currency, "EUR");
        assert_eq!(pair.exchange_rate, 1000000);
        assert_eq!(pair.status, ForexPairStatus::Active);
        assert_eq!(pair.created_at, 1000);
        assert_eq!(pair.updated_at, 1000);
        assert_eq!(pair.pair_data_hash, [1u8; 32]);
        assert_eq!(pair.bump, 255);
    }

    #[test]
    fn test_initialize_forex_pair_invalid_id() {
        let mut pair = ForexPairMetadata {
            pair_id: 0,
            base_currency: String::new(),
            quote_currency: String::new(),
            exchange_rate: 0,
            status: ForexPairStatus::Inactive,
            created_at: 0,
            updated_at: 0,
            pair_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_forex_pair(
            &mut pair,
            0, // Invalid: must be > 0
            "USD".to_string(),
            "EUR".to_string(),
            1000000,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_forex_pair_empty_base_currency() {
        let mut pair = ForexPairMetadata {
            pair_id: 0,
            base_currency: String::new(),
            quote_currency: String::new(),
            exchange_rate: 0,
            status: ForexPairStatus::Inactive,
            created_at: 0,
            updated_at: 0,
            pair_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_forex_pair(
            &mut pair,
            1,
            String::new(), // Invalid: empty
            "EUR".to_string(),
            1000000,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_forex_pair_empty_quote_currency() {
        let mut pair = ForexPairMetadata {
            pair_id: 0,
            base_currency: String::new(),
            quote_currency: String::new(),
            exchange_rate: 0,
            status: ForexPairStatus::Inactive,
            created_at: 0,
            updated_at: 0,
            pair_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_forex_pair(
            &mut pair,
            1,
            "USD".to_string(),
            String::new(), // Invalid: empty
            1000000,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_forex_pair_too_long_base_currency() {
        let mut pair = ForexPairMetadata {
            pair_id: 0,
            base_currency: String::new(),
            quote_currency: String::new(),
            exchange_rate: 0,
            status: ForexPairStatus::Inactive,
            created_at: 0,
            updated_at: 0,
            pair_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_forex_pair(
            &mut pair,
            1,
            "ABCDEFGHIJK".to_string(), // 11 chars, max is 10
            "EUR".to_string(),
            1000000,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_forex_pair_too_long_quote_currency() {
        let mut pair = ForexPairMetadata {
            pair_id: 0,
            base_currency: String::new(),
            quote_currency: String::new(),
            exchange_rate: 0,
            status: ForexPairStatus::Inactive,
            created_at: 0,
            updated_at: 0,
            pair_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_forex_pair(
            &mut pair,
            1,
            "USD".to_string(),
            "ABCDEFGHIJK".to_string(), // 11 chars, max is 10
            1000000,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_forex_pair_invalid_exchange_rate() {
        let mut pair = ForexPairMetadata {
            pair_id: 0,
            base_currency: String::new(),
            quote_currency: String::new(),
            exchange_rate: 0,
            status: ForexPairStatus::Inactive,
            created_at: 0,
            updated_at: 0,
            pair_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_forex_pair(
            &mut pair,
            1,
            "USD".to_string(),
            "EUR".to_string(),
            0, // Invalid: must be > 0
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_forex_pair_max_length_currencies() {
        let mut pair = ForexPairMetadata {
            pair_id: 0,
            base_currency: String::new(),
            quote_currency: String::new(),
            exchange_rate: 0,
            status: ForexPairStatus::Inactive,
            created_at: 0,
            updated_at: 0,
            pair_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_forex_pair(
            &mut pair,
            1,
            "ABCDEFGHIJ".to_string(), // Exactly 10 chars
            "KLMNOPQRST".to_string(), // Exactly 10 chars
            1000000,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(pair.base_currency.len(), 10);
        assert_eq!(pair.quote_currency.len(), 10);
    }

    #[test]
    fn test_forex_pair_status_variants() {
        assert_eq!(ForexPairStatus::Active, ForexPairStatus::Active);
        assert_eq!(ForexPairStatus::Inactive, ForexPairStatus::Inactive);
    }

    #[test]
    fn test_forex_pair_status_all_variants_unique() {
        let variants = vec![
            ForexPairStatus::Active,
            ForexPairStatus::Inactive,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_initialize_forex_pair_always_active_on_init() {
        let mut pair = ForexPairMetadata {
            pair_id: 0,
            base_currency: String::new(),
            quote_currency: String::new(),
            exchange_rate: 0,
            status: ForexPairStatus::Inactive, // Will be reset
            created_at: 0,
            updated_at: 0,
            pair_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_forex_pair(
            &mut pair,
            1,
            "USD".to_string(),
            "EUR".to_string(),
            1000000,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(pair.status, ForexPairStatus::Active);
    }

    #[test]
    fn test_offchain_update_exchange_rate() {
        let result = offchain::update_exchange_rate(1, 2000000);
        assert_eq!(result, false);
    }
}
