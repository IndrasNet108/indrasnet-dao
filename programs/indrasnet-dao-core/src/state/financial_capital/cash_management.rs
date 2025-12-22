//! Financial Cash Management module
//!
//! Financial cash management
//!
//! On-chain: Metadata for cash management
//! Off-chain: Actual management, optimization

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Management strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCashManagementStrategy {
    /// Cash pooling
    CashPooling,
    /// Cash concentration
    CashConcentration,
    /// Cash forecasting
    CashForecasting,
    /// Custom strategy
    Custom,
}

/// Management status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCashManagementStatus {
    /// Management active
    Active,
    /// Management paused
    Paused,
    /// Management optimized
    Optimized,
}

/// Financial cash management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialCashManagementMetadata {
    /// Management ID
    pub management_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Management strategy
    pub management_strategy: FinancialCashManagementStrategy,
    /// Status
    pub status: FinancialCashManagementStatus,
    /// Created at
    pub created_at: i64,
    /// Management config hash
    pub management_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_cash_management(
        management: &mut FinancialCashManagementMetadata,
        management_id: u64,
        entity_id: u64,
        management_strategy: FinancialCashManagementStrategy,
        management_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(management_id > 0, IndrasError::InvalidInput);
        management.management_id = management_id;
        management.entity_id = entity_id;
        management.management_strategy = management_strategy;
        management.status = FinancialCashManagementStatus::Active;
        management.created_at = current_time;
        management.management_config_hash = management_config_hash;
        management.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_cash(_management_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_cash_management() {
        let mut management = FinancialCashManagementMetadata {
            management_id: 0,
            entity_id: 0,
            management_strategy: FinancialCashManagementStrategy::CashPooling,
            status: FinancialCashManagementStatus::Optimized,
            created_at: 0,
            management_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_cash_management(
            &mut management,
            1,
            10,
            FinancialCashManagementStrategy::CashConcentration,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(management.management_id, 1);
        assert_eq!(management.entity_id, 10);
        assert_eq!(management.management_strategy, FinancialCashManagementStrategy::CashConcentration);
        assert_eq!(management.status, FinancialCashManagementStatus::Active);
        assert_eq!(management.created_at, 1000);
        assert_eq!(management.management_config_hash, [1u8; 32]);
        assert_eq!(management.bump, 255);
    }

    #[test]
    fn test_initialize_financial_cash_management_invalid_id() {
        let mut management = FinancialCashManagementMetadata {
            management_id: 0,
            entity_id: 0,
            management_strategy: FinancialCashManagementStrategy::CashPooling,
            status: FinancialCashManagementStatus::Active,
            created_at: 0,
            management_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_cash_management(
            &mut management,
            0, // Invalid: must be > 0
            10,
            FinancialCashManagementStrategy::CashConcentration,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_cash_management_all_strategies() {
        let strategies = vec![
            FinancialCashManagementStrategy::CashPooling,
            FinancialCashManagementStrategy::CashConcentration,
            FinancialCashManagementStrategy::CashForecasting,
            FinancialCashManagementStrategy::Custom,
        ];

        for strategy in strategies {
            let mut management = FinancialCashManagementMetadata {
                management_id: 0,
                entity_id: 0,
                management_strategy: FinancialCashManagementStrategy::CashPooling,
                status: FinancialCashManagementStatus::Active,
                created_at: 0,
                management_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_cash_management(
                &mut management,
                1,
                10,
                strategy,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(management.management_strategy, strategy);
        }
    }

    #[test]
    fn test_financial_cash_management_strategy_variants() {
        assert_eq!(FinancialCashManagementStrategy::CashPooling, FinancialCashManagementStrategy::CashPooling);
        assert_eq!(FinancialCashManagementStrategy::CashConcentration, FinancialCashManagementStrategy::CashConcentration);
        assert_eq!(FinancialCashManagementStrategy::CashForecasting, FinancialCashManagementStrategy::CashForecasting);
        assert_eq!(FinancialCashManagementStrategy::Custom, FinancialCashManagementStrategy::Custom);
    }

    #[test]
    fn test_financial_cash_management_status_variants() {
        assert_eq!(FinancialCashManagementStatus::Active, FinancialCashManagementStatus::Active);
        assert_eq!(FinancialCashManagementStatus::Paused, FinancialCashManagementStatus::Paused);
        assert_eq!(FinancialCashManagementStatus::Optimized, FinancialCashManagementStatus::Optimized);
    }

    #[test]
    fn test_financial_cash_management_strategy_all_variants_unique() {
        let variants = vec![
            FinancialCashManagementStrategy::CashPooling,
            FinancialCashManagementStrategy::CashConcentration,
            FinancialCashManagementStrategy::CashForecasting,
            FinancialCashManagementStrategy::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_cash_management_status_all_variants_unique() {
        let variants = vec![
            FinancialCashManagementStatus::Active,
            FinancialCashManagementStatus::Paused,
            FinancialCashManagementStatus::Optimized,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_manage_cash() {
        let result = offchain::manage_cash(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
