//! Financial KPIs module
//!
//! Financial key performance indicators
//!
//! On-chain: Metadata for financial KPIs
//! Off-chain: Actual KPI calculation, tracking

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// KPI category
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialKPICategory {
    /// Revenue KPIs
    Revenue,
    /// Profitability KPIs
    Profitability,
    /// Efficiency KPIs
    Efficiency,
    /// Custom KPI
    Custom,
}

/// KPI status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialKPIStatus {
    /// KPI active
    Active,
    /// KPI paused
    Paused,
    /// KPI disabled
    Disabled,
}

/// Financial KPIs metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialKPIsMetadata {
    /// KPI ID
    pub kpi_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// KPI category
    pub kpi_category: FinancialKPICategory,
    /// Status
    pub status: FinancialKPIStatus,
    /// Created at
    pub created_at: i64,
    /// KPI config hash
    pub kpi_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_kpis(
        kpi: &mut FinancialKPIsMetadata,
        kpi_id: u64,
        entity_id: u64,
        kpi_category: FinancialKPICategory,
        kpi_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(kpi_id > 0, IndrasError::InvalidInput);
        kpi.kpi_id = kpi_id;
        kpi.entity_id = entity_id;
        kpi.kpi_category = kpi_category;
        kpi.status = FinancialKPIStatus::Active;
        kpi.created_at = current_time;
        kpi.kpi_config_hash = kpi_config_hash;
        kpi.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn calculate_financial_kpis(_kpi_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_kpis() {
        let mut kpi = FinancialKPIsMetadata {
            kpi_id: 0,
            entity_id: 0,
            kpi_category: FinancialKPICategory::Revenue,
            status: FinancialKPIStatus::Disabled,
            created_at: 0,
            kpi_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_kpis(
            &mut kpi,
            1,
            10,
            FinancialKPICategory::Profitability,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(kpi.kpi_id, 1);
        assert_eq!(kpi.entity_id, 10);
        assert_eq!(kpi.kpi_category, FinancialKPICategory::Profitability);
        assert_eq!(kpi.status, FinancialKPIStatus::Active);
        assert_eq!(kpi.created_at, 1000);
        assert_eq!(kpi.kpi_config_hash, [1u8; 32]);
        assert_eq!(kpi.bump, 255);
    }

    #[test]
    fn test_initialize_financial_kpis_invalid_id() {
        let mut kpi = FinancialKPIsMetadata {
            kpi_id: 0,
            entity_id: 0,
            kpi_category: FinancialKPICategory::Revenue,
            status: FinancialKPIStatus::Active,
            created_at: 0,
            kpi_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_kpis(
            &mut kpi,
            0, // Invalid: must be > 0
            10,
            FinancialKPICategory::Profitability,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_kpis_all_categories() {
        let categories = vec![
            FinancialKPICategory::Revenue,
            FinancialKPICategory::Profitability,
            FinancialKPICategory::Efficiency,
            FinancialKPICategory::Custom,
        ];

        for kpi_category in categories {
            let mut kpi = FinancialKPIsMetadata {
                kpi_id: 0,
                entity_id: 0,
                kpi_category: FinancialKPICategory::Revenue,
                status: FinancialKPIStatus::Active,
                created_at: 0,
                kpi_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_kpis(
                &mut kpi,
                1,
                10,
                kpi_category,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(kpi.kpi_category, kpi_category);
        }
    }

    #[test]
    fn test_financial_kpi_category_variants() {
        assert_eq!(FinancialKPICategory::Revenue, FinancialKPICategory::Revenue);
        assert_eq!(FinancialKPICategory::Profitability, FinancialKPICategory::Profitability);
        assert_eq!(FinancialKPICategory::Efficiency, FinancialKPICategory::Efficiency);
        assert_eq!(FinancialKPICategory::Custom, FinancialKPICategory::Custom);
    }

    #[test]
    fn test_financial_kpi_status_variants() {
        assert_eq!(FinancialKPIStatus::Active, FinancialKPIStatus::Active);
        assert_eq!(FinancialKPIStatus::Paused, FinancialKPIStatus::Paused);
        assert_eq!(FinancialKPIStatus::Disabled, FinancialKPIStatus::Disabled);
    }

    #[test]
    fn test_financial_kpi_category_all_variants_unique() {
        let variants = vec![
            FinancialKPICategory::Revenue,
            FinancialKPICategory::Profitability,
            FinancialKPICategory::Efficiency,
            FinancialKPICategory::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_kpi_status_all_variants_unique() {
        let variants = vec![
            FinancialKPIStatus::Active,
            FinancialKPIStatus::Paused,
            FinancialKPIStatus::Disabled,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_calculate_financial_kpis() {
        let result = offchain::calculate_financial_kpis(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
