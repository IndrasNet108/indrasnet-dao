//! Member Contribution module
//!
//! Member contribution tracking
//!
//! On-chain: Metadata for member contributions
//! Off-chain: Actual tracking, evaluation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Contribution type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MemberContributionType {
    /// Code contribution
    Code,
    /// Documentation contribution
    Documentation,
    /// Design contribution
    Design,
    /// Custom contribution
    Custom,
}

/// Contribution status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MemberContributionStatus {
    /// Contribution pending
    Pending,
    /// Contribution reviewed
    Reviewed,
    /// Contribution accepted
    Accepted,
}

/// Member contribution metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct MemberContributionMetadata {
    /// Contribution ID
    pub contribution_id: u64,
    /// Member ID
    pub member_id: u64,
    /// Contribution type
    pub contribution_type: MemberContributionType,
    /// Status
    pub status: MemberContributionStatus,
    /// Created at
    pub created_at: i64,
    /// Contribution data hash
    pub contribution_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    
    pub fn initialize_member_contribution(
        contribution: &mut MemberContributionMetadata,
        contribution_id: u64,
        member_id: u64,
        contribution_type: MemberContributionType,
        contribution_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(contribution_id > 0, IndrasError::InvalidInput);
        contribution.contribution_id = contribution_id;
        contribution.member_id = member_id;
        contribution.contribution_type = contribution_type;
        contribution.status = MemberContributionStatus::Pending;
        contribution.created_at = current_time;
        contribution.contribution_data_hash = contribution_data_hash;
        contribution.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn track_member_contribution(_contribution_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_member_contribution() {
        let mut contribution = MemberContributionMetadata {
            contribution_id: 0,
            member_id: 0,
            contribution_type: MemberContributionType::Code,
            status: MemberContributionStatus::Accepted,
            created_at: 0,
            contribution_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_member_contribution(
            &mut contribution,
            1,
            10,
            MemberContributionType::Documentation,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(contribution.contribution_id, 1);
        assert_eq!(contribution.member_id, 10);
        assert_eq!(contribution.contribution_type, MemberContributionType::Documentation);
        assert_eq!(contribution.status, MemberContributionStatus::Pending);
        assert_eq!(contribution.created_at, 1000);
        assert_eq!(contribution.bump, 255);
    }

    #[test]
    fn test_initialize_member_contribution_invalid_id() {
        let mut contribution = MemberContributionMetadata {
            contribution_id: 0,
            member_id: 0,
            contribution_type: MemberContributionType::Code,
            status: MemberContributionStatus::Accepted,
            created_at: 0,
            contribution_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_member_contribution(
            &mut contribution,
            0, // Invalid: must be > 0
            10,
            MemberContributionType::Documentation,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_member_contribution_type_variants() {
        assert_eq!(MemberContributionType::Code, MemberContributionType::Code);
        assert_eq!(MemberContributionType::Documentation, MemberContributionType::Documentation);
        assert_eq!(MemberContributionType::Design, MemberContributionType::Design);
        assert_eq!(MemberContributionType::Custom, MemberContributionType::Custom);
    }

    #[test]
    fn test_member_contribution_status_variants() {
        assert_eq!(MemberContributionStatus::Pending, MemberContributionStatus::Pending);
        assert_eq!(MemberContributionStatus::Reviewed, MemberContributionStatus::Reviewed);
        assert_eq!(MemberContributionStatus::Accepted, MemberContributionStatus::Accepted);
    }

    fn create_test_contribution() -> MemberContributionMetadata {
        MemberContributionMetadata {
            contribution_id: 1,
            member_id: 100,
            contribution_type: MemberContributionType::Code,
            status: MemberContributionStatus::Pending,
            created_at: 1000,
            contribution_data_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_member_contribution_metadata_structure() {
        let contribution = create_test_contribution();
        assert_eq!(contribution.contribution_id, 1);
        assert_eq!(contribution.member_id, 100);
        assert_eq!(contribution.contribution_type, MemberContributionType::Code);
        assert_eq!(contribution.status, MemberContributionStatus::Pending);
        assert_eq!(contribution.created_at, 1000);
        assert_eq!(contribution.bump, 255);
    }

    #[test]
    fn test_initialize_member_contribution_all_types() {
        let types = vec![
            MemberContributionType::Code,
            MemberContributionType::Documentation,
            MemberContributionType::Design,
            MemberContributionType::Custom,
        ];

        for contribution_type in types {
            let mut contribution = MemberContributionMetadata {
                contribution_id: 0,
                member_id: 0,
                contribution_type: MemberContributionType::Code,
                status: MemberContributionStatus::Accepted,
                created_at: 0,
                contribution_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_member_contribution(
                &mut contribution,
                1,
                1,
                contribution_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(contribution.contribution_type, contribution_type);
        }
    }

    #[test]
    fn test_initialize_member_contribution_status_always_pending_on_init() {
        let mut contribution = create_test_contribution();
        contribution.status = MemberContributionStatus::Accepted;

        let result = onchain::initialize_member_contribution(
            &mut contribution,
            1,
            1,
            MemberContributionType::Code,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // Status should always be set to Pending on initialization
        assert_eq!(contribution.status, MemberContributionStatus::Pending);
    }

    #[test]
    fn test_initialize_member_contribution_data_hash() {
        let mut contribution = create_test_contribution();
        let custom_hash = [133u8; 32];

        let result = onchain::initialize_member_contribution(
            &mut contribution,
            1,
            1,
            MemberContributionType::Design,
            custom_hash,
            3000,
            150,
        );

        assert!(result.is_ok());
        assert_eq!(contribution.contribution_data_hash, custom_hash);
    }

    #[test]
    fn test_initialize_member_contribution_member_id() {
        let mut contribution = create_test_contribution();

        let result = onchain::initialize_member_contribution(
            &mut contribution,
            1,
            88888,
            MemberContributionType::Documentation,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(contribution.member_id, 88888);
    }

    #[test]
    fn test_initialize_member_contribution_timestamp() {
        let mut contribution = create_test_contribution();

        let result = onchain::initialize_member_contribution(
            &mut contribution,
            1,
            1,
            MemberContributionType::Custom,
            [0u8; 32],
            33333,
            180,
        );

        assert!(result.is_ok());
        assert_eq!(contribution.created_at, 33333);
    }

    #[test]
    fn test_initialize_member_contribution_bump_seed() {
        let mut contribution = create_test_contribution();

        for bump in [0u8, 75u8, 255u8] {
            let result = onchain::initialize_member_contribution(
                &mut contribution,
                1,
                1,
                MemberContributionType::Code,
                [0u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(contribution.bump, bump);
        }
    }

    #[test]
    fn test_member_contribution_enum_equality() {
        // Test that enum variants can be compared
        let type1 = MemberContributionType::Code;
        let type2 = MemberContributionType::Code;
        let type3 = MemberContributionType::Documentation;

        assert_eq!(type1, type2);
        assert_ne!(type1, type3);

        let status1 = MemberContributionStatus::Pending;
        let status2 = MemberContributionStatus::Pending;
        let status3 = MemberContributionStatus::Reviewed;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_member_contribution_type_all_variants_unique() {
        let types = vec![
            MemberContributionType::Code,
            MemberContributionType::Documentation,
            MemberContributionType::Design,
            MemberContributionType::Custom,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_member_contribution_status_all_variants_unique() {
        let statuses = vec![
            MemberContributionStatus::Pending,
            MemberContributionStatus::Reviewed,
            MemberContributionStatus::Accepted,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_member_contribution_type_equality() {
        assert_eq!(MemberContributionType::Code, MemberContributionType::Code);
        assert_ne!(MemberContributionType::Code, MemberContributionType::Documentation);
        assert_eq!(MemberContributionType::Documentation, MemberContributionType::Documentation);
        assert_ne!(MemberContributionType::Documentation, MemberContributionType::Design);
        assert_eq!(MemberContributionType::Design, MemberContributionType::Design);
        assert_ne!(MemberContributionType::Design, MemberContributionType::Custom);
        assert_eq!(MemberContributionType::Custom, MemberContributionType::Custom);
    }

    #[test]
    fn test_member_contribution_status_equality() {
        assert_eq!(MemberContributionStatus::Pending, MemberContributionStatus::Pending);
        assert_ne!(MemberContributionStatus::Pending, MemberContributionStatus::Reviewed);
        assert_eq!(MemberContributionStatus::Reviewed, MemberContributionStatus::Reviewed);
        assert_ne!(MemberContributionStatus::Reviewed, MemberContributionStatus::Accepted);
        assert_eq!(MemberContributionStatus::Accepted, MemberContributionStatus::Accepted);
    }

    #[test]
    fn test_member_contribution_type_copy() {
        let type1 = MemberContributionType::Code;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_member_contribution_status_copy() {
        let status1 = MemberContributionStatus::Pending;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_member_contribution_type_space() {
        assert_eq!(<MemberContributionType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_member_contribution_status_space() {
        assert_eq!(<MemberContributionStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_member_contribution_large_ids() {
        let mut contribution = MemberContributionMetadata {
            contribution_id: 0,
            member_id: 0,
            contribution_type: MemberContributionType::Code,
            status: MemberContributionStatus::Accepted,
            created_at: 0,
            contribution_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_member_contribution(
            &mut contribution,
            u64::MAX,
            u64::MAX,
            MemberContributionType::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(contribution.contribution_id, u64::MAX);
        assert_eq!(contribution.member_id, u64::MAX);
    }

    #[test]
    fn test_member_contribution_metadata_all_fields() {
        let contribution = MemberContributionMetadata {
            contribution_id: 123,
            member_id: 456,
            contribution_type: MemberContributionType::Design,
            status: MemberContributionStatus::Reviewed,
            created_at: 5000,
            contribution_data_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(contribution.contribution_id, 123);
        assert_eq!(contribution.member_id, 456);
        assert_eq!(contribution.contribution_type, MemberContributionType::Design);
        assert_eq!(contribution.status, MemberContributionStatus::Reviewed);
        assert_eq!(contribution.created_at, 5000);
        assert_eq!(contribution.contribution_data_hash, [42u8; 32]);
        assert_eq!(contribution.bump, 128);
    }

}
