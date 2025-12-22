//! Financial Planning module
//!
//! Financial planning management (basic and advanced)
//!
//! On-chain: Metadata for financial plans
//! Off-chain: Actual planning calculations, forecasting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Plan status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialPlanStatus {
    /// Plan draft
    Draft,
    /// Plan approved
    Approved,
    /// Plan active
    Active,
    /// Plan closed
    Closed,
    /// Plan archived (advanced)
    Archived,
}

/// Planning type (advanced)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialPlanningType {
    /// Basic planning
    Basic,
    /// Strategic planning
    Strategic,
    /// Tactical planning
    Tactical,
    /// Operational planning
    Operational,
    /// Custom planning
    Custom,
}

/// Financial plan metadata (on-chain)
///
/// Stores metadata for financial plans (basic and advanced)
#[account]
#[derive(InitSpace)]
pub struct FinancialPlanMetadata {
    /// Plan ID
    pub plan_id: u64,
    /// Entity ID (for advanced planning, optional for basic)
    pub entity_id: Option<u64>,
    /// Planning type (basic or advanced)
    pub planning_type: FinancialPlanningType,
    /// Total budget (in smallest unit)
    pub total_budget: u64,
    /// Status
    pub status: FinancialPlanStatus,
    /// Created at
    pub created_at: i64,
    /// Period start (optional for advanced)
    pub period_start: Option<i64>,
    /// Period end (optional for advanced)
    pub period_end: Option<i64>,
    /// Plan data hash (for basic) or planning config hash (for advanced)
    pub plan_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for financial planning
pub mod onchain {
    use super::*;

    /// Initialize financial plan (basic)
    pub fn initialize_financial_plan(
        plan: &mut FinancialPlanMetadata,
        plan_id: u64,
        total_budget: u64,
        plan_data_hash: [u8; 32],
        period_start: i64,
        period_end: i64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(plan_id > 0, IndrasError::InvalidInput);
        require!(total_budget > 0, IndrasError::InvalidInput);
        require!(period_end > period_start, IndrasError::InvalidInput);
        
        plan.plan_id = plan_id;
        plan.entity_id = None;
        plan.planning_type = FinancialPlanningType::Basic;
        plan.total_budget = total_budget;
        plan.status = FinancialPlanStatus::Draft;
        plan.created_at = current_time;
        plan.period_start = Some(period_start);
        plan.period_end = Some(period_end);
        plan.plan_data_hash = plan_data_hash;
        plan.bump = bump;
        
        Ok(())
    }

