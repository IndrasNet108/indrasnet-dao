//! Mesh Group Governance module
//!
//! Mesh group governance
//!
//! On-chain: Metadata for mesh group governance
//! Off-chain: Actual governance, decision-making

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Governance model
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MeshGroupGovernanceModel {
    /// Consensus-based
    Consensus,
    /// Voting-based
    Voting,
    /// Delegation-based
    Delegation,
    /// Custom model
    Custom,
}

/// Governance status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MeshGroupGovernanceStatus {
    /// Governance active
    Active,
    /// Governance paused
    Paused,
    /// Governance disabled
    Disabled,
}

/// Mesh group governance metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct MeshGroupGovernanceMetadata {
    /// Governance ID
    pub governance_id: u64,
    /// Mesh group ID
    pub mesh_group_id: u64,
    /// Governance model
    pub governance_model: MeshGroupGovernanceModel,
    /// Status
    pub status: MeshGroupGovernanceStatus,
    /// Created at
    pub created_at: i64,
    /// Governance config hash
    pub governance_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    
    pub fn initialize_mesh_group_governance(
        governance: &mut MeshGroupGovernanceMetadata,
        governance_id: u64,
        mesh_group_id: u64,
        governance_model: MeshGroupGovernanceModel,
        governance_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(governance_id > 0, IndrasError::InvalidInput);
        governance.governance_id = governance_id;
        governance.mesh_group_id = mesh_group_id;
        governance.governance_model = governance_model;
        governance.status = MeshGroupGovernanceStatus::Active;
        governance.created_at = current_time;
        governance.governance_config_hash = governance_config_hash;
        governance.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn govern_mesh_group(_governance_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_mesh_group_governance() {
        let mut governance = MeshGroupGovernanceMetadata {
            governance_id: 0,
            mesh_group_id: 0,
            governance_model: MeshGroupGovernanceModel::Consensus,
            status: MeshGroupGovernanceStatus::Disabled,
            created_at: 0,
            governance_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_mesh_group_governance(
            &mut governance,
            1,
            10,
            MeshGroupGovernanceModel::Voting,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(governance.governance_id, 1);
        assert_eq!(governance.mesh_group_id, 10);
        assert_eq!(governance.governance_model, MeshGroupGovernanceModel::Voting);
        assert_eq!(governance.status, MeshGroupGovernanceStatus::Active);
        assert_eq!(governance.created_at, 1000);
        assert_eq!(governance.bump, 255);
    }

    #[test]
    fn test_initialize_mesh_group_governance_invalid_id() {
        let mut governance = MeshGroupGovernanceMetadata {
            governance_id: 0,
            mesh_group_id: 0,
            governance_model: MeshGroupGovernanceModel::Consensus,
            status: MeshGroupGovernanceStatus::Disabled,
            created_at: 0,
            governance_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_mesh_group_governance(
            &mut governance,
            0, // Invalid: must be > 0
            10,
            MeshGroupGovernanceModel::Voting,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_mesh_group_governance_model_variants() {
        assert_eq!(MeshGroupGovernanceModel::Consensus, MeshGroupGovernanceModel::Consensus);
        assert_eq!(MeshGroupGovernanceModel::Voting, MeshGroupGovernanceModel::Voting);
        assert_eq!(MeshGroupGovernanceModel::Delegation, MeshGroupGovernanceModel::Delegation);
        assert_eq!(MeshGroupGovernanceModel::Custom, MeshGroupGovernanceModel::Custom);
    }

    #[test]
    fn test_mesh_group_governance_status_variants() {
        assert_eq!(MeshGroupGovernanceStatus::Active, MeshGroupGovernanceStatus::Active);
        assert_eq!(MeshGroupGovernanceStatus::Paused, MeshGroupGovernanceStatus::Paused);
        assert_eq!(MeshGroupGovernanceStatus::Disabled, MeshGroupGovernanceStatus::Disabled);
    }

    #[test]
    fn test_mesh_group_governance_model_all_variants_unique() {
        let models = vec![
            MeshGroupGovernanceModel::Consensus,
            MeshGroupGovernanceModel::Voting,
            MeshGroupGovernanceModel::Delegation,
            MeshGroupGovernanceModel::Custom,
        ];
        
        for i in 0..models.len() {
            for j in (i + 1)..models.len() {
                assert_ne!(models[i], models[j], "Duplicate model found");
            }
        }
    }

    #[test]
    fn test_mesh_group_governance_status_all_variants_unique() {
        let statuses = vec![
            MeshGroupGovernanceStatus::Active,
            MeshGroupGovernanceStatus::Paused,
            MeshGroupGovernanceStatus::Disabled,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_mesh_group_governance_model_equality() {
        assert_eq!(MeshGroupGovernanceModel::Consensus, MeshGroupGovernanceModel::Consensus);
        assert_ne!(MeshGroupGovernanceModel::Consensus, MeshGroupGovernanceModel::Voting);
        assert_eq!(MeshGroupGovernanceModel::Voting, MeshGroupGovernanceModel::Voting);
        assert_ne!(MeshGroupGovernanceModel::Voting, MeshGroupGovernanceModel::Delegation);
        assert_eq!(MeshGroupGovernanceModel::Delegation, MeshGroupGovernanceModel::Delegation);
        assert_ne!(MeshGroupGovernanceModel::Delegation, MeshGroupGovernanceModel::Custom);
        assert_eq!(MeshGroupGovernanceModel::Custom, MeshGroupGovernanceModel::Custom);
    }

    #[test]
    fn test_mesh_group_governance_status_equality() {
        assert_eq!(MeshGroupGovernanceStatus::Active, MeshGroupGovernanceStatus::Active);
        assert_ne!(MeshGroupGovernanceStatus::Active, MeshGroupGovernanceStatus::Paused);
        assert_eq!(MeshGroupGovernanceStatus::Paused, MeshGroupGovernanceStatus::Paused);
        assert_ne!(MeshGroupGovernanceStatus::Paused, MeshGroupGovernanceStatus::Disabled);
        assert_eq!(MeshGroupGovernanceStatus::Disabled, MeshGroupGovernanceStatus::Disabled);
    }

    #[test]
    fn test_mesh_group_governance_model_copy() {
        let model1 = MeshGroupGovernanceModel::Consensus;
        let model2 = model1; // Copy trait
        assert_eq!(model1, model2);
    }

    #[test]
    fn test_mesh_group_governance_status_copy() {
        let status1 = MeshGroupGovernanceStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_mesh_group_governance_model_space() {
        assert_eq!(<MeshGroupGovernanceModel as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_mesh_group_governance_status_space() {
        assert_eq!(<MeshGroupGovernanceStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_mesh_group_governance_all_models() {
        let models = vec![
            MeshGroupGovernanceModel::Consensus,
            MeshGroupGovernanceModel::Voting,
            MeshGroupGovernanceModel::Delegation,
            MeshGroupGovernanceModel::Custom,
        ];
        
        for governance_model in models {
            let mut governance = MeshGroupGovernanceMetadata {
                governance_id: 0,
                mesh_group_id: 0,
                governance_model: MeshGroupGovernanceModel::Consensus,
                status: MeshGroupGovernanceStatus::Disabled,
                created_at: 0,
                governance_config_hash: [0u8; 32],
                bump: 0,
            };
            
            let result = onchain::initialize_mesh_group_governance(
                &mut governance,
                1,
                10,
                governance_model,
                [0u8; 32],
                1000,
                255,
            );
            
            assert!(result.is_ok());
            assert_eq!(governance.governance_model, governance_model);
        }
    }

    #[test]
    fn test_initialize_mesh_group_governance_large_ids() {
        let mut governance = MeshGroupGovernanceMetadata {
            governance_id: 0,
            mesh_group_id: 0,
            governance_model: MeshGroupGovernanceModel::Consensus,
            status: MeshGroupGovernanceStatus::Disabled,
            created_at: 0,
            governance_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_mesh_group_governance(
            &mut governance,
            u64::MAX,
            u64::MAX,
            MeshGroupGovernanceModel::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(governance.governance_id, u64::MAX);
        assert_eq!(governance.mesh_group_id, u64::MAX);
    }

    #[test]
    fn test_initialize_mesh_group_governance_custom_hash() {
        let mut governance = MeshGroupGovernanceMetadata {
            governance_id: 0,
            mesh_group_id: 0,
            governance_model: MeshGroupGovernanceModel::Consensus,
            status: MeshGroupGovernanceStatus::Disabled,
            created_at: 0,
            governance_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let custom_hash = [255u8; 32];
        let result = onchain::initialize_mesh_group_governance(
            &mut governance,
            1,
            10,
            MeshGroupGovernanceModel::Custom,
            custom_hash,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(governance.governance_config_hash, custom_hash);
    }

    #[test]
    fn test_initialize_mesh_group_governance_always_active_on_init() {
        let mut governance = MeshGroupGovernanceMetadata {
            governance_id: 0,
            mesh_group_id: 0,
            governance_model: MeshGroupGovernanceModel::Consensus,
            status: MeshGroupGovernanceStatus::Disabled, // Will be reset
            created_at: 0,
            governance_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_mesh_group_governance(
            &mut governance,
            1,
            10,
            MeshGroupGovernanceModel::Consensus,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        // Status should always be set to Active on initialization
        assert_eq!(governance.status, MeshGroupGovernanceStatus::Active);
    }

    #[test]
    fn test_mesh_group_governance_metadata_all_fields() {
        let governance = MeshGroupGovernanceMetadata {
            governance_id: 123,
            mesh_group_id: 456,
            governance_model: MeshGroupGovernanceModel::Delegation,
            status: MeshGroupGovernanceStatus::Paused,
            created_at: 5000,
            governance_config_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(governance.governance_id, 123);
        assert_eq!(governance.mesh_group_id, 456);
        assert_eq!(governance.governance_model, MeshGroupGovernanceModel::Delegation);
        assert_eq!(governance.status, MeshGroupGovernanceStatus::Paused);
        assert_eq!(governance.created_at, 5000);
        assert_eq!(governance.governance_config_hash, [42u8; 32]);
        assert_eq!(governance.bump, 128);
    }
}
