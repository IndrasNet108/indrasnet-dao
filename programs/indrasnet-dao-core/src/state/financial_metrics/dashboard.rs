//! Financial Dashboard module
//!
//! Financial dashboard and visualization
//!
//! On-chain: Metadata for financial dashboard
//! Off-chain: Actual dashboard, visualization

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Dashboard type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDashboardType {
    /// Executive dashboard
    Executive,
    /// Operational dashboard
    Operational,
    /// Analytical dashboard
    Analytical,
    /// Custom dashboard
    Custom,
}

/// Dashboard status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDashboardStatus {
    /// Dashboard active
    Active,
    /// Dashboard paused
    Paused,
    /// Dashboard disabled
    Disabled,
}

/// Financial dashboard metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDashboardMetadata {
    /// Dashboard ID
    pub dashboard_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Dashboard type
    pub dashboard_type: FinancialDashboardType,
    /// Status
    pub status: FinancialDashboardStatus,
    /// Created at
    pub created_at: i64,
    /// Dashboard config hash
    pub dashboard_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_dashboard(
        dashboard: &mut FinancialDashboardMetadata,
        dashboard_id: u64,
        entity_id: u64,
        dashboard_type: FinancialDashboardType,
        dashboard_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(dashboard_id > 0, IndrasError::InvalidInput);
        dashboard.dashboard_id = dashboard_id;
        dashboard.entity_id = entity_id;
        dashboard.dashboard_type = dashboard_type;
        dashboard.status = FinancialDashboardStatus::Active;
        dashboard.created_at = current_time;
        dashboard.dashboard_config_hash = dashboard_config_hash;
        dashboard.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_financial_dashboard(_dashboard_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_dashboard() {
        let mut dashboard = FinancialDashboardMetadata {
            dashboard_id: 0,
            entity_id: 0,
            dashboard_type: FinancialDashboardType::Executive,
            status: FinancialDashboardStatus::Disabled,
            created_at: 0,
            dashboard_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_dashboard(
            &mut dashboard,
            1,
            10,
            FinancialDashboardType::Operational,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(dashboard.dashboard_id, 1);
        assert_eq!(dashboard.entity_id, 10);
        assert_eq!(dashboard.dashboard_type, FinancialDashboardType::Operational);
        assert_eq!(dashboard.status, FinancialDashboardStatus::Active);
        assert_eq!(dashboard.created_at, 1000);
        assert_eq!(dashboard.dashboard_config_hash, [1u8; 32]);
        assert_eq!(dashboard.bump, 255);
    }

    #[test]
    fn test_initialize_financial_dashboard_invalid_id() {
        let mut dashboard = FinancialDashboardMetadata {
            dashboard_id: 0,
            entity_id: 0,
            dashboard_type: FinancialDashboardType::Executive,
            status: FinancialDashboardStatus::Active,
            created_at: 0,
            dashboard_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_dashboard(
            &mut dashboard,
            0, // Invalid: must be > 0
            10,
            FinancialDashboardType::Operational,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_dashboard_all_types() {
        let types = vec![
            FinancialDashboardType::Executive,
            FinancialDashboardType::Operational,
            FinancialDashboardType::Analytical,
            FinancialDashboardType::Custom,
        ];

        for dashboard_type in types {
            let mut dashboard = FinancialDashboardMetadata {
                dashboard_id: 0,
                entity_id: 0,
                dashboard_type: FinancialDashboardType::Executive,
                status: FinancialDashboardStatus::Active,
                created_at: 0,
                dashboard_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_dashboard(
                &mut dashboard,
                1,
                10,
                dashboard_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(dashboard.dashboard_type, dashboard_type);
        }
    }

    #[test]
    fn test_financial_dashboard_type_variants() {
        assert_eq!(FinancialDashboardType::Executive, FinancialDashboardType::Executive);
        assert_eq!(FinancialDashboardType::Operational, FinancialDashboardType::Operational);
        assert_eq!(FinancialDashboardType::Analytical, FinancialDashboardType::Analytical);
        assert_eq!(FinancialDashboardType::Custom, FinancialDashboardType::Custom);
    }

    #[test]
    fn test_financial_dashboard_status_variants() {
        assert_eq!(FinancialDashboardStatus::Active, FinancialDashboardStatus::Active);
        assert_eq!(FinancialDashboardStatus::Paused, FinancialDashboardStatus::Paused);
        assert_eq!(FinancialDashboardStatus::Disabled, FinancialDashboardStatus::Disabled);
    }

    #[test]
    fn test_financial_dashboard_type_all_variants_unique() {
        let variants = vec![
            FinancialDashboardType::Executive,
            FinancialDashboardType::Operational,
            FinancialDashboardType::Analytical,
            FinancialDashboardType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_dashboard_status_all_variants_unique() {
        let variants = vec![
            FinancialDashboardStatus::Active,
            FinancialDashboardStatus::Paused,
            FinancialDashboardStatus::Disabled,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_generate_financial_dashboard() {
        let result = offchain::generate_financial_dashboard(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
