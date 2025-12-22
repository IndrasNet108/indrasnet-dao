//! Capital Management module
//!
//! Capital management and allocation
//!
//! On-chain: Metadata for capital management
//! Off-chain: Actual management, allocation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Management strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum CapitalManagementStrategy {
    /// Conservative strategy
    Conservative,
    /// Balanced strategy
    Balanced,
    /// Aggressive strategy
    Aggressive,
    /// Custom strategy
    Custom,
}

/// Management status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum CapitalManagementStatus {
    /// Management active
    Active,
    /// Management paused
    Paused,
    /// Management disabled
    Disabled,
}

/// Capital management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct CapitalManagementMetadata {
    /// Management ID
    pub management_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Management strategy
    pub management_strategy: CapitalManagementStrategy,
    /// Status
    pub status: CapitalManagementStatus,
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
    pub fn initialize_capital_management(
        management: &mut CapitalManagementMetadata,
        management_id: u64,
        entity_id: u64,
        management_strategy: CapitalManagementStrategy,
        management_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(management_id > 0, IndrasError::InvalidInput);
        management.management_id = management_id;
        management.entity_id = entity_id;
        management.management_strategy = management_strategy;
        management.status = CapitalManagementStatus::Active;
        management.created_at = current_time;
        management.management_config_hash = management_config_hash;
        management.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_capital(_management_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_capital_management() {
        let mut management = CapitalManagementMetadata {
            management_id: 0,
            entity_id: 0,
            management_strategy: CapitalManagementStrategy::Conservative,
            status: CapitalManagementStatus::Disabled,
            created_at: 0,
            management_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_capital_management(
            &mut management,
            1,
            10,
            CapitalManagementStrategy::Balanced,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(management.management_id, 1);
        assert_eq!(management.entity_id, 10);
        assert_eq!(management.management_strategy, CapitalManagementStrategy::Balanced);
        assert_eq!(management.status, CapitalManagementStatus::Active);
        assert_eq!(management.created_at, 1000);
        assert_eq!(management.management_config_hash, [1u8; 32]);
        assert_eq!(management.bump, 255);
    }

    #[test]
    fn test_initialize_capital_management_invalid_id() {
        let mut management = CapitalManagementMetadata {
            management_id: 0,
            entity_id: 0,
            management_strategy: CapitalManagementStrategy::Conservative,
            status: CapitalManagementStatus::Active,
            created_at: 0,
            management_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_capital_management(
            &mut management,
            0, // Invalid: must be > 0
            10,
            CapitalManagementStrategy::Balanced,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_capital_management_all_strategies() {
        let strategies = vec![
            CapitalManagementStrategy::Conservative,
            CapitalManagementStrategy::Balanced,
            CapitalManagementStrategy::Aggressive,
            CapitalManagementStrategy::Custom,
        ];

        for strategy in strategies {
            let mut management = CapitalManagementMetadata {
                management_id: 0,
                entity_id: 0,
                management_strategy: CapitalManagementStrategy::Conservative,
                status: CapitalManagementStatus::Active,
                created_at: 0,
                management_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_capital_management(
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
    fn test_capital_management_strategy_variants() {
        assert_eq!(CapitalManagementStrategy::Conservative, CapitalManagementStrategy::Conservative);
        assert_eq!(CapitalManagementStrategy::Balanced, CapitalManagementStrategy::Balanced);
        assert_eq!(CapitalManagementStrategy::Aggressive, CapitalManagementStrategy::Aggressive);
        assert_eq!(CapitalManagementStrategy::Custom, CapitalManagementStrategy::Custom);
    }

    #[test]
    fn test_capital_management_status_variants() {
        assert_eq!(CapitalManagementStatus::Active, CapitalManagementStatus::Active);
        assert_eq!(CapitalManagementStatus::Paused, CapitalManagementStatus::Paused);
        assert_eq!(CapitalManagementStatus::Disabled, CapitalManagementStatus::Disabled);
    }

    #[test]
    fn test_capital_management_strategy_all_variants_unique() {
        let variants = vec![
            CapitalManagementStrategy::Conservative,
            CapitalManagementStrategy::Balanced,
            CapitalManagementStrategy::Aggressive,
            CapitalManagementStrategy::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_capital_management_status_all_variants_unique() {
        let variants = vec![
            CapitalManagementStatus::Active,
            CapitalManagementStatus::Paused,
            CapitalManagementStatus::Disabled,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_manage_capital() {
        let result = offchain::manage_capital(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
