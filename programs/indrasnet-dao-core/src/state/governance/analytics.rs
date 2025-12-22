//! Governance Analytics module
//!
//! Governance analytics and metrics
//!
//! On-chain: Metadata for governance analytics
//! Off-chain: Actual analytics, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analytics type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GovernanceAnalyticsType {
    /// Participation analytics
    Participation,
    /// Voting analytics
    Voting,
    /// Proposal analytics
    Proposal,
    /// Custom analytics
    Custom,
}

/// Analytics status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GovernanceAnalyticsStatus {
    /// Analytics active
    Active,
    /// Analytics paused
    Paused,
    /// Analytics disabled
    Disabled,
}

/// Governance analytics metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct GovernanceAnalyticsMetadata {
    /// Analytics ID
    pub analytics_id: u64,
    /// Governance ID
    pub governance_id: u64,
    /// Analytics type
    pub analytics_type: GovernanceAnalyticsType,
    /// Status
    pub status: GovernanceAnalyticsStatus,
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
    
    pub fn initialize_governance_analytics(
        analytics: &mut GovernanceAnalyticsMetadata,
        analytics_id: u64,
        governance_id: u64,
        analytics_type: GovernanceAnalyticsType,
        analytics_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analytics_id > 0, IndrasError::InvalidInput);
        analytics.analytics_id = analytics_id;
        analytics.governance_id = governance_id;
        analytics.analytics_type = analytics_type;
        analytics.status = GovernanceAnalyticsStatus::Active;
        analytics.created_at = current_time;
        analytics.analytics_config_hash = analytics_config_hash;
        analytics.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_governance_analytics(_analytics_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_analytics() -> GovernanceAnalyticsMetadata {
        GovernanceAnalyticsMetadata {
            analytics_id: 1,
            governance_id: 1,
            analytics_type: GovernanceAnalyticsType::Participation,
            status: GovernanceAnalyticsStatus::Active,
            created_at: 1000,
            analytics_config_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_governance_analytics_type_variants() {
        assert_eq!(GovernanceAnalyticsType::Participation, GovernanceAnalyticsType::Participation);
        assert_eq!(GovernanceAnalyticsType::Voting, GovernanceAnalyticsType::Voting);
        assert_eq!(GovernanceAnalyticsType::Proposal, GovernanceAnalyticsType::Proposal);
        assert_eq!(GovernanceAnalyticsType::Custom, GovernanceAnalyticsType::Custom);
    }

    #[test]
    fn test_governance_analytics_status_variants() {
        assert_eq!(GovernanceAnalyticsStatus::Active, GovernanceAnalyticsStatus::Active);
        assert_eq!(GovernanceAnalyticsStatus::Paused, GovernanceAnalyticsStatus::Paused);
        assert_eq!(GovernanceAnalyticsStatus::Disabled, GovernanceAnalyticsStatus::Disabled);
    }

    #[test]
    fn test_governance_analytics_metadata_structure() {
        let analytics = create_test_analytics();
        assert_eq!(analytics.analytics_id, 1);
        assert_eq!(analytics.governance_id, 1);
        assert_eq!(analytics.analytics_type, GovernanceAnalyticsType::Participation);
        assert_eq!(analytics.status, GovernanceAnalyticsStatus::Active);
        assert_eq!(analytics.created_at, 1000);
        assert_eq!(analytics.bump, 255);
    }

    #[test]
    fn test_initialize_governance_analytics() {
        let mut analytics = GovernanceAnalyticsMetadata {
            analytics_id: 0,
            governance_id: 0,
            analytics_type: GovernanceAnalyticsType::Participation,
            status: GovernanceAnalyticsStatus::Active,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };

        let config_hash = [1u8; 32];
        let result = onchain::initialize_governance_analytics(
            &mut analytics,
            100,
            200,
            GovernanceAnalyticsType::Voting,
            config_hash,
            5000,
            128,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, 100);
        assert_eq!(analytics.governance_id, 200);
        assert_eq!(analytics.analytics_type, GovernanceAnalyticsType::Voting);
        assert_eq!(analytics.status, GovernanceAnalyticsStatus::Active);
        assert_eq!(analytics.created_at, 5000);
        assert_eq!(analytics.analytics_config_hash, config_hash);
        assert_eq!(analytics.bump, 128);
    }

    #[test]
    fn test_initialize_governance_analytics_invalid_id() {
        let mut analytics = create_test_analytics();
        let config_hash = [1u8; 32];

        let result = onchain::initialize_governance_analytics(
            &mut analytics,
            0, // Invalid: analytics_id must be > 0
            200,
            GovernanceAnalyticsType::Voting,
            config_hash,
            5000,
            128,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_governance_analytics_all_types() {
        let types = vec![
            GovernanceAnalyticsType::Participation,
            GovernanceAnalyticsType::Voting,
            GovernanceAnalyticsType::Proposal,
            GovernanceAnalyticsType::Custom,
        ];

        for analytics_type in types {
            let mut analytics = GovernanceAnalyticsMetadata {
                analytics_id: 0,
                governance_id: 0,
                analytics_type: GovernanceAnalyticsType::Participation,
                status: GovernanceAnalyticsStatus::Active,
                created_at: 0,
                analytics_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_governance_analytics(
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
    fn test_governance_analytics_config_hash() {
        let mut analytics = create_test_analytics();
        let custom_hash = [42u8; 32];

        let result = onchain::initialize_governance_analytics(
            &mut analytics,
            1,
            1,
            GovernanceAnalyticsType::Custom,
            custom_hash,
            2000,
            200,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.analytics_config_hash, custom_hash);
    }

    #[test]
    fn test_governance_analytics_timestamp() {
        let mut analytics = create_test_analytics();

        let result = onchain::initialize_governance_analytics(
            &mut analytics,
            1,
            1,
            GovernanceAnalyticsType::Proposal,
            [0u8; 32],
            9999,
            100,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.created_at, 9999);
    }

    #[test]
    fn test_governance_analytics_status_always_active_on_init() {
        let mut analytics = create_test_analytics();
        analytics.status = GovernanceAnalyticsStatus::Paused;

        let result = onchain::initialize_governance_analytics(
            &mut analytics,
            1,
            1,
            GovernanceAnalyticsType::Voting,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // Status should always be set to Active on initialization
        assert_eq!(analytics.status, GovernanceAnalyticsStatus::Active);
    }

    #[test]
    fn test_governance_analytics_bump_seed() {
        let mut analytics = create_test_analytics();

        for bump in [0u8, 128u8, 255u8] {
            let result = onchain::initialize_governance_analytics(
                &mut analytics,
                1,
                1,
                GovernanceAnalyticsType::Participation,
                [0u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(analytics.bump, bump);
        }
    }

    #[test]
    fn test_governance_analytics_enum_equality() {
        // Test that enum variants can be compared
        let type1 = GovernanceAnalyticsType::Voting;
        let type2 = GovernanceAnalyticsType::Voting;
        let type3 = GovernanceAnalyticsType::Proposal;

        assert_eq!(type1, type2);
        assert_ne!(type1, type3);

        let status1 = GovernanceAnalyticsStatus::Active;
        let status2 = GovernanceAnalyticsStatus::Active;
        let status3 = GovernanceAnalyticsStatus::Paused;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_governance_analytics_type_all_variants_unique() {
        let types = vec![
            GovernanceAnalyticsType::Participation,
            GovernanceAnalyticsType::Voting,
            GovernanceAnalyticsType::Proposal,
            GovernanceAnalyticsType::Custom,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_governance_analytics_status_all_variants_unique() {
        let statuses = vec![
            GovernanceAnalyticsStatus::Active,
            GovernanceAnalyticsStatus::Paused,
            GovernanceAnalyticsStatus::Disabled,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_governance_analytics_type_copy() {
        let type1 = GovernanceAnalyticsType::Voting;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_governance_analytics_status_copy() {
        let status1 = GovernanceAnalyticsStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_governance_analytics_type_space() {
        assert_eq!(<GovernanceAnalyticsType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_governance_analytics_status_space() {
        assert_eq!(<GovernanceAnalyticsStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_governance_analytics_large_ids() {
        let mut analytics = create_test_analytics();
        
        let result = onchain::initialize_governance_analytics(
            &mut analytics,
            u64::MAX,
            u64::MAX,
            GovernanceAnalyticsType::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, u64::MAX);
        assert_eq!(analytics.governance_id, u64::MAX);
    }

    #[test]
    fn test_initialize_governance_analytics_preserves_other_fields() {
        let mut analytics = GovernanceAnalyticsMetadata {
            analytics_id: 999,
            governance_id: 888,
            analytics_type: GovernanceAnalyticsType::Participation,
            status: GovernanceAnalyticsStatus::Disabled,
            created_at: 1000,
            analytics_config_hash: [1u8; 32],
            bump: 50,
        };
        
        let new_hash = [2u8; 32];
        let result = onchain::initialize_governance_analytics(
            &mut analytics,
            1,
            2,
            GovernanceAnalyticsType::Voting,
            new_hash,
            3000,
            100,
        );
        
        assert!(result.is_ok());
        // All fields should be updated
        assert_eq!(analytics.analytics_id, 1);
        assert_eq!(analytics.governance_id, 2);
        assert_eq!(analytics.analytics_type, GovernanceAnalyticsType::Voting);
        assert_eq!(analytics.status, GovernanceAnalyticsStatus::Active); // Always set to Active
        assert_eq!(analytics.created_at, 3000);
        assert_eq!(analytics.analytics_config_hash, new_hash);
        assert_eq!(analytics.bump, 100);
    }

    #[test]
    fn test_governance_analytics_metadata_all_fields() {
        let analytics = GovernanceAnalyticsMetadata {
            analytics_id: 123,
            governance_id: 456,
            analytics_type: GovernanceAnalyticsType::Proposal,
            status: GovernanceAnalyticsStatus::Paused,
            created_at: 5000,
            analytics_config_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(analytics.analytics_id, 123);
        assert_eq!(analytics.governance_id, 456);
        assert_eq!(analytics.analytics_type, GovernanceAnalyticsType::Proposal);
        assert_eq!(analytics.status, GovernanceAnalyticsStatus::Paused);
        assert_eq!(analytics.created_at, 5000);
        assert_eq!(analytics.analytics_config_hash, [42u8; 32]);
        assert_eq!(analytics.bump, 128);
    }

    #[test]
    fn test_offchain_generate_governance_analytics() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::generate_governance_analytics(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_generate_governance_analytics_different_ids() {
        // Test with different IDs
        let result1 = offchain::generate_governance_analytics(1);
        let result2 = offchain::generate_governance_analytics(999);
        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
    }
}
