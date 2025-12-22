//! Financial Strategic Planning module
//!
//! Financial strategic planning
//!
//! On-chain: Metadata for strategic planning
//! Off-chain: Actual planning, strategy development

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Planning horizon
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialStrategicPlanningHorizon {
    /// Short-term
    ShortTerm,
    /// Medium-term
    MediumTerm,
    /// Long-term
    LongTerm,
    /// Custom horizon
    Custom,
}

/// Planning status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialStrategicPlanningStatus {
    /// Planning active
    Active,
    /// Planning paused
    Paused,
    /// Planning archived
    Archived,
}

/// Financial strategic planning metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialStrategicPlanningMetadata {
    /// Planning ID
    pub planning_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Planning horizon
    pub planning_horizon: FinancialStrategicPlanningHorizon,
    /// Status
    pub status: FinancialStrategicPlanningStatus,
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
    pub fn initialize_financial_strategic_planning(
        planning: &mut FinancialStrategicPlanningMetadata,
        planning_id: u64,
        entity_id: u64,
        planning_horizon: FinancialStrategicPlanningHorizon,
        planning_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(planning_id > 0, IndrasError::InvalidInput);
        planning.planning_id = planning_id;
        planning.entity_id = entity_id;
        planning.planning_horizon = planning_horizon;
        planning.status = FinancialStrategicPlanningStatus::Active;
        planning.created_at = current_time;
        planning.planning_config_hash = planning_config_hash;
        planning.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn develop_strategic_plan(_planning_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_strategic_planning() {
        let mut planning = FinancialStrategicPlanningMetadata {
            planning_id: 0,
            entity_id: 0,
            planning_horizon: FinancialStrategicPlanningHorizon::ShortTerm,
            status: FinancialStrategicPlanningStatus::Archived,
            created_at: 0,
            planning_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_strategic_planning(
            &mut planning,
            1,
            10,
            FinancialStrategicPlanningHorizon::LongTerm,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(planning.planning_id, 1);
        assert_eq!(planning.entity_id, 10);
        assert_eq!(planning.planning_horizon, FinancialStrategicPlanningHorizon::LongTerm);
        assert_eq!(planning.status, FinancialStrategicPlanningStatus::Active);
        assert_eq!(planning.created_at, 1000);
        assert_eq!(planning.planning_config_hash, [1u8; 32]);
        assert_eq!(planning.bump, 255);
    }

    #[test]
    fn test_initialize_financial_strategic_planning_invalid_id() {
        let mut planning = FinancialStrategicPlanningMetadata {
            planning_id: 0,
            entity_id: 0,
            planning_horizon: FinancialStrategicPlanningHorizon::ShortTerm,
            status: FinancialStrategicPlanningStatus::Active,
            created_at: 0,
            planning_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_strategic_planning(
            &mut planning,
            0, // Invalid: must be > 0
            10,
            FinancialStrategicPlanningHorizon::LongTerm,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_strategic_planning_all_horizons() {
        let horizons = vec![
            FinancialStrategicPlanningHorizon::ShortTerm,
            FinancialStrategicPlanningHorizon::MediumTerm,
            FinancialStrategicPlanningHorizon::LongTerm,
            FinancialStrategicPlanningHorizon::Custom,
        ];

        for horizon in horizons {
            let mut planning = FinancialStrategicPlanningMetadata {
                planning_id: 0,
                entity_id: 0,
                planning_horizon: FinancialStrategicPlanningHorizon::ShortTerm,
                status: FinancialStrategicPlanningStatus::Active,
                created_at: 0,
                planning_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_strategic_planning(
                &mut planning,
                1,
                10,
                horizon,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(planning.planning_horizon, horizon);
        }
    }

    #[test]
    fn test_financial_strategic_planning_horizon_variants() {
        assert_eq!(FinancialStrategicPlanningHorizon::ShortTerm, FinancialStrategicPlanningHorizon::ShortTerm);
        assert_eq!(FinancialStrategicPlanningHorizon::MediumTerm, FinancialStrategicPlanningHorizon::MediumTerm);
        assert_eq!(FinancialStrategicPlanningHorizon::LongTerm, FinancialStrategicPlanningHorizon::LongTerm);
        assert_eq!(FinancialStrategicPlanningHorizon::Custom, FinancialStrategicPlanningHorizon::Custom);
    }

    #[test]
    fn test_financial_strategic_planning_status_variants() {
        assert_eq!(FinancialStrategicPlanningStatus::Active, FinancialStrategicPlanningStatus::Active);
        assert_eq!(FinancialStrategicPlanningStatus::Paused, FinancialStrategicPlanningStatus::Paused);
        assert_eq!(FinancialStrategicPlanningStatus::Archived, FinancialStrategicPlanningStatus::Archived);
    }

    #[test]
    fn test_financial_strategic_planning_horizon_all_variants_unique() {
        let variants = vec![
            FinancialStrategicPlanningHorizon::ShortTerm,
            FinancialStrategicPlanningHorizon::MediumTerm,
            FinancialStrategicPlanningHorizon::LongTerm,
            FinancialStrategicPlanningHorizon::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_strategic_planning_status_all_variants_unique() {
        let variants = vec![
            FinancialStrategicPlanningStatus::Active,
            FinancialStrategicPlanningStatus::Paused,
            FinancialStrategicPlanningStatus::Archived,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_develop_strategic_plan() {
        let result = offchain::develop_strategic_plan(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
