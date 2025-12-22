//! Financial Scenario Planning module
//!
//! Financial scenario planning
//!
//! On-chain: Metadata for scenario planning
//! Off-chain: Actual planning, scenario analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Scenario type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialScenarioType {
    /// Best case
    BestCase,
    /// Base case
    BaseCase,
    /// Worst case
    WorstCase,
    /// Custom scenario
    Custom,
}

/// Planning status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialScenarioPlanningStatus {
    /// Planning active
    Active,
    /// Planning paused
    Paused,
    /// Planning completed
    Completed,
}

/// Financial scenario planning metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialScenarioPlanningMetadata {
    /// Planning ID
    pub planning_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Scenario type
    pub scenario_type: FinancialScenarioType,
    /// Status
    pub status: FinancialScenarioPlanningStatus,
    /// Created at
    pub created_at: i64,
    /// Planning config hash
    pub planning_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_scenario_planning(
        planning: &mut FinancialScenarioPlanningMetadata,
        planning_id: u64,
        entity_id: u64,
        scenario_type: FinancialScenarioType,
        planning_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(planning_id > 0, IndrasError::InvalidInput);
        planning.planning_id = planning_id;
        planning.entity_id = entity_id;
        planning.scenario_type = scenario_type;
        planning.status = FinancialScenarioPlanningStatus::Active;
        planning.created_at = current_time;
        planning.planning_config_hash = planning_config_hash;
        planning.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn analyze_scenario(_planning_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_scenario_planning() {
        let mut planning = FinancialScenarioPlanningMetadata {
            planning_id: 0,
            entity_id: 0,
            scenario_type: FinancialScenarioType::BaseCase,
            status: FinancialScenarioPlanningStatus::Completed,
            created_at: 0,
            planning_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_scenario_planning(
            &mut planning,
            1,
            10,
            FinancialScenarioType::BestCase,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(planning.planning_id, 1);
        assert_eq!(planning.entity_id, 10);
        assert_eq!(planning.scenario_type, FinancialScenarioType::BestCase);
        assert_eq!(planning.status, FinancialScenarioPlanningStatus::Active);
        assert_eq!(planning.created_at, 1000);
        assert_eq!(planning.planning_config_hash, [1u8; 32]);
        assert_eq!(planning.bump, 255);
    }

    #[test]
    fn test_initialize_financial_scenario_planning_invalid_id() {
        let mut planning = FinancialScenarioPlanningMetadata {
            planning_id: 0,
            entity_id: 0,
            scenario_type: FinancialScenarioType::BaseCase,
            status: FinancialScenarioPlanningStatus::Active,
            created_at: 0,
            planning_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_scenario_planning(
            &mut planning,
            0, // Invalid: must be > 0
            10,
            FinancialScenarioType::BestCase,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_scenario_planning_all_types() {
        let types = vec![
            FinancialScenarioType::BestCase,
            FinancialScenarioType::BaseCase,
            FinancialScenarioType::WorstCase,
            FinancialScenarioType::Custom,
        ];

        for scenario_type in types {
            let mut planning = FinancialScenarioPlanningMetadata {
                planning_id: 0,
                entity_id: 0,
                scenario_type: FinancialScenarioType::BaseCase,
                status: FinancialScenarioPlanningStatus::Active,
                created_at: 0,
                planning_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_scenario_planning(
                &mut planning,
                1,
                10,
                scenario_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(planning.scenario_type, scenario_type);
        }
    }

    #[test]
    fn test_financial_scenario_type_variants() {
        assert_eq!(FinancialScenarioType::BestCase, FinancialScenarioType::BestCase);
        assert_eq!(FinancialScenarioType::BaseCase, FinancialScenarioType::BaseCase);
        assert_eq!(FinancialScenarioType::WorstCase, FinancialScenarioType::WorstCase);
        assert_eq!(FinancialScenarioType::Custom, FinancialScenarioType::Custom);
    }

    #[test]
    fn test_financial_scenario_planning_status_variants() {
        assert_eq!(FinancialScenarioPlanningStatus::Active, FinancialScenarioPlanningStatus::Active);
        assert_eq!(FinancialScenarioPlanningStatus::Paused, FinancialScenarioPlanningStatus::Paused);
        assert_eq!(FinancialScenarioPlanningStatus::Completed, FinancialScenarioPlanningStatus::Completed);
    }

    #[test]
    fn test_financial_scenario_type_all_variants_unique() {
        let variants = vec![
            FinancialScenarioType::BestCase,
            FinancialScenarioType::BaseCase,
            FinancialScenarioType::WorstCase,
            FinancialScenarioType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_scenario_planning_status_all_variants_unique() {
        let variants = vec![
            FinancialScenarioPlanningStatus::Active,
            FinancialScenarioPlanningStatus::Paused,
            FinancialScenarioPlanningStatus::Completed,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_analyze_scenario() {
        let result = offchain::analyze_scenario(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
