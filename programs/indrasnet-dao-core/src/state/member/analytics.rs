//! Member Analytics module
//!
//! Member analytics and metrics
//!
//! On-chain: Metadata for member analytics
//! Off-chain: Actual analytics, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analytics type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MemberAnalyticsType {
    /// Activity analytics
    Activity,
    /// Contribution analytics
    Contribution,
    /// Engagement analytics
    Engagement,
    /// Custom analytics
    Custom,
}

/// Analytics status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MemberAnalyticsStatus {
    /// Analytics active
    Active,
    /// Analytics paused
    Paused,
    /// Analytics disabled
    Disabled,
}

/// Member analytics metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct MemberAnalyticsMetadata {
    /// Analytics ID
    pub analytics_id: u64,
    /// Member ID
    pub member_id: u64,
    /// Analytics type
    pub analytics_type: MemberAnalyticsType,
    /// Status
    pub status: MemberAnalyticsStatus,
    /// Created at
    pub created_at: i64,
    /// Analytics config hash
    pub analytics_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    
    pub fn initialize_member_analytics(
        analytics: &mut MemberAnalyticsMetadata,
        analytics_id: u64,
        member_id: u64,
        analytics_type: MemberAnalyticsType,
        analytics_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analytics_id > 0, IndrasError::InvalidInput);
        analytics.analytics_id = analytics_id;
        analytics.member_id = member_id;
        analytics.analytics_type = analytics_type;
        analytics.status = MemberAnalyticsStatus::Active;
        analytics.created_at = current_time;
        analytics.analytics_config_hash = analytics_config_hash;
        analytics.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_member_analytics(_analytics_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_member_analytics() {
        let mut analytics = MemberAnalyticsMetadata {
            analytics_id: 0,
            member_id: 0,
            analytics_type: MemberAnalyticsType::Activity,
            status: MemberAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_member_analytics(
            &mut analytics,
            1,
            10,
            MemberAnalyticsType::Contribution,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, 1);
        assert_eq!(analytics.member_id, 10);
        assert_eq!(analytics.analytics_type, MemberAnalyticsType::Contribution);
        assert_eq!(analytics.status, MemberAnalyticsStatus::Active);
        assert_eq!(analytics.created_at, 1000);
        assert_eq!(analytics.bump, 255);
    }

    #[test]
    fn test_initialize_member_analytics_invalid_id() {
        let mut analytics = MemberAnalyticsMetadata {
            analytics_id: 0,
            member_id: 0,
            analytics_type: MemberAnalyticsType::Activity,
            status: MemberAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_member_analytics(
            &mut analytics,
            0, // Invalid: must be > 0
            10,
            MemberAnalyticsType::Contribution,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_member_analytics_type_variants() {
        assert_eq!(MemberAnalyticsType::Activity, MemberAnalyticsType::Activity);
        assert_eq!(MemberAnalyticsType::Contribution, MemberAnalyticsType::Contribution);
        assert_eq!(MemberAnalyticsType::Engagement, MemberAnalyticsType::Engagement);
        assert_eq!(MemberAnalyticsType::Custom, MemberAnalyticsType::Custom);
    }

    #[test]
    fn test_member_analytics_status_variants() {
        assert_eq!(MemberAnalyticsStatus::Active, MemberAnalyticsStatus::Active);
        assert_eq!(MemberAnalyticsStatus::Paused, MemberAnalyticsStatus::Paused);
        assert_eq!(MemberAnalyticsStatus::Disabled, MemberAnalyticsStatus::Disabled);
    }

    fn create_test_analytics() -> MemberAnalyticsMetadata {
        MemberAnalyticsMetadata {
            analytics_id: 1,
            member_id: 100,
            analytics_type: MemberAnalyticsType::Activity,
            status: MemberAnalyticsStatus::Active,
            created_at: 1000,
            analytics_config_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_member_analytics_metadata_structure() {
        let analytics = create_test_analytics();
        assert_eq!(analytics.analytics_id, 1);
        assert_eq!(analytics.member_id, 100);
        assert_eq!(analytics.analytics_type, MemberAnalyticsType::Activity);
        assert_eq!(analytics.status, MemberAnalyticsStatus::Active);
        assert_eq!(analytics.created_at, 1000);
        assert_eq!(analytics.bump, 255);
    }

    #[test]
    fn test_initialize_member_analytics_all_types() {
        let types = vec![
            MemberAnalyticsType::Activity,
            MemberAnalyticsType::Contribution,
            MemberAnalyticsType::Engagement,
            MemberAnalyticsType::Custom,
        ];

        for analytics_type in types {
            let mut analytics = MemberAnalyticsMetadata {
                analytics_id: 0,
                member_id: 0,
                analytics_type: MemberAnalyticsType::Activity,
                status: MemberAnalyticsStatus::Disabled,
                created_at: 0,
                analytics_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_member_analytics(
                &mut analytics,
                1,
                1,
                analytics_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(analytics.analytics_type, analytics_type);
        }
    }

    #[test]
    fn test_initialize_member_analytics_status_always_active_on_init() {
        let mut analytics = create_test_analytics();
        analytics.status = MemberAnalyticsStatus::Paused;

        let result = onchain::initialize_member_analytics(
            &mut analytics,
            1,
            1,
            MemberAnalyticsType::Activity,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // Status should always be set to Active on initialization
        assert_eq!(analytics.status, MemberAnalyticsStatus::Active);
    }

    #[test]
    fn test_initialize_member_analytics_config_hash() {
        let mut analytics = create_test_analytics();
        let custom_hash = [222u8; 32];

        let result = onchain::initialize_member_analytics(
            &mut analytics,
            1,
            1,
            MemberAnalyticsType::Custom,
            custom_hash,
            2000,
            128,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.analytics_config_hash, custom_hash);
    }

    #[test]
    fn test_initialize_member_analytics_member_id() {
        let mut analytics = create_test_analytics();

        let result = onchain::initialize_member_analytics(
            &mut analytics,
            1,
            77777,
            MemberAnalyticsType::Engagement,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.member_id, 77777);
    }

    #[test]
    fn test_initialize_member_analytics_timestamp() {
        let mut analytics = create_test_analytics();

        let result = onchain::initialize_member_analytics(
            &mut analytics,
            1,
            1,
            MemberAnalyticsType::Contribution,
            [0u8; 32],
            45678,
            200,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.created_at, 45678);
    }

    #[test]
    fn test_initialize_member_analytics_bump_seed() {
        let mut analytics = create_test_analytics();

        for bump in [0u8, 50u8, 255u8] {
            let result = onchain::initialize_member_analytics(
                &mut analytics,
                1,
                1,
                MemberAnalyticsType::Activity,
                [0u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(analytics.bump, bump);
        }
    }

    #[test]
    fn test_member_analytics_enum_equality() {
        // Test that enum variants can be compared
        let type1 = MemberAnalyticsType::Activity;
        let type2 = MemberAnalyticsType::Activity;
        let type3 = MemberAnalyticsType::Contribution;

        assert_eq!(type1, type2);
        assert_ne!(type1, type3);

        let status1 = MemberAnalyticsStatus::Active;
        let status2 = MemberAnalyticsStatus::Active;
        let status3 = MemberAnalyticsStatus::Paused;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_member_analytics_type_all_variants_unique() {
        let types = vec![
            MemberAnalyticsType::Activity,
            MemberAnalyticsType::Contribution,
            MemberAnalyticsType::Engagement,
            MemberAnalyticsType::Custom,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_member_analytics_status_all_variants_unique() {
        let statuses = vec![
            MemberAnalyticsStatus::Active,
            MemberAnalyticsStatus::Paused,
            MemberAnalyticsStatus::Disabled,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_member_analytics_type_equality() {
        assert_eq!(MemberAnalyticsType::Activity, MemberAnalyticsType::Activity);
        assert_ne!(MemberAnalyticsType::Activity, MemberAnalyticsType::Contribution);
        assert_eq!(MemberAnalyticsType::Contribution, MemberAnalyticsType::Contribution);
        assert_ne!(MemberAnalyticsType::Contribution, MemberAnalyticsType::Engagement);
        assert_eq!(MemberAnalyticsType::Engagement, MemberAnalyticsType::Engagement);
        assert_ne!(MemberAnalyticsType::Engagement, MemberAnalyticsType::Custom);
        assert_eq!(MemberAnalyticsType::Custom, MemberAnalyticsType::Custom);
    }

    #[test]
    fn test_member_analytics_status_equality() {
        assert_eq!(MemberAnalyticsStatus::Active, MemberAnalyticsStatus::Active);
        assert_ne!(MemberAnalyticsStatus::Active, MemberAnalyticsStatus::Paused);
        assert_eq!(MemberAnalyticsStatus::Paused, MemberAnalyticsStatus::Paused);
        assert_ne!(MemberAnalyticsStatus::Paused, MemberAnalyticsStatus::Disabled);
        assert_eq!(MemberAnalyticsStatus::Disabled, MemberAnalyticsStatus::Disabled);
    }

    #[test]
    fn test_member_analytics_type_copy() {
        let type1 = MemberAnalyticsType::Activity;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_member_analytics_status_copy() {
        let status1 = MemberAnalyticsStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_member_analytics_type_space() {
        assert_eq!(<MemberAnalyticsType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_member_analytics_status_space() {
        assert_eq!(<MemberAnalyticsStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_member_analytics_large_ids() {
        let mut analytics = MemberAnalyticsMetadata {
            analytics_id: 0,
            member_id: 0,
            analytics_type: MemberAnalyticsType::Activity,
            status: MemberAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_member_analytics(
            &mut analytics,
            u64::MAX,
            u64::MAX,
            MemberAnalyticsType::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, u64::MAX);
        assert_eq!(analytics.member_id, u64::MAX);
    }

    #[test]
    fn test_member_analytics_metadata_all_fields() {
        let analytics = MemberAnalyticsMetadata {
            analytics_id: 123,
            member_id: 456,
            analytics_type: MemberAnalyticsType::Engagement,
            status: MemberAnalyticsStatus::Paused,
            created_at: 5000,
            analytics_config_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(analytics.analytics_id, 123);
        assert_eq!(analytics.member_id, 456);
        assert_eq!(analytics.analytics_type, MemberAnalyticsType::Engagement);
        assert_eq!(analytics.status, MemberAnalyticsStatus::Paused);
        assert_eq!(analytics.created_at, 5000);
        assert_eq!(analytics.analytics_config_hash, [42u8; 32]);
        assert_eq!(analytics.bump, 128);
    }
}
