//! Proposal Analytics module
//!
//! Proposal analytics and metrics
//!
//! On-chain: Metadata for proposal analytics
//! Off-chain: Actual analytics, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analytics type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ProposalAnalyticsType {
    /// Support analytics
    Support,
    /// Opposition analytics
    Opposition,
    /// Engagement analytics
    Engagement,
    /// Custom analytics
    Custom,
}

/// Analytics status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ProposalAnalyticsStatus {
    /// Analytics active
    Active,
    /// Analytics paused
    Paused,
    /// Analytics disabled
    Disabled,
}

/// Proposal analytics metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct ProposalAnalyticsMetadata {
    /// Analytics ID
    pub analytics_id: u64,
    /// Proposal ID
    pub proposal_id: u64,
    /// Analytics type
    pub analytics_type: ProposalAnalyticsType,
    /// Status
    pub status: ProposalAnalyticsStatus,
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
    
    pub fn initialize_proposal_analytics(
        analytics: &mut ProposalAnalyticsMetadata,
        analytics_id: u64,
        proposal_id: u64,
        analytics_type: ProposalAnalyticsType,
        analytics_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analytics_id > 0, IndrasError::InvalidInput);
        analytics.analytics_id = analytics_id;
        analytics.proposal_id = proposal_id;
        analytics.analytics_type = analytics_type;
        analytics.status = ProposalAnalyticsStatus::Active;
        analytics.created_at = current_time;
        analytics.analytics_config_hash = analytics_config_hash;
        analytics.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_proposal_analytics(_analytics_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_proposal_analytics() {
        let mut analytics = ProposalAnalyticsMetadata {
            analytics_id: 0,
            proposal_id: 0,
            analytics_type: ProposalAnalyticsType::Support,
            status: ProposalAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_proposal_analytics(
            &mut analytics,
            1,
            10,
            ProposalAnalyticsType::Engagement,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, 1);
        assert_eq!(analytics.proposal_id, 10);
        assert_eq!(analytics.analytics_type, ProposalAnalyticsType::Engagement);
        assert_eq!(analytics.status, ProposalAnalyticsStatus::Active);
        assert_eq!(analytics.created_at, 1000);
        assert_eq!(analytics.bump, 255);
    }

    #[test]
    fn test_initialize_proposal_analytics_invalid_id() {
        let mut analytics = ProposalAnalyticsMetadata {
            analytics_id: 0,
            proposal_id: 0,
            analytics_type: ProposalAnalyticsType::Support,
            status: ProposalAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_proposal_analytics(
            &mut analytics,
            0, // Invalid: must be > 0
            10,
            ProposalAnalyticsType::Engagement,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_proposal_analytics_type_variants() {
        assert_eq!(ProposalAnalyticsType::Support, ProposalAnalyticsType::Support);
        assert_eq!(ProposalAnalyticsType::Opposition, ProposalAnalyticsType::Opposition);
        assert_eq!(ProposalAnalyticsType::Engagement, ProposalAnalyticsType::Engagement);
        assert_eq!(ProposalAnalyticsType::Custom, ProposalAnalyticsType::Custom);
    }

    #[test]
    fn test_proposal_analytics_status_variants() {
        assert_eq!(ProposalAnalyticsStatus::Active, ProposalAnalyticsStatus::Active);
        assert_eq!(ProposalAnalyticsStatus::Paused, ProposalAnalyticsStatus::Paused);
        assert_eq!(ProposalAnalyticsStatus::Disabled, ProposalAnalyticsStatus::Disabled);
    }

    fn create_test_analytics() -> ProposalAnalyticsMetadata {
        ProposalAnalyticsMetadata {
            analytics_id: 1,
            proposal_id: 100,
            analytics_type: ProposalAnalyticsType::Support,
            status: ProposalAnalyticsStatus::Active,
            created_at: 1000,
            analytics_config_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_proposal_analytics_metadata_structure() {
        let analytics = create_test_analytics();
        assert_eq!(analytics.analytics_id, 1);
        assert_eq!(analytics.proposal_id, 100);
        assert_eq!(analytics.analytics_type, ProposalAnalyticsType::Support);
        assert_eq!(analytics.status, ProposalAnalyticsStatus::Active);
        assert_eq!(analytics.created_at, 1000);
        assert_eq!(analytics.bump, 255);
    }

    #[test]
    fn test_initialize_proposal_analytics_all_types() {
        let types = vec![
            ProposalAnalyticsType::Support,
            ProposalAnalyticsType::Opposition,
            ProposalAnalyticsType::Engagement,
            ProposalAnalyticsType::Custom,
        ];

        for analytics_type in types {
            let mut analytics = ProposalAnalyticsMetadata {
                analytics_id: 0,
                proposal_id: 0,
                analytics_type: ProposalAnalyticsType::Support,
                status: ProposalAnalyticsStatus::Disabled,
                created_at: 0,
                analytics_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_proposal_analytics(
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
    fn test_initialize_proposal_analytics_status_always_active_on_init() {
        let mut analytics = create_test_analytics();
        analytics.status = ProposalAnalyticsStatus::Paused;

        let result = onchain::initialize_proposal_analytics(
            &mut analytics,
            1,
            1,
            ProposalAnalyticsType::Support,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // Status should always be set to Active on initialization
        assert_eq!(analytics.status, ProposalAnalyticsStatus::Active);
    }

    #[test]
    fn test_initialize_proposal_analytics_config_hash() {
        let mut analytics = create_test_analytics();
        let custom_hash = [177u8; 32];

        let result = onchain::initialize_proposal_analytics(
            &mut analytics,
            1,
            1,
            ProposalAnalyticsType::Opposition,
            custom_hash,
            7000,
            140,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.analytics_config_hash, custom_hash);
    }

    #[test]
    fn test_initialize_proposal_analytics_proposal_id() {
        let mut analytics = create_test_analytics();

        let result = onchain::initialize_proposal_analytics(
            &mut analytics,
            1,
            99999,
            ProposalAnalyticsType::Engagement,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.proposal_id, 99999);
    }

    #[test]
    fn test_initialize_proposal_analytics_timestamp() {
        let mut analytics = create_test_analytics();

        let result = onchain::initialize_proposal_analytics(
            &mut analytics,
            1,
            1,
            ProposalAnalyticsType::Custom,
            [0u8; 32],
            33445,
            200,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.created_at, 33445);
    }

    #[test]
    fn test_initialize_proposal_analytics_bump_seed() {
        let mut analytics = create_test_analytics();

        for bump in [0u8, 135u8, 255u8] {
            let result = onchain::initialize_proposal_analytics(
                &mut analytics,
                1,
                1,
                ProposalAnalyticsType::Support,
                [0u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(analytics.bump, bump);
        }
    }

    #[test]
    fn test_proposal_analytics_enum_equality() {
        // Test that enum variants can be compared
        let type1 = ProposalAnalyticsType::Support;
        let type2 = ProposalAnalyticsType::Support;
        let type3 = ProposalAnalyticsType::Opposition;

        assert_eq!(type1, type2);
        assert_ne!(type1, type3);

        let status1 = ProposalAnalyticsStatus::Active;
        let status2 = ProposalAnalyticsStatus::Active;
        let status3 = ProposalAnalyticsStatus::Paused;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_initialize_proposal_analytics_analytics_id_boundary() {
        let mut analytics = create_test_analytics();

        // Test with maximum valid ID (u64::MAX)
        let result = onchain::initialize_proposal_analytics(
            &mut analytics,
            u64::MAX,
            1,
            ProposalAnalyticsType::Support,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, u64::MAX);
    }

    #[test]
    fn test_proposal_analytics_type_all_variants_unique() {
        let types = vec![
            ProposalAnalyticsType::Support,
            ProposalAnalyticsType::Opposition,
            ProposalAnalyticsType::Engagement,
            ProposalAnalyticsType::Custom,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_proposal_analytics_status_all_variants_unique() {
        let statuses = vec![
            ProposalAnalyticsStatus::Active,
            ProposalAnalyticsStatus::Paused,
            ProposalAnalyticsStatus::Disabled,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_proposal_analytics_type_equality() {
        assert_eq!(ProposalAnalyticsType::Support, ProposalAnalyticsType::Support);
        assert_ne!(ProposalAnalyticsType::Support, ProposalAnalyticsType::Opposition);
        assert_eq!(ProposalAnalyticsType::Opposition, ProposalAnalyticsType::Opposition);
        assert_ne!(ProposalAnalyticsType::Opposition, ProposalAnalyticsType::Engagement);
        assert_eq!(ProposalAnalyticsType::Engagement, ProposalAnalyticsType::Engagement);
        assert_ne!(ProposalAnalyticsType::Engagement, ProposalAnalyticsType::Custom);
        assert_eq!(ProposalAnalyticsType::Custom, ProposalAnalyticsType::Custom);
    }

    #[test]
    fn test_proposal_analytics_status_equality() {
        assert_eq!(ProposalAnalyticsStatus::Active, ProposalAnalyticsStatus::Active);
        assert_ne!(ProposalAnalyticsStatus::Active, ProposalAnalyticsStatus::Paused);
        assert_eq!(ProposalAnalyticsStatus::Paused, ProposalAnalyticsStatus::Paused);
        assert_ne!(ProposalAnalyticsStatus::Paused, ProposalAnalyticsStatus::Disabled);
        assert_eq!(ProposalAnalyticsStatus::Disabled, ProposalAnalyticsStatus::Disabled);
    }

    #[test]
    fn test_proposal_analytics_type_copy() {
        let type1 = ProposalAnalyticsType::Support;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_proposal_analytics_status_copy() {
        let status1 = ProposalAnalyticsStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_proposal_analytics_type_space() {
        assert_eq!(<ProposalAnalyticsType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_proposal_analytics_status_space() {
        assert_eq!(<ProposalAnalyticsStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_proposal_analytics_large_ids() {
        let mut analytics = ProposalAnalyticsMetadata {
            analytics_id: 0,
            proposal_id: 0,
            analytics_type: ProposalAnalyticsType::Support,
            status: ProposalAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_proposal_analytics(
            &mut analytics,
            u64::MAX,
            u64::MAX,
            ProposalAnalyticsType::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, u64::MAX);
        assert_eq!(analytics.proposal_id, u64::MAX);
    }

    #[test]
    fn test_proposal_analytics_metadata_all_fields() {
        let analytics = ProposalAnalyticsMetadata {
            analytics_id: 123,
            proposal_id: 456,
            analytics_type: ProposalAnalyticsType::Opposition,
            status: ProposalAnalyticsStatus::Paused,
            created_at: 5000,
            analytics_config_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(analytics.analytics_id, 123);
        assert_eq!(analytics.proposal_id, 456);
        assert_eq!(analytics.analytics_type, ProposalAnalyticsType::Opposition);
        assert_eq!(analytics.status, ProposalAnalyticsStatus::Paused);
        assert_eq!(analytics.created_at, 5000);
        assert_eq!(analytics.analytics_config_hash, [42u8; 32]);
        assert_eq!(analytics.bump, 128);
    }
}
