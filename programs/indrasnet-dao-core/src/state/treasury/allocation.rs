//! Treasury Allocation module
//!
//! Treasury allocation management
//!
//! On-chain: Metadata for treasury allocation
//! Off-chain: Actual allocation, distribution
//!
//! NOTE: This module provides allocation strategy-based metadata.
//! For detailed allocation with amounts, see advanced_management.rs

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Allocation strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TreasuryAllocationStrategy {
    /// Equal allocation
    Equal,
    /// Proportional allocation
    Proportional,
    /// Priority-based allocation
    PriorityBased,
    /// Custom strategy
    Custom,
}

/// Allocation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TreasuryAllocationStatus {
    /// Allocation active
    Active,
    /// Allocation paused
    Paused,
    /// Allocation completed
    Completed,
}

/// Treasury allocation strategy metadata (on-chain)
///
/// This is for strategy-based allocation tracking.
/// For detailed allocation with amounts, see advanced_management::TreasuryAllocationMetadata
#[account]
#[derive(InitSpace)]
pub struct TreasuryAllocationStrategyMetadata {
    /// Allocation ID
    pub allocation_id: u64,
    /// Treasury ID
    pub treasury_id: u64,
    /// Allocation strategy
    pub allocation_strategy: TreasuryAllocationStrategy,
    /// Status
    pub status: TreasuryAllocationStatus,
    /// Created at
    pub created_at: i64,
    /// Allocation config hash
    pub allocation_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    
    pub fn initialize_treasury_allocation(
        allocation: &mut TreasuryAllocationStrategyMetadata,
        allocation_id: u64,
        treasury_id: u64,
        allocation_strategy: TreasuryAllocationStrategy,
        allocation_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(allocation_id > 0, IndrasError::InvalidInput);
        allocation.allocation_id = allocation_id;
        allocation.treasury_id = treasury_id;
        allocation.allocation_strategy = allocation_strategy;
        allocation.status = TreasuryAllocationStatus::Active;
        allocation.created_at = current_time;
        allocation.allocation_config_hash = allocation_config_hash;
        allocation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn allocate_treasury(_allocation_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_treasury_allocation() {
        let mut allocation = TreasuryAllocationStrategyMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocation_strategy: TreasuryAllocationStrategy::Equal,
            status: TreasuryAllocationStatus::Completed,
            created_at: 0,
            allocation_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_allocation(
            &mut allocation,
            1,
            10,
            TreasuryAllocationStrategy::PriorityBased,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(allocation.allocation_id, 1);
        assert_eq!(allocation.treasury_id, 10);
        assert_eq!(allocation.allocation_strategy, TreasuryAllocationStrategy::PriorityBased);
        assert_eq!(allocation.status, TreasuryAllocationStatus::Active);
        assert_eq!(allocation.created_at, 1000);
        assert_eq!(allocation.bump, 255);
    }

    #[test]
    fn test_initialize_treasury_allocation_invalid_id() {
        let mut allocation = TreasuryAllocationStrategyMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocation_strategy: TreasuryAllocationStrategy::Equal,
            status: TreasuryAllocationStatus::Completed,
            created_at: 0,
            allocation_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_allocation(
            &mut allocation,
            0, // Invalid: must be > 0
            10,
            TreasuryAllocationStrategy::PriorityBased,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_treasury_allocation_strategy_variants() {
        assert_eq!(TreasuryAllocationStrategy::Equal, TreasuryAllocationStrategy::Equal);
        assert_eq!(TreasuryAllocationStrategy::Proportional, TreasuryAllocationStrategy::Proportional);
        assert_eq!(TreasuryAllocationStrategy::PriorityBased, TreasuryAllocationStrategy::PriorityBased);
        assert_eq!(TreasuryAllocationStrategy::Custom, TreasuryAllocationStrategy::Custom);
    }

    #[test]
    fn test_treasury_allocation_status_variants() {
        assert_eq!(TreasuryAllocationStatus::Active, TreasuryAllocationStatus::Active);
        assert_eq!(TreasuryAllocationStatus::Paused, TreasuryAllocationStatus::Paused);
        assert_eq!(TreasuryAllocationStatus::Completed, TreasuryAllocationStatus::Completed);
    }

    #[test]
    fn test_treasury_allocation_strategy_all_variants_unique() {
        let strategies = vec![
            TreasuryAllocationStrategy::Equal,
            TreasuryAllocationStrategy::Proportional,
            TreasuryAllocationStrategy::PriorityBased,
            TreasuryAllocationStrategy::Custom,
        ];
        
        for i in 0..strategies.len() {
            for j in (i + 1)..strategies.len() {
                assert_ne!(strategies[i], strategies[j], "Duplicate strategy found");
            }
        }
    }

    #[test]
    fn test_treasury_allocation_status_all_variants_unique() {
        let statuses = vec![
            TreasuryAllocationStatus::Active,
            TreasuryAllocationStatus::Paused,
            TreasuryAllocationStatus::Completed,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_treasury_allocation_strategy_equality() {
        assert_eq!(TreasuryAllocationStrategy::Equal, TreasuryAllocationStrategy::Equal);
        assert_ne!(TreasuryAllocationStrategy::Equal, TreasuryAllocationStrategy::Proportional);
        assert_eq!(TreasuryAllocationStrategy::Proportional, TreasuryAllocationStrategy::Proportional);
        assert_ne!(TreasuryAllocationStrategy::Proportional, TreasuryAllocationStrategy::PriorityBased);
        assert_eq!(TreasuryAllocationStrategy::PriorityBased, TreasuryAllocationStrategy::PriorityBased);
        assert_ne!(TreasuryAllocationStrategy::PriorityBased, TreasuryAllocationStrategy::Custom);
        assert_eq!(TreasuryAllocationStrategy::Custom, TreasuryAllocationStrategy::Custom);
    }

    #[test]
    fn test_treasury_allocation_status_equality() {
        assert_eq!(TreasuryAllocationStatus::Active, TreasuryAllocationStatus::Active);
        assert_ne!(TreasuryAllocationStatus::Active, TreasuryAllocationStatus::Paused);
        assert_eq!(TreasuryAllocationStatus::Paused, TreasuryAllocationStatus::Paused);
        assert_ne!(TreasuryAllocationStatus::Paused, TreasuryAllocationStatus::Completed);
        assert_eq!(TreasuryAllocationStatus::Completed, TreasuryAllocationStatus::Completed);
    }

    #[test]
    fn test_treasury_allocation_strategy_copy() {
        let strategy1 = TreasuryAllocationStrategy::Equal;
        let strategy2 = strategy1; // Copy trait
        assert_eq!(strategy1, strategy2);
    }

    #[test]
    fn test_treasury_allocation_status_copy() {
        let status1 = TreasuryAllocationStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_treasury_allocation_strategy_space() {
        assert_eq!(<TreasuryAllocationStrategy as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_treasury_allocation_status_space() {
        assert_eq!(<TreasuryAllocationStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_treasury_allocation_all_strategies() {
        let strategies = vec![
            TreasuryAllocationStrategy::Equal,
            TreasuryAllocationStrategy::Proportional,
            TreasuryAllocationStrategy::PriorityBased,
            TreasuryAllocationStrategy::Custom,
        ];
        
        for strategy in strategies {
            let mut allocation = TreasuryAllocationStrategyMetadata {
                allocation_id: 0,
                treasury_id: 0,
                allocation_strategy: TreasuryAllocationStrategy::Equal,
                status: TreasuryAllocationStatus::Completed,
                created_at: 0,
                allocation_config_hash: [0u8; 32],
                bump: 0,
            };
            
            let result = onchain::initialize_treasury_allocation(
                &mut allocation,
                1,
                10,
                strategy,
                [0u8; 32],
                1000,
                255,
            );
            
            assert!(result.is_ok());
            assert_eq!(allocation.allocation_strategy, strategy);
        }
    }

    #[test]
    fn test_initialize_treasury_allocation_large_ids() {
        let mut allocation = TreasuryAllocationStrategyMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocation_strategy: TreasuryAllocationStrategy::Equal,
            status: TreasuryAllocationStatus::Completed,
            created_at: 0,
            allocation_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_allocation(
            &mut allocation,
            u64::MAX,
            u64::MAX,
            TreasuryAllocationStrategy::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(allocation.allocation_id, u64::MAX);
        assert_eq!(allocation.treasury_id, u64::MAX);
    }

    #[test]
    fn test_initialize_treasury_allocation_custom_hash() {
        let mut allocation = TreasuryAllocationStrategyMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocation_strategy: TreasuryAllocationStrategy::Equal,
            status: TreasuryAllocationStatus::Completed,
            created_at: 0,
            allocation_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let custom_hash = [255u8; 32];
        let result = onchain::initialize_treasury_allocation(
            &mut allocation,
            1,
            10,
            TreasuryAllocationStrategy::Custom,
            custom_hash,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(allocation.allocation_config_hash, custom_hash);
    }

    #[test]
    fn test_initialize_treasury_allocation_always_active_on_init() {
        let mut allocation = TreasuryAllocationStrategyMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocation_strategy: TreasuryAllocationStrategy::Equal,
            status: TreasuryAllocationStatus::Completed, // Will be reset
            created_at: 0,
            allocation_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_allocation(
            &mut allocation,
            1,
            10,
            TreasuryAllocationStrategy::Equal,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        // Status should always be set to Active on initialization
        assert_eq!(allocation.status, TreasuryAllocationStatus::Active);
    }

    #[test]
    fn test_treasury_allocation_metadata_all_fields() {
        let allocation = TreasuryAllocationStrategyMetadata {
            allocation_id: 123,
            treasury_id: 456,
            allocation_strategy: TreasuryAllocationStrategy::Proportional,
            status: TreasuryAllocationStatus::Paused,
            created_at: 5000,
            allocation_config_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(allocation.allocation_id, 123);
        assert_eq!(allocation.treasury_id, 456);
        assert_eq!(allocation.allocation_strategy, TreasuryAllocationStrategy::Proportional);
        assert_eq!(allocation.status, TreasuryAllocationStatus::Paused);
        assert_eq!(allocation.created_at, 5000);
        assert_eq!(allocation.allocation_config_hash, [42u8; 32]);
        assert_eq!(allocation.bump, 128);
    }

    #[test]
    fn test_offchain_allocate_treasury() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::allocate_treasury(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_allocate_treasury_different_ids() {
        // Test with different IDs
        let result1 = offchain::allocate_treasury(1);
        let result2 = offchain::allocate_treasury(999);
        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
    }
}
