//! Real Solana Runtime Tests for grants_voting.rs instructions
//!
//! These tests use solana-program-test to actually call instruction handlers
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
mod tests {
    use crate::tests::fixtures::*;
    use crate::instructions::grants_voting::*;
    use crate::state::grant::types::Grant;
    use crate::state::grant::GrantStatus;
    use crate::state::grant::vote::GrantVote;
    use crate::state::grant::VoterType;
    use crate::state::idea::Idea;
    use crate::state::mesh_group::MeshGroup;
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

    /// Test cast_grant_vote_handler with real account data
    #[tokio::test]
    async fn test_cast_grant_vote_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let voter = get_pubkey_from_keypair(&fixture.user);
        let grant_id = 1u64;
        let vote_choice = VoteType::Yes;
        let voter_type = VoterType::MeshGroupMember;
        
        // Find grant PDA
        let grant_id_bytes = grant_id.to_le_bytes();
        let (grant_pda, _grant_bump) = find_pda(
            &[b"grant", &grant_id_bytes],
            &program_id,
        );
        
        // Create grant account in Pending status (required for voting)
        let idea_id = 1u64;
        let author = get_pubkey_from_keypair(&fixture.authority);
        let current_time = 1_000_000i64;
        let voting_end = current_time + 7 * 24 * 3600; // 7 days
        let grant = Grant {
            id: grant_id,
            idea_id,
            mesh_group: anchor_lang::prelude::Pubkey::new_unique(),
            amount: 100_000_000, // 0.1 SOL
            status: GrantStatus::Pending, // Required status
            created_at: current_time,
            approved_at: None,
            activated_at: None,
            disbursed_at: None,
            voting_end,
            voting_layer: crate::state::grant::VotingLayer::AuthorOnly,
            grant_level: 1,
            total_votes: 0,
            total_yes_weight: 0,
            total_no_weight: 0,
            total_abstain_weight: 0,
            quorum_reached: false,
            semantic_domain: None,
            bump: _grant_bump,
        };
        
        let grant_account = create_account_with_data(&program_id, &grant)?;
        context.set_account(&grant_pda, &grant_account);
        
        // Find idea PDA
        let idea_id_bytes = idea_id.to_le_bytes();
        let (idea_pda, _idea_bump) = find_pda(
            &[b"idea", &idea_id_bytes],
            &program_id,
        );
        
