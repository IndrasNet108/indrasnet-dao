//! Grant Analytics module
//!
//! Grant analytics and metrics
//!
//! On-chain: Metadata for grant analytics
//! Off-chain: Actual analytics, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analytics type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GrantAnalyticsType {
    /// Distribution analytics
    Distribution,
    /// Impact analytics
    Impact,
    /// Performance analytics
    Performance,
    /// Custom analytics
    Custom,
}

/// Analytics status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GrantAnalyticsStatus {
    /// Analytics active
    Active,
    /// Analytics paused
    Paused,
    /// Analytics disabled
    Disabled,
}

/// Grant analytics metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct GrantAnalyticsMetadata {
    /// Analytics ID
    pub analytics_id: u64,
    /// Grant ID
    pub grant_id: u64,
    /// Analytics type
    pub analytics_type: GrantAnalyticsType,
    /// Status
    pub status: GrantAnalyticsStatus,
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
    
    pub fn initialize_grant_analytics(
        analytics: &mut GrantAnalyticsMetadata,
        analytics_id: u64,
        grant_id: u64,
        analytics_type: GrantAnalyticsType,
        analytics_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analytics_id > 0, IndrasError::InvalidInput);
        analytics.analytics_id = analytics_id;
        analytics.grant_id = grant_id;
        analytics.analytics_type = analytics_type;
        analytics.status = GrantAnalyticsStatus::Active;
        analytics.created_at = current_time;
        analytics.analytics_config_hash = analytics_config_hash;
        analytics.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_grant_analytics(_analytics_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_grant_analytics() {
        let mut analytics = GrantAnalyticsMetadata {
            analytics_id: 0,
            grant_id: 0,
            analytics_type: GrantAnalyticsType::Distribution,
            status: GrantAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_grant_analytics(
            &mut analytics,
            1,
            10,
            GrantAnalyticsType::Impact,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, 1);
        assert_eq!(analytics.grant_id, 10);
        assert_eq!(analytics.analytics_type, GrantAnalyticsType::Impact);
        assert_eq!(analytics.status, GrantAnalyticsStatus::Active);
        assert_eq!(analytics.created_at, 1000);
        assert_eq!(analytics.bump, 255);
    }

    #[test]
    fn test_initialize_grant_analytics_invalid_id() {
        let mut analytics = GrantAnalyticsMetadata {
            analytics_id: 0,
            grant_id: 0,
            analytics_type: GrantAnalyticsType::Distribution,
            status: GrantAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_grant_analytics(
            &mut analytics,
            0, // Invalid: must be > 0
            10,
            GrantAnalyticsType::Impact,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_grant_analytics_type_variants() {
        assert_eq!(GrantAnalyticsType::Distribution, GrantAnalyticsType::Distribution);
        assert_eq!(GrantAnalyticsType::Impact, GrantAnalyticsType::Impact);
        assert_eq!(GrantAnalyticsType::Performance, GrantAnalyticsType::Performance);
        assert_eq!(GrantAnalyticsType::Custom, GrantAnalyticsType::Custom);
    }

    #[test]
    fn test_grant_analytics_status_variants() {
        assert_eq!(GrantAnalyticsStatus::Active, GrantAnalyticsStatus::Active);
        assert_eq!(GrantAnalyticsStatus::Paused, GrantAnalyticsStatus::Paused);
        assert_eq!(GrantAnalyticsStatus::Disabled, GrantAnalyticsStatus::Disabled);
    }

    fn create_test_analytics() -> GrantAnalyticsMetadata {
        GrantAnalyticsMetadata {
            analytics_id: 1,
            grant_id: 100,
            analytics_type: GrantAnalyticsType::Distribution,
            status: GrantAnalyticsStatus::Active,
            created_at: 1000,
            analytics_config_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_grant_analytics_metadata_structure() {
        let analytics = create_test_analytics();
        assert_eq!(analytics.analytics_id, 1);
        assert_eq!(analytics.grant_id, 100);
        assert_eq!(analytics.analytics_type, GrantAnalyticsType::Distribution);
        assert_eq!(analytics.status, GrantAnalyticsStatus::Active);
        assert_eq!(analytics.created_at, 1000);
        assert_eq!(analytics.bump, 255);
    }

    #[test]
    fn test_initialize_grant_analytics_all_types() {
        let types = vec![
            GrantAnalyticsType::Distribution,
            GrantAnalyticsType::Impact,
            GrantAnalyticsType::Performance,
            GrantAnalyticsType::Custom,
        ];

        for analytics_type in types {
            let mut analytics = GrantAnalyticsMetadata {
                analytics_id: 0,
                grant_id: 0,
                analytics_type: GrantAnalyticsType::Distribution,
                status: GrantAnalyticsStatus::Disabled,
                created_at: 0,
                analytics_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_grant_analytics(
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
    fn test_initialize_grant_analytics_status_always_active_on_init() {
        let mut analytics = create_test_analytics();
        analytics.status = GrantAnalyticsStatus::Paused;

        let result = onchain::initialize_grant_analytics(
            &mut analytics,
            1,
            1,
            GrantAnalyticsType::Distribution,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // Status should always be set to Active on initialization
        assert_eq!(analytics.status, GrantAnalyticsStatus::Active);
    }

    #[test]
    fn test_initialize_grant_analytics_config_hash() {
        let mut analytics = create_test_analytics();
        let custom_hash = [144u8; 32];

        let result = onchain::initialize_grant_analytics(
            &mut analytics,
            1,
            1,
            GrantAnalyticsType::Performance,
            custom_hash,
            4000,
            100,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.analytics_config_hash, custom_hash);
    }

    #[test]
    fn test_initialize_grant_analytics_grant_id() {
        let mut analytics = create_test_analytics();

        let result = onchain::initialize_grant_analytics(
            &mut analytics,
            1,
            66666,
            GrantAnalyticsType::Impact,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.grant_id, 66666);
    }

    #[test]
    fn test_initialize_grant_analytics_timestamp() {
        let mut analytics = create_test_analytics();

        let result = onchain::initialize_grant_analytics(
            &mut analytics,
            1,
            1,
            GrantAnalyticsType::Custom,
            [0u8; 32],
            78901,
            190,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.created_at, 78901);
    }

    #[test]
    fn test_initialize_grant_analytics_bump_seed() {
        let mut analytics = create_test_analytics();

        for bump in [0u8, 90u8, 255u8] {
            let result = onchain::initialize_grant_analytics(
                &mut analytics,
                1,
                1,
                GrantAnalyticsType::Distribution,
                [0u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(analytics.bump, bump);
        }
    }

    #[test]
    fn test_grant_analytics_enum_equality() {
        // Test that enum variants can be compared
        let type1 = GrantAnalyticsType::Distribution;
        let type2 = GrantAnalyticsType::Distribution;
        let type3 = GrantAnalyticsType::Impact;

        assert_eq!(type1, type2);
        assert_ne!(type1, type3);

        let status1 = GrantAnalyticsStatus::Active;
        let status2 = GrantAnalyticsStatus::Active;
        let status3 = GrantAnalyticsStatus::Paused;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_grant_analytics_type_all_variants_unique() {
        let types = vec![
            GrantAnalyticsType::Distribution,
            GrantAnalyticsType::Impact,
            GrantAnalyticsType::Performance,
            GrantAnalyticsType::Custom,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_grant_analytics_status_all_variants_unique() {
        let statuses = vec![
            GrantAnalyticsStatus::Active,
            GrantAnalyticsStatus::Paused,
            GrantAnalyticsStatus::Disabled,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_grant_analytics_type_equality() {
        assert_eq!(GrantAnalyticsType::Distribution, GrantAnalyticsType::Distribution);
        assert_ne!(GrantAnalyticsType::Distribution, GrantAnalyticsType::Impact);
        assert_eq!(GrantAnalyticsType::Impact, GrantAnalyticsType::Impact);
        assert_ne!(GrantAnalyticsType::Impact, GrantAnalyticsType::Performance);
        assert_eq!(GrantAnalyticsType::Performance, GrantAnalyticsType::Performance);
        assert_ne!(GrantAnalyticsType::Performance, GrantAnalyticsType::Custom);
        assert_eq!(GrantAnalyticsType::Custom, GrantAnalyticsType::Custom);
    }

    #[test]
    fn test_grant_analytics_status_equality() {
        assert_eq!(GrantAnalyticsStatus::Active, GrantAnalyticsStatus::Active);
        assert_ne!(GrantAnalyticsStatus::Active, GrantAnalyticsStatus::Paused);
        assert_eq!(GrantAnalyticsStatus::Paused, GrantAnalyticsStatus::Paused);
        assert_ne!(GrantAnalyticsStatus::Paused, GrantAnalyticsStatus::Disabled);
        assert_eq!(GrantAnalyticsStatus::Disabled, GrantAnalyticsStatus::Disabled);
    }

    #[test]
    fn test_grant_analytics_type_copy() {
        let type1 = GrantAnalyticsType::Distribution;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_grant_analytics_status_copy() {
        let status1 = GrantAnalyticsStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_grant_analytics_type_space() {
        assert_eq!(<GrantAnalyticsType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_grant_analytics_status_space() {
        assert_eq!(<GrantAnalyticsStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_grant_analytics_large_ids() {
        let mut analytics = GrantAnalyticsMetadata {
            analytics_id: 0,
            grant_id: 0,
            analytics_type: GrantAnalyticsType::Distribution,
            status: GrantAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_grant_analytics(
            &mut analytics,
            u64::MAX,
            u64::MAX,
            GrantAnalyticsType::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, u64::MAX);
        assert_eq!(analytics.grant_id, u64::MAX);
    }

    #[test]
    fn test_grant_analytics_metadata_all_fields() {
        let analytics = GrantAnalyticsMetadata {
            analytics_id: 123,
            grant_id: 456,
            analytics_type: GrantAnalyticsType::Performance,
            status: GrantAnalyticsStatus::Paused,
            created_at: 5000,
            analytics_config_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(analytics.analytics_id, 123);
        assert_eq!(analytics.grant_id, 456);
        assert_eq!(analytics.analytics_type, GrantAnalyticsType::Performance);
        assert_eq!(analytics.status, GrantAnalyticsStatus::Paused);
        assert_eq!(analytics.created_at, 5000);
        assert_eq!(analytics.analytics_config_hash, [42u8; 32]);
        assert_eq!(analytics.bump, 128);
    }
}
