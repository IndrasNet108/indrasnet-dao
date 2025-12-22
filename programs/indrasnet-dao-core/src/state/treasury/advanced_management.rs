//! Advanced Treasury Management
//!
//! Advanced treasury operations and features
//!
//! On-chain: Metadata for advanced operations
//! Off-chain: Actual advanced operations, analytics, optimization

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Treasury strategy type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TreasuryStrategyType {
    /// Conservative strategy
    Conservative,
    /// Balanced strategy
    Balanced,
    /// Aggressive strategy
    Aggressive,
}

/// Treasury allocation metadata (on-chain)
///
/// Stores metadata for treasury allocations
#[account]
#[derive(InitSpace)]
pub struct TreasuryAllocationMetadata {
    /// Allocation ID
    pub allocation_id: u64,
    /// Treasury ID
    pub treasury_id: u64,
    /// Allocated amount
    pub allocated_amount: u64,
    /// Allocated to (Pubkey or category)
    #[max_len(100)]
    pub allocated_to: String,
    /// Allocation type
    #[max_len(50)]
    pub allocation_type: String,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Bump seed
    pub bump: u8,
}

/// Treasury strategy metadata (on-chain)
///
/// Stores metadata for treasury investment strategy
#[account]
#[derive(InitSpace)]
pub struct TreasuryStrategyMetadata {
    /// Strategy ID
    pub strategy_id: u64,
    /// Treasury ID
    pub treasury_id: u64,
    /// Strategy type
    pub strategy_type: TreasuryStrategyType,
    /// Risk level (0-100)
    pub risk_level: u8,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Bump seed
    pub bump: u8,
}

/// Parameters for initializing treasury allocation
pub struct InitializeAllocationParams {
    pub allocation_id: u64,
    pub treasury_id: u64,
    pub allocated_amount: u64,
    pub allocated_to: String,
    pub allocation_type: String,
    pub current_time: i64,
    pub bump: u8,
}

/// On-chain functions for advanced treasury management
pub mod onchain {
    use super::*;

    /// Initialize treasury allocation
    #[allow(clippy::too_many_arguments)]
    pub fn initialize_allocation(
        allocation: &mut TreasuryAllocationMetadata,
        allocation_id: u64,
        treasury_id: u64,
        allocated_amount: u64,
        allocated_to: String,
        allocation_type: String,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        let params = InitializeAllocationParams {
            allocation_id,
            treasury_id,
            allocated_amount,
            allocated_to,
            allocation_type,
            current_time,
            bump,
        };
        initialize_allocation_with_params(allocation, params)
    }
    
    /// Initialize treasury allocation with parameters struct
    pub fn initialize_allocation_with_params(
        allocation: &mut TreasuryAllocationMetadata,
        params: InitializeAllocationParams,
    ) -> Result<()> {
        require!(params.allocation_id > 0, IndrasError::InvalidInput);
        require!(!params.allocated_to.is_empty(), IndrasError::InvalidInput);
        require!(params.allocated_to.len() <= 100, IndrasError::InvalidInput);
        require!(!params.allocation_type.is_empty(), IndrasError::InvalidInput);
        require!(params.allocation_type.len() <= 50, IndrasError::InvalidInput);
        
        allocation.allocation_id = params.allocation_id;
        allocation.treasury_id = params.treasury_id;
        allocation.allocated_amount = params.allocated_amount;
        allocation.allocated_to = params.allocated_to;
        allocation.allocation_type = params.allocation_type;
        allocation.created_at = params.current_time;
        allocation.updated_at = params.current_time;
        allocation.bump = params.bump;
        
        Ok(())
    }

