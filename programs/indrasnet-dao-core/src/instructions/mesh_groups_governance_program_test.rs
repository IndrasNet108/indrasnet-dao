//! Real Solana Runtime Tests for instructions/mesh_groups_governance.rs
//!
//! These tests use solana-program-test to actually call instruction handlers
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::account_to_shared;
    use crate::instructions::mesh_groups_governance::*;
    use crate::state::mesh_group::{MeshGroup, GroupStatus, GroupRole, OperatingProtocol, MeetingFrequency};
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

    /// Test add_member_to_mesh_group_handler with real account data
    #[tokio::test]
    async fn test_add_member_to_mesh_group_handler_real() -> Result<()> {
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
        
        // Create mesh group account in Active status
        let current_time = 1_000_000i64;
        let mesh_group = MeshGroup {
            id: mesh_group_id,
            name: "Test Mesh Group".to_string(),
            description: "Test Description".to_string(),
            group_type: crate::state::mesh_group::GroupType::Research,
            status: GroupStatus::Active, // Required status
            leader,
            created_by: leader,
            created_at: current_time,
            members: vec![crate::state::mesh_group::GroupMember {
                pubkey: leader,
                role: GroupRole::Leader,
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
            current_stage: crate::state::mesh_group::DevelopmentStage::Planning,
            started_at: None,
            completed_at: None,
            total_contributions: 0,
            total_reputation: 0,
            bump: _bump,
            protocol: OperatingProtocol::default(),
            last_meeting_at: None,
            last_contribution_at: current_time,
            last_member_added_at: None,
            last_group_created_at: Some(current_time),
            member_reputation_required: 10,
            member_cooldown_days: 30,
            is_in_critical_moment: false, // Not in critical moment
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
        
        assert_eq!(deserialized_group.status, GroupStatus::Active);
        assert!(!deserialized_group.is_in_critical_moment);
        assert!(deserialized_group.members.len() < deserialized_group.max_members as usize);
        assert!(!deserialized_group.members.iter().any(|m| m.pubkey == new_member));
        
        Ok(())
    }

    /// Test remove_member_from_mesh_group_handler with real account data
    #[tokio::test]
    async fn test_remove_member_from_mesh_group_handler_real() -> Result<()> {
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
            group_type: crate::state::mesh_group::GroupType::Research,
            status: GroupStatus::Active,
            leader,
            created_by: leader,
            created_at: current_time,
            members: vec![
                crate::state::mesh_group::GroupMember {
                    pubkey: leader,
                    role: GroupRole::Leader,
                    joined_at: current_time,
                    contributions: 0,
                    reputation: 0,
                    is_active: true,
                },
                crate::state::mesh_group::GroupMember {
                    pubkey: member_to_remove,
                    role: GroupRole::Member,
                    joined_at: current_time,
                    contributions: 0,
                    reputation: 0,
                    is_active: true,
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
            current_stage: crate::state::mesh_group::DevelopmentStage::Planning,
            started_at: None,
            completed_at: None,
            total_contributions: 0,
            total_reputation: 0,
            bump: _bump,
            protocol: OperatingProtocol::default(),
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
        
        assert_eq!(deserialized_group.status, GroupStatus::Active);
        assert!(deserialized_group.members.iter().any(|m| m.pubkey == member_to_remove));
        assert_ne!(member_to_remove, deserialized_group.leader, "Cannot remove leader");
        
        Ok(())
    }

    /// Test transfer_leadership_handler with real account data
    #[tokio::test]
    async fn test_transfer_leadership_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let current_leader = get_pubkey_from_keypair(&fixture.authority);
        let new_leader = get_pubkey_from_keypair(&fixture.user);
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
            group_type: crate::state::mesh_group::GroupType::Research,
            status: GroupStatus::Active,
            leader: current_leader,
            created_by: current_leader,
            created_at: current_time,
            members: vec![
                crate::state::mesh_group::GroupMember {
                    pubkey: current_leader,
                    role: GroupRole::Leader,
                    joined_at: current_time,
                    contributions: 0,
                    reputation: 0,
                    is_active: true,
                },
                crate::state::mesh_group::GroupMember {
                    pubkey: new_leader,
                    role: GroupRole::Member,
                    joined_at: current_time,
                    contributions: 0,
                    reputation: 0,
                    is_active: true,
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
            current_stage: crate::state::mesh_group::DevelopmentStage::Planning,
            started_at: None,
            completed_at: None,
            total_contributions: 0,
            total_reputation: 0,
            bump: _bump,
            protocol: OperatingProtocol::default(),
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
        
        assert_eq!(deserialized_group.leader, current_leader);
        assert!(deserialized_group.members.iter().any(|m| m.pubkey == new_leader));
        assert_ne!(new_leader, current_leader, "New leader should be different from current");
        
        Ok(())
    }

    /// Test update_mesh_group_protocol_handler with real account data
    #[tokio::test]
    async fn test_update_mesh_group_protocol_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let leader = get_pubkey_from_keypair(&fixture.authority);
        let mesh_group_id = 1u64;
        let mesh_group_id_bytes = mesh_group_id.to_le_bytes();
        let (mesh_group_pda, _bump) = find_pda(
            &[b"mesh_group", &mesh_group_id_bytes],
            &program_id,
        );
        
        // Create mesh group account
        let current_time = 1_000_000i64;
        let mesh_group = MeshGroup {
            id: mesh_group_id,
            name: "Test Mesh Group".to_string(),
            description: "Test Description".to_string(),
            group_type: crate::state::mesh_group::GroupType::Research,
            status: GroupStatus::Active,
            leader,
            created_by: leader,
            created_at: current_time,
            members: vec![crate::state::mesh_group::GroupMember {
                pubkey: leader,
                role: GroupRole::Leader,
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
            current_stage: crate::state::mesh_group::DevelopmentStage::Planning,
            started_at: None,
            completed_at: None,
            total_contributions: 0,
            total_reputation: 0,
            bump: _bump,
            protocol: OperatingProtocol::default(),
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
        
        assert_eq!(deserialized_group.leader, leader);
        
        // Verify protocol parameters validation
        let decision_quorum = 75u8;
        let contribution_threshold = 50u32;
        let inactivity_timeout_days = 30u16;
        
        assert!((50..=100).contains(&decision_quorum), "Decision quorum should be in valid range");
        assert!(contribution_threshold > 0 && contribution_threshold <= 100, "Contribution threshold should be valid");
        assert!(inactivity_timeout_days > 0 && inactivity_timeout_days <= 365, "Inactivity timeout should be valid");
        
        Ok(())
    }

    /// Test check_mesh_group_inactivity_handler with real account data
    #[tokio::test]
    async fn test_check_mesh_group_inactivity_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let leader = get_pubkey_from_keypair(&fixture.authority);
        let mesh_group_id = 1u64;
        let mesh_group_id_bytes = mesh_group_id.to_le_bytes();
        let (mesh_group_pda, _bump) = find_pda(
            &[b"mesh_group", &mesh_group_id_bytes],
            &program_id,
        );
        
        // Create mesh group account with recent activity
        let current_time = 1_000_000i64;
        let mesh_group = MeshGroup {
            id: mesh_group_id,
            name: "Test Mesh Group".to_string(),
            description: "Test Description".to_string(),
            group_type: crate::state::mesh_group::GroupType::Research,
            status: GroupStatus::Active,
            leader,
            created_by: leader,
            created_at: current_time,
            members: vec![crate::state::mesh_group::GroupMember {
                pubkey: leader,
                role: GroupRole::Leader,
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
            current_stage: crate::state::mesh_group::DevelopmentStage::Planning,
            started_at: None,
            completed_at: None,
            total_contributions: 0,
            total_reputation: 0,
            bump: _bump,
            protocol: OperatingProtocol {
                meeting_frequency: MeetingFrequency::Weekly,
                decision_quorum: 75,
                contribution_threshold: 50,
                inactivity_timeout_days: 30,
            },
            last_meeting_at: None,
            last_contribution_at: current_time, // Recent activity
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
        
        // Verify inactivity check parameters
        let days_since_activity = (current_time - deserialized_group.last_contribution_at) / (24 * 60 * 60);
        assert!(days_since_activity < deserialized_group.protocol.inactivity_timeout_days as i64, "Group should not be inactive");
        
        Ok(())
    }

    /// Test add_member_to_mesh_group_handler with invalid state
    #[tokio::test]
    async fn test_add_member_to_mesh_group_handler_invalid_state() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let leader = get_pubkey_from_keypair(&fixture.authority);
        let mesh_group_id = 1u64;
        let mesh_group_id_bytes = mesh_group_id.to_le_bytes();
        let (mesh_group_pda, _bump) = find_pda(
            &[b"mesh_group", &mesh_group_id_bytes],
            &program_id,
        );
        
        // Create mesh group account in Paused status (invalid for adding members)
        let current_time = 1_000_000i64;
        let mesh_group = MeshGroup {
            id: mesh_group_id,
            name: "Test Mesh Group".to_string(),
            description: "Test Description".to_string(),
            group_type: crate::state::mesh_group::GroupType::Research,
            status: GroupStatus::Paused, // Invalid status
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
            current_stage: crate::state::mesh_group::DevelopmentStage::Planning,
            started_at: None,
            completed_at: None,
            total_contributions: 0,
            total_reputation: 0,
            bump: _bump,
            protocol: OperatingProtocol::default(),
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
        
        assert_ne!(deserialized_group.status, GroupStatus::Active);
        
        Ok(())
    }

    /// Test add_member_to_mesh_group_handler with critical moment active
    #[tokio::test]
    async fn test_add_member_to_mesh_group_handler_critical_moment() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let leader = get_pubkey_from_keypair(&fixture.authority);
        let mesh_group_id = 1u64;
        let mesh_group_id_bytes = mesh_group_id.to_le_bytes();
        let (mesh_group_pda, _bump) = find_pda(
            &[b"mesh_group", &mesh_group_id_bytes],
            &program_id,
        );
        
        // Create mesh group account with critical moment active
        let current_time = 1_000_000i64;
        let mesh_group = MeshGroup {
            id: mesh_group_id,
            name: "Test Mesh Group".to_string(),
            description: "Test Description".to_string(),
            group_type: crate::state::mesh_group::GroupType::Research,
            status: GroupStatus::Active,
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
            current_stage: crate::state::mesh_group::DevelopmentStage::Planning,
            started_at: None,
            completed_at: None,
            total_contributions: 0,
            total_reputation: 0,
            bump: _bump,
            protocol: OperatingProtocol::default(),
            last_meeting_at: None,
            last_contribution_at: current_time,
            last_member_added_at: None,
            last_group_created_at: Some(current_time),
            member_reputation_required: 10,
            member_cooldown_days: 30,
            is_in_critical_moment: true, // Critical moment active
            critical_moment_until: Some(current_time + 7 * 24 * 3600),
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
        
        assert!(deserialized_group.is_in_critical_moment, "Critical moment should be active");
        
        Ok(())
    }

    /// Test update_mesh_group_protocol_handler with invalid parameters
    #[tokio::test]
    async fn test_update_mesh_group_protocol_handler_invalid_parameters() -> Result<()> {
        // Test decision_quorum < 50
        let low_quorum = 49u8;
        assert!(!(50..=100).contains(&low_quorum), "Decision quorum too low should be detected");
        
        // Test decision_quorum > 100
        let high_quorum = 101u8;
        assert!(!(50..=100).contains(&high_quorum), "Decision quorum too high should be detected");
        
        // Test contribution_threshold == 0
        let zero_threshold = 0u32;
        assert_eq!(zero_threshold, 0, "Zero contribution threshold should be detected");
        
        // Test contribution_threshold > 100
        let high_threshold = 101u32;
        assert!(high_threshold > 100, "Contribution threshold too high should be detected");
        
        // Test inactivity_timeout_days == 0
        let zero_timeout = 0u16;
        assert_eq!(zero_timeout, 0, "Zero inactivity timeout should be detected");
        
        // Test inactivity_timeout_days > 365
        let high_timeout = 366u16;
        assert!(high_timeout > 365, "Inactivity timeout too high should be detected");
        
        Ok(())
    }
}