//! Membership module
//!
//! DAO membership management
//!
//! On-chain: Metadata for memberships
//! Off-chain: Actual membership processing, validation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Membership status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MembershipStatus {
    /// Membership active
    Active,
    /// Membership inactive
    Inactive,
    /// Membership suspended
    Suspended,
}

/// Membership tier
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MembershipTier {
    /// Basic tier
    Basic,
    /// Standard tier
    Standard,
    /// Premium tier
    Premium,
    /// Enterprise tier
    Enterprise,
}

/// Membership metadata (on-chain)
///
/// Stores metadata for DAO memberships
#[account]
#[derive(InitSpace)]
pub struct MembershipMetadata {
    /// Membership ID
    pub membership_id: u64,
    /// Member pubkey
    pub member_pubkey: Pubkey,
    /// Tier
    pub tier: MembershipTier,
    /// Status
    pub status: MembershipStatus,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Membership data hash
    pub membership_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for membership
pub mod onchain {
    use super::*;

    /// Initialize membership
    pub fn initialize_membership(
        membership: &mut MembershipMetadata,
        membership_id: u64,
        member_pubkey: Pubkey,
        tier: MembershipTier,
        membership_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(membership_id > 0, IndrasError::InvalidInput);
        
        membership.membership_id = membership_id;
        membership.member_pubkey = member_pubkey;
        membership.tier = tier;
        membership.status = MembershipStatus::Active;
        membership.created_at = current_time;
        membership.updated_at = current_time;
        membership.membership_data_hash = membership_data_hash;
        membership.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for membership
pub mod offchain {
    /// Process membership
    pub fn process_membership(_membership_id: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    #[test]
    fn test_membership_status_variants() {
        assert_eq!(MembershipStatus::Active, MembershipStatus::Active);
        assert_eq!(MembershipStatus::Inactive, MembershipStatus::Inactive);
        assert_eq!(MembershipStatus::Suspended, MembershipStatus::Suspended);
    }

    #[test]
    fn test_membership_tier_variants() {
        assert_eq!(MembershipTier::Basic, MembershipTier::Basic);
        assert_eq!(MembershipTier::Standard, MembershipTier::Standard);
        assert_eq!(MembershipTier::Premium, MembershipTier::Premium);
        assert_eq!(MembershipTier::Enterprise, MembershipTier::Enterprise);
    }

    #[test]
    fn test_initialize_membership() {
        let mut membership = MembershipMetadata {
            membership_id: 0,
            member_pubkey: create_test_pubkey(1),
            tier: MembershipTier::Basic,
            status: MembershipStatus::Inactive,
            created_at: 0,
            updated_at: 0,
            membership_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_membership(
            &mut membership,
            1,
            create_test_pubkey(2),
            MembershipTier::Premium,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(membership.membership_id, 1);
        assert_eq!(membership.tier, MembershipTier::Premium);
        assert_eq!(membership.status, MembershipStatus::Active);
        assert_eq!(membership.created_at, 1000);
        assert_eq!(membership.updated_at, 1000);
        assert_eq!(membership.bump, 255);
    }

    #[test]
    fn test_initialize_membership_invalid_id() {
        let mut membership = MembershipMetadata {
            membership_id: 0,
            member_pubkey: create_test_pubkey(1),
            tier: MembershipTier::Basic,
            status: MembershipStatus::Inactive,
            created_at: 0,
            updated_at: 0,
            membership_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_membership(
            &mut membership,
            0, // Invalid: must be > 0
            create_test_pubkey(2),
            MembershipTier::Premium,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    fn create_test_membership() -> MembershipMetadata {
        MembershipMetadata {
            membership_id: 1,
            member_pubkey: create_test_pubkey(100),
            tier: MembershipTier::Standard,
            status: MembershipStatus::Active,
            created_at: 1000,
            updated_at: 1000,
            membership_data_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_membership_metadata_structure() {
        let membership = create_test_membership();
        assert_eq!(membership.membership_id, 1);
        assert_eq!(membership.member_pubkey, create_test_pubkey(100));
        assert_eq!(membership.tier, MembershipTier::Standard);
        assert_eq!(membership.status, MembershipStatus::Active);
        assert_eq!(membership.created_at, 1000);
        assert_eq!(membership.updated_at, 1000);
        assert_eq!(membership.bump, 255);
    }

    #[test]
    fn test_initialize_membership_all_tiers() {
        let tiers = vec![
            MembershipTier::Basic,
            MembershipTier::Standard,
            MembershipTier::Premium,
            MembershipTier::Enterprise,
        ];

        for tier in tiers {
            let mut membership = MembershipMetadata {
                membership_id: 0,
                member_pubkey: create_test_pubkey(1),
                tier: MembershipTier::Basic,
                status: MembershipStatus::Inactive,
                created_at: 0,
                updated_at: 0,
                membership_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_membership(
                &mut membership,
                1,
                create_test_pubkey(2),
                tier,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(membership.tier, tier);
        }
    }

    #[test]
    fn test_initialize_membership_status_always_active_on_init() {
        let mut membership = create_test_membership();
        membership.status = MembershipStatus::Suspended;

        let result = onchain::initialize_membership(
            &mut membership,
            1,
            create_test_pubkey(2),
            MembershipTier::Basic,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // Status should always be set to Active on initialization
        assert_eq!(membership.status, MembershipStatus::Active);
    }

    #[test]
    fn test_initialize_membership_data_hash() {
        let mut membership = create_test_membership();
        let custom_hash = [188u8; 32];

        let result = onchain::initialize_membership(
            &mut membership,
            1,
            create_test_pubkey(2),
            MembershipTier::Enterprise,
            custom_hash,
            3000,
            150,
        );

        assert!(result.is_ok());
        assert_eq!(membership.membership_data_hash, custom_hash);
    }

    #[test]
    fn test_initialize_membership_member_pubkey() {
        let mut membership = create_test_membership();
        let member = create_test_pubkey(77);

        let result = onchain::initialize_membership(
            &mut membership,
            1,
            member,
            MembershipTier::Premium,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(membership.member_pubkey, member);
    }

    #[test]
    fn test_initialize_membership_timestamp() {
        let mut membership = create_test_membership();

        let result = onchain::initialize_membership(
            &mut membership,
            1,
            create_test_pubkey(2),
            MembershipTier::Basic,
            [0u8; 32],
            44556,
            200,
        );

        assert!(result.is_ok());
        assert_eq!(membership.created_at, 44556);
        assert_eq!(membership.updated_at, 44556); // Both should be set
    }

    #[test]
    fn test_initialize_membership_bump_seed() {
        let mut membership = create_test_membership();

        for bump in [0u8, 140u8, 255u8] {
            let result = onchain::initialize_membership(
                &mut membership,
                1,
                create_test_pubkey(2),
                MembershipTier::Standard,
                [0u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(membership.bump, bump);
        }
    }

    #[test]
    fn test_membership_status_equality() {
        assert_eq!(MembershipStatus::Active, MembershipStatus::Active);
        assert_ne!(MembershipStatus::Active, MembershipStatus::Inactive);
        assert_ne!(MembershipStatus::Active, MembershipStatus::Suspended);
        assert_eq!(MembershipStatus::Inactive, MembershipStatus::Inactive);
        assert_ne!(MembershipStatus::Inactive, MembershipStatus::Suspended);
        assert_eq!(MembershipStatus::Suspended, MembershipStatus::Suspended);
    }

    #[test]
    fn test_membership_tier_equality() {
        assert_eq!(MembershipTier::Basic, MembershipTier::Basic);
        assert_ne!(MembershipTier::Basic, MembershipTier::Standard);
        assert_eq!(MembershipTier::Standard, MembershipTier::Standard);
        assert_ne!(MembershipTier::Standard, MembershipTier::Premium);
        assert_eq!(MembershipTier::Premium, MembershipTier::Premium);
        assert_ne!(MembershipTier::Premium, MembershipTier::Enterprise);
        assert_eq!(MembershipTier::Enterprise, MembershipTier::Enterprise);
    }

    #[test]
    fn test_initialize_membership_analytics_id_boundary() {
        let mut membership = create_test_membership();

        // Test with maximum valid ID (u64::MAX)
        let result = onchain::initialize_membership(
            &mut membership,
            u64::MAX,
            create_test_pubkey(2),
            MembershipTier::Basic,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(membership.membership_id, u64::MAX);
    }

    #[test]
    fn test_membership_status_all_variants_unique() {
        let statuses = vec![
            MembershipStatus::Active,
            MembershipStatus::Inactive,
            MembershipStatus::Suspended,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_membership_tier_all_variants_unique() {
        let tiers = vec![
            MembershipTier::Basic,
            MembershipTier::Standard,
            MembershipTier::Premium,
            MembershipTier::Enterprise,
        ];
        
        for i in 0..tiers.len() {
            for j in (i + 1)..tiers.len() {
                assert_ne!(tiers[i], tiers[j], "Duplicate tier found");
            }
        }
    }

    #[test]
    fn test_membership_status_copy() {
        let status1 = MembershipStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_membership_tier_copy() {
        let tier1 = MembershipTier::Premium;
        let tier2 = tier1; // Copy trait
        assert_eq!(tier1, tier2);
    }

    #[test]
    fn test_membership_status_space() {
        assert_eq!(<MembershipStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_membership_tier_space() {
        assert_eq!(<MembershipTier as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_membership_large_ids() {
        let mut membership = MembershipMetadata {
            membership_id: 0,
            member_pubkey: create_test_pubkey(1),
            tier: MembershipTier::Basic,
            status: MembershipStatus::Inactive,
            created_at: 0,
            updated_at: 0,
            membership_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_membership(
            &mut membership,
            u64::MAX,
            create_test_pubkey(2),
            MembershipTier::Enterprise,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(membership.membership_id, u64::MAX);
    }

    #[test]
    fn test_membership_metadata_all_fields() {
        let membership = MembershipMetadata {
            membership_id: 123,
            member_pubkey: create_test_pubkey(42),
            tier: MembershipTier::Premium,
            status: MembershipStatus::Suspended,
            created_at: 5000,
            updated_at: 6000,
            membership_data_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(membership.membership_id, 123);
        assert_eq!(membership.member_pubkey, create_test_pubkey(42));
        assert_eq!(membership.tier, MembershipTier::Premium);
        assert_eq!(membership.status, MembershipStatus::Suspended);
        assert_eq!(membership.created_at, 5000);
        assert_eq!(membership.updated_at, 6000);
        assert_eq!(membership.membership_data_hash, [42u8; 32]);
        assert_eq!(membership.bump, 128);
    }
}
