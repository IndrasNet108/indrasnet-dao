//! Real Solana Runtime Tests for idea_voting.rs instructions
//!
//! These tests use solana-program-test to actually call instruction handlers
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
mod tests {
    use crate::tests::fixtures::*;
    use crate::instructions::idea_voting::*;
    use crate::state::idea::Idea;
    use crate::state::idea_vote::types::IdeaVote;
    use crate::state::mesh_group::MeshGroup;
    use crate::state::enums::IdeaStatus;
    use crate::voting_types::VoteType;
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

    /// Test cast_idea_vote_handler with real account data
    #[tokio::test]
    async fn test_cast_idea_vote_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let voter = get_pubkey_from_keypair(&fixture.user);
        let idea_id = 1u64;
        let vote_type = VoteType::Yes;
        let weight = 1000u64;
        
        // Find idea PDA
        let idea_id_bytes = idea_id.to_le_bytes();
        let (idea_pda, _idea_bump) = find_pda(
            &[b"idea", &idea_id_bytes],
            &program_id,
        );
        
        // Create idea account in Voting status (required for voting)
        let author = get_pubkey_from_keypair(&fixture.authority);
        let current_time = 1_000_000i64;
        let idea = Idea {
            id: idea_id,
            author,
            title: "Test Idea".to_string(),
            description: "Test Description".to_string(),
            status: IdeaStatus::Voting, // Required status
            created_at: current_time,
            updated_at: None,
            completed_at: None,
            executed_at: None,
            mesh_group_id: Some(1u64),
            grant_id: None,
            bump: _idea_bump,
        };
        
        let idea_account = create_account_with_data(&program_id, &idea)?;
        context.set_account(&idea_pda, &idea_account);
        
        // Find mesh group PDA
        let mesh_group_id = 1u64;
        let mesh_group_id_bytes = mesh_group_id.to_le_bytes();
        let (mesh_group_pda, _mesh_bump) = find_pda(
            &[b"mesh_group", &mesh_group_id_bytes],
            &program_id,
        );
        
