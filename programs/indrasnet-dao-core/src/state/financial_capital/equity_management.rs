//! Financial Equity Management module
//!
//! Financial equity management
//!
//! On-chain: Metadata for equity management
//! Off-chain: Actual management, optimization

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Equity type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialEquityType {
    /// Common equity
    Common,
    /// Preferred equity
    Preferred,
    /// Treasury stock
    TreasuryStock,
    /// Custom equity
    Custom,
}

/// Management status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialEquityManagementStatus {
    /// Management active
    Active,
    /// Management paused
    Paused,
    /// Management optimized
    Optimized,
}

/// Financial equity management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialEquityManagementMetadata {
    /// Management ID
    pub management_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Equity type
    pub equity_type: FinancialEquityType,
    /// Status
    pub status: FinancialEquityManagementStatus,
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
    pub fn initialize_financial_equity_management(
        management: &mut FinancialEquityManagementMetadata,
        management_id: u64,
        entity_id: u64,
        equity_type: FinancialEquityType,
        management_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(management_id > 0, IndrasError::InvalidInput);
        management.management_id = management_id;
        management.entity_id = entity_id;
        management.equity_type = equity_type;
        management.status = FinancialEquityManagementStatus::Active;
        management.created_at = current_time;
        management.management_config_hash = management_config_hash;
        management.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_equity(_management_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_equity_management() {
        let mut management = FinancialEquityManagementMetadata {
            management_id: 0,
            entity_id: 0,
            equity_type: FinancialEquityType::Common,
            status: FinancialEquityManagementStatus::Optimized,
            created_at: 0,
            management_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_equity_management(
            &mut management,
            1,
            10,
            FinancialEquityType::Preferred,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(management.management_id, 1);
        assert_eq!(management.entity_id, 10);
        assert_eq!(management.equity_type, FinancialEquityType::Preferred);
        assert_eq!(management.status, FinancialEquityManagementStatus::Active);
        assert_eq!(management.created_at, 1000);
        assert_eq!(management.management_config_hash, [1u8; 32]);
        assert_eq!(management.bump, 255);
    }

    #[test]
    fn test_initialize_financial_equity_management_invalid_id() {
        let mut management = FinancialEquityManagementMetadata {
            management_id: 0,
            entity_id: 0,
            equity_type: FinancialEquityType::Common,
            status: FinancialEquityManagementStatus::Active,
            created_at: 0,
            management_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_equity_management(
            &mut management,
            0, // Invalid: must be > 0
            10,
            FinancialEquityType::Preferred,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_equity_management_all_types() {
        let types = vec![
            FinancialEquityType::Common,
            FinancialEquityType::Preferred,
            FinancialEquityType::TreasuryStock,
            FinancialEquityType::Custom,
        ];

        for equity_type in types {
            let mut management = FinancialEquityManagementMetadata {
                management_id: 0,
                entity_id: 0,
                equity_type: FinancialEquityType::Common,
                status: FinancialEquityManagementStatus::Active,
                created_at: 0,
                management_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_equity_management(
                &mut management,
                1,
                10,
                equity_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(management.equity_type, equity_type);
        }
    }

    #[test]
    fn test_financial_equity_type_variants() {
        assert_eq!(FinancialEquityType::Common, FinancialEquityType::Common);
        assert_eq!(FinancialEquityType::Preferred, FinancialEquityType::Preferred);
        assert_eq!(FinancialEquityType::TreasuryStock, FinancialEquityType::TreasuryStock);
        assert_eq!(FinancialEquityType::Custom, FinancialEquityType::Custom);
    }

    #[test]
    fn test_financial_equity_management_status_variants() {
        assert_eq!(FinancialEquityManagementStatus::Active, FinancialEquityManagementStatus::Active);
        assert_eq!(FinancialEquityManagementStatus::Paused, FinancialEquityManagementStatus::Paused);
        assert_eq!(FinancialEquityManagementStatus::Optimized, FinancialEquityManagementStatus::Optimized);
    }

    #[test]
    fn test_financial_equity_type_all_variants_unique() {
        let variants = vec![
            FinancialEquityType::Common,
            FinancialEquityType::Preferred,
            FinancialEquityType::TreasuryStock,
            FinancialEquityType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_equity_management_status_all_variants_unique() {
        let variants = vec![
            FinancialEquityManagementStatus::Active,
            FinancialEquityManagementStatus::Paused,
            FinancialEquityManagementStatus::Optimized,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_manage_equity() {
        let result = offchain::manage_equity(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
