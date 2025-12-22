//! Mesh Group Collaboration module
//!
//! Mesh group collaboration management
//!
//! On-chain: Metadata for mesh group collaboration
//! Off-chain: Actual collaboration, coordination

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Collaboration type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MeshGroupCollaborationType {
    /// Synchronous collaboration
    Synchronous,
    /// Asynchronous collaboration
    Asynchronous,
    /// Hybrid collaboration
    Hybrid,
    /// Custom collaboration
    Custom,
}

/// Collaboration status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MeshGroupCollaborationStatus {
    /// Collaboration active
    Active,
    /// Collaboration paused
    Paused,
    /// Collaboration completed
    Completed,
}

/// Mesh group collaboration metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct MeshGroupCollaborationMetadata {
    /// Collaboration ID
    pub collaboration_id: u64,
    /// Mesh group ID
    pub mesh_group_id: u64,
    /// Collaboration type
    pub collaboration_type: MeshGroupCollaborationType,
    /// Status
    pub status: MeshGroupCollaborationStatus,
    /// Created at
    pub created_at: i64,
    /// Collaboration config hash
    pub collaboration_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    
    pub fn initialize_mesh_group_collaboration(
        collaboration: &mut MeshGroupCollaborationMetadata,
        collaboration_id: u64,
        mesh_group_id: u64,
        collaboration_type: MeshGroupCollaborationType,
        collaboration_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(collaboration_id > 0, IndrasError::InvalidInput);
        collaboration.collaboration_id = collaboration_id;
        collaboration.mesh_group_id = mesh_group_id;
        collaboration.collaboration_type = collaboration_type;
        collaboration.status = MeshGroupCollaborationStatus::Active;
        collaboration.created_at = current_time;
        collaboration.collaboration_config_hash = collaboration_config_hash;
        collaboration.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn facilitate_collaboration(_collaboration_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_mesh_group_collaboration() {
        let mut collaboration = MeshGroupCollaborationMetadata {
            collaboration_id: 0,
            mesh_group_id: 0,
            collaboration_type: MeshGroupCollaborationType::Synchronous,
            status: MeshGroupCollaborationStatus::Completed,
            created_at: 0,
            collaboration_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_mesh_group_collaboration(
            &mut collaboration,
            1,
            10,
            MeshGroupCollaborationType::Hybrid,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(collaboration.collaboration_id, 1);
        assert_eq!(collaboration.mesh_group_id, 10);
        assert_eq!(collaboration.collaboration_type, MeshGroupCollaborationType::Hybrid);
        assert_eq!(collaboration.status, MeshGroupCollaborationStatus::Active);
        assert_eq!(collaboration.created_at, 1000);
        assert_eq!(collaboration.bump, 255);
    }

    #[test]
    fn test_initialize_mesh_group_collaboration_invalid_id() {
        let mut collaboration = MeshGroupCollaborationMetadata {
            collaboration_id: 0,
            mesh_group_id: 0,
            collaboration_type: MeshGroupCollaborationType::Synchronous,
            status: MeshGroupCollaborationStatus::Completed,
            created_at: 0,
            collaboration_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_mesh_group_collaboration(
            &mut collaboration,
            0, // Invalid: must be > 0
            10,
            MeshGroupCollaborationType::Hybrid,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_mesh_group_collaboration_type_all_variants_unique() {
        let types = vec![
            MeshGroupCollaborationType::Synchronous,
            MeshGroupCollaborationType::Asynchronous,
            MeshGroupCollaborationType::Hybrid,
            MeshGroupCollaborationType::Custom,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_mesh_group_collaboration_status_all_variants_unique() {
        let statuses = vec![
            MeshGroupCollaborationStatus::Active,
            MeshGroupCollaborationStatus::Paused,
            MeshGroupCollaborationStatus::Completed,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_mesh_group_collaboration_type_variants() {
        assert_eq!(MeshGroupCollaborationType::Synchronous, MeshGroupCollaborationType::Synchronous);
        assert_eq!(MeshGroupCollaborationType::Asynchronous, MeshGroupCollaborationType::Asynchronous);
        assert_eq!(MeshGroupCollaborationType::Hybrid, MeshGroupCollaborationType::Hybrid);
        assert_eq!(MeshGroupCollaborationType::Custom, MeshGroupCollaborationType::Custom);
    }

    #[test]
    fn test_mesh_group_collaboration_status_variants() {
        assert_eq!(MeshGroupCollaborationStatus::Active, MeshGroupCollaborationStatus::Active);
        assert_eq!(MeshGroupCollaborationStatus::Paused, MeshGroupCollaborationStatus::Paused);
        assert_eq!(MeshGroupCollaborationStatus::Completed, MeshGroupCollaborationStatus::Completed);
    }

    #[test]
    fn test_mesh_group_collaboration_type_equality() {
        assert_eq!(MeshGroupCollaborationType::Synchronous, MeshGroupCollaborationType::Synchronous);
        assert_ne!(MeshGroupCollaborationType::Synchronous, MeshGroupCollaborationType::Asynchronous);
        assert_eq!(MeshGroupCollaborationType::Asynchronous, MeshGroupCollaborationType::Asynchronous);
        assert_ne!(MeshGroupCollaborationType::Asynchronous, MeshGroupCollaborationType::Hybrid);
        assert_eq!(MeshGroupCollaborationType::Hybrid, MeshGroupCollaborationType::Hybrid);
        assert_ne!(MeshGroupCollaborationType::Hybrid, MeshGroupCollaborationType::Custom);
        assert_eq!(MeshGroupCollaborationType::Custom, MeshGroupCollaborationType::Custom);
    }

    #[test]
    fn test_mesh_group_collaboration_status_equality() {
        assert_eq!(MeshGroupCollaborationStatus::Active, MeshGroupCollaborationStatus::Active);
        assert_ne!(MeshGroupCollaborationStatus::Active, MeshGroupCollaborationStatus::Paused);
        assert_eq!(MeshGroupCollaborationStatus::Paused, MeshGroupCollaborationStatus::Paused);
        assert_ne!(MeshGroupCollaborationStatus::Paused, MeshGroupCollaborationStatus::Completed);
        assert_eq!(MeshGroupCollaborationStatus::Completed, MeshGroupCollaborationStatus::Completed);
    }

    #[test]
    fn test_mesh_group_collaboration_type_copy() {
        let type1 = MeshGroupCollaborationType::Synchronous;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_mesh_group_collaboration_status_copy() {
        let status1 = MeshGroupCollaborationStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_mesh_group_collaboration_type_space() {
        assert_eq!(<MeshGroupCollaborationType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_mesh_group_collaboration_status_space() {
        assert_eq!(<MeshGroupCollaborationStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_collaboration_metadata_structure() {
        let collaboration = MeshGroupCollaborationMetadata {
            collaboration_id: 1,
            mesh_group_id: 10,
            collaboration_type: MeshGroupCollaborationType::Synchronous,
            status: MeshGroupCollaborationStatus::Active,
            created_at: 1000,
            collaboration_config_hash: [1u8; 32],
            bump: 255,
        };
        
        assert_eq!(collaboration.collaboration_id, 1);
        assert_eq!(collaboration.mesh_group_id, 10);
        assert_eq!(collaboration.collaboration_type, MeshGroupCollaborationType::Synchronous);
        assert_eq!(collaboration.status, MeshGroupCollaborationStatus::Active);
        assert_eq!(collaboration.created_at, 1000);
        assert_eq!(collaboration.bump, 255);
    }

    #[test]
    fn test_initialize_collaboration_all_types() {
        let types = vec![
            MeshGroupCollaborationType::Synchronous,
            MeshGroupCollaborationType::Asynchronous,
            MeshGroupCollaborationType::Hybrid,
            MeshGroupCollaborationType::Custom,
        ];
        
        for (idx, collab_type) in types.iter().enumerate() {
            let mut collaboration = MeshGroupCollaborationMetadata {
                collaboration_id: 0,
                mesh_group_id: 0,
                collaboration_type: MeshGroupCollaborationType::Synchronous,
                status: MeshGroupCollaborationStatus::Completed,
                created_at: 0,
                collaboration_config_hash: [0u8; 32],
                bump: 0,
            };
            
            let result = onchain::initialize_mesh_group_collaboration(
                &mut collaboration,
                (idx + 1) as u64,
                10,
                *collab_type,
                [1u8; 32],
                1000,
                255,
            );
            
            assert!(result.is_ok());
            assert_eq!(collaboration.collaboration_type, *collab_type);
            assert_eq!(collaboration.status, MeshGroupCollaborationStatus::Active);
        }
    }

    #[test]
    fn test_initialize_collaboration_always_active_on_init() {
        let mut collaboration = MeshGroupCollaborationMetadata {
            collaboration_id: 0,
            mesh_group_id: 0,
            collaboration_type: MeshGroupCollaborationType::Synchronous,
            status: MeshGroupCollaborationStatus::Completed, // Will be reset
            created_at: 0,
            collaboration_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_mesh_group_collaboration(
            &mut collaboration,
            1,
            10,
            MeshGroupCollaborationType::Hybrid,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        // Status should always be set to Active on initialization
        assert_eq!(collaboration.status, MeshGroupCollaborationStatus::Active);
    }

    #[test]
    fn test_mesh_group_collaboration_metadata_all_fields() {
        let collaboration = MeshGroupCollaborationMetadata {
            collaboration_id: 123,
            mesh_group_id: 456,
            collaboration_type: MeshGroupCollaborationType::Asynchronous,
            status: MeshGroupCollaborationStatus::Paused,
            created_at: 5000,
            collaboration_config_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(collaboration.collaboration_id, 123);
        assert_eq!(collaboration.mesh_group_id, 456);
        assert_eq!(collaboration.collaboration_type, MeshGroupCollaborationType::Asynchronous);
        assert_eq!(collaboration.status, MeshGroupCollaborationStatus::Paused);
        assert_eq!(collaboration.created_at, 5000);
        assert_eq!(collaboration.collaboration_config_hash, [42u8; 32]);
        assert_eq!(collaboration.bump, 128);
    }

    #[test]
    fn test_initialize_collaboration_zero_mesh_group_id() {
        let mut collaboration = MeshGroupCollaborationMetadata {
            collaboration_id: 0,
            mesh_group_id: 0,
            collaboration_type: MeshGroupCollaborationType::Synchronous,
            status: MeshGroupCollaborationStatus::Completed,
            created_at: 0,
            collaboration_config_hash: [0u8; 32],
            bump: 0,
        };
        
        // Zero mesh_group_id is allowed (not validated)
        let result = onchain::initialize_mesh_group_collaboration(
            &mut collaboration,
            1,
            0, // Zero is allowed
            MeshGroupCollaborationType::Hybrid,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(collaboration.mesh_group_id, 0);
    }

    #[test]
    fn test_initialize_collaboration_large_ids() {
        let mut collaboration = MeshGroupCollaborationMetadata {
            collaboration_id: 0,
            mesh_group_id: 0,
            collaboration_type: MeshGroupCollaborationType::Synchronous,
            status: MeshGroupCollaborationStatus::Completed,
            created_at: 0,
            collaboration_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_mesh_group_collaboration(
            &mut collaboration,
            u64::MAX,
            u64::MAX,
            MeshGroupCollaborationType::Hybrid,
            [255u8; 32],
            i64::MAX,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(collaboration.collaboration_id, u64::MAX);
        assert_eq!(collaboration.mesh_group_id, u64::MAX);
    }

    #[test]
    fn test_initialize_collaboration_negative_timestamp() {
        let mut collaboration = MeshGroupCollaborationMetadata {
            collaboration_id: 0,
            mesh_group_id: 0,
            collaboration_type: MeshGroupCollaborationType::Synchronous,
            status: MeshGroupCollaborationStatus::Completed,
            created_at: 0,
            collaboration_config_hash: [0u8; 32],
            bump: 0,
        };
        
        // Negative timestamp is allowed (not validated)
        let result = onchain::initialize_mesh_group_collaboration(
            &mut collaboration,
            1,
            10,
            MeshGroupCollaborationType::Hybrid,
            [1u8; 32],
            -1000, // Negative timestamp
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(collaboration.created_at, -1000);
    }

    #[test]
    fn test_initialize_collaboration_different_config_hashes() {
        let hashes = vec![[0u8; 32], [1u8; 32], [255u8; 32], [42u8; 32]];
        
        for hash in &hashes {
            let mut collaboration = MeshGroupCollaborationMetadata {
                collaboration_id: 0,
                mesh_group_id: 0,
                collaboration_type: MeshGroupCollaborationType::Synchronous,
                status: MeshGroupCollaborationStatus::Completed,
                created_at: 0,
                collaboration_config_hash: [0u8; 32],
                bump: 0,
            };
            
            let result = onchain::initialize_mesh_group_collaboration(
                &mut collaboration,
                1,
                10,
                MeshGroupCollaborationType::Hybrid,
                *hash,
                1000,
                255,
            );
            
            assert!(result.is_ok());
            assert_eq!(collaboration.collaboration_config_hash, *hash);
        }
    }

    #[test]
    fn test_offchain_facilitate_collaboration() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::facilitate_collaboration(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_facilitate_collaboration_different_ids() {
        // Test with different IDs
        let result1 = offchain::facilitate_collaboration(1);
        let result2 = offchain::facilitate_collaboration(999);
        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
    }
}
