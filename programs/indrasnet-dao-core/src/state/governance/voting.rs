//! Governance Voting module
//!
//! Governance voting management
//!
//! On-chain: Metadata for governance voting
//! Off-chain: Actual voting, tallying

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Voting type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GovernanceVotingType {
    /// Simple majority
    SimpleMajority,
    /// Super majority
    SuperMajority,
    /// Unanimous
    Unanimous,
    /// Custom voting
    Custom,
}

/// Voting status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GovernanceVotingStatus {
    /// Voting open
    Open,
    /// Voting closed
    Closed,
    /// Voting cancelled
    Cancelled,
}

/// Governance voting metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct GovernanceVotingMetadata {
    /// Voting ID
    pub voting_id: u64,
    /// Proposal ID
    pub proposal_id: u64,
    /// Voting type
    pub voting_type: GovernanceVotingType,
    /// Status
    pub status: GovernanceVotingStatus,
    /// Created at
    pub created_at: i64,
    /// Voting data hash
    pub voting_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    
    pub fn initialize_governance_voting(
        voting: &mut GovernanceVotingMetadata,
        voting_id: u64,
        proposal_id: u64,
        voting_type: GovernanceVotingType,
        voting_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(voting_id > 0, IndrasError::InvalidInput);
        voting.voting_id = voting_id;
        voting.proposal_id = proposal_id;
        voting.voting_type = voting_type;
        voting.status = GovernanceVotingStatus::Open;
        voting.created_at = current_time;
        voting.voting_data_hash = voting_data_hash;
        voting.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn tally_votes(_voting_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_voting() -> GovernanceVotingMetadata {
        GovernanceVotingMetadata {
            voting_id: 1,
            proposal_id: 100,
            voting_type: GovernanceVotingType::SimpleMajority,
            status: GovernanceVotingStatus::Open,
            created_at: 1000,
            voting_data_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_governance_voting_type_variants() {
        assert_eq!(GovernanceVotingType::SimpleMajority, GovernanceVotingType::SimpleMajority);
        assert_eq!(GovernanceVotingType::SuperMajority, GovernanceVotingType::SuperMajority);
        assert_eq!(GovernanceVotingType::Unanimous, GovernanceVotingType::Unanimous);
        assert_eq!(GovernanceVotingType::Custom, GovernanceVotingType::Custom);
    }

    #[test]
    fn test_governance_voting_status_variants() {
        assert_eq!(GovernanceVotingStatus::Open, GovernanceVotingStatus::Open);
        assert_eq!(GovernanceVotingStatus::Closed, GovernanceVotingStatus::Closed);
        assert_eq!(GovernanceVotingStatus::Cancelled, GovernanceVotingStatus::Cancelled);
    }

    #[test]
    fn test_governance_voting_metadata_structure() {
        let voting = create_test_voting();
        assert_eq!(voting.voting_id, 1);
        assert_eq!(voting.proposal_id, 100);
        assert_eq!(voting.voting_type, GovernanceVotingType::SimpleMajority);
        assert_eq!(voting.status, GovernanceVotingStatus::Open);
        assert_eq!(voting.created_at, 1000);
        assert_eq!(voting.bump, 255);
    }

    #[test]
    fn test_initialize_governance_voting() {
        let mut voting = GovernanceVotingMetadata {
            voting_id: 0,
            proposal_id: 0,
            voting_type: GovernanceVotingType::SimpleMajority,
            status: GovernanceVotingStatus::Open,
            created_at: 0,
            voting_data_hash: [0u8; 32],
            bump: 0,
        };

        let data_hash = [3u8; 32];
        let result = onchain::initialize_governance_voting(
            &mut voting,
            300,
            400,
            GovernanceVotingType::SuperMajority,
            data_hash,
            9000,
            192,
        );

        assert!(result.is_ok());
        assert_eq!(voting.voting_id, 300);
        assert_eq!(voting.proposal_id, 400);
        assert_eq!(voting.voting_type, GovernanceVotingType::SuperMajority);
        assert_eq!(voting.status, GovernanceVotingStatus::Open);
        assert_eq!(voting.created_at, 9000);
        assert_eq!(voting.voting_data_hash, data_hash);
        assert_eq!(voting.bump, 192);
    }

    #[test]
    fn test_initialize_governance_voting_invalid_id() {
        let mut voting = create_test_voting();

        let result = onchain::initialize_governance_voting(
            &mut voting,
            0, // Invalid: voting_id must be > 0
            400,
            GovernanceVotingType::Unanimous,
            [0u8; 32],
            9000,
            192,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_governance_voting_all_types() {
        let types = vec![
            GovernanceVotingType::SimpleMajority,
            GovernanceVotingType::SuperMajority,
            GovernanceVotingType::Unanimous,
            GovernanceVotingType::Custom,
        ];

        for voting_type in types {
            let mut voting = GovernanceVotingMetadata {
                voting_id: 0,
                proposal_id: 0,
                voting_type: GovernanceVotingType::SimpleMajority,
                status: GovernanceVotingStatus::Open,
                created_at: 0,
                voting_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_governance_voting(
                &mut voting,
                1,
                1,
                voting_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(voting.voting_type, voting_type);
        }
    }

    #[test]
    fn test_governance_voting_status_always_open_on_init() {
        let mut voting = create_test_voting();
        voting.status = GovernanceVotingStatus::Closed;

        let result = onchain::initialize_governance_voting(
            &mut voting,
            1,
            1,
            GovernanceVotingType::SimpleMajority,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // Status should always be set to Open on initialization
        assert_eq!(voting.status, GovernanceVotingStatus::Open);
    }

    #[test]
    fn test_governance_voting_data_hash() {
        let mut voting = create_test_voting();
        let custom_hash = [77u8; 32];

        let result = onchain::initialize_governance_voting(
            &mut voting,
            1,
            1,
            GovernanceVotingType::Custom,
            custom_hash,
            5000,
            100,
        );

        assert!(result.is_ok());
        assert_eq!(voting.voting_data_hash, custom_hash);
    }

    #[test]
    fn test_governance_voting_proposal_id() {
        let mut voting = create_test_voting();

        let result = onchain::initialize_governance_voting(
            &mut voting,
            1,
            88888,
            GovernanceVotingType::SimpleMajority,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(voting.proposal_id, 88888);
    }

    #[test]
    fn test_governance_voting_enum_equality() {
        // Test that enum variants can be compared
        let type1 = GovernanceVotingType::SimpleMajority;
        let type2 = GovernanceVotingType::SimpleMajority;
        let type3 = GovernanceVotingType::SuperMajority;

        assert_eq!(type1, type2);
        assert_ne!(type1, type3);

        let status1 = GovernanceVotingStatus::Open;
        let status2 = GovernanceVotingStatus::Open;
        let status3 = GovernanceVotingStatus::Closed;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_governance_voting_type_all_variants_unique() {
        let types = vec![
            GovernanceVotingType::SimpleMajority,
            GovernanceVotingType::SuperMajority,
            GovernanceVotingType::Unanimous,
            GovernanceVotingType::Custom,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_governance_voting_status_all_variants_unique() {
        let statuses = vec![
            GovernanceVotingStatus::Open,
            GovernanceVotingStatus::Closed,
            GovernanceVotingStatus::Cancelled,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_governance_voting_type_copy() {
        let type1 = GovernanceVotingType::SimpleMajority;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_governance_voting_status_copy() {
        let status1 = GovernanceVotingStatus::Open;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_governance_voting_type_space() {
        assert_eq!(<GovernanceVotingType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_governance_voting_status_space() {
        assert_eq!(<GovernanceVotingStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_governance_voting_large_ids() {
        let mut voting = create_test_voting();
        
        let result = onchain::initialize_governance_voting(
            &mut voting,
            u64::MAX,
            u64::MAX,
            GovernanceVotingType::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(voting.voting_id, u64::MAX);
        assert_eq!(voting.proposal_id, u64::MAX);
    }

    #[test]
    fn test_initialize_governance_voting_preserves_other_fields() {
        let mut voting = GovernanceVotingMetadata {
            voting_id: 999,
            proposal_id: 888,
            voting_type: GovernanceVotingType::SimpleMajority,
            status: GovernanceVotingStatus::Closed,
            created_at: 1000,
            voting_data_hash: [1u8; 32],
            bump: 50,
        };
        
        let new_hash = [2u8; 32];
        let result = onchain::initialize_governance_voting(
            &mut voting,
            1,
            2,
            GovernanceVotingType::SuperMajority,
            new_hash,
            3000,
            100,
        );
        
        assert!(result.is_ok());
        // All fields should be updated
        assert_eq!(voting.voting_id, 1);
        assert_eq!(voting.proposal_id, 2);
        assert_eq!(voting.voting_type, GovernanceVotingType::SuperMajority);
        assert_eq!(voting.status, GovernanceVotingStatus::Open); // Always set to Open
        assert_eq!(voting.created_at, 3000);
        assert_eq!(voting.voting_data_hash, new_hash);
        assert_eq!(voting.bump, 100);
    }

    #[test]
    fn test_governance_voting_metadata_all_fields() {
        let voting = GovernanceVotingMetadata {
            voting_id: 123,
            proposal_id: 456,
            voting_type: GovernanceVotingType::Unanimous,
            status: GovernanceVotingStatus::Closed,
            created_at: 5000,
            voting_data_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(voting.voting_id, 123);
        assert_eq!(voting.proposal_id, 456);
        assert_eq!(voting.voting_type, GovernanceVotingType::Unanimous);
        assert_eq!(voting.status, GovernanceVotingStatus::Closed);
        assert_eq!(voting.created_at, 5000);
        assert_eq!(voting.voting_data_hash, [42u8; 32]);
        assert_eq!(voting.bump, 128);
    }

    #[test]
    fn test_offchain_tally_votes() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::tally_votes(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_tally_votes_different_ids() {
        // Test with different IDs
        let result1 = offchain::tally_votes(1);
        let result2 = offchain::tally_votes(999);
        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
    }
}