    /// Initialize advanced financial planning
    pub fn initialize_advanced_financial_planning(
        plan: &mut FinancialPlanMetadata,
        planning_id: u64,
        entity_id: u64,
        planning_type: FinancialPlanningType,
        planning_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(planning_id > 0, IndrasError::InvalidInput);
        require!(
            planning_type != FinancialPlanningType::Basic,
            IndrasError::InvalidInput
        );
        
        plan.plan_id = planning_id;
        plan.entity_id = Some(entity_id);
        plan.planning_type = planning_type;
        plan.total_budget = 0; // Set separately for advanced planning
        plan.status = FinancialPlanStatus::Draft;
        plan.created_at = current_time;
        plan.period_start = None;
        plan.period_end = None;
        plan.plan_data_hash = planning_config_hash;
        plan.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for financial planning
pub mod offchain {
    /// Generate financial forecast (basic)
    pub fn generate_financial_forecast(_plan_id: u64) -> Vec<u8> {
        // Implementation in off-chain service
        vec![]
    }

    /// Develop financial plan (advanced)
    pub fn develop_financial_plan(_planning_id: u64) -> Vec<u8> {
        // Implementation in off-chain service
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_plan() {
        let mut plan = FinancialPlanMetadata {
            plan_id: 0,
            entity_id: Some(999),
            planning_type: FinancialPlanningType::Strategic,
            total_budget: 999,
            status: FinancialPlanStatus::Archived,
            created_at: 0,
            period_start: Some(0),
            period_end: Some(0),
            plan_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_plan(
            &mut plan,
            1,
            10000,
            [1u8; 32],
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(plan.plan_id, 1);
        assert_eq!(plan.entity_id, None);
        assert_eq!(plan.planning_type, FinancialPlanningType::Basic);
        assert_eq!(plan.total_budget, 10000);
        assert_eq!(plan.status, FinancialPlanStatus::Draft);
        assert_eq!(plan.created_at, 1500);
        assert_eq!(plan.period_start, Some(1000));
        assert_eq!(plan.period_end, Some(2000));
        assert_eq!(plan.plan_data_hash, [1u8; 32]);
        assert_eq!(plan.bump, 255);
    }

    #[test]
    fn test_initialize_financial_plan_invalid_id() {
        let mut plan = FinancialPlanMetadata {
            plan_id: 0,
            entity_id: None,
            planning_type: FinancialPlanningType::Basic,
            total_budget: 0,
            status: FinancialPlanStatus::Draft,
            created_at: 0,
            period_start: None,
            period_end: None,
            plan_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_plan(
            &mut plan,
            0, // Invalid: must be > 0
            10000,
            [1u8; 32],
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_plan_invalid_budget() {
        let mut plan = FinancialPlanMetadata {
            plan_id: 0,
            entity_id: None,
            planning_type: FinancialPlanningType::Basic,
            total_budget: 0,
            status: FinancialPlanStatus::Draft,
            created_at: 0,
            period_start: None,
            period_end: None,
            plan_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_plan(
            &mut plan,
            1,
            0, // Invalid: must be > 0
            [1u8; 32],
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_plan_invalid_period() {
        let mut plan = FinancialPlanMetadata {
            plan_id: 0,
            entity_id: None,
            planning_type: FinancialPlanningType::Basic,
            total_budget: 0,
            status: FinancialPlanStatus::Draft,
            created_at: 0,
            period_start: None,
            period_end: None,
            plan_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_plan(
            &mut plan,
            1,
            10000,
            [1u8; 32],
            2000,
            1000, // Invalid: end <= start
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_advanced_financial_planning() {
        let mut plan = FinancialPlanMetadata {
            plan_id: 0,
            entity_id: None,
            planning_type: FinancialPlanningType::Basic,
            total_budget: 999,
            status: FinancialPlanStatus::Archived,
            created_at: 0,
            period_start: Some(999),
            period_end: Some(999),
            plan_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_advanced_financial_planning(
            &mut plan,
            1,
            10,
            FinancialPlanningType::Strategic,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(plan.plan_id, 1);
        assert_eq!(plan.entity_id, Some(10));
        assert_eq!(plan.planning_type, FinancialPlanningType::Strategic);
        assert_eq!(plan.total_budget, 0);
        assert_eq!(plan.status, FinancialPlanStatus::Draft);
        assert_eq!(plan.created_at, 1000);
        assert_eq!(plan.period_start, None);
        assert_eq!(plan.period_end, None);
        assert_eq!(plan.plan_data_hash, [1u8; 32]);
        assert_eq!(plan.bump, 255);
    }

    #[test]
    fn test_initialize_advanced_financial_planning_invalid_id() {
        let mut plan = FinancialPlanMetadata {
            plan_id: 0,
            entity_id: None,
            planning_type: FinancialPlanningType::Basic,
            total_budget: 0,
            status: FinancialPlanStatus::Draft,
            created_at: 0,
            period_start: None,
            period_end: None,
            plan_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_advanced_financial_planning(
            &mut plan,
            0, // Invalid: must be > 0
            10,
            FinancialPlanningType::Strategic,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_advanced_financial_planning_basic_type() {
        let mut plan = FinancialPlanMetadata {
            plan_id: 0,
            entity_id: None,
            planning_type: FinancialPlanningType::Basic,
            total_budget: 0,
            status: FinancialPlanStatus::Draft,
            created_at: 0,
            period_start: None,
            period_end: None,
            plan_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_advanced_financial_planning(
            &mut plan,
            1,
            10,
            FinancialPlanningType::Basic, // Invalid: cannot use Basic for advanced planning
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_advanced_financial_planning_all_types() {
        let types = vec![
            FinancialPlanningType::Strategic,
            FinancialPlanningType::Tactical,
            FinancialPlanningType::Operational,
            FinancialPlanningType::Custom,
        ];

        for planning_type in types {
            let mut plan = FinancialPlanMetadata {
                plan_id: 0,
                entity_id: None,
                planning_type: FinancialPlanningType::Basic,
                total_budget: 0,
                status: FinancialPlanStatus::Draft,
                created_at: 0,
                period_start: None,
                period_end: None,
                plan_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_advanced_financial_planning(
                &mut plan,
                1,
                10,
                planning_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(plan.planning_type, planning_type);
        }
    }

    #[test]
    fn test_financial_plan_status_variants() {
        assert_eq!(FinancialPlanStatus::Draft, FinancialPlanStatus::Draft);
        assert_eq!(FinancialPlanStatus::Approved, FinancialPlanStatus::Approved);
        assert_eq!(FinancialPlanStatus::Active, FinancialPlanStatus::Active);
        assert_eq!(FinancialPlanStatus::Closed, FinancialPlanStatus::Closed);
        assert_eq!(FinancialPlanStatus::Archived, FinancialPlanStatus::Archived);
    }

    #[test]
    fn test_financial_planning_type_variants() {
        assert_eq!(FinancialPlanningType::Basic, FinancialPlanningType::Basic);
        assert_eq!(FinancialPlanningType::Strategic, FinancialPlanningType::Strategic);
        assert_eq!(FinancialPlanningType::Tactical, FinancialPlanningType::Tactical);
        assert_eq!(FinancialPlanningType::Operational, FinancialPlanningType::Operational);
        assert_eq!(FinancialPlanningType::Custom, FinancialPlanningType::Custom);
    }

    #[test]
    fn test_financial_plan_status_all_variants_unique() {
        let variants = vec![
            FinancialPlanStatus::Draft,
            FinancialPlanStatus::Approved,
            FinancialPlanStatus::Active,
            FinancialPlanStatus::Closed,
            FinancialPlanStatus::Archived,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_planning_type_all_variants_unique() {
        let variants = vec![
            FinancialPlanningType::Basic,
            FinancialPlanningType::Strategic,
            FinancialPlanningType::Tactical,
            FinancialPlanningType::Operational,
            FinancialPlanningType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_generate_financial_forecast() {
        let result = offchain::generate_financial_forecast(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_develop_financial_plan() {
        let result = offchain::develop_financial_plan(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
