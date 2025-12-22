//! Grant Evaluation module
//!
//! Grant evaluation and assessment
//!
//! On-chain: Metadata for grant evaluation
//! Off-chain: Actual evaluation, assessment

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Evaluation criteria
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GrantEvaluationCriteria {
    /// Need assessment
    Need,
    /// Impact assessment
    Impact,
    /// Feasibility assessment
    Feasibility,
    /// Custom criteria
    Custom,
}

/// Evaluation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GrantEvaluationStatus {
    /// Evaluation pending
    Pending,
    /// Evaluation in progress
    InProgress,
    /// Evaluation completed
    Completed,
}

/// Grant evaluation metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct GrantEvaluationMetadata {
    /// Evaluation ID
    pub evaluation_id: u64,
    /// Grant ID
    pub grant_id: u64,
    /// Evaluation criteria
    pub evaluation_criteria: GrantEvaluationCriteria,
    /// Status
    pub status: GrantEvaluationStatus,
    /// Created at
    pub created_at: i64,
    /// Evaluation data hash
    pub evaluation_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    
    pub fn initialize_grant_evaluation(
        evaluation: &mut GrantEvaluationMetadata,
        evaluation_id: u64,
        grant_id: u64,
        evaluation_criteria: GrantEvaluationCriteria,
        evaluation_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(evaluation_id > 0, IndrasError::InvalidInput);
        evaluation.evaluation_id = evaluation_id;
        evaluation.grant_id = grant_id;
        evaluation.evaluation_criteria = evaluation_criteria;
        evaluation.status = GrantEvaluationStatus::Pending;
        evaluation.created_at = current_time;
        evaluation.evaluation_data_hash = evaluation_data_hash;
        evaluation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn evaluate_grant(_evaluation_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_grant_evaluation() {
        let mut evaluation = GrantEvaluationMetadata {
            evaluation_id: 0,
            grant_id: 0,
            evaluation_criteria: GrantEvaluationCriteria::Need,
            status: GrantEvaluationStatus::Pending,
            created_at: 0,
            evaluation_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_grant_evaluation(
            &mut evaluation,
            1,
            10,
            GrantEvaluationCriteria::Impact,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(evaluation.evaluation_id, 1);
        assert_eq!(evaluation.grant_id, 10);
        assert_eq!(evaluation.evaluation_criteria, GrantEvaluationCriteria::Impact);
        assert_eq!(evaluation.status, GrantEvaluationStatus::Pending);
        assert_eq!(evaluation.created_at, 1000);
        assert_eq!(evaluation.bump, 255);
    }

    #[test]
    fn test_initialize_grant_evaluation_invalid_id() {
        let mut evaluation = GrantEvaluationMetadata {
            evaluation_id: 0,
            grant_id: 0,
            evaluation_criteria: GrantEvaluationCriteria::Need,
            status: GrantEvaluationStatus::Pending,
            created_at: 0,
            evaluation_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_grant_evaluation(
            &mut evaluation,
            0, // Invalid: must be > 0
            10,
            GrantEvaluationCriteria::Impact,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_grant_evaluation_criteria_variants() {
        assert_eq!(GrantEvaluationCriteria::Need, GrantEvaluationCriteria::Need);
        assert_eq!(GrantEvaluationCriteria::Impact, GrantEvaluationCriteria::Impact);
        assert_eq!(GrantEvaluationCriteria::Feasibility, GrantEvaluationCriteria::Feasibility);
        assert_eq!(GrantEvaluationCriteria::Custom, GrantEvaluationCriteria::Custom);
    }

    #[test]
    fn test_grant_evaluation_status_variants() {
        assert_eq!(GrantEvaluationStatus::Pending, GrantEvaluationStatus::Pending);
        assert_eq!(GrantEvaluationStatus::InProgress, GrantEvaluationStatus::InProgress);
        assert_eq!(GrantEvaluationStatus::Completed, GrantEvaluationStatus::Completed);
    }

    fn create_test_evaluation() -> GrantEvaluationMetadata {
        GrantEvaluationMetadata {
            evaluation_id: 1,
            grant_id: 100,
            evaluation_criteria: GrantEvaluationCriteria::Need,
            status: GrantEvaluationStatus::Pending,
            created_at: 1000,
            evaluation_data_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_grant_evaluation_metadata_structure() {
        let evaluation = create_test_evaluation();
        assert_eq!(evaluation.evaluation_id, 1);
        assert_eq!(evaluation.grant_id, 100);
        assert_eq!(evaluation.evaluation_criteria, GrantEvaluationCriteria::Need);
        assert_eq!(evaluation.status, GrantEvaluationStatus::Pending);
        assert_eq!(evaluation.created_at, 1000);
        assert_eq!(evaluation.bump, 255);
    }

    #[test]
    fn test_initialize_grant_evaluation_all_criteria() {
        let criteria = vec![
            GrantEvaluationCriteria::Need,
            GrantEvaluationCriteria::Impact,
            GrantEvaluationCriteria::Feasibility,
            GrantEvaluationCriteria::Custom,
        ];

        for evaluation_criteria in criteria {
            let mut evaluation = GrantEvaluationMetadata {
                evaluation_id: 0,
                grant_id: 0,
                evaluation_criteria: GrantEvaluationCriteria::Need,
                status: GrantEvaluationStatus::Completed,
                created_at: 0,
                evaluation_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_grant_evaluation(
                &mut evaluation,
                1,
                1,
                evaluation_criteria,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(evaluation.evaluation_criteria, evaluation_criteria);
        }
    }

    #[test]
    fn test_initialize_grant_evaluation_status_always_pending_on_init() {
        let mut evaluation = create_test_evaluation();
        evaluation.status = GrantEvaluationStatus::Completed;

        let result = onchain::initialize_grant_evaluation(
            &mut evaluation,
            1,
            1,
            GrantEvaluationCriteria::Need,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // Status should always be set to Pending on initialization
        assert_eq!(evaluation.status, GrantEvaluationStatus::Pending);
    }

    #[test]
    fn test_initialize_grant_evaluation_data_hash() {
        let mut evaluation = create_test_evaluation();
        let custom_hash = [155u8; 32];

        let result = onchain::initialize_grant_evaluation(
            &mut evaluation,
            1,
            1,
            GrantEvaluationCriteria::Feasibility,
            custom_hash,
            5000,
            120,
        );

        assert!(result.is_ok());
        assert_eq!(evaluation.evaluation_data_hash, custom_hash);
    }

    #[test]
    fn test_initialize_grant_evaluation_grant_id() {
        let mut evaluation = create_test_evaluation();

        let result = onchain::initialize_grant_evaluation(
            &mut evaluation,
            1,
            55555,
            GrantEvaluationCriteria::Impact,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(evaluation.grant_id, 55555);
    }

    #[test]
    fn test_initialize_grant_evaluation_timestamp() {
        let mut evaluation = create_test_evaluation();

        let result = onchain::initialize_grant_evaluation(
            &mut evaluation,
            1,
            1,
            GrantEvaluationCriteria::Custom,
            [0u8; 32],
            11223,
            170,
        );

        assert!(result.is_ok());
        assert_eq!(evaluation.created_at, 11223);
    }

    #[test]
    fn test_initialize_grant_evaluation_bump_seed() {
        let mut evaluation = create_test_evaluation();

        for bump in [0u8, 110u8, 255u8] {
            let result = onchain::initialize_grant_evaluation(
                &mut evaluation,
                1,
                1,
                GrantEvaluationCriteria::Need,
                [0u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(evaluation.bump, bump);
        }
    }

    #[test]
    fn test_grant_evaluation_enum_equality() {
        // Test that enum variants can be compared
        let criteria1 = GrantEvaluationCriteria::Need;
        let criteria2 = GrantEvaluationCriteria::Need;
        let criteria3 = GrantEvaluationCriteria::Impact;

        assert_eq!(criteria1, criteria2);
        assert_ne!(criteria1, criteria3);

        let status1 = GrantEvaluationStatus::Pending;
        let status2 = GrantEvaluationStatus::Pending;
        let status3 = GrantEvaluationStatus::InProgress;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_grant_evaluation_criteria_all_variants_unique() {
        let criteria = vec![
            GrantEvaluationCriteria::Need,
            GrantEvaluationCriteria::Impact,
            GrantEvaluationCriteria::Feasibility,
            GrantEvaluationCriteria::Custom,
        ];
        
        for i in 0..criteria.len() {
            for j in (i + 1)..criteria.len() {
                assert_ne!(criteria[i], criteria[j], "Duplicate criteria found");
            }
        }
    }

    #[test]
    fn test_grant_evaluation_status_all_variants_unique() {
        let statuses = vec![
            GrantEvaluationStatus::Pending,
            GrantEvaluationStatus::InProgress,
            GrantEvaluationStatus::Completed,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_grant_evaluation_criteria_equality() {
        assert_eq!(GrantEvaluationCriteria::Need, GrantEvaluationCriteria::Need);
        assert_ne!(GrantEvaluationCriteria::Need, GrantEvaluationCriteria::Impact);
        assert_eq!(GrantEvaluationCriteria::Impact, GrantEvaluationCriteria::Impact);
        assert_ne!(GrantEvaluationCriteria::Impact, GrantEvaluationCriteria::Feasibility);
        assert_eq!(GrantEvaluationCriteria::Feasibility, GrantEvaluationCriteria::Feasibility);
        assert_ne!(GrantEvaluationCriteria::Feasibility, GrantEvaluationCriteria::Custom);
        assert_eq!(GrantEvaluationCriteria::Custom, GrantEvaluationCriteria::Custom);
    }

    #[test]
    fn test_grant_evaluation_status_equality() {
        assert_eq!(GrantEvaluationStatus::Pending, GrantEvaluationStatus::Pending);
        assert_ne!(GrantEvaluationStatus::Pending, GrantEvaluationStatus::InProgress);
        assert_eq!(GrantEvaluationStatus::InProgress, GrantEvaluationStatus::InProgress);
        assert_ne!(GrantEvaluationStatus::InProgress, GrantEvaluationStatus::Completed);
        assert_eq!(GrantEvaluationStatus::Completed, GrantEvaluationStatus::Completed);
    }

    #[test]
    fn test_grant_evaluation_criteria_copy() {
        let criteria1 = GrantEvaluationCriteria::Need;
        let criteria2 = criteria1; // Copy trait
        assert_eq!(criteria1, criteria2);
    }

    #[test]
    fn test_grant_evaluation_status_copy() {
        let status1 = GrantEvaluationStatus::Pending;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_grant_evaluation_criteria_space() {
        assert_eq!(<GrantEvaluationCriteria as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_grant_evaluation_status_space() {
        assert_eq!(<GrantEvaluationStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_grant_evaluation_large_ids() {
        let mut evaluation = GrantEvaluationMetadata {
            evaluation_id: 0,
            grant_id: 0,
            evaluation_criteria: GrantEvaluationCriteria::Need,
            status: GrantEvaluationStatus::Completed,
            created_at: 0,
            evaluation_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_grant_evaluation(
            &mut evaluation,
            u64::MAX,
            u64::MAX,
            GrantEvaluationCriteria::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(evaluation.evaluation_id, u64::MAX);
        assert_eq!(evaluation.grant_id, u64::MAX);
    }

    #[test]
    fn test_grant_evaluation_metadata_all_fields() {
        let evaluation = GrantEvaluationMetadata {
            evaluation_id: 123,
            grant_id: 456,
            evaluation_criteria: GrantEvaluationCriteria::Feasibility,
            status: GrantEvaluationStatus::InProgress,
            created_at: 5000,
            evaluation_data_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(evaluation.evaluation_id, 123);
        assert_eq!(evaluation.grant_id, 456);
        assert_eq!(evaluation.evaluation_criteria, GrantEvaluationCriteria::Feasibility);
        assert_eq!(evaluation.status, GrantEvaluationStatus::InProgress);
        assert_eq!(evaluation.created_at, 5000);
        assert_eq!(evaluation.evaluation_data_hash, [42u8; 32]);
        assert_eq!(evaluation.bump, 128);
    }
}