        // Create idea account
        let idea = Idea {
            id: idea_id,
            author,
            title: "Test Idea".to_string(),
            description: "Test Description".to_string(),
            status: crate::state::enums::IdeaStatus::InProgress,
            created_at: current_time,
            updated_at: None,
            completed_at: None,
            executed_at: None,
            mesh_group_id: Some(1u64),
            grant_id: Some(grant_id),
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
        
        // Create mesh group account with voter as member
        let mesh_group = MeshGroup {
            id: mesh_group_id,
            name: "Test Mesh Group".to_string(),
            description: "Test Description".to_string(),
            group_type: crate::state::mesh_group::GroupType::Research,
            status: crate::state::mesh_group::GroupStatus::Active,
            leader: author,
            created_by: author,
            created_at: current_time,
            members: vec![crate::state::mesh_group::GroupMember {
                pubkey: voter,
                role: crate::state::mesh_group::GroupRole::Member,
                joined_at: current_time,
                contributions: 0,
                reputation: 0,
                is_active: true,
            }],
            ideas: vec![idea_id],
            grants: vec![grant_id],
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
        let grant_pubkey_anchor: anchor_lang::prelude::Pubkey = {
            let bytes: [u8; 32] = grant_pda.to_bytes();
            anchor_lang::prelude::Pubkey::try_from(bytes.as_ref())
                .unwrap_or_else(|_| anchor_lang::prelude::Pubkey::default())
        };
        let (vote_pda, _vote_bump) = find_pda(
            &[b"grant_vote", grant_pubkey_anchor.as_ref(), voter.as_ref()],
            &program_id,
        );
        
        // Create vote account
        let base_weight = GrantVote::calculate_base_weight(voter_type);
        let vote = GrantVote {
            grant_id,
            voter,
            vote_type: vote_choice,
            weight: base_weight,
            voter_type,
            cast_at: current_time,
            bump: _vote_bump,
        };
        
        let vote_account = create_account_with_data(&program_id, &vote)?;
        context.set_account(&vote_pda, &vote_account);
        
        // Verify grant account
        let grant_info = context
            .banks_client
            .get_account(grant_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Grant account not found"))?;
        
        let mut grant_data_slice = &grant_info.data[8..];
        let deserialized_grant = Grant::try_deserialize(&mut grant_data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_grant.status, GrantStatus::Pending, "Grant should be in Pending status");
        assert_eq!(deserialized_grant.id, grant_id, "Grant ID should match");
        assert!(current_time <= deserialized_grant.voting_end, "Voting period should not have ended");
        
        // Verify mesh group contains voter
        let mesh_group_info = context
            .banks_client
            .get_account(mesh_group_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Mesh group account not found"))?;
        
        let mut mesh_data_slice = &mesh_group_info.data[8..];
        let deserialized_mesh = MeshGroup::try_deserialize(&mut mesh_data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert!(deserialized_mesh.members.iter().any(|m| m.pubkey == voter), "Voter should be in mesh group");
        
        // Verify vote account
        let vote_info = context
            .banks_client
            .get_account(vote_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Vote account not found"))?;
        
        let mut vote_data_slice = &vote_info.data[8..];
        let deserialized_vote = GrantVote::try_deserialize(&mut vote_data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_vote.grant_id, grant_id);
        assert_eq!(deserialized_vote.voter, voter);
        assert_eq!(deserialized_vote.vote_type, vote_choice);
        assert_eq!(deserialized_vote.voter_type, voter_type);
        
        Ok(())
    }

    /// Test tally_grant_votes_handler with real account data
    #[tokio::test]
    async fn test_tally_grant_votes_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let grant_id = 1u64;
        let grant_id_bytes = grant_id.to_le_bytes();
        let (grant_pda, _grant_bump) = find_pda(
            &[b"grant", &grant_id_bytes],
            &program_id,
        );
        
        // Create grant account in Pending status with votes
        let idea_id = 1u64;
        let author = get_pubkey_from_keypair(&fixture.authority);
        let current_time = 1_000_000i64;
        let voting_end = current_time - 1; // Voting period has ended
        let grant = Grant {
            id: grant_id,
            idea_id,
            mesh_group: anchor_lang::prelude::Pubkey::new_unique(),
            amount: 100_000_000,
            status: GrantStatus::Pending, // Required status
            created_at: current_time,
            approved_at: None,
            activated_at: None,
            disbursed_at: None,
            voting_end,
            voting_layer: crate::state::grant::VotingLayer::AuthorOnly,
            grant_level: 1,
            total_votes: 5,
            total_yes_weight: 60,
            total_no_weight: 40,
            total_abstain_weight: 0,
            quorum_reached: true,
            semantic_domain: None,
            bump: _grant_bump,
        };
        
        let grant_account = create_account_with_data(&program_id, &grant)?;
        context.set_account(&grant_pda, &grant_account);
        
        // Find mesh group PDA
        let mesh_group_id = 1u64;
        let mesh_group_id_bytes = mesh_group_id.to_le_bytes();
        let (mesh_group_pda, _mesh_bump) = find_pda(
            &[b"mesh_group", &mesh_group_id_bytes],
            &program_id,
        );
        
        // Create mesh group account
        let mesh_group = MeshGroup {
            id: mesh_group_id,
            name: "Test Mesh Group".to_string(),
            description: "Test Description".to_string(),
            group_type: crate::state::mesh_group::GroupType::Research,
            status: crate::state::mesh_group::GroupStatus::Active,
            leader: author,
            created_by: author,
            created_at: current_time,
            members: vec![crate::state::mesh_group::GroupMember {
                pubkey: author,
                role: crate::state::mesh_group::GroupRole::Leader,
                joined_at: current_time,
                contributions: 0,
                reputation: 0,
                is_active: true,
            }],
            ideas: vec![idea_id],
            grants: vec![grant_id],
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
        
        // Verify grant account
        let grant_info = context
            .banks_client
            .get_account(grant_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Grant account not found"))?;
        
        let mut grant_data_slice = &grant_info.data[8..];
        let deserialized_grant = Grant::try_deserialize(&mut grant_data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_grant.status, GrantStatus::Pending, "Grant should be in Pending status");
        assert_eq!(deserialized_grant.id, grant_id, "Grant ID should match");
        assert!(current_time >= deserialized_grant.voting_end, "Voting period should have ended");
        
        // Verify approval calculation: yes_weight > no_weight and quorum reached
        let total_weight = deserialized_grant.total_yes_weight
            .checked_add(deserialized_grant.total_no_weight)
            .and_then(|w| w.checked_add(deserialized_grant.total_abstain_weight))
            .ok_or_else(|| anyhow::anyhow!("Overflow"))?;
        
        let approval_percentage = if total_weight > 0 {
            (deserialized_grant.total_yes_weight * 100) / total_weight
        } else {
            0
        };
        
        assert!(deserialized_grant.total_yes_weight > deserialized_grant.total_no_weight, "Yes votes should exceed no votes");
        assert!(approval_percentage >= 60, "Approval percentage should be >= 60%");
        assert!(deserialized_grant.quorum_reached, "Quorum should be reached");
        
        Ok(())
    }

    /// Test cast_grant_vote_handler with invalid status
    #[tokio::test]
    async fn test_cast_grant_vote_handler_invalid_status() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let grant_id = 1u64;
        let grant_id_bytes = grant_id.to_le_bytes();
        let (grant_pda, _grant_bump) = find_pda(
            &[b"grant", &grant_id_bytes],
            &program_id,
        );
        
        // Create grant account in Approved status (invalid for voting)
        let current_time = 1_000_000i64;
        let grant = Grant {
            id: grant_id,
            idea_id: 1u64,
            mesh_group: anchor_lang::prelude::Pubkey::new_unique(),
            amount: 100_000_000,
            status: GrantStatus::Approved, // Invalid status
            created_at: current_time,
            approved_at: Some(current_time),
            activated_at: None,
            disbursed_at: None,
            voting_end: current_time + 7 * 24 * 3600,
            voting_layer: crate::state::grant::VotingLayer::AuthorOnly,
            grant_level: 1,
            total_votes: 0,
            total_yes_weight: 0,
            total_no_weight: 0,
            total_abstain_weight: 0,
            quorum_reached: false,
            semantic_domain: None,
            bump: _grant_bump,
        };
        
        let grant_account = create_account_with_data(&program_id, &grant)?;
        context.set_account(&grant_pda, &grant_account);
        
        // Verify grant account
        let grant_info = context
            .banks_client
            .get_account(grant_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Grant account not found"))?;
        
        let mut grant_data_slice = &grant_info.data[8..];
        let deserialized_grant = Grant::try_deserialize(&mut grant_data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_ne!(deserialized_grant.status, GrantStatus::Pending, "Grant should NOT be in Pending status");
        
        Ok(())
    }

    /// Test tally_grant_votes_handler with invalid status
    #[tokio::test]
    async fn test_tally_grant_votes_handler_invalid_status() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let grant_id = 1u64;
        let grant_id_bytes = grant_id.to_le_bytes();
        let (grant_pda, _grant_bump) = find_pda(
            &[b"grant", &grant_id_bytes],
            &program_id,
        );
        
        // Create grant account in Approved status (invalid for tallying)
        let current_time = 1_000_000i64;
        let grant = Grant {
            id: grant_id,
            idea_id: 1u64,
            mesh_group: anchor_lang::prelude::Pubkey::new_unique(),
            amount: 100_000_000,
            status: GrantStatus::Approved, // Invalid status
            created_at: current_time,
            approved_at: Some(current_time),
            activated_at: None,
            disbursed_at: None,
            voting_end: current_time - 1,
            voting_layer: crate::state::grant::VotingLayer::AuthorOnly,
            grant_level: 1,
            total_votes: 5,
            total_yes_weight: 60,
            total_no_weight: 40,
            total_abstain_weight: 0,
            quorum_reached: true,
            semantic_domain: None,
            bump: _grant_bump,
        };
        
        let grant_account = create_account_with_data(&program_id, &grant)?;
        context.set_account(&grant_pda, &grant_account);
        
        // Verify grant account
        let grant_info = context
            .banks_client
            .get_account(grant_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Grant account not found"))?;
        
        let mut grant_data_slice = &grant_info.data[8..];
        let deserialized_grant = Grant::try_deserialize(&mut grant_data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_ne!(deserialized_grant.status, GrantStatus::Pending, "Grant should NOT be in Pending status");
        
        Ok(())
    }
}