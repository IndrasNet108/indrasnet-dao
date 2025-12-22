//! Grant Tracking module
//!
//! Grant tracking and monitoring
//!
//! On-chain: Metadata for grant tracking
//! Off-chain: Actual tracking, monitoring

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Tracking metric
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GrantTrackingMetric {
    /// Progress tracking
    Progress,
    /// Impact tracking
    Impact,
    /// Milestone tracking
    Milestone,
    /// Custom metric
    Custom,
}

/// Tracking status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GrantTrackingStatus {
    /// Tracking active
    Active,
    /// Tracking paused
    Paused,
    /// Tracking stopped
    Stopped,
}

/// Grant tracking metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct GrantTrackingMetadata {
    /// Tracking ID
    pub tracking_id: u64,
    /// Grant ID
    pub grant_id: u64,
    /// Tracking metric
    pub tracking_metric: GrantTrackingMetric,
    /// Status
    pub status: GrantTrackingStatus,
    /// Created at
    pub created_at: i64,
    /// Tracking config hash
    pub tracking_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    
    pub fn initialize_grant_tracking(
        tracking: &mut GrantTrackingMetadata,
        tracking_id: u64,
        grant_id: u64,
        tracking_metric: GrantTrackingMetric,
        tracking_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(tracking_id > 0, IndrasError::InvalidInput);
        tracking.tracking_id = tracking_id;
        tracking.grant_id = grant_id;
        tracking.tracking_metric = tracking_metric;
        tracking.status = GrantTrackingStatus::Active;
        tracking.created_at = current_time;
        tracking.tracking_config_hash = tracking_config_hash;
        tracking.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn track_grant(_tracking_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_grant_tracking() {
        let mut tracking = GrantTrackingMetadata {
            tracking_id: 0,
            grant_id: 0,
            tracking_metric: GrantTrackingMetric::Progress,
            status: GrantTrackingStatus::Active,
            created_at: 0,
            tracking_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_grant_tracking(
            &mut tracking,
            1,
            10,
            GrantTrackingMetric::Impact,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(tracking.tracking_id, 1);
        assert_eq!(tracking.grant_id, 10);
        assert_eq!(tracking.tracking_metric, GrantTrackingMetric::Impact);
        assert_eq!(tracking.status, GrantTrackingStatus::Active);
        assert_eq!(tracking.created_at, 1000);
        assert_eq!(tracking.bump, 255);
    }

    #[test]
    fn test_initialize_grant_tracking_invalid_id() {
        let mut tracking = GrantTrackingMetadata {
            tracking_id: 0,
            grant_id: 0,
            tracking_metric: GrantTrackingMetric::Progress,
            status: GrantTrackingStatus::Active,
            created_at: 0,
            tracking_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_grant_tracking(
            &mut tracking,
            0, // Invalid: must be > 0
            10,
            GrantTrackingMetric::Impact,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_grant_tracking_metric_variants() {
        assert_eq!(GrantTrackingMetric::Progress, GrantTrackingMetric::Progress);
        assert_eq!(GrantTrackingMetric::Impact, GrantTrackingMetric::Impact);
        assert_eq!(GrantTrackingMetric::Milestone, GrantTrackingMetric::Milestone);
        assert_eq!(GrantTrackingMetric::Custom, GrantTrackingMetric::Custom);
    }

    #[test]
    fn test_grant_tracking_status_variants() {
        assert_eq!(GrantTrackingStatus::Active, GrantTrackingStatus::Active);
        assert_eq!(GrantTrackingStatus::Paused, GrantTrackingStatus::Paused);
        assert_eq!(GrantTrackingStatus::Stopped, GrantTrackingStatus::Stopped);
    }

    fn create_test_tracking() -> GrantTrackingMetadata {
        GrantTrackingMetadata {
            tracking_id: 1,
            grant_id: 100,
            tracking_metric: GrantTrackingMetric::Progress,
            status: GrantTrackingStatus::Active,
            created_at: 1000,
            tracking_config_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_grant_tracking_metadata_structure() {
        let tracking = create_test_tracking();
        assert_eq!(tracking.tracking_id, 1);
        assert_eq!(tracking.grant_id, 100);
        assert_eq!(tracking.tracking_metric, GrantTrackingMetric::Progress);
        assert_eq!(tracking.status, GrantTrackingStatus::Active);
        assert_eq!(tracking.created_at, 1000);
        assert_eq!(tracking.bump, 255);
    }

    #[test]
    fn test_initialize_grant_tracking_all_metrics() {
        let metrics = vec![
            GrantTrackingMetric::Progress,
            GrantTrackingMetric::Impact,
            GrantTrackingMetric::Milestone,
            GrantTrackingMetric::Custom,
        ];

        for tracking_metric in metrics {
            let mut tracking = GrantTrackingMetadata {
                tracking_id: 0,
                grant_id: 0,
                tracking_metric: GrantTrackingMetric::Progress,
                status: GrantTrackingStatus::Stopped,
                created_at: 0,
                tracking_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_grant_tracking(
                &mut tracking,
                1,
                1,
                tracking_metric,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(tracking.tracking_metric, tracking_metric);
        }
    }

    #[test]
    fn test_initialize_grant_tracking_status_always_active_on_init() {
        let mut tracking = create_test_tracking();
        tracking.status = GrantTrackingStatus::Stopped;

        let result = onchain::initialize_grant_tracking(
            &mut tracking,
            1,
            1,
            GrantTrackingMetric::Progress,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // Status should always be set to Active on initialization
        assert_eq!(tracking.status, GrantTrackingStatus::Active);
    }

    #[test]
    fn test_initialize_grant_tracking_config_hash() {
        let mut tracking = create_test_tracking();
        let custom_hash = [166u8; 32];

        let result = onchain::initialize_grant_tracking(
            &mut tracking,
            1,
            1,
            GrantTrackingMetric::Milestone,
            custom_hash,
            6000,
            130,
        );

        assert!(result.is_ok());
        assert_eq!(tracking.tracking_config_hash, custom_hash);
    }

    #[test]
    fn test_initialize_grant_tracking_grant_id() {
        let mut tracking = create_test_tracking();

        let result = onchain::initialize_grant_tracking(
            &mut tracking,
            1,
            44444,
            GrantTrackingMetric::Impact,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(tracking.grant_id, 44444);
    }

    #[test]
    fn test_initialize_grant_tracking_timestamp() {
        let mut tracking = create_test_tracking();

        let result = onchain::initialize_grant_tracking(
            &mut tracking,
            1,
            1,
            GrantTrackingMetric::Custom,
            [0u8; 32],
            22334,
            160,
        );

        assert!(result.is_ok());
        assert_eq!(tracking.created_at, 22334);
    }

    #[test]
    fn test_initialize_grant_tracking_bump_seed() {
        let mut tracking = create_test_tracking();

        for bump in [0u8, 125u8, 255u8] {
            let result = onchain::initialize_grant_tracking(
                &mut tracking,
                1,
                1,
                GrantTrackingMetric::Progress,
                [0u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(tracking.bump, bump);
        }
    }

    #[test]
    fn test_grant_tracking_enum_equality() {
        // Test that enum variants can be compared
        let metric1 = GrantTrackingMetric::Progress;
        let metric2 = GrantTrackingMetric::Progress;
        let metric3 = GrantTrackingMetric::Impact;

        assert_eq!(metric1, metric2);
        assert_ne!(metric1, metric3);

        let status1 = GrantTrackingStatus::Active;
        let status2 = GrantTrackingStatus::Active;
        let status3 = GrantTrackingStatus::Paused;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_grant_tracking_metric_all_variants_unique() {
        let metrics = vec![
            GrantTrackingMetric::Progress,
            GrantTrackingMetric::Impact,
            GrantTrackingMetric::Milestone,
            GrantTrackingMetric::Custom,
        ];
        
        for i in 0..metrics.len() {
            for j in (i + 1)..metrics.len() {
                assert_ne!(metrics[i], metrics[j], "Duplicate metric found");
            }
        }
    }

    #[test]
    fn test_grant_tracking_status_all_variants_unique() {
        let statuses = vec![
            GrantTrackingStatus::Active,
            GrantTrackingStatus::Paused,
            GrantTrackingStatus::Stopped,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_grant_tracking_metric_equality() {
        assert_eq!(GrantTrackingMetric::Progress, GrantTrackingMetric::Progress);
        assert_ne!(GrantTrackingMetric::Progress, GrantTrackingMetric::Impact);
        assert_eq!(GrantTrackingMetric::Impact, GrantTrackingMetric::Impact);
        assert_ne!(GrantTrackingMetric::Impact, GrantTrackingMetric::Milestone);
        assert_eq!(GrantTrackingMetric::Milestone, GrantTrackingMetric::Milestone);
        assert_ne!(GrantTrackingMetric::Milestone, GrantTrackingMetric::Custom);
        assert_eq!(GrantTrackingMetric::Custom, GrantTrackingMetric::Custom);
    }

    #[test]
    fn test_grant_tracking_status_equality() {
        assert_eq!(GrantTrackingStatus::Active, GrantTrackingStatus::Active);
        assert_ne!(GrantTrackingStatus::Active, GrantTrackingStatus::Paused);
        assert_eq!(GrantTrackingStatus::Paused, GrantTrackingStatus::Paused);
        assert_ne!(GrantTrackingStatus::Paused, GrantTrackingStatus::Stopped);
        assert_eq!(GrantTrackingStatus::Stopped, GrantTrackingStatus::Stopped);
    }

    #[test]
    fn test_grant_tracking_metric_copy() {
        let metric1 = GrantTrackingMetric::Progress;
        let metric2 = metric1; // Copy trait
        assert_eq!(metric1, metric2);
    }

    #[test]
    fn test_grant_tracking_status_copy() {
        let status1 = GrantTrackingStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_grant_tracking_metric_space() {
        assert_eq!(<GrantTrackingMetric as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_grant_tracking_status_space() {
        assert_eq!(<GrantTrackingStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_grant_tracking_large_ids() {
        let mut tracking = GrantTrackingMetadata {
            tracking_id: 0,
            grant_id: 0,
            tracking_metric: GrantTrackingMetric::Progress,
            status: GrantTrackingStatus::Stopped,
            created_at: 0,
            tracking_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_grant_tracking(
            &mut tracking,
            u64::MAX,
            u64::MAX,
            GrantTrackingMetric::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(tracking.tracking_id, u64::MAX);
        assert_eq!(tracking.grant_id, u64::MAX);
    }

    #[test]
    fn test_grant_tracking_metadata_all_fields() {
        let tracking = GrantTrackingMetadata {
            tracking_id: 123,
            grant_id: 456,
            tracking_metric: GrantTrackingMetric::Milestone,
            status: GrantTrackingStatus::Paused,
            created_at: 5000,
            tracking_config_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(tracking.tracking_id, 123);
        assert_eq!(tracking.grant_id, 456);
        assert_eq!(tracking.tracking_metric, GrantTrackingMetric::Milestone);
        assert_eq!(tracking.status, GrantTrackingStatus::Paused);
        assert_eq!(tracking.created_at, 5000);
        assert_eq!(tracking.tracking_config_hash, [42u8; 32]);
        assert_eq!(tracking.bump, 128);
    }
}
