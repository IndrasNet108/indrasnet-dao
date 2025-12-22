//! Security Committees module
//!
//! Security committees management
//!
//! On-chain: Metadata for security committees
//! Off-chain: Actual committee coordination, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Committee member role
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum CommitteeMemberRole {
    /// Chairperson
    Chairperson,
    /// Member
    Member,
}

/// Security committee metadata (on-chain)
///
/// Stores metadata for security committees
#[account]
#[derive(InitSpace)]
pub struct SecurityCommitteeMetadata {
    /// Committee ID
    pub committee_id: u64,
    /// Committee name
    #[max_len(100)]
    pub name: String,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for security committees
pub mod onchain {
    use super::*;

    /// Initialize security committee
    pub fn initialize_committee(
        committee: &mut SecurityCommitteeMetadata,
        committee_id: u64,
        name: String,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(committee_id > 0, IndrasError::InvalidInput);
        require!(!name.is_empty(), IndrasError::InvalidInput);
        require!(name.len() <= 100, IndrasError::InvalidInput);
        
        committee.committee_id = committee_id;
        committee.name = name;
        committee.created_at = current_time;
        committee.updated_at = current_time;
        committee.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for security committees
pub mod offchain {
    /// Coordinate committee meeting
    pub fn coordinate_meeting(_committee_id: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_committee() -> SecurityCommitteeMetadata {
        SecurityCommitteeMetadata {
            committee_id: 1,
            name: "Test Committee".to_string(),
            created_at: 1000,
            updated_at: 1000,
            bump: 255,
        }
    }

    #[test]
    fn test_committee_member_role_variants() {
        assert_eq!(CommitteeMemberRole::Chairperson, CommitteeMemberRole::Chairperson);
        assert_eq!(CommitteeMemberRole::Member, CommitteeMemberRole::Member);
    }

    #[test]
    fn test_security_committee_metadata_structure() {
        let committee = create_test_committee();
        assert_eq!(committee.committee_id, 1);
        assert_eq!(committee.name, "Test Committee");
        assert_eq!(committee.created_at, 1000);
        assert_eq!(committee.updated_at, 1000);
        assert_eq!(committee.bump, 255);
    }

    #[test]
    fn test_initialize_committee() {
        let mut committee = SecurityCommitteeMetadata {
            committee_id: 0,
            name: String::new(),
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let result = onchain::initialize_committee(
            &mut committee,
            700,
            "Security Review Committee".to_string(),
            12000,
            32,
        );

        assert!(result.is_ok());
        assert_eq!(committee.committee_id, 700);
        assert_eq!(committee.name, "Security Review Committee");
        assert_eq!(committee.created_at, 12000);
        assert_eq!(committee.updated_at, 12000);
        assert_eq!(committee.bump, 32);
    }

    #[test]
    fn test_initialize_committee_invalid_id() {
        let mut committee = create_test_committee();

        let result = onchain::initialize_committee(
            &mut committee,
            0, // Invalid: committee_id must be > 0
            "Committee Name".to_string(),
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_committee_empty_name() {
        let mut committee = create_test_committee();

        let result = onchain::initialize_committee(
            &mut committee,
            1,
            String::new(), // Invalid: name must not be empty
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_committee_name_too_long() {
        let mut committee = create_test_committee();
        let long_name = "a".repeat(101); // 101 chars, max is 100

        let result = onchain::initialize_committee(
            &mut committee,
            1,
            long_name,
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_committee_name_max_length() {
        let mut committee = create_test_committee();
        let max_name = "a".repeat(100); // Exactly 100 chars

        let result = onchain::initialize_committee(
            &mut committee,
            1,
            max_name.clone(),
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(committee.name.len(), 100);
    }

    #[test]
    fn test_initialize_committee_timestamps() {
        let mut committee = create_test_committee();

        let result = onchain::initialize_committee(
            &mut committee,
            1,
            "Committee Name".to_string(),
            23456,
            100,
        );

        assert!(result.is_ok());
        // Both created_at and updated_at should be set to current_time
        assert_eq!(committee.created_at, 23456);
        assert_eq!(committee.updated_at, 23456);
    }

    #[test]
    fn test_initialize_committee_bump_seed() {
        let mut committee = create_test_committee();

        for bump in [0u8, 64u8, 255u8] {
            let result = onchain::initialize_committee(
                &mut committee,
                1,
                "Committee".to_string(),
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(committee.bump, bump);
        }
    }

    #[test]
    fn test_committee_member_role_enum_equality() {
        // Test that enum variants can be compared
        let role1 = CommitteeMemberRole::Chairperson;
        let role2 = CommitteeMemberRole::Chairperson;
        let role3 = CommitteeMemberRole::Member;

        assert_eq!(role1, role2);
        assert_ne!(role1, role3);
    }

    #[test]
    fn test_initialize_committee_various_names() {
        let names = vec![
            "A".to_string(),
            "Security Committee".to_string(),
            "a".repeat(50),
            "a".repeat(100),
        ];

        for name in names {
            let mut committee = create_test_committee();
            let result = onchain::initialize_committee(
                &mut committee,
                1,
                name.clone(),
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(committee.name, name);
        }
    }

    #[test]
    fn test_committee_member_role_all_variants_unique() {
        let roles = vec![
            CommitteeMemberRole::Chairperson,
            CommitteeMemberRole::Member,
        ];
        
        for i in 0..roles.len() {
            for j in (i + 1)..roles.len() {
                assert_ne!(roles[i], roles[j], "Duplicate role found");
            }
        }
    }

    #[test]
    fn test_committee_member_role_equality() {
        assert_eq!(CommitteeMemberRole::Chairperson, CommitteeMemberRole::Chairperson);
        assert_ne!(CommitteeMemberRole::Chairperson, CommitteeMemberRole::Member);
        assert_eq!(CommitteeMemberRole::Member, CommitteeMemberRole::Member);
    }

    #[test]
    fn test_committee_member_role_copy() {
        let role1 = CommitteeMemberRole::Chairperson;
        let role2 = role1; // Copy trait
        assert_eq!(role1, role2);
    }

    #[test]
    fn test_committee_member_role_space() {
        assert_eq!(<CommitteeMemberRole as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_committee_large_committee_id() {
        let mut committee = create_test_committee();
        
        let result = onchain::initialize_committee(
            &mut committee,
            u64::MAX,
            "Committee".to_string(),
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(committee.committee_id, u64::MAX);
    }

    #[test]
    fn test_initialize_committee_preserves_other_fields() {
        let mut committee = SecurityCommitteeMetadata {
            committee_id: 999,
            name: "Old Name".to_string(),
            created_at: 1000,
            updated_at: 2000,
            bump: 50,
        };
        
        let result = onchain::initialize_committee(
            &mut committee,
            1,
            "New Name".to_string(),
            3000,
            100,
        );
        
        assert!(result.is_ok());
        // All fields should be updated
        assert_eq!(committee.committee_id, 1);
        assert_eq!(committee.name, "New Name");
        assert_eq!(committee.created_at, 3000);
        assert_eq!(committee.updated_at, 3000);
        assert_eq!(committee.bump, 100);
    }

    #[test]
    fn test_security_committee_metadata_all_fields() {
        let committee = SecurityCommitteeMetadata {
            committee_id: 123,
            name: "Test Committee Name".to_string(),
            created_at: 5000,
            updated_at: 6000,
            bump: 128,
        };
        
        assert_eq!(committee.committee_id, 123);
        assert_eq!(committee.name, "Test Committee Name");
        assert_eq!(committee.created_at, 5000);
        assert_eq!(committee.updated_at, 6000);
        assert_eq!(committee.bump, 128);
    }

    #[test]
    fn test_offchain_coordinate_meeting() {
        // Test that offchain function exists and returns false (default)
        let result = offchain::coordinate_meeting(1);
        assert_eq!(result, false);
    }

    #[test]
    fn test_offchain_coordinate_meeting_different_ids() {
        // Test with different IDs
        let result1 = offchain::coordinate_meeting(1);
        let result2 = offchain::coordinate_meeting(999);
        assert_eq!(result1, false);
        assert_eq!(result2, false);
    }
}
