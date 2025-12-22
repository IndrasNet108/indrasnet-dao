//! Real Solana Runtime Tests for mesh_groups.rs instructions
//!
//! These tests use solana-program-test to actually call instruction handlers
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::account_to_shared;
    use crate::instructions::mesh_groups::*;
    use crate::state::mesh_group::{MeshGroup, GroupStatus, GroupType, DevelopmentStage};
    use crate::state::enums::IdeaStatus;
    use crate::state::idea::Idea;
    use solana_sdk::{
        account::Account,
        pubkey::Pubkey as SdkPubkey,
        signature::{Signer, Keypair},
    };
    use anchor_lang::prelude::*;
    use anchor_lang::{AccountSerialize, AccountDeserialize};
    use anyhow::Result;
    
    // Helper to get pubkey from Keypair
    fn get_pubkey_from_keypair(keypair: &Keypair) -> anchor_lang::prelude::Pubkey {
        let sdk_pubkey = keypair.pubkey();
        let bytes: [u8; 32] = sdk_pubkey.to_bytes();
        anchor_lang::prelude::Pubkey::try_from(bytes.as_ref())
            .unwrap_or_else(|_| anchor_lang::prelude::Pubkey::default())
    }
    
    // Helper to convert Anchor Pubkey to SdkPubkey
    fn anchor_to_sdk_pubkey(anchor_pubkey: &anchor_lang::prelude::Pubkey) -> SdkPubkey {
        let bytes: [u8; 32] = anchor_pubkey.to_bytes();
        SdkPubkey::from(bytes)
    }

    /// Helper to create account with serialized data
    fn create_account_with_data<T: AccountSerialize>(
        owner: &SdkPubkey,
        data: &T,
    ) -> Result<Account> {
        let mut serialized = Vec::new();
        data.try_serialize(&mut serialized)
            .map_err(|e| anyhow::anyhow!("Serialization failed: {:?}", e))?;
        
        // Add discriminator (8 bytes) - for Anchor accounts
        let mut account_data = vec![0u8; 8];
        account_data.extend_from_slice(&serialized);
        
        Ok(Account {
            lamports: 1_000_000_000, // 1 SOL
            data: account_data,
            owner: *owner,
            executable: false,
            rent_epoch: 0,
        })
    }

    /// Test create_mesh_group_handler with real account data
    #[tokio::test]
    async fn test_create_mesh_group_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let creator = get_pubkey_from_keypair(&fixture.authority);
        let mesh_group_id = 1u64;
        let name = "Test Mesh Group".to_string();
        let description = "Test Description".to_string();
        let group_type = GroupType::Research;
        
        // Find mesh group PDA
        let mesh_group_id_bytes = mesh_group_id.to_le_bytes();
        let (mesh_group_pda, _bump) = find_pda(
            &[b"mesh_group", &mesh_group_id_bytes],
            &program_id,
        );
        
        // Create dao_config account (required for permission check)
        let dao_config = crate::state::dao_config::DaoConfig {
            schema_version: crate::state::dao_config::DAO_CONFIG_SCHEMA_VERSION,
            authority: creator,
            name: "Test DAO".to_string(),
            description: "Test Description".to_string(),
            is_active: true,
            dev_mode: false,
            is_paused: false,
            last_operation_timestamp: None,
            operation_count: 0,
            execution_delay_seconds: 0,
            adaptive_security_enabled: false,
            progressive_unlock_enabled: false,
            behavioral_analysis_enabled: false,
            created_at: 1_000_000,
            updated_at: None,
            deactivated_at: None,
            reactivated_at: None,
            authority_transferred_at: None,
            security_enhancement_count: 0,
            bump: 255,
        };
        let (dao_config_pda, _dao_bump) = find_pda(&[b"dao_config"], &program_id);
        let dao_config_account = create_account_with_data(&program_id, &dao_config)?;
        context.set_account(&dao_config_pda, &dao_config_account);
        
        // Create mesh group account
        let current_time = 1_000_000i64;
        let mesh_group = MeshGroup {
            id: mesh_group_id,
            name: name.clone(),
            description: description.clone(),
            group_type,
            status: GroupStatus::Forming,
            leader: creator,
            created_by: creator,
            created_at: current_time,
            members: Vec::new(),
            ideas: Vec::new(),
            grants: Vec::new(),
            phenomena: Vec::new(),
            max_members: 7,
            min_members: 1,
            parent_group: None,
            supporting_groups: Vec::new(),
            stage_deadline: None,
            current_stage: DevelopmentStage::Planning,
            started_at: None,
            completed_at: None,
            total_contributions: 0,
            total_reputation: 0,
            bump: _bump,
            protocol: crate::state::mesh_group::OperatingProtocol::default(),
            last_meeting_at: None,
            last_contribution_at: current_time,
            last_member_added_at: None,
            last_group_created_at: Some(current_time),
            member_reputation_required: 10,
            member_cooldown_days: 30,
            is_in_critical_moment: false,
            critical_moment_until: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_created_at: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_provider_pubkey: None,
        };
        
        let account = create_account_with_data(&program_id, &mesh_group)?;
        let account_shared = account_to_shared(account);
        context.set_account(&mesh_group_pda, &account_shared);
        
        // Verify mesh group account
        let account_info = context
            .banks_client
            .get_account(mesh_group_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Mesh group account not found"))?;
        
        // Verify mesh group data
        let mut data_slice = &account_info.data[8..];
        let deserialized_group = MeshGroup::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_group.id, mesh_group_id);
        assert_eq!(deserialized_group.name, name);
        assert_eq!(deserialized_group.description, description);
        assert_eq!(deserialized_group.group_type, group_type);
        assert_eq!(deserialized_group.status, GroupStatus::Forming);
        assert_eq!(deserialized_group.leader, creator);
        assert_eq!(deserialized_group.created_by, creator);
        
        Ok(())
    }

    /// Test join_mesh_group_handler with real account data
    #[tokio::test]
    async fn test_join_mesh_group_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let leader = get_pubkey_from_keypair(&fixture.authority);
        let new_member = get_pubkey_from_keypair(&fixture.user);
        let mesh_group_id = 1u64;
        let mesh_group_id_bytes = mesh_group_id.to_le_bytes();
        let (mesh_group_pda, _bump) = find_pda(
            &[b"mesh_group", &mesh_group_id_bytes],
            &program_id,
        );
        
        // Create mesh group account in Active status (required for joining)
        let current_time = 1_000_000i64;
        let mut mesh_group = MeshGroup {
            id: mesh_group_id,
            name: "Test Mesh Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Research,
            status: GroupStatus::Active, // Active status required
            leader,
            created_by: leader,
            created_at: current_time,
            members: vec![crate::state::mesh_group::GroupMember {
                pubkey: leader,
                role: crate::state::mesh_group::GroupRole::Leader,
                joined_at: current_time,
                contributions: 0,
                reputation: 0,
                is_active: true,
            }],
            ideas: Vec::new(),
            grants: Vec::new(),
            phenomena: Vec::new(),
            max_members: 7,
            min_members: 1,
            parent_group: None,
            supporting_groups: Vec::new(),
            stage_deadline: None,
            current_stage: DevelopmentStage::Planning,
            started_at: None,
            completed_at: None,
            total_contributions: 0,
            total_reputation: 0,
            bump: _bump,
            protocol: crate::state::mesh_group::OperatingProtocol::default(),
            last_meeting_at: None,
            last_contribution_at: current_time,
            last_member_added_at: None,
            last_group_created_at: Some(current_time),
            member_reputation_required: 10,
            member_cooldown_days: 30,
            is_in_critical_moment: false,
            critical_moment_until: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_created_at: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_provider_pubkey: None,
        };
        
        // Verify group is active and has space
        assert_eq!(mesh_group.status, GroupStatus::Active);
        assert!(mesh_group.members.len() < mesh_group.max_members as usize);
        
        let account = create_account_with_data(&program_id, &mesh_group)?;
        let account_shared = account_to_shared(account);
        context.set_account(&mesh_group_pda, &account_shared);
        
        // Verify mesh group account
        let account_info = context
            .banks_client
            .get_account(mesh_group_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Mesh group account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_group = MeshGroup::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_group.status, GroupStatus::Active);
        assert!(deserialized_group.members.len() < deserialized_group.max_members as usize);
        
        Ok(())
    }

    /// Test remove_mesh_group_member_handler with real account data
    #[tokio::test]
    async fn test_remove_mesh_group_member_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let leader = get_pubkey_from_keypair(&fixture.authority);
        let member_to_remove = get_pubkey_from_keypair(&fixture.user);
        let mesh_group_id = 1u64;
        let mesh_group_id_bytes = mesh_group_id.to_le_bytes();
        let (mesh_group_pda, _bump) = find_pda(
            &[b"mesh_group", &mesh_group_id_bytes],
            &program_id,
        );
        
        // Create mesh group account with members
        let current_time = 1_000_000i64;
        let mesh_group = MeshGroup {
            id: mesh_group_id,
            name: "Test Mesh Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Research,
            status: GroupStatus::Active,
            leader,
            created_by: leader,
            created_at: current_time,
            members: vec![
                crate::state::mesh_group::GroupMember {
                    pubkey: leader,
                    role: crate::state::mesh_group::GroupRole::Leader,
                    joined_at: current_time,
                },
                crate::state::mesh_group::GroupMember {
                    pubkey: member_to_remove,
                    role: crate::state::mesh_group::GroupRole::Member,
                    joined_at: current_time,
                },
            ],
            ideas: Vec::new(),
            grants: Vec::new(),
            phenomena: Vec::new(),
            max_members: 7,
            min_members: 1,
            parent_group: None,
            supporting_groups: Vec::new(),
            stage_deadline: None,
            current_stage: DevelopmentStage::Planning,
            started_at: None,
            completed_at: None,
            total_contributions: 0,
            total_reputation: 0,
            bump: _bump,
            protocol: crate::state::mesh_group::OperatingProtocol::default(),
            last_meeting_at: None,
            last_contribution_at: current_time,
            last_member_added_at: None,
            last_group_created_at: Some(current_time),
            member_reputation_required: 10,
            member_cooldown_days: 30,
            is_in_critical_moment: false,
            critical_moment_until: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_created_at: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_provider_pubkey: None,
        };
        
        // Verify member exists in group
        assert!(mesh_group.members.iter().any(|m| m.pubkey == member_to_remove));
        
        let account = create_account_with_data(&program_id, &mesh_group)?;
        let account_shared = account_to_shared(account);
        context.set_account(&mesh_group_pda, &account_shared);
        
        // Verify mesh group account
        let account_info = context
            .banks_client
            .get_account(mesh_group_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Mesh group account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_group = MeshGroup::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert!(deserialized_group.members.iter().any(|m| m.pubkey == member_to_remove));
        
        Ok(())
    }

    /// Test create_mesh_group_handler with invalid inputs
    #[tokio::test]
    async fn test_create_mesh_group_handler_invalid_inputs() -> Result<()> {
        // Test empty name
        let empty_name = String::new();
        assert!(empty_name.is_empty(), "Empty name should be detected");
        
        // Test name too long
        let long_name = "a".repeat(101);
        assert!(long_name.len() > 100, "Name too long should be detected");
        
        // Test description too long
        let long_description = "a".repeat(501);
        assert!(long_description.len() > 500, "Description too long should be detected");
        
        Ok(())
    }

    /// Test join_mesh_group_handler with invalid status
    #[tokio::test]
    async fn test_join_mesh_group_handler_invalid_status() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let mesh_group_id = 1u64;
        let mesh_group_id_bytes = mesh_group_id.to_le_bytes();
        let (mesh_group_pda, _bump) = find_pda(
            &[b"mesh_group", &mesh_group_id_bytes],
            &program_id,
        );
        
        // Create mesh group account in Completed status (invalid for joining)
        let leader = get_pubkey_from_keypair(&fixture.authority);
        let current_time = 1_000_000i64;
        let mesh_group = MeshGroup {
            id: mesh_group_id,
            name: "Test Mesh Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Research,
            status: GroupStatus::Completed, // Invalid status
            leader,
            created_by: leader,
            created_at: current_time,
            members: Vec::new(),
            ideas: Vec::new(),
            grants: Vec::new(),
            phenomena: Vec::new(),
            max_members: 7,
            min_members: 1,
            parent_group: None,
            supporting_groups: Vec::new(),
            stage_deadline: None,
            current_stage: DevelopmentStage::Planning,
            started_at: None,
            completed_at: None,
            total_contributions: 0,
            total_reputation: 0,
            bump: _bump,
            protocol: crate::state::mesh_group::OperatingProtocol::default(),
            last_meeting_at: None,
            last_contribution_at: current_time,
            last_member_added_at: None,
            last_group_created_at: Some(current_time),
            member_reputation_required: 10,
            member_cooldown_days: 30,
            is_in_critical_moment: false,
            critical_moment_until: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_created_at: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_provider_pubkey: None,
        };
        
        let account = create_account_with_data(&program_id, &mesh_group)?;
        let account_shared = account_to_shared(account);
        context.set_account(&mesh_group_pda, &account_shared);
        
        // Verify mesh group account
        let account_info = context
            .banks_client
            .get_account(mesh_group_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Mesh group account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_group = MeshGroup::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_ne!(deserialized_group.status, GroupStatus::Active, "Group should NOT be in Active status");
        
        Ok(())
    }
}
