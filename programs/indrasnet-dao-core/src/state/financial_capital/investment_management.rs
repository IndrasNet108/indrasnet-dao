//! Financial Investment Management module
//!
//! Financial investment management
//!
//! On-chain: Metadata for investment management
//! Off-chain: Actual management, optimization

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Investment strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialInvestmentStrategy {
    /// Growth strategy
    Growth,
    /// Value strategy
    Value,
    /// Income strategy
    Income,
    /// Custom strategy
    Custom,
}

/// Management status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialInvestmentManagementStatus {
    /// Management active
    Active,
    /// Management paused
    Paused,
    /// Management optimized
    Optimized,
}

/// Financial investment management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialInvestmentManagementMetadata {
    /// Management ID
    pub management_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Investment strategy
    pub investment_strategy: FinancialInvestmentStrategy,
    /// Status
    pub status: FinancialInvestmentManagementStatus,
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
    pub fn initialize_financial_investment_management(
        management: &mut FinancialInvestmentManagementMetadata,
        management_id: u64,
        entity_id: u64,
        investment_strategy: FinancialInvestmentStrategy,
        management_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(management_id > 0, IndrasError::InvalidInput);
        management.management_id = management_id;
        management.entity_id = entity_id;
        management.investment_strategy = investment_strategy;
        management.status = FinancialInvestmentManagementStatus::Active;
        management.created_at = current_time;
        management.management_config_hash = management_config_hash;
        management.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_investment(_management_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_investment_management() {
        let mut management = FinancialInvestmentManagementMetadata {
            management_id: 0,
            entity_id: 0,
            investment_strategy: FinancialInvestmentStrategy::Growth,
            status: FinancialInvestmentManagementStatus::Optimized,
            created_at: 0,
            management_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_investment_management(
            &mut management,
            1,
            10,
            FinancialInvestmentStrategy::Value,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(management.management_id, 1);
        assert_eq!(management.entity_id, 10);
        assert_eq!(management.investment_strategy, FinancialInvestmentStrategy::Value);
        assert_eq!(management.status, FinancialInvestmentManagementStatus::Active);
        assert_eq!(management.created_at, 1000);
        assert_eq!(management.management_config_hash, [1u8; 32]);
        assert_eq!(management.bump, 255);
    }

    #[test]
    fn test_initialize_financial_investment_management_invalid_id() {
        let mut management = FinancialInvestmentManagementMetadata {
            management_id: 0,
            entity_id: 0,
            investment_strategy: FinancialInvestmentStrategy::Growth,
            status: FinancialInvestmentManagementStatus::Active,
            created_at: 0,
            management_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_investment_management(
            &mut management,
            0, // Invalid: must be > 0
            10,
            FinancialInvestmentStrategy::Value,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_investment_management_all_strategies() {
        let strategies = vec![
            FinancialInvestmentStrategy::Growth,
            FinancialInvestmentStrategy::Value,
            FinancialInvestmentStrategy::Income,
            FinancialInvestmentStrategy::Custom,
        ];

        for strategy in strategies {
            let mut management = FinancialInvestmentManagementMetadata {
                management_id: 0,
                entity_id: 0,
                investment_strategy: FinancialInvestmentStrategy::Growth,
                status: FinancialInvestmentManagementStatus::Active,
                created_at: 0,
                management_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_investment_management(
                &mut management,
                1,
                10,
                strategy,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(management.investment_strategy, strategy);
        }
    }

    #[test]
    fn test_financial_investment_strategy_variants() {
        assert_eq!(FinancialInvestmentStrategy::Growth, FinancialInvestmentStrategy::Growth);
        assert_eq!(FinancialInvestmentStrategy::Value, FinancialInvestmentStrategy::Value);
        assert_eq!(FinancialInvestmentStrategy::Income, FinancialInvestmentStrategy::Income);
        assert_eq!(FinancialInvestmentStrategy::Custom, FinancialInvestmentStrategy::Custom);
    }

    #[test]
    fn test_financial_investment_management_status_variants() {
        assert_eq!(FinancialInvestmentManagementStatus::Active, FinancialInvestmentManagementStatus::Active);
        assert_eq!(FinancialInvestmentManagementStatus::Paused, FinancialInvestmentManagementStatus::Paused);
        assert_eq!(FinancialInvestmentManagementStatus::Optimized, FinancialInvestmentManagementStatus::Optimized);
    }

    #[test]
    fn test_financial_investment_strategy_all_variants_unique() {
        let variants = vec![
            FinancialInvestmentStrategy::Growth,
            FinancialInvestmentStrategy::Value,
            FinancialInvestmentStrategy::Income,
            FinancialInvestmentStrategy::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_investment_management_status_all_variants_unique() {
        let variants = vec![
            FinancialInvestmentManagementStatus::Active,
            FinancialInvestmentManagementStatus::Paused,
            FinancialInvestmentManagementStatus::Optimized,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_manage_investment() {
        let result = offchain::manage_investment(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
