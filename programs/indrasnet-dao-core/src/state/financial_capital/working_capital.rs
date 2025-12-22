//! Financial Working Capital module
//!
//! Financial working capital management
//!
//! On-chain: Metadata for working capital
//! Off-chain: Actual management, optimization

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Working capital component
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialWorkingCapitalComponent {
    /// Current assets
    CurrentAssets,
    /// Current liabilities
    CurrentLiabilities,
    /// Net working capital
    NetWorkingCapital,
    /// Custom component
    Custom,
}

/// Management status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialWorkingCapitalStatus {
    /// Management active
    Active,
    /// Management paused
    Paused,
    /// Management optimized
    Optimized,
}

/// Financial working capital metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialWorkingCapitalMetadata {
    /// Working capital ID
    pub working_capital_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Working capital component
    pub working_capital_component: FinancialWorkingCapitalComponent,
    /// Status
    pub status: FinancialWorkingCapitalStatus,
    /// Created at
    pub created_at: i64,
    /// Working capital config hash
    pub working_capital_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_working_capital(
        working_capital: &mut FinancialWorkingCapitalMetadata,
        working_capital_id: u64,
        entity_id: u64,
        working_capital_component: FinancialWorkingCapitalComponent,
        working_capital_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(working_capital_id > 0, IndrasError::InvalidInput);
        working_capital.working_capital_id = working_capital_id;
        working_capital.entity_id = entity_id;
        working_capital.working_capital_component = working_capital_component;
        working_capital.status = FinancialWorkingCapitalStatus::Active;
        working_capital.created_at = current_time;
        working_capital.working_capital_config_hash = working_capital_config_hash;
        working_capital.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn optimize_working_capital(_working_capital_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_working_capital() {
        let mut working_capital = FinancialWorkingCapitalMetadata {
            working_capital_id: 0,
            entity_id: 0,
            working_capital_component: FinancialWorkingCapitalComponent::CurrentAssets,
            status: FinancialWorkingCapitalStatus::Optimized,
            created_at: 0,
            working_capital_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_working_capital(
            &mut working_capital,
            1,
            10,
            FinancialWorkingCapitalComponent::CurrentLiabilities,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(working_capital.working_capital_id, 1);
        assert_eq!(working_capital.entity_id, 10);
        assert_eq!(working_capital.working_capital_component, FinancialWorkingCapitalComponent::CurrentLiabilities);
        assert_eq!(working_capital.status, FinancialWorkingCapitalStatus::Active);
        assert_eq!(working_capital.created_at, 1000);
        assert_eq!(working_capital.working_capital_config_hash, [1u8; 32]);
        assert_eq!(working_capital.bump, 255);
    }

    #[test]
    fn test_initialize_financial_working_capital_invalid_id() {
        let mut working_capital = FinancialWorkingCapitalMetadata {
            working_capital_id: 0,
            entity_id: 0,
            working_capital_component: FinancialWorkingCapitalComponent::CurrentAssets,
            status: FinancialWorkingCapitalStatus::Active,
            created_at: 0,
            working_capital_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_working_capital(
            &mut working_capital,
            0, // Invalid: must be > 0
            10,
            FinancialWorkingCapitalComponent::CurrentLiabilities,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_working_capital_all_components() {
        let components = vec![
            FinancialWorkingCapitalComponent::CurrentAssets,
            FinancialWorkingCapitalComponent::CurrentLiabilities,
            FinancialWorkingCapitalComponent::NetWorkingCapital,
            FinancialWorkingCapitalComponent::Custom,
        ];

        for component in components {
            let mut working_capital = FinancialWorkingCapitalMetadata {
                working_capital_id: 0,
                entity_id: 0,
                working_capital_component: FinancialWorkingCapitalComponent::CurrentAssets,
                status: FinancialWorkingCapitalStatus::Active,
                created_at: 0,
                working_capital_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_working_capital(
                &mut working_capital,
                1,
                10,
                component,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(working_capital.working_capital_component, component);
        }
    }

    #[test]
    fn test_financial_working_capital_component_variants() {
        assert_eq!(FinancialWorkingCapitalComponent::CurrentAssets, FinancialWorkingCapitalComponent::CurrentAssets);
        assert_eq!(FinancialWorkingCapitalComponent::CurrentLiabilities, FinancialWorkingCapitalComponent::CurrentLiabilities);
        assert_eq!(FinancialWorkingCapitalComponent::NetWorkingCapital, FinancialWorkingCapitalComponent::NetWorkingCapital);
        assert_eq!(FinancialWorkingCapitalComponent::Custom, FinancialWorkingCapitalComponent::Custom);
    }

    #[test]
    fn test_financial_working_capital_status_variants() {
        assert_eq!(FinancialWorkingCapitalStatus::Active, FinancialWorkingCapitalStatus::Active);
        assert_eq!(FinancialWorkingCapitalStatus::Paused, FinancialWorkingCapitalStatus::Paused);
        assert_eq!(FinancialWorkingCapitalStatus::Optimized, FinancialWorkingCapitalStatus::Optimized);
    }

    #[test]
    fn test_financial_working_capital_component_all_variants_unique() {
        let variants = vec![
            FinancialWorkingCapitalComponent::CurrentAssets,
            FinancialWorkingCapitalComponent::CurrentLiabilities,
            FinancialWorkingCapitalComponent::NetWorkingCapital,
            FinancialWorkingCapitalComponent::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_working_capital_status_all_variants_unique() {
        let variants = vec![
            FinancialWorkingCapitalStatus::Active,
            FinancialWorkingCapitalStatus::Paused,
            FinancialWorkingCapitalStatus::Optimized,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_optimize_working_capital() {
        let result = offchain::optimize_working_capital(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
