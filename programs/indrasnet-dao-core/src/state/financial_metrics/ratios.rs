//! Financial Ratios module
//!
//! Financial ratios calculation
//!
//! On-chain: Metadata for financial ratios
//! Off-chain: Actual calculation, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Ratio type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialRatioType {
    /// Liquidity ratios
    Liquidity,
    /// Profitability ratios
    Profitability,
    /// Efficiency ratios
    Efficiency,
    /// Custom ratio
    Custom,
}

/// Ratio status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialRatioStatus {
    /// Ratio active
    Active,
    /// Ratio paused
    Paused,
    /// Ratio disabled
    Disabled,
}

/// Financial ratios metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialRatiosMetadata {
    /// Ratio ID
    pub ratio_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Ratio type
    pub ratio_type: FinancialRatioType,
    /// Status
    pub status: FinancialRatioStatus,
    /// Created at
    pub created_at: i64,
    /// Ratio config hash
    pub ratio_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_ratios(
        ratio: &mut FinancialRatiosMetadata,
        ratio_id: u64,
        entity_id: u64,
        ratio_type: FinancialRatioType,
        ratio_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(ratio_id > 0, IndrasError::InvalidInput);
        ratio.ratio_id = ratio_id;
        ratio.entity_id = entity_id;
        ratio.ratio_type = ratio_type;
        ratio.status = FinancialRatioStatus::Active;
        ratio.created_at = current_time;
        ratio.ratio_config_hash = ratio_config_hash;
        ratio.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn calculate_financial_ratios(_ratio_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_ratios() {
        let mut ratio = FinancialRatiosMetadata {
            ratio_id: 0,
            entity_id: 0,
            ratio_type: FinancialRatioType::Liquidity,
            status: FinancialRatioStatus::Disabled,
            created_at: 0,
            ratio_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_ratios(
            &mut ratio,
            1,
            10,
            FinancialRatioType::Profitability,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(ratio.ratio_id, 1);
        assert_eq!(ratio.entity_id, 10);
        assert_eq!(ratio.ratio_type, FinancialRatioType::Profitability);
        assert_eq!(ratio.status, FinancialRatioStatus::Active);
        assert_eq!(ratio.created_at, 1000);
        assert_eq!(ratio.ratio_config_hash, [1u8; 32]);
        assert_eq!(ratio.bump, 255);
    }

    #[test]
    fn test_initialize_financial_ratios_invalid_id() {
        let mut ratio = FinancialRatiosMetadata {
            ratio_id: 0,
            entity_id: 0,
            ratio_type: FinancialRatioType::Liquidity,
            status: FinancialRatioStatus::Active,
            created_at: 0,
            ratio_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_ratios(
            &mut ratio,
            0, // Invalid: must be > 0
            10,
            FinancialRatioType::Profitability,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_ratios_all_types() {
        let types = vec![
            FinancialRatioType::Liquidity,
            FinancialRatioType::Profitability,
            FinancialRatioType::Efficiency,
            FinancialRatioType::Custom,
        ];

        for ratio_type in types {
            let mut ratio = FinancialRatiosMetadata {
                ratio_id: 0,
                entity_id: 0,
                ratio_type: FinancialRatioType::Liquidity,
                status: FinancialRatioStatus::Active,
                created_at: 0,
                ratio_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_ratios(
                &mut ratio,
                1,
                10,
                ratio_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(ratio.ratio_type, ratio_type);
        }
    }

    #[test]
    fn test_financial_ratio_type_variants() {
        assert_eq!(FinancialRatioType::Liquidity, FinancialRatioType::Liquidity);
        assert_eq!(FinancialRatioType::Profitability, FinancialRatioType::Profitability);
        assert_eq!(FinancialRatioType::Efficiency, FinancialRatioType::Efficiency);
        assert_eq!(FinancialRatioType::Custom, FinancialRatioType::Custom);
    }

    #[test]
    fn test_financial_ratio_status_variants() {
        assert_eq!(FinancialRatioStatus::Active, FinancialRatioStatus::Active);
        assert_eq!(FinancialRatioStatus::Paused, FinancialRatioStatus::Paused);
        assert_eq!(FinancialRatioStatus::Disabled, FinancialRatioStatus::Disabled);
    }

    #[test]
    fn test_financial_ratio_type_all_variants_unique() {
        let variants = vec![
            FinancialRatioType::Liquidity,
            FinancialRatioType::Profitability,
            FinancialRatioType::Efficiency,
            FinancialRatioType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_ratio_status_all_variants_unique() {
        let variants = vec![
            FinancialRatioStatus::Active,
            FinancialRatioStatus::Paused,
            FinancialRatioStatus::Disabled,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_calculate_financial_ratios() {
        let result = offchain::calculate_financial_ratios(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