    /// Initialize treasury strategy
    pub fn initialize_strategy(
        strategy: &mut TreasuryStrategyMetadata,
        strategy_id: u64,
        treasury_id: u64,
        strategy_type: TreasuryStrategyType,
        risk_level: u8,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(strategy_id > 0, IndrasError::InvalidInput);
        require!(risk_level <= 100, IndrasError::InvalidInput);
        
        strategy.strategy_id = strategy_id;
        strategy.treasury_id = treasury_id;
        strategy.strategy_type = strategy_type;
        strategy.risk_level = risk_level;
        strategy.created_at = current_time;
        strategy.updated_at = current_time;
        strategy.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for advanced treasury management
///
/// These functions should be implemented in off-chain service
/// for actual advanced operations and optimization.
pub mod offchain {
    // Off-chain functions will be implemented in separate service
    
    /// Optimize treasury allocation
    pub fn optimize_allocation(_treasury_id: u64) -> Vec<u64> {
        // Implementation in off-chain service
        // Optimizes treasury allocation based on strategy
        vec![]
    }

    /// Analyze treasury performance
    pub fn analyze_performance(_treasury_id: u64) -> Vec<String> {
        // Implementation in off-chain service
        // Analyzes treasury performance and returns recommendations
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_allocation() {
        let mut allocation = TreasuryAllocationMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocated_amount: 0,
            allocated_to: String::new(),
            allocation_type: String::new(),
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };
        
        let result = onchain::initialize_allocation(
            &mut allocation,
            1,
            10,
            1000,
            "grant_1".to_string(),
            "grant".to_string(),
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(allocation.allocation_id, 1);
        assert_eq!(allocation.treasury_id, 10);
        assert_eq!(allocation.allocated_amount, 1000);
        assert_eq!(allocation.allocated_to, "grant_1");
        assert_eq!(allocation.allocation_type, "grant");
        assert_eq!(allocation.created_at, 1000);
        assert_eq!(allocation.updated_at, 1000);
        assert_eq!(allocation.bump, 255);
    }

    #[test]
    fn test_initialize_allocation_invalid_id() {
        let mut allocation = TreasuryAllocationMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocated_amount: 0,
            allocated_to: String::new(),
            allocation_type: String::new(),
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };
        
        let result = onchain::initialize_allocation(
            &mut allocation,
            0, // Invalid: must be > 0
            10,
            1000,
            "grant_1".to_string(),
            "grant".to_string(),
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_allocation_with_params() {
        let mut allocation = TreasuryAllocationMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocated_amount: 0,
            allocated_to: String::new(),
            allocation_type: String::new(),
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };
        
        let params = InitializeAllocationParams {
            allocation_id: 1,
            treasury_id: 10,
            allocated_amount: 2000,
            allocated_to: "grant_2".to_string(),
            allocation_type: "grant".to_string(),
            current_time: 1000,
            bump: 255,
        };
        
        let result = onchain::initialize_allocation_with_params(&mut allocation, params);
        
        assert!(result.is_ok());
        assert_eq!(allocation.allocation_id, 1);
        assert_eq!(allocation.allocated_amount, 2000);
    }

    #[test]
    fn test_initialize_strategy() {
        let mut strategy = TreasuryStrategyMetadata {
            strategy_id: 0,
            treasury_id: 0,
            strategy_type: TreasuryStrategyType::Conservative,
            risk_level: 0,
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };
        
        let result = onchain::initialize_strategy(
            &mut strategy,
            1,
            10,
            TreasuryStrategyType::Balanced,
            50,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(strategy.strategy_id, 1);
        assert_eq!(strategy.treasury_id, 10);
        assert_eq!(strategy.strategy_type, TreasuryStrategyType::Balanced);
        assert_eq!(strategy.risk_level, 50);
        assert_eq!(strategy.created_at, 1000);
        assert_eq!(strategy.updated_at, 1000);
        assert_eq!(strategy.bump, 255);
    }

    #[test]
    fn test_initialize_strategy_invalid_id() {
        let mut strategy = TreasuryStrategyMetadata {
            strategy_id: 0,
            treasury_id: 0,
            strategy_type: TreasuryStrategyType::Conservative,
            risk_level: 0,
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };
        
        let result = onchain::initialize_strategy(
            &mut strategy,
            0, // Invalid: must be > 0
            10,
            TreasuryStrategyType::Balanced,
            50,
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_strategy_invalid_risk_level() {
        let mut strategy = TreasuryStrategyMetadata {
            strategy_id: 0,
            treasury_id: 0,
            strategy_type: TreasuryStrategyType::Conservative,
            risk_level: 0,
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };
        
        let result = onchain::initialize_strategy(
            &mut strategy,
            1,
            10,
            TreasuryStrategyType::Balanced,
            101, // Invalid: must be <= 100
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_treasury_strategy_type_variants() {
        assert_eq!(TreasuryStrategyType::Conservative, TreasuryStrategyType::Conservative);
        assert_eq!(TreasuryStrategyType::Balanced, TreasuryStrategyType::Balanced);
        assert_eq!(TreasuryStrategyType::Aggressive, TreasuryStrategyType::Aggressive);
    }

    #[test]
    fn test_initialize_allocation_empty_allocated_to() {
        let mut allocation = TreasuryAllocationMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocated_amount: 0,
            allocated_to: String::new(),
            allocation_type: String::new(),
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let result = onchain::initialize_allocation(
            &mut allocation,
            1,
            10,
            1000,
            String::new(), // Invalid: empty
            "grant".to_string(),
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_allocation_empty_allocation_type() {
        let mut allocation = TreasuryAllocationMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocated_amount: 0,
            allocated_to: String::new(),
            allocation_type: String::new(),
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let result = onchain::initialize_allocation(
            &mut allocation,
            1,
            10,
            1000,
            "grant_1".to_string(),
            String::new(), // Invalid: empty
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_allocation_allocated_to_too_long() {
        let mut allocation = TreasuryAllocationMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocated_amount: 0,
            allocated_to: String::new(),
            allocation_type: String::new(),
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let too_long = "a".repeat(101); // > 100
        let result = onchain::initialize_allocation(
            &mut allocation,
            1,
            10,
            1000,
            too_long,
            "grant".to_string(),
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_allocation_allocated_to_max_length() {
        let mut allocation = TreasuryAllocationMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocated_amount: 0,
            allocated_to: String::new(),
            allocation_type: String::new(),
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let max_length = "a".repeat(100); // Exactly 100
        let result = onchain::initialize_allocation(
            &mut allocation,
            1,
            10,
            1000,
            max_length.clone(),
            "grant".to_string(),
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(allocation.allocated_to, max_length);
    }

    #[test]
    fn test_initialize_allocation_allocation_type_too_long() {
        let mut allocation = TreasuryAllocationMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocated_amount: 0,
            allocated_to: String::new(),
            allocation_type: String::new(),
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let too_long = "a".repeat(51); // > 50
        let result = onchain::initialize_allocation(
            &mut allocation,
            1,
            10,
            1000,
            "grant_1".to_string(),
            too_long,
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_allocation_allocation_type_max_length() {
        let mut allocation = TreasuryAllocationMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocated_amount: 0,
            allocated_to: String::new(),
            allocation_type: String::new(),
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let max_length = "a".repeat(50); // Exactly 50
        let result = onchain::initialize_allocation(
            &mut allocation,
            1,
            10,
            1000,
            "grant_1".to_string(),
            max_length.clone(),
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(allocation.allocation_type, max_length);
    }

    #[test]
    fn test_initialize_allocation_all_fields() {
        let mut allocation = TreasuryAllocationMetadata {
            allocation_id: 999,
            treasury_id: 888,
            allocated_amount: 777,
            allocated_to: "old".to_string(),
            allocation_type: "old_type".to_string(),
            created_at: 666,
            updated_at: 555,
            bump: 44,
        };

        let allocation_id = 1u64;
        let treasury_id = 10u64;
        let allocated_amount = 5000u64;
        let allocated_to = "grant_3".to_string();
        let allocation_type = "grant".to_string();
        let current_time = 2000i64;
        let bump = 128u8;

        let result = onchain::initialize_allocation(
            &mut allocation,
            allocation_id,
            treasury_id,
            allocated_amount,
            allocated_to.clone(),
            allocation_type.clone(),
            current_time,
            bump,
        );

        assert!(result.is_ok());
        assert_eq!(allocation.allocation_id, allocation_id);
        assert_eq!(allocation.treasury_id, treasury_id);
        assert_eq!(allocation.allocated_amount, allocated_amount);
        assert_eq!(allocation.allocated_to, allocated_to);
        assert_eq!(allocation.allocation_type, allocation_type);
        assert_eq!(allocation.created_at, current_time);
        assert_eq!(allocation.updated_at, current_time);
        assert_eq!(allocation.bump, bump);
    }

    #[test]
    fn test_initialize_allocation_with_params_all_validations() {
        // Test empty allocated_to
        let mut allocation1 = TreasuryAllocationMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocated_amount: 0,
            allocated_to: String::new(),
            allocation_type: String::new(),
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let params1 = InitializeAllocationParams {
            allocation_id: 1,
            treasury_id: 10,
            allocated_amount: 1000,
            allocated_to: String::new(), // Invalid
            allocation_type: "grant".to_string(),
            current_time: 1000,
            bump: 255,
        };
        assert!(onchain::initialize_allocation_with_params(&mut allocation1, params1).is_err());

        // Test empty allocation_type
        let mut allocation2 = TreasuryAllocationMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocated_amount: 0,
            allocated_to: String::new(),
            allocation_type: String::new(),
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let params2 = InitializeAllocationParams {
            allocation_id: 1,
            treasury_id: 10,
            allocated_amount: 1000,
            allocated_to: "grant_1".to_string(),
            allocation_type: String::new(), // Invalid
            current_time: 1000,
            bump: 255,
        };
        assert!(onchain::initialize_allocation_with_params(&mut allocation2, params2).is_err());
    }

    #[test]
    fn test_initialize_strategy_all_strategy_types() {
        let strategy_types = vec![
            TreasuryStrategyType::Conservative,
            TreasuryStrategyType::Balanced,
            TreasuryStrategyType::Aggressive,
        ];

        for strategy_type in strategy_types {
            let mut strategy = TreasuryStrategyMetadata {
                strategy_id: 0,
                treasury_id: 0,
                strategy_type: TreasuryStrategyType::Conservative,
                risk_level: 0,
                created_at: 0,
                updated_at: 0,
                bump: 0,
            };

            let result = onchain::initialize_strategy(
                &mut strategy,
                1,
                10,
                strategy_type,
                50,
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(strategy.strategy_type, strategy_type);
        }
    }

    #[test]
    fn test_initialize_strategy_risk_level_boundaries() {
        // Test risk_level = 0
        let mut strategy1 = TreasuryStrategyMetadata {
            strategy_id: 0,
            treasury_id: 0,
            strategy_type: TreasuryStrategyType::Conservative,
            risk_level: 0,
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let result1 = onchain::initialize_strategy(
            &mut strategy1,
            1,
            10,
            TreasuryStrategyType::Conservative,
            0,
            1000,
            255,
        );
        assert!(result1.is_ok());
        assert_eq!(strategy1.risk_level, 0);

        // Test risk_level = 100
        let mut strategy2 = TreasuryStrategyMetadata {
            strategy_id: 0,
            treasury_id: 0,
            strategy_type: TreasuryStrategyType::Aggressive,
            risk_level: 0,
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let result2 = onchain::initialize_strategy(
            &mut strategy2,
            1,
            10,
            TreasuryStrategyType::Aggressive,
            100,
            1000,
            255,
        );
        assert!(result2.is_ok());
        assert_eq!(strategy2.risk_level, 100);

        // Test risk_level = 101 (invalid)
        let mut strategy3 = TreasuryStrategyMetadata {
            strategy_id: 0,
            treasury_id: 0,
            strategy_type: TreasuryStrategyType::Aggressive,
            risk_level: 0,
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let result3 = onchain::initialize_strategy(
            &mut strategy3,
            1,
            10,
            TreasuryStrategyType::Aggressive,
            101, // Invalid
            1000,
            255,
        );
        assert!(result3.is_err());
    }

    #[test]
    fn test_initialize_strategy_all_risk_levels() {
        for risk_level in 0..=100 {
            let mut strategy = TreasuryStrategyMetadata {
                strategy_id: 0,
                treasury_id: 0,
                strategy_type: TreasuryStrategyType::Balanced,
                risk_level: 0,
                created_at: 0,
                updated_at: 0,
                bump: 0,
            };

            let result = onchain::initialize_strategy(
                &mut strategy,
                1,
                10,
                TreasuryStrategyType::Balanced,
                risk_level,
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(strategy.risk_level, risk_level);
        }
    }

    #[test]
    fn test_initialize_strategy_all_fields() {
        let mut strategy = TreasuryStrategyMetadata {
            strategy_id: 999,
            treasury_id: 888,
            strategy_type: TreasuryStrategyType::Conservative,
            risk_level: 77,
            created_at: 666,
            updated_at: 555,
            bump: 44,
        };

        let strategy_id = 1u64;
        let treasury_id = 10u64;
        let strategy_type = TreasuryStrategyType::Aggressive;
        let risk_level = 75u8;
        let current_time = 2000i64;
        let bump = 128u8;

        let result = onchain::initialize_strategy(
            &mut strategy,
            strategy_id,
            treasury_id,
            strategy_type,
            risk_level,
            current_time,
            bump,
        );

        assert!(result.is_ok());
        assert_eq!(strategy.strategy_id, strategy_id);
        assert_eq!(strategy.treasury_id, treasury_id);
        assert_eq!(strategy.strategy_type, strategy_type);
        assert_eq!(strategy.risk_level, risk_level);
        assert_eq!(strategy.created_at, current_time);
        assert_eq!(strategy.updated_at, current_time);
        assert_eq!(strategy.bump, bump);
    }

    #[test]
    fn test_treasury_strategy_type_all_variants_unique() {
        let variants = vec![
            TreasuryStrategyType::Conservative,
            TreasuryStrategyType::Balanced,
            TreasuryStrategyType::Aggressive,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_optimize_allocation() {
        let result = offchain::optimize_allocation(1);
        assert_eq!(result, Vec::<u64>::new());
    }

    #[test]
    fn test_offchain_optimize_allocation_different_ids() {
        let result1 = offchain::optimize_allocation(1);
        let result2 = offchain::optimize_allocation(999);
        let result3 = offchain::optimize_allocation(u64::MAX);

        assert_eq!(result1, Vec::<u64>::new());
        assert_eq!(result2, Vec::<u64>::new());
        assert_eq!(result3, Vec::<u64>::new());
    }

    #[test]
    fn test_offchain_analyze_performance() {
        let result = offchain::analyze_performance(1);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_offchain_analyze_performance_different_ids() {
        let result1 = offchain::analyze_performance(1);
        let result2 = offchain::analyze_performance(999);
        let result3 = offchain::analyze_performance(u64::MAX);

        assert_eq!(result1, Vec::<String>::new());
        assert_eq!(result2, Vec::<String>::new());
        assert_eq!(result3, Vec::<String>::new());
    }
}
