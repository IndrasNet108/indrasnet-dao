//! Governance Participation module
//!
//! Governance participation tracking
//!
//! On-chain: Metadata for governance participation
//! Off-chain: Actual tracking, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Participation type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GovernanceParticipationType {
    /// Voting participation
    Voting,
    /// Proposal participation
    Proposal,
    /// Discussion participation
    Discussion,
    /// Custom participation
    Custom,
}

/// Participation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GovernanceParticipationStatus {
    /// Participation active
    Active,
    /// Participation paused
    Paused,
    /// Participation disabled
    Disabled,
}

/// Governance participation metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct GovernanceParticipationMetadata {
    /// Participation ID
    pub participation_id: u64,
    /// Member ID
    pub member_id: u64,
    /// Participation type
    pub participation_type: GovernanceParticipationType,
    /// Status
    pub status: GovernanceParticipationStatus,
    /// Created at
    pub created_at: i64,
    /// Participation config hash
    pub participation_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    
    pub fn initialize_governance_participation(
        participation: &mut GovernanceParticipationMetadata,
        participation_id: u64,
        member_id: u64,
        participation_type: GovernanceParticipationType,
        participation_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(participation_id > 0, IndrasError::InvalidInput);
        participation.participation_id = participation_id;
        participation.member_id = member_id;
        participation.participation_type = participation_type;
        participation.status = GovernanceParticipationStatus::Active;
        participation.created_at = current_time;
        participation.participation_config_hash = participation_config_hash;
        participation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn track_participation(_participation_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_participation() -> GovernanceParticipationMetadata {
        GovernanceParticipationMetadata {
            participation_id: 1,
            member_id: 100,
            participation_type: GovernanceParticipationType::Voting,
            status: GovernanceParticipationStatus::Active,
            created_at: 1000,
            participation_config_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_governance_participation_type_variants() {
        assert_eq!(GovernanceParticipationType::Voting, GovernanceParticipationType::Voting);
        assert_eq!(GovernanceParticipationType::Proposal, GovernanceParticipationType::Proposal);
        assert_eq!(GovernanceParticipationType::Discussion, GovernanceParticipationType::Discussion);
        assert_eq!(GovernanceParticipationType::Custom, GovernanceParticipationType::Custom);
    }

    #[test]
    fn test_governance_participation_status_variants() {
        assert_eq!(GovernanceParticipationStatus::Active, GovernanceParticipationStatus::Active);
        assert_eq!(GovernanceParticipationStatus::Paused, GovernanceParticipationStatus::Paused);
        assert_eq!(GovernanceParticipationStatus::Disabled, GovernanceParticipationStatus::Disabled);
    }

    #[test]
    fn test_governance_participation_metadata_structure() {
        let participation = create_test_participation();
        assert_eq!(participation.participation_id, 1);
        assert_eq!(participation.member_id, 100);
        assert_eq!(participation.participation_type, GovernanceParticipationType::Voting);
        assert_eq!(participation.status, GovernanceParticipationStatus::Active);
        assert_eq!(participation.created_at, 1000);
        assert_eq!(participation.bump, 255);
    }

    #[test]
    fn test_initialize_governance_participation() {
        let mut participation = GovernanceParticipationMetadata {
            participation_id: 0,
            member_id: 0,
            participation_type: GovernanceParticipationType::Voting,
            status: GovernanceParticipationStatus::Active,
            created_at: 0,
            participation_config_hash: [0u8; 32],
            bump: 0,
        };

        let config_hash = [2u8; 32];
        let result = onchain::initialize_governance_participation(
            &mut participation,
            200,
            300,
            GovernanceParticipationType::Proposal,
            config_hash,
            6000,
            64,
        );

        assert!(result.is_ok());
        assert_eq!(participation.participation_id, 200);
        assert_eq!(participation.member_id, 300);
        assert_eq!(participation.participation_type, GovernanceParticipationType::Proposal);
        assert_eq!(participation.status, GovernanceParticipationStatus::Active);
        assert_eq!(participation.created_at, 6000);
        assert_eq!(participation.participation_config_hash, config_hash);
        assert_eq!(participation.bump, 64);
    }

    #[test]
    fn test_initialize_governance_participation_invalid_id() {
        let mut participation = create_test_participation();
        let config_hash = [1u8; 32];

        let result = onchain::initialize_governance_participation(
            &mut participation,
            0, // Invalid: participation_id must be > 0
            300,
            GovernanceParticipationType::Discussion,
            config_hash,
            6000,
            64,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_governance_participation_all_types() {
        let types = vec![
            GovernanceParticipationType::Voting,
            GovernanceParticipationType::Proposal,
            GovernanceParticipationType::Discussion,
            GovernanceParticipationType::Custom,
        ];

        for participation_type in types {
            let mut participation = GovernanceParticipationMetadata {
                participation_id: 0,
                member_id: 0,
                participation_type: GovernanceParticipationType::Voting,
                status: GovernanceParticipationStatus::Active,
                created_at: 0,
                participation_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_governance_participation(
                &mut participation,
                1,
                1,
                participation_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(participation.participation_type, participation_type);
        }
    }

    #[test]
    fn test_governance_participation_member_id() {
        let mut participation = create_test_participation();

        let result = onchain::initialize_governance_participation(
            &mut participation,
            1,
            9999,
            GovernanceParticipationType::Voting,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(participation.member_id, 9999);
    }

    #[test]
    fn test_governance_participation_config_hash() {
        let mut participation = create_test_participation();
        let custom_hash = [99u8; 32];

        let result = onchain::initialize_governance_participation(
            &mut participation,
            1,
            1,
            GovernanceParticipationType::Custom,
            custom_hash,
            3000,
            150,
        );

        assert!(result.is_ok());
        assert_eq!(participation.participation_config_hash, custom_hash);
    }

    #[test]
    fn test_governance_participation_status_always_active_on_init() {
        let mut participation = create_test_participation();
        participation.status = GovernanceParticipationStatus::Disabled;

        let result = onchain::initialize_governance_participation(
            &mut participation,
            1,
            1,
            GovernanceParticipationType::Voting,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // Status should always be set to Active on initialization
        assert_eq!(participation.status, GovernanceParticipationStatus::Active);
    }

    #[test]
    fn test_governance_participation_timestamp() {
        let mut participation = create_test_participation();

        let result = onchain::initialize_governance_participation(
            &mut participation,
            1,
            1,
            GovernanceParticipationType::Discussion,
            [0u8; 32],
            7777,
            200,
        );

        assert!(result.is_ok());
        assert_eq!(participation.created_at, 7777);
    }

    #[test]
    fn test_governance_participation_enum_equality() {
        // Test that enum variants can be compared
        let type1 = GovernanceParticipationType::Voting;
        let type2 = GovernanceParticipationType::Voting;
        let type3 = GovernanceParticipationType::Proposal;

        assert_eq!(type1, type2);
        assert_ne!(type1, type3);

        let status1 = GovernanceParticipationStatus::Active;
        let status2 = GovernanceParticipationStatus::Active;
        let status3 = GovernanceParticipationStatus::Paused;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_governance_participation_type_all_variants_unique() {
        let types = vec![
            GovernanceParticipationType::Voting,
            GovernanceParticipationType::Proposal,
            GovernanceParticipationType::Discussion,
            GovernanceParticipationType::Custom,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_governance_participation_status_all_variants_unique() {
        let statuses = vec![
            GovernanceParticipationStatus::Active,
            GovernanceParticipationStatus::Paused,
            GovernanceParticipationStatus::Disabled,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_governance_participation_type_copy() {
        let type1 = GovernanceParticipationType::Voting;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_governance_participation_status_copy() {
        let status1 = GovernanceParticipationStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_governance_participation_type_space() {
        assert_eq!(<GovernanceParticipationType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_governance_participation_status_space() {
        assert_eq!(<GovernanceParticipationStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_governance_participation_large_ids() {
        let mut participation = create_test_participation();
        
        let result = onchain::initialize_governance_participation(
            &mut participation,
            u64::MAX,
            u64::MAX,
            GovernanceParticipationType::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(participation.participation_id, u64::MAX);
        assert_eq!(participation.member_id, u64::MAX);
    }

    #[test]
    fn test_initialize_governance_participation_preserves_other_fields() {
        let mut participation = GovernanceParticipationMetadata {
            participation_id: 999,
            member_id: 888,
            participation_type: GovernanceParticipationType::Voting,
            status: GovernanceParticipationStatus::Disabled,
            created_at: 1000,
            participation_config_hash: [1u8; 32],
            bump: 50,
        };
        
        let new_hash = [2u8; 32];
        let result = onchain::initialize_governance_participation(
            &mut participation,
            1,
            2,
            GovernanceParticipationType::Proposal,
            new_hash,
            3000,
            100,
        );
        
        assert!(result.is_ok());
        // All fields should be updated
        assert_eq!(participation.participation_id, 1);
        assert_eq!(participation.member_id, 2);
        assert_eq!(participation.participation_type, GovernanceParticipationType::Proposal);
        assert_eq!(participation.status, GovernanceParticipationStatus::Active); // Always set to Active
        assert_eq!(participation.created_at, 3000);
        assert_eq!(participation.participation_config_hash, new_hash);
        assert_eq!(participation.bump, 100);
    }

    #[test]
    fn test_governance_participation_metadata_all_fields() {
        let participation = GovernanceParticipationMetadata {
            participation_id: 123,
            member_id: 456,
            participation_type: GovernanceParticipationType::Discussion,
            status: GovernanceParticipationStatus::Paused,
            created_at: 5000,
            participation_config_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(participation.participation_id, 123);
        assert_eq!(participation.member_id, 456);
        assert_eq!(participation.participation_type, GovernanceParticipationType::Discussion);
        assert_eq!(participation.status, GovernanceParticipationStatus::Paused);
        assert_eq!(participation.created_at, 5000);
        assert_eq!(participation.participation_config_hash, [42u8; 32]);
        assert_eq!(participation.bump, 128);
    }

    #[test]
    fn test_offchain_track_participation() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::track_participation(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_track_participation_different_ids() {
        // Test with different IDs
        let result1 = offchain::track_participation(1);
        let result2 = offchain::track_participation(999);
        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
    }
}
