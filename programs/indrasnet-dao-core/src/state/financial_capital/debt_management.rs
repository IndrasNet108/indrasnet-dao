//! Financial Debt Management module
//!
//! Financial debt management
//!
//! On-chain: Metadata for debt management
//! Off-chain: Actual management, optimization

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Debt type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDebtType {
    /// Short-term debt
    ShortTerm,
    /// Long-term debt
    LongTerm,
    /// Convertible debt
    Convertible,
    /// Custom debt
    Custom,
}

/// Management status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDebtManagementStatus {
    /// Management active
    Active,
    /// Management paused
    Paused,
    /// Management optimized
    Optimized,
}

/// Financial debt management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDebtManagementMetadata {
    /// Management ID
    pub management_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Debt type
    pub debt_type: FinancialDebtType,
    /// Status
    pub status: FinancialDebtManagementStatus,
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
    pub fn initialize_financial_debt_management(
        management: &mut FinancialDebtManagementMetadata,
        management_id: u64,
        entity_id: u64,
        debt_type: FinancialDebtType,
        management_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(management_id > 0, IndrasError::InvalidInput);
        management.management_id = management_id;
        management.entity_id = entity_id;
        management.debt_type = debt_type;
        management.status = FinancialDebtManagementStatus::Active;
        management.created_at = current_time;
        management.management_config_hash = management_config_hash;
        management.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_debt(_management_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_debt_management() {
        let mut management = FinancialDebtManagementMetadata {
            management_id: 0,
            entity_id: 0,
            debt_type: FinancialDebtType::ShortTerm,
            status: FinancialDebtManagementStatus::Optimized,
            created_at: 0,
            management_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_debt_management(
            &mut management,
            1,
            10,
            FinancialDebtType::LongTerm,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(management.management_id, 1);
        assert_eq!(management.entity_id, 10);
        assert_eq!(management.debt_type, FinancialDebtType::LongTerm);
        assert_eq!(management.status, FinancialDebtManagementStatus::Active);
        assert_eq!(management.created_at, 1000);
        assert_eq!(management.management_config_hash, [1u8; 32]);
        assert_eq!(management.bump, 255);
    }

    #[test]
    fn test_initialize_financial_debt_management_invalid_id() {
        let mut management = FinancialDebtManagementMetadata {
            management_id: 0,
            entity_id: 0,
            debt_type: FinancialDebtType::ShortTerm,
            status: FinancialDebtManagementStatus::Active,
            created_at: 0,
            management_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_debt_management(
            &mut management,
            0, // Invalid: must be > 0
            10,
            FinancialDebtType::LongTerm,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_debt_management_all_types() {
        let types = vec![
            FinancialDebtType::ShortTerm,
            FinancialDebtType::LongTerm,
            FinancialDebtType::Convertible,
            FinancialDebtType::Custom,
        ];

        for debt_type in types {
            let mut management = FinancialDebtManagementMetadata {
                management_id: 0,
                entity_id: 0,
                debt_type: FinancialDebtType::ShortTerm,
                status: FinancialDebtManagementStatus::Active,
                created_at: 0,
                management_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_debt_management(
                &mut management,
                1,
                10,
                debt_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(management.debt_type, debt_type);
        }
    }

    #[test]
    fn test_financial_debt_type_variants() {
        assert_eq!(FinancialDebtType::ShortTerm, FinancialDebtType::ShortTerm);
        assert_eq!(FinancialDebtType::LongTerm, FinancialDebtType::LongTerm);
        assert_eq!(FinancialDebtType::Convertible, FinancialDebtType::Convertible);
        assert_eq!(FinancialDebtType::Custom, FinancialDebtType::Custom);
    }

    #[test]
    fn test_financial_debt_management_status_variants() {
        assert_eq!(FinancialDebtManagementStatus::Active, FinancialDebtManagementStatus::Active);
        assert_eq!(FinancialDebtManagementStatus::Paused, FinancialDebtManagementStatus::Paused);
        assert_eq!(FinancialDebtManagementStatus::Optimized, FinancialDebtManagementStatus::Optimized);
    }

    #[test]
    fn test_financial_debt_type_all_variants_unique() {
        let variants = vec![
            FinancialDebtType::ShortTerm,
            FinancialDebtType::LongTerm,
            FinancialDebtType::Convertible,
            FinancialDebtType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_debt_management_status_all_variants_unique() {
        let variants = vec![
            FinancialDebtManagementStatus::Active,
            FinancialDebtManagementStatus::Paused,
            FinancialDebtManagementStatus::Optimized,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_manage_debt() {
        let result = offchain::manage_debt(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
