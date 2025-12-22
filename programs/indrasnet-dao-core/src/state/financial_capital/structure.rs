//! Financial Capital Structure module
//!
//! Financial capital structure management
//!
//! On-chain: Metadata for capital structure
//! Off-chain: Actual management, optimization

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Capital component
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCapitalComponent {
    /// Equity
    Equity,
    /// Debt
    Debt,
    /// Hybrid
    Hybrid,
    /// Custom component
    Custom,
}

/// Structure status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCapitalStructureStatus {
    /// Structure active
    Active,
    /// Structure paused
    Paused,
    /// Structure optimized
    Optimized,
}

/// Financial capital structure metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialCapitalStructureMetadata {
    /// Structure ID
    pub structure_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Capital component
    pub capital_component: FinancialCapitalComponent,
    /// Status
    pub status: FinancialCapitalStructureStatus,
    /// Created at
    pub created_at: i64,
    /// Structure config hash
    pub structure_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_capital_structure(
        structure: &mut FinancialCapitalStructureMetadata,
        structure_id: u64,
        entity_id: u64,
        capital_component: FinancialCapitalComponent,
        structure_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(structure_id > 0, IndrasError::InvalidInput);
        structure.structure_id = structure_id;
        structure.entity_id = entity_id;
        structure.capital_component = capital_component;
        structure.status = FinancialCapitalStructureStatus::Active;
        structure.created_at = current_time;
        structure.structure_config_hash = structure_config_hash;
        structure.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn optimize_capital_structure(_structure_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_capital_structure() {
        let mut structure = FinancialCapitalStructureMetadata {
            structure_id: 0,
            entity_id: 0,
            capital_component: FinancialCapitalComponent::Equity,
            status: FinancialCapitalStructureStatus::Optimized,
            created_at: 0,
            structure_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_capital_structure(
            &mut structure,
            1,
            10,
            FinancialCapitalComponent::Debt,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(structure.structure_id, 1);
        assert_eq!(structure.entity_id, 10);
        assert_eq!(structure.capital_component, FinancialCapitalComponent::Debt);
        assert_eq!(structure.status, FinancialCapitalStructureStatus::Active);
        assert_eq!(structure.created_at, 1000);
        assert_eq!(structure.structure_config_hash, [1u8; 32]);
        assert_eq!(structure.bump, 255);
    }

    #[test]
    fn test_initialize_financial_capital_structure_invalid_id() {
        let mut structure = FinancialCapitalStructureMetadata {
            structure_id: 0,
            entity_id: 0,
            capital_component: FinancialCapitalComponent::Equity,
            status: FinancialCapitalStructureStatus::Active,
            created_at: 0,
            structure_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_capital_structure(
            &mut structure,
            0, // Invalid: must be > 0
            10,
            FinancialCapitalComponent::Debt,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_capital_structure_all_components() {
        let components = vec![
            FinancialCapitalComponent::Equity,
            FinancialCapitalComponent::Debt,
            FinancialCapitalComponent::Hybrid,
            FinancialCapitalComponent::Custom,
        ];

        for component in components {
            let mut structure = FinancialCapitalStructureMetadata {
                structure_id: 0,
                entity_id: 0,
                capital_component: FinancialCapitalComponent::Equity,
                status: FinancialCapitalStructureStatus::Active,
                created_at: 0,
                structure_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_capital_structure(
                &mut structure,
                1,
                10,
                component,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(structure.capital_component, component);
        }
    }

    #[test]
    fn test_financial_capital_component_variants() {
        assert_eq!(FinancialCapitalComponent::Equity, FinancialCapitalComponent::Equity);
        assert_eq!(FinancialCapitalComponent::Debt, FinancialCapitalComponent::Debt);
        assert_eq!(FinancialCapitalComponent::Hybrid, FinancialCapitalComponent::Hybrid);
        assert_eq!(FinancialCapitalComponent::Custom, FinancialCapitalComponent::Custom);
    }

    #[test]
    fn test_financial_capital_structure_status_variants() {
        assert_eq!(FinancialCapitalStructureStatus::Active, FinancialCapitalStructureStatus::Active);
        assert_eq!(FinancialCapitalStructureStatus::Paused, FinancialCapitalStructureStatus::Paused);
        assert_eq!(FinancialCapitalStructureStatus::Optimized, FinancialCapitalStructureStatus::Optimized);
    }

    #[test]
    fn test_financial_capital_component_all_variants_unique() {
        let variants = vec![
            FinancialCapitalComponent::Equity,
            FinancialCapitalComponent::Debt,
            FinancialCapitalComponent::Hybrid,
            FinancialCapitalComponent::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_capital_structure_status_all_variants_unique() {
        let variants = vec![
            FinancialCapitalStructureStatus::Active,
            FinancialCapitalStructureStatus::Paused,
            FinancialCapitalStructureStatus::Optimized,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_optimize_capital_structure() {
        let result = offchain::optimize_capital_structure(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