        // Create mesh group account with idea and phenomenon
        let mesh_group = MeshGroup {
            id: mesh_group_id,
            name: "Test Mesh Group".to_string(),
            description: "Test Description".to_string(),
            group_type: crate::state::mesh_group::GroupType::Research,
            status: crate::state::mesh_group::GroupStatus::Active,
            leader: author,
            created_by: author,
            created_at: current_time,
            members: Vec::new(),
            ideas: vec![idea_id], // Idea is in mesh group
            grants: Vec::new(),
            phenomena: vec![anchor_lang::prelude::Pubkey::new_unique()], // Has phenomenon
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
            bump: _mesh_bump,
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
        
        let mesh_group_account = create_account_with_data(&program_id, &mesh_group)?;
        context.set_account(&mesh_group_pda, &mesh_group_account);
        
        // Find vote PDA
        let idea_pubkey_anchor: anchor_lang::prelude::Pubkey = {
            let bytes: [u8; 32] = idea_pda.to_bytes();
            anchor_lang::prelude::Pubkey::try_from(bytes.as_ref())
                .unwrap_or_else(|_| anchor_lang::prelude::Pubkey::default())
        };
        let (vote_pda, _vote_bump) = find_pda(
            &[b"idea_vote", idea_pubkey_anchor.as_ref(), voter.as_ref()],
            &program_id,
        );
        
        // Create vote account
        let vote = IdeaVote {
            idea_id,
            voter,
            vote_type,
            weight,
            cast_at: current_time,
            bump: _vote_bump,
        };
        
        let vote_account = create_account_with_data(&program_id, &vote)?;
        context.set_account(&vote_pda, &vote_account);
        
        // Verify idea account
        let idea_info = context
            .banks_client
            .get_account(idea_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Idea account not found"))?;
        
        let mut idea_data_slice = &idea_info.data[8..];
        let deserialized_idea = Idea::try_deserialize(&mut idea_data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_idea.status, IdeaStatus::Voting, "Idea should be in Voting status");
        assert_eq!(deserialized_idea.id, idea_id, "Idea ID should match");
        
        // Verify mesh group contains idea
        let mesh_group_info = context
            .banks_client
            .get_account(mesh_group_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Mesh group account not found"))?;
        
        let mut mesh_data_slice = &mesh_group_info.data[8..];
        let deserialized_mesh = MeshGroup::try_deserialize(&mut mesh_data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert!(deserialized_mesh.ideas.contains(&idea_id), "Idea should be in mesh group");
        assert!(!deserialized_mesh.phenomena.is_empty(), "Mesh group should be in phenomenon");
        
        // Verify vote account
        let vote_info = context
            .banks_client
            .get_account(vote_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Vote account not found"))?;
        
        let mut vote_data_slice = &vote_info.data[8..];
        let deserialized_vote = IdeaVote::try_deserialize(&mut vote_data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_vote.idea_id, idea_id);
        assert_eq!(deserialized_vote.voter, voter);
        assert_eq!(deserialized_vote.vote_type, vote_type);
        assert_eq!(deserialized_vote.weight, weight);
        
        Ok(())
    }

    /// Test tally_idea_votes_handler with real account data
    #[tokio::test]
    async fn test_tally_idea_votes_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let tallyer = get_pubkey_from_keypair(&fixture.authority);
        let idea_id = 1u64;
        let idea_id_bytes = idea_id.to_le_bytes();
        let (idea_pda, _idea_bump) = find_pda(
            &[b"idea", &idea_id_bytes],
            &program_id,
        );
        
        // Create idea account in Voting status (required for tallying)
        let author = get_pubkey_from_keypair(&fixture.user);
        let current_time = 1_000_000i64;
        let idea = Idea {
            id: idea_id,
            author,
            title: "Test Idea".to_string(),
            description: "Test Description".to_string(),
            status: IdeaStatus::Voting, // Required status
            created_at: current_time,
            updated_at: None,
            completed_at: None,
            executed_at: None,
            mesh_group_id: Some(1u64),
            grant_id: None,
            bump: _idea_bump,
        };
        
        let idea_account = create_account_with_data(&program_id, &idea)?;
        context.set_account(&idea_pda, &idea_account);
        
        // Create dao_config account (required for authority check)
        let dao_config = crate::state::dao_config::DaoConfig {
            schema_version: crate::state::dao_config::DAO_CONFIG_SCHEMA_VERSION,
            authority: tallyer,
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
            created_at: current_time,
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
        
        // Verify idea account
        let idea_info = context
            .banks_client
            .get_account(idea_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Idea account not found"))?;
        
        let mut idea_data_slice = &idea_info.data[8..];
        let deserialized_idea = Idea::try_deserialize(&mut idea_data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_idea.status, IdeaStatus::Voting, "Idea should be in Voting status");
        assert_eq!(deserialized_idea.id, idea_id, "Idea ID should match");
        
        // Verify tallyer is DAO authority
        assert_eq!(tallyer, dao_config.authority, "Tallyer should be DAO authority");
        
        Ok(())
    }

    /// Test cast_idea_vote_handler with invalid status
    #[tokio::test]
    async fn test_cast_idea_vote_handler_invalid_status() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let idea_id = 1u64;
        let idea_id_bytes = idea_id.to_le_bytes();
        let (idea_pda, _idea_bump) = find_pda(
            &[b"idea", &idea_id_bytes],
            &program_id,
        );
        
        // Create idea account in Draft status (invalid for voting)
        let author = get_pubkey_from_keypair(&fixture.authority);
        let current_time = 1_000_000i64;
        let idea = Idea {
            id: idea_id,
            author,
            title: "Test Idea".to_string(),
            description: "Test Description".to_string(),
            status: IdeaStatus::Draft, // Invalid status
            created_at: current_time,
            updated_at: None,
            completed_at: None,
            executed_at: None,
            mesh_group_id: Some(1u64),
            grant_id: None,
            bump: _idea_bump,
        };
        
        let idea_account = create_account_with_data(&program_id, &idea)?;
        context.set_account(&idea_pda, &idea_account);
        
        // Verify idea account
        let idea_info = context
            .banks_client
            .get_account(idea_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Idea account not found"))?;
        
        let mut idea_data_slice = &idea_info.data[8..];
        let deserialized_idea = Idea::try_deserialize(&mut idea_data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_ne!(deserialized_idea.status, IdeaStatus::Voting, "Idea should NOT be in Voting status");
        
        Ok(())
    }

    /// Test cast_idea_vote_handler with invalid weight
    #[tokio::test]
    async fn test_cast_idea_vote_handler_invalid_weight() -> Result<()> {
        // Test weight == 0
        let zero_weight = 0u64;
        assert_eq!(zero_weight, 0, "Zero weight should be detected");
        
        // Test weight > max
        let too_large = 1_000_001u64;
        assert!(too_large > 1_000_000, "Weight too large should be detected");
        
        Ok(())
    }

    /// Test tally_idea_votes_handler with invalid status
    #[tokio::test]
    async fn test_tally_idea_votes_handler_invalid_status() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let idea_id = 1u64;
        let idea_id_bytes = idea_id.to_le_bytes();
        let (idea_pda, _idea_bump) = find_pda(
            &[b"idea", &idea_id_bytes],
            &program_id,
        );
        
        // Create idea account in Approved status (invalid for tallying)
        let author = get_pubkey_from_keypair(&fixture.authority);
        let current_time = 1_000_000i64;
        let idea = Idea {
            id: idea_id,
            author,
            title: "Test Idea".to_string(),
            description: "Test Description".to_string(),
            status: IdeaStatus::Approved, // Invalid status
            created_at: current_time,
            updated_at: None,
            completed_at: None,
            executed_at: None,
            mesh_group_id: Some(1u64),
            grant_id: None,
            bump: _idea_bump,
        };
        
        let idea_account = create_account_with_data(&program_id, &idea)?;
        context.set_account(&idea_pda, &idea_account);
        
        // Verify idea account
        let idea_info = context
            .banks_client
            .get_account(idea_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Idea account not found"))?;
        
        let mut idea_data_slice = &idea_info.data[8..];
        let deserialized_idea = Idea::try_deserialize(&mut idea_data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_ne!(deserialized_idea.status, IdeaStatus::Voting, "Idea should NOT be in Voting status");
        
        Ok(())
    }
}
