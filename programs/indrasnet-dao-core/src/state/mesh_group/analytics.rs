//! Mesh Group Analytics module
//!
//! Mesh group analytics and metrics
//!
//! On-chain: Metadata for mesh group analytics
//! Off-chain: Actual analytics, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analytics type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MeshGroupAnalyticsType {
    /// Collaboration analytics
    Collaboration,
    /// Productivity analytics
    Productivity,
    /// Growth analytics
    Growth,
    /// Custom analytics
    Custom,
}

/// Analytics status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MeshGroupAnalyticsStatus {
    /// Analytics active
    Active,
    /// Analytics paused
    Paused,
    /// Analytics disabled
    Disabled,
}

/// Mesh group analytics metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct MeshGroupAnalyticsMetadata {
    /// Analytics ID
    pub analytics_id: u64,
    /// Mesh group ID
    pub mesh_group_id: u64,
    /// Analytics type
    pub analytics_type: MeshGroupAnalyticsType,
    /// Status
    pub status: MeshGroupAnalyticsStatus,
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
    
    pub fn initialize_mesh_group_analytics(
        analytics: &mut MeshGroupAnalyticsMetadata,
        analytics_id: u64,
        mesh_group_id: u64,
        analytics_type: MeshGroupAnalyticsType,
        analytics_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analytics_id > 0, IndrasError::InvalidInput);
        analytics.analytics_id = analytics_id;
        analytics.mesh_group_id = mesh_group_id;
        analytics.analytics_type = analytics_type;
        analytics.status = MeshGroupAnalyticsStatus::Active;
        analytics.created_at = current_time;
        analytics.analytics_config_hash = analytics_config_hash;
        analytics.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_mesh_group_analytics(_analytics_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_mesh_group_analytics() {
        let mut analytics = MeshGroupAnalyticsMetadata {
            analytics_id: 0,
            mesh_group_id: 0,
            analytics_type: MeshGroupAnalyticsType::Collaboration,
            status: MeshGroupAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_mesh_group_analytics(
            &mut analytics,
            1,
            10,
            MeshGroupAnalyticsType::Productivity,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, 1);
        assert_eq!(analytics.mesh_group_id, 10);
        assert_eq!(analytics.analytics_type, MeshGroupAnalyticsType::Productivity);
        assert_eq!(analytics.status, MeshGroupAnalyticsStatus::Active);
        assert_eq!(analytics.created_at, 1000);
        assert_eq!(analytics.bump, 255);
    }

    #[test]
    fn test_initialize_mesh_group_analytics_invalid_id() {
        let mut analytics = MeshGroupAnalyticsMetadata {
            analytics_id: 0,
            mesh_group_id: 0,
            analytics_type: MeshGroupAnalyticsType::Collaboration,
            status: MeshGroupAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_mesh_group_analytics(
            &mut analytics,
            0, // Invalid: must be > 0
            10,
            MeshGroupAnalyticsType::Productivity,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_mesh_group_analytics_type_variants() {
        assert_eq!(MeshGroupAnalyticsType::Collaboration, MeshGroupAnalyticsType::Collaboration);
        assert_eq!(MeshGroupAnalyticsType::Productivity, MeshGroupAnalyticsType::Productivity);
        assert_eq!(MeshGroupAnalyticsType::Growth, MeshGroupAnalyticsType::Growth);
        assert_eq!(MeshGroupAnalyticsType::Custom, MeshGroupAnalyticsType::Custom);
    }

    #[test]
    fn test_mesh_group_analytics_status_variants() {
        assert_eq!(MeshGroupAnalyticsStatus::Active, MeshGroupAnalyticsStatus::Active);
        assert_eq!(MeshGroupAnalyticsStatus::Paused, MeshGroupAnalyticsStatus::Paused);
        assert_eq!(MeshGroupAnalyticsStatus::Disabled, MeshGroupAnalyticsStatus::Disabled);
    }

    #[test]
    fn test_mesh_group_analytics_type_all_variants_unique() {
        let types = vec![
            MeshGroupAnalyticsType::Collaboration,
            MeshGroupAnalyticsType::Productivity,
            MeshGroupAnalyticsType::Growth,
            MeshGroupAnalyticsType::Custom,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_mesh_group_analytics_status_all_variants_unique() {
        let statuses = vec![
            MeshGroupAnalyticsStatus::Active,
            MeshGroupAnalyticsStatus::Paused,
            MeshGroupAnalyticsStatus::Disabled,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_mesh_group_analytics_type_equality() {
        assert_eq!(MeshGroupAnalyticsType::Collaboration, MeshGroupAnalyticsType::Collaboration);
        assert_ne!(MeshGroupAnalyticsType::Collaboration, MeshGroupAnalyticsType::Productivity);
        assert_eq!(MeshGroupAnalyticsType::Productivity, MeshGroupAnalyticsType::Productivity);
        assert_ne!(MeshGroupAnalyticsType::Productivity, MeshGroupAnalyticsType::Growth);
        assert_eq!(MeshGroupAnalyticsType::Growth, MeshGroupAnalyticsType::Growth);
        assert_ne!(MeshGroupAnalyticsType::Growth, MeshGroupAnalyticsType::Custom);
        assert_eq!(MeshGroupAnalyticsType::Custom, MeshGroupAnalyticsType::Custom);
    }

    #[test]
    fn test_mesh_group_analytics_status_equality() {
        assert_eq!(MeshGroupAnalyticsStatus::Active, MeshGroupAnalyticsStatus::Active);
        assert_ne!(MeshGroupAnalyticsStatus::Active, MeshGroupAnalyticsStatus::Paused);
        assert_eq!(MeshGroupAnalyticsStatus::Paused, MeshGroupAnalyticsStatus::Paused);
        assert_ne!(MeshGroupAnalyticsStatus::Paused, MeshGroupAnalyticsStatus::Disabled);
        assert_eq!(MeshGroupAnalyticsStatus::Disabled, MeshGroupAnalyticsStatus::Disabled);
    }

    #[test]
    fn test_mesh_group_analytics_type_copy() {
        let type1 = MeshGroupAnalyticsType::Collaboration;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_mesh_group_analytics_status_copy() {
        let status1 = MeshGroupAnalyticsStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_mesh_group_analytics_type_space() {
        assert_eq!(<MeshGroupAnalyticsType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_mesh_group_analytics_status_space() {
        assert_eq!(<MeshGroupAnalyticsStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_mesh_group_analytics_all_types() {
        let types = vec![
            MeshGroupAnalyticsType::Collaboration,
            MeshGroupAnalyticsType::Productivity,
            MeshGroupAnalyticsType::Growth,
            MeshGroupAnalyticsType::Custom,
        ];
        
        for analytics_type in types {
            let mut analytics = MeshGroupAnalyticsMetadata {
                analytics_id: 0,
                mesh_group_id: 0,
                analytics_type: MeshGroupAnalyticsType::Collaboration,
                status: MeshGroupAnalyticsStatus::Disabled,
                created_at: 0,
                analytics_config_hash: [0u8; 32],
                bump: 0,
            };
            
            let result = onchain::initialize_mesh_group_analytics(
                &mut analytics,
                1,
                10,
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
    fn test_initialize_mesh_group_analytics_large_ids() {
        let mut analytics = MeshGroupAnalyticsMetadata {
            analytics_id: 0,
            mesh_group_id: 0,
            analytics_type: MeshGroupAnalyticsType::Collaboration,
            status: MeshGroupAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_mesh_group_analytics(
            &mut analytics,
            u64::MAX,
            u64::MAX,
            MeshGroupAnalyticsType::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, u64::MAX);
        assert_eq!(analytics.mesh_group_id, u64::MAX);
    }

    #[test]
    fn test_initialize_mesh_group_analytics_custom_hash() {
        let mut analytics = MeshGroupAnalyticsMetadata {
            analytics_id: 0,
            mesh_group_id: 0,
            analytics_type: MeshGroupAnalyticsType::Collaboration,
            status: MeshGroupAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let custom_hash = [255u8; 32];
        let result = onchain::initialize_mesh_group_analytics(
            &mut analytics,
            1,
            10,
            MeshGroupAnalyticsType::Custom,
            custom_hash,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_config_hash, custom_hash);
    }

    #[test]
    fn test_initialize_mesh_group_analytics_always_active_on_init() {
        let mut analytics = MeshGroupAnalyticsMetadata {
            analytics_id: 0,
            mesh_group_id: 0,
            analytics_type: MeshGroupAnalyticsType::Collaboration,
            status: MeshGroupAnalyticsStatus::Disabled, // Will be reset
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_mesh_group_analytics(
            &mut analytics,
            1,
            10,
            MeshGroupAnalyticsType::Collaboration,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        // Status should always be set to Active on initialization
        assert_eq!(analytics.status, MeshGroupAnalyticsStatus::Active);
    }

    #[test]
    fn test_mesh_group_analytics_metadata_all_fields() {
        let analytics = MeshGroupAnalyticsMetadata {
            analytics_id: 123,
            mesh_group_id: 456,
            analytics_type: MeshGroupAnalyticsType::Growth,
            status: MeshGroupAnalyticsStatus::Paused,
            created_at: 5000,
            analytics_config_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(analytics.analytics_id, 123);
        assert_eq!(analytics.mesh_group_id, 456);
        assert_eq!(analytics.analytics_type, MeshGroupAnalyticsType::Growth);
        assert_eq!(analytics.status, MeshGroupAnalyticsStatus::Paused);
        assert_eq!(analytics.created_at, 5000);
        assert_eq!(analytics.analytics_config_hash, [42u8; 32]);
        assert_eq!(analytics.bump, 128);
    }
}
