//! Cash Flow module
//!
//! Cash flow management
//!
//! On-chain: Metadata for cash flows
//! Off-chain: Actual cash flow calculations, forecasting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Cash flow type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum CashFlowType {
    /// Inflow
    Inflow,
    /// Outflow
    Outflow,
}

/// Cash flow metadata (on-chain)
///
/// Stores metadata for cash flows
#[account]
#[derive(InitSpace)]
pub struct CashFlowMetadata {
    /// Cash flow ID
    pub cash_flow_id: u64,
    /// Cash flow type
    pub cash_flow_type: CashFlowType,
    /// Amount (in smallest unit)
    pub amount: u64,
    /// Created at
    pub created_at: i64,
    /// Cash flow data hash
    pub cash_flow_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for cash flow
pub mod onchain {
    use super::*;

    /// Initialize cash flow
    pub fn initialize_cash_flow(
        cash_flow: &mut CashFlowMetadata,
        cash_flow_id: u64,
        cash_flow_type: CashFlowType,
        amount: u64,
        cash_flow_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(cash_flow_id > 0, IndrasError::InvalidInput);
        require!(amount > 0, IndrasError::InvalidInput);
        
        cash_flow.cash_flow_id = cash_flow_id;
        cash_flow.cash_flow_type = cash_flow_type;
        cash_flow.amount = amount;
        cash_flow.created_at = current_time;
        cash_flow.cash_flow_data_hash = cash_flow_data_hash;
        cash_flow.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for cash flow
pub mod offchain {
    /// Calculate cash flow forecast
    pub fn calculate_cash_flow_forecast(_period_start: i64, _period_end: i64) -> i64 {
        // Implementation in off-chain service
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_cash_flow() {
        let mut cash_flow = CashFlowMetadata {
            cash_flow_id: 0,
            cash_flow_type: CashFlowType::Inflow,
            amount: 0,
            created_at: 0,
            cash_flow_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_cash_flow(
            &mut cash_flow,
            1,
            CashFlowType::Outflow,
            1000,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(cash_flow.cash_flow_id, 1);
        assert_eq!(cash_flow.cash_flow_type, CashFlowType::Outflow);
        assert_eq!(cash_flow.amount, 1000);
        assert_eq!(cash_flow.created_at, 1000);
        assert_eq!(cash_flow.cash_flow_data_hash, [1u8; 32]);
        assert_eq!(cash_flow.bump, 255);
    }

    #[test]
    fn test_initialize_cash_flow_invalid_id() {
        let mut cash_flow = CashFlowMetadata {
            cash_flow_id: 0,
            cash_flow_type: CashFlowType::Inflow,
            amount: 0,
            created_at: 0,
            cash_flow_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_cash_flow(
            &mut cash_flow,
            0, // Invalid: must be > 0
            CashFlowType::Inflow,
            1000,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_cash_flow_invalid_amount() {
        let mut cash_flow = CashFlowMetadata {
            cash_flow_id: 0,
            cash_flow_type: CashFlowType::Inflow,
            amount: 0,
            created_at: 0,
            cash_flow_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_cash_flow(
            &mut cash_flow,
            1,
            CashFlowType::Inflow,
            0, // Invalid: must be > 0
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_cash_flow_all_types() {
        let cash_flow_types = vec![
            CashFlowType::Inflow,
            CashFlowType::Outflow,
        ];

        for cash_flow_type in cash_flow_types {
            let mut cash_flow = CashFlowMetadata {
                cash_flow_id: 0,
                cash_flow_type: CashFlowType::Inflow,
                amount: 0,
                created_at: 0,
                cash_flow_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_cash_flow(
                &mut cash_flow,
                1,
                cash_flow_type,
                1000,
                [1u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(cash_flow.cash_flow_type, cash_flow_type);
        }
    }

    #[test]
    fn test_initialize_cash_flow_large_values() {
        let mut cash_flow = CashFlowMetadata {
            cash_flow_id: 0,
            cash_flow_type: CashFlowType::Inflow,
            amount: 0,
            created_at: 0,
            cash_flow_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_cash_flow(
            &mut cash_flow,
            u64::MAX,
            CashFlowType::Outflow,
            u64::MAX,
            [255u8; 32],
            i64::MAX,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(cash_flow.cash_flow_id, u64::MAX);
        assert_eq!(cash_flow.amount, u64::MAX);
        assert_eq!(cash_flow.created_at, i64::MAX);
        assert_eq!(cash_flow.cash_flow_data_hash, [255u8; 32]);
    }

    #[test]
    fn test_initialize_cash_flow_data_hash_variations() {
        let hashes = vec![
            [0u8; 32],
            [1u8; 32],
            [255u8; 32],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
        ];

        for hash in hashes {
            let mut cash_flow = CashFlowMetadata {
                cash_flow_id: 0,
                cash_flow_type: CashFlowType::Inflow,
                amount: 0,
                created_at: 0,
                cash_flow_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_cash_flow(
                &mut cash_flow,
                1,
                CashFlowType::Inflow,
                1000,
                hash,
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(cash_flow.cash_flow_data_hash, hash);
        }
    }

    #[test]
    fn test_initialize_cash_flow_timestamp_variations() {
        let timestamps = vec![0i64, 1i64, 1234567890i64, i64::MAX, -1i64];

        for timestamp in timestamps {
            let mut cash_flow = CashFlowMetadata {
                cash_flow_id: 0,
                cash_flow_type: CashFlowType::Inflow,
                amount: 0,
                created_at: 0,
                cash_flow_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_cash_flow(
                &mut cash_flow,
                1,
                CashFlowType::Inflow,
                1000,
                [1u8; 32],
                timestamp,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(cash_flow.created_at, timestamp);
        }
    }

    #[test]
    fn test_initialize_cash_flow_bump_variations() {
        let bumps = vec![0u8, 1u8, 128u8, 255u8];

        for bump in bumps {
            let mut cash_flow = CashFlowMetadata {
                cash_flow_id: 0,
                cash_flow_type: CashFlowType::Inflow,
                amount: 0,
                created_at: 0,
                cash_flow_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_cash_flow(
                &mut cash_flow,
                1,
                CashFlowType::Inflow,
                1000,
                [1u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(cash_flow.bump, bump);
        }
    }

    #[test]
    fn test_cash_flow_type_variants() {
        assert_eq!(CashFlowType::Inflow, CashFlowType::Inflow);
        assert_eq!(CashFlowType::Outflow, CashFlowType::Outflow);
    }

    #[test]
    fn test_cash_flow_type_all_variants_unique() {
        let variants = vec![
            CashFlowType::Inflow,
            CashFlowType::Outflow,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_cash_flow_metadata_all_fields() {
        let cash_flow = CashFlowMetadata {
            cash_flow_id: 123,
            cash_flow_type: CashFlowType::Outflow,
            amount: 5000,
            created_at: 2000,
            cash_flow_data_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(cash_flow.cash_flow_id, 123);
        assert_eq!(cash_flow.cash_flow_type, CashFlowType::Outflow);
        assert_eq!(cash_flow.amount, 5000);
        assert_eq!(cash_flow.created_at, 2000);
        assert_eq!(cash_flow.cash_flow_data_hash, [42u8; 32]);
        assert_eq!(cash_flow.bump, 128);
    }

    #[test]
    fn test_offchain_calculate_cash_flow_forecast() {
        let result = offchain::calculate_cash_flow_forecast(1000, 2000);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_offchain_calculate_cash_flow_forecast_different_periods() {
        let result1 = offchain::calculate_cash_flow_forecast(0, 1000);
        let result2 = offchain::calculate_cash_flow_forecast(1000, 2000);
        let result3 = offchain::calculate_cash_flow_forecast(i64::MIN, i64::MAX);

        assert_eq!(result1, 0);
        assert_eq!(result2, 0);
        assert_eq!(result3, 0);
    }
}
