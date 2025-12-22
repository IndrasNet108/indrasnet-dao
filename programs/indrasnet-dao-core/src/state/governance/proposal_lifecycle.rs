//! Proposal Lifecycle module
//!
//! Proposal lifecycle management
//!
//! On-chain: Metadata for proposal lifecycle stages
//! Off-chain: Actual lifecycle management, automation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Proposal lifecycle stage
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ProposalLifecycleStage {
    /// Draft stage
    Draft,
    /// Review stage
    Review,
    /// Voting stage
    Voting,
    /// Execution stage
    Execution,
    /// Completed stage
    Completed,
}

/// Proposal lifecycle metadata (on-chain)
///
/// Stores proposal lifecycle information
#[account]
#[derive(InitSpace)]
pub struct ProposalLifecycleMetadata {
    /// Lifecycle ID
    pub lifecycle_id: u64,
    /// Proposal ID
    pub proposal_id: u64,
    /// Current stage
    pub current_stage: ProposalLifecycleStage,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for proposal lifecycle
pub mod onchain {
    use super::*;

    /// Initialize proposal lifecycle
    pub fn initialize_lifecycle(
        lifecycle: &mut ProposalLifecycleMetadata,
        lifecycle_id: u64,
        proposal_id: u64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(lifecycle_id > 0, IndrasError::InvalidInput);
        
        lifecycle.lifecycle_id = lifecycle_id;
        lifecycle.proposal_id = proposal_id;
        lifecycle.current_stage = ProposalLifecycleStage::Draft;
        lifecycle.created_at = current_time;
        lifecycle.updated_at = current_time;
        lifecycle.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for proposal lifecycle
pub mod offchain {
    /// Advance proposal lifecycle
    pub fn advance_lifecycle(_proposal_id: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_lifecycle() -> ProposalLifecycleMetadata {
        ProposalLifecycleMetadata {
            lifecycle_id: 1,
            proposal_id: 100,
            current_stage: ProposalLifecycleStage::Draft,
            created_at: 1000,
            updated_at: 1000,
            bump: 255,
        }
    }

    #[test]
    fn test_proposal_lifecycle_stage_variants() {
        assert_eq!(ProposalLifecycleStage::Draft, ProposalLifecycleStage::Draft);
        assert_eq!(ProposalLifecycleStage::Review, ProposalLifecycleStage::Review);
        assert_eq!(ProposalLifecycleStage::Voting, ProposalLifecycleStage::Voting);
        assert_eq!(ProposalLifecycleStage::Execution, ProposalLifecycleStage::Execution);
        assert_eq!(ProposalLifecycleStage::Completed, ProposalLifecycleStage::Completed);
    }

    #[test]
    fn test_proposal_lifecycle_metadata_structure() {
        let lifecycle = create_test_lifecycle();
        assert_eq!(lifecycle.lifecycle_id, 1);
        assert_eq!(lifecycle.proposal_id, 100);
        assert_eq!(lifecycle.current_stage, ProposalLifecycleStage::Draft);
        assert_eq!(lifecycle.created_at, 1000);
        assert_eq!(lifecycle.updated_at, 1000);
        assert_eq!(lifecycle.bump, 255);
    }

    #[test]
    fn test_initialize_lifecycle() {
        let mut lifecycle = ProposalLifecycleMetadata {
            lifecycle_id: 0,
            proposal_id: 0,
            current_stage: ProposalLifecycleStage::Draft,
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let result = onchain::initialize_lifecycle(
            &mut lifecycle,
            500,
            600,
            8000,
            128,
        );

        assert!(result.is_ok());
        assert_eq!(lifecycle.lifecycle_id, 500);
        assert_eq!(lifecycle.proposal_id, 600);
        assert_eq!(lifecycle.current_stage, ProposalLifecycleStage::Draft);
        assert_eq!(lifecycle.created_at, 8000);
        assert_eq!(lifecycle.updated_at, 8000);
        assert_eq!(lifecycle.bump, 128);
    }

    #[test]
    fn test_initialize_lifecycle_invalid_id() {
        let mut lifecycle = create_test_lifecycle();

        let result = onchain::initialize_lifecycle(
            &mut lifecycle,
            0, // Invalid: lifecycle_id must be > 0
            600,
            8000,
            128,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_lifecycle_always_starts_at_draft() {
        let mut lifecycle = create_test_lifecycle();
        lifecycle.current_stage = ProposalLifecycleStage::Completed;

        let result = onchain::initialize_lifecycle(
            &mut lifecycle,
            1,
            1,
            1000,
            255,
        );

        assert!(result.is_ok());
        // Should always start at Draft stage
        assert_eq!(lifecycle.current_stage, ProposalLifecycleStage::Draft);
    }

    #[test]
    fn test_initialize_lifecycle_timestamps() {
        let mut lifecycle = create_test_lifecycle();

        let result = onchain::initialize_lifecycle(
            &mut lifecycle,
            1,
            1,
            12345,
            200,
        );

        assert!(result.is_ok());
        // Both created_at and updated_at should be set to current_time
        assert_eq!(lifecycle.created_at, 12345);
        assert_eq!(lifecycle.updated_at, 12345);
    }

    #[test]
    fn test_initialize_lifecycle_proposal_id() {
        let mut lifecycle = create_test_lifecycle();

        let result = onchain::initialize_lifecycle(
            &mut lifecycle,
            1,
            99999,
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(lifecycle.proposal_id, 99999);
    }

    #[test]
    fn test_proposal_lifecycle_stage_enum_equality() {
        // Test that enum variants can be compared
        let stage1 = ProposalLifecycleStage::Draft;
        let stage2 = ProposalLifecycleStage::Draft;
        let stage3 = ProposalLifecycleStage::Voting;

        assert_eq!(stage1, stage2);
        assert_ne!(stage1, stage3);
    }

    #[test]
    fn test_proposal_lifecycle_all_stages() {
        let stages = vec![
            ProposalLifecycleStage::Draft,
            ProposalLifecycleStage::Review,
            ProposalLifecycleStage::Voting,
            ProposalLifecycleStage::Execution,
            ProposalLifecycleStage::Completed,
        ];

        for stage in stages {
            let mut lifecycle = create_test_lifecycle();
            lifecycle.current_stage = stage;
            assert_eq!(lifecycle.current_stage, stage);
        }
    }

    #[test]
    fn test_initialize_lifecycle_bump_seed() {
        let mut lifecycle = create_test_lifecycle();

        for bump in [0u8, 100u8, 255u8] {
            let result = onchain::initialize_lifecycle(
                &mut lifecycle,
                1,
                1,
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(lifecycle.bump, bump);
        }
    }

    #[test]
    fn test_proposal_lifecycle_metadata_fields() {
        let lifecycle = ProposalLifecycleMetadata {
            lifecycle_id: 42,
            proposal_id: 84,
            current_stage: ProposalLifecycleStage::Voting,
            created_at: 5000,
            updated_at: 6000,
            bump: 128,
        };

        assert_eq!(lifecycle.lifecycle_id, 42);
        assert_eq!(lifecycle.proposal_id, 84);
        assert_eq!(lifecycle.current_stage, ProposalLifecycleStage::Voting);
        assert_eq!(lifecycle.created_at, 5000);
        assert_eq!(lifecycle.updated_at, 6000);
        assert_eq!(lifecycle.bump, 128);
    }

    #[test]
    fn test_initialize_lifecycle_large_ids() {
        let mut lifecycle = create_test_lifecycle();
        
        let result = onchain::initialize_lifecycle(
            &mut lifecycle,
            u64::MAX,
            u64::MAX,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(lifecycle.lifecycle_id, u64::MAX);
        assert_eq!(lifecycle.proposal_id, u64::MAX);
    }

    #[test]
    fn test_initialize_lifecycle_stage_always_draft() {
        let mut lifecycle = ProposalLifecycleMetadata {
            lifecycle_id: 0,
            proposal_id: 0,
            current_stage: ProposalLifecycleStage::Completed, // Will be reset
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };
        
        let result = onchain::initialize_lifecycle(
            &mut lifecycle,
            1,
            1,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(lifecycle.current_stage, ProposalLifecycleStage::Draft);
    }

    #[test]
    fn test_proposal_lifecycle_stage_all_variants_unique() {
        let stages = vec![
            ProposalLifecycleStage::Draft,
            ProposalLifecycleStage::Review,
            ProposalLifecycleStage::Voting,
            ProposalLifecycleStage::Execution,
            ProposalLifecycleStage::Completed,
        ];
        
        for i in 0..stages.len() {
            for j in (i + 1)..stages.len() {
                assert_ne!(stages[i], stages[j], "Duplicate stage found");
            }
        }
    }

    #[test]
    fn test_proposal_lifecycle_stage_equality() {
        assert_eq!(ProposalLifecycleStage::Draft, ProposalLifecycleStage::Draft);
        assert_ne!(ProposalLifecycleStage::Draft, ProposalLifecycleStage::Review);
        assert_ne!(ProposalLifecycleStage::Draft, ProposalLifecycleStage::Voting);
        assert_eq!(ProposalLifecycleStage::Review, ProposalLifecycleStage::Review);
        assert_eq!(ProposalLifecycleStage::Voting, ProposalLifecycleStage::Voting);
        assert_eq!(ProposalLifecycleStage::Execution, ProposalLifecycleStage::Execution);
        assert_eq!(ProposalLifecycleStage::Completed, ProposalLifecycleStage::Completed);
    }

    #[test]
    fn test_initialize_lifecycle_timestamps_same() {
        let mut lifecycle = create_test_lifecycle();
        
        let result = onchain::initialize_lifecycle(
            &mut lifecycle,
            1,
            1,
            7777,
            255,
        );
        
        assert!(result.is_ok());
        // Both timestamps should be the same
        assert_eq!(lifecycle.created_at, lifecycle.updated_at);
        assert_eq!(lifecycle.created_at, 7777);
    }

    #[test]
    fn test_initialize_lifecycle_different_proposal_ids() {
        let mut lifecycle = create_test_lifecycle();
        
        let proposal_ids = vec![1u64, 100u64, 1000u64, u64::MAX];
        
        for proposal_id in proposal_ids {
            let result = onchain::initialize_lifecycle(
                &mut lifecycle,
                1,
                proposal_id,
                1000,
                255,
            );
            
            assert!(result.is_ok());
            assert_eq!(lifecycle.proposal_id, proposal_id);
        }
    }

    #[test]
    fn test_proposal_lifecycle_metadata_clone() {
        let lifecycle1 = ProposalLifecycleMetadata {
            lifecycle_id: 1,
            proposal_id: 100,
            current_stage: ProposalLifecycleStage::Draft,
            created_at: 1000,
            updated_at: 1000,
            bump: 255,
        };
        
        // Test that Clone trait works
        let lifecycle2 = lifecycle1.clone();
        assert_eq!(lifecycle1.lifecycle_id, lifecycle2.lifecycle_id);
        assert_eq!(lifecycle1.proposal_id, lifecycle2.proposal_id);
        assert_eq!(lifecycle1.current_stage, lifecycle2.current_stage);
        assert_eq!(lifecycle1.created_at, lifecycle2.created_at);
        assert_eq!(lifecycle1.updated_at, lifecycle2.updated_at);
        assert_eq!(lifecycle1.bump, lifecycle2.bump);
    }

    #[test]
    fn test_initialize_lifecycle_zero_proposal_id_allowed() {
        let mut lifecycle = create_test_lifecycle();
        
        // proposal_id can be 0 (not validated)
        let result = onchain::initialize_lifecycle(
            &mut lifecycle,
            1,
            0,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(lifecycle.proposal_id, 0);
    }

    #[test]
    fn test_proposal_lifecycle_stage_copy() {
        let stage1 = ProposalLifecycleStage::Voting;
        let stage2 = stage1; // Copy trait
        assert_eq!(stage1, stage2);
    }

    #[test]
    fn test_proposal_lifecycle_stage_space() {
        assert_eq!(<ProposalLifecycleStage as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_offchain_advance_lifecycle() {
        // Test that offchain function exists and returns false (default)
        let result = offchain::advance_lifecycle(1);
        assert_eq!(result, false);
    }

    #[test]
    fn test_offchain_advance_lifecycle_different_ids() {
        // Test with different IDs
        let result1 = offchain::advance_lifecycle(1);
        let result2 = offchain::advance_lifecycle(999);
        assert_eq!(result1, false);
        assert_eq!(result2, false);
    }
}
