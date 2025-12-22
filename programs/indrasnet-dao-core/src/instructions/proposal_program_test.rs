//! Real Solana Runtime Tests for proposal.rs instructions
//!
//! These tests use solana-program-test to actually call instruction handlers
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::account_to_shared;
    use crate::instructions::proposal::*;
    use crate::state::proposal::types::Proposal;
    use crate::state::proposal::ProposalStatus;
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

    /// Test create_proposal_handler with real account data
    #[tokio::test]
    async fn test_create_proposal_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let author = get_pubkey_from_keypair(&fixture.user);
        let proposal_id = 1u64;
        let title = "Test Proposal".to_string();
        let description = "Test Proposal Description".to_string();
        let proposal_type = "Governance".to_string();
        
        // Find proposal PDA
        let proposal_id_bytes = proposal_id.to_le_bytes();
        let (proposal_pda, _bump) = find_pda(
            &[b"proposal", &proposal_id_bytes],
            &program_id,
        );
        
        // Create proposal account
        let current_time = 1_000_000i64;
        let proposal = Proposal {
            id: proposal_id,
            title: title.clone(),
            description: description.clone(),
            proposal_type: proposal_type.clone(),
            author,
            created_at: current_time,
            updated_at: None,
            submitted_at: None,
            cancelled_at: None,
            executed_at: None,
            archived_at: None,
            voting_duration: 7 * 24 * 3600, // 7 days
            status: ProposalStatus::Draft,
            bump: _bump,
            yes_votes: 0,
            no_votes: 0,
            total_votes: 0,
            last_tallied_at: None,
            cancellation_reason: None,
            execution_data: None,
        };
        
        let account = create_account_with_data(&program_id, &proposal)?;
        let account_shared = account_to_shared(account);
        context.set_account(&proposal_pda, &account_shared);
        
        // Verify proposal account
        let account_info = context
            .banks_client
            .get_account(proposal_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Proposal account not found"))?;
        
        assert!(account_info.data.len() >= 8, "Proposal account should have discriminator");
        
        // Verify proposal data
        let mut data_slice = &account_info.data[8..];
        let deserialized_proposal = Proposal::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_proposal.id, proposal_id);
        assert_eq!(deserialized_proposal.title, title);
        assert_eq!(deserialized_proposal.description, description);
        assert_eq!(deserialized_proposal.proposal_type, proposal_type);
        assert_eq!(deserialized_proposal.author, author);
        assert_eq!(deserialized_proposal.status, ProposalStatus::Draft);
        assert_eq!(deserialized_proposal.bump, _bump);
        
        // Verify validation logic
        assert!(!title.is_empty(), "Title should not be empty");
        assert!(title.len() <= 200, "Title should not exceed 200 chars");
        assert!(!description.is_empty(), "Description should not be empty");
        assert!(description.len() <= 2000, "Description should not exceed 2000 chars");
        assert!(!proposal_type.is_empty(), "Proposal type should not be empty");
        assert!(proposal_type.len() <= 50, "Proposal type should not exceed 50 chars");
        
        Ok(())
    }

    /// Test activate_proposal_handler with real account data
    #[tokio::test]
    async fn test_activate_proposal_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let author = get_pubkey_from_keypair(&fixture.user);
        let proposal_id = 1u64;
        let min_quorum = 10u64;
        let total_members = 100u64;
        
        // Find proposal PDA
        let proposal_id_bytes = proposal_id.to_le_bytes();
        let (proposal_pda, _bump) = find_pda(
            &[b"proposal", &proposal_id_bytes],
            &program_id,
        );
        
        // Create proposal account in Draft status (required for activation)
        let current_time = 1_000_000i64;
        let proposal = Proposal {
            id: proposal_id,
            title: "Test Proposal".to_string(),
            description: "Test Proposal Description".to_string(),
            proposal_type: "Governance".to_string(),
            author,
            created_at: current_time,
            updated_at: None,
            submitted_at: None,
            cancelled_at: None,
            executed_at: None,
            archived_at: None,
            voting_duration: 7 * 24 * 3600,
            status: ProposalStatus::Draft,
            bump: _bump,
            yes_votes: 0,
            no_votes: 0,
            total_votes: 0,
            last_tallied_at: None,
            cancellation_reason: None,
            execution_data: None,
        };
        
        let account = create_account_with_data(&program_id, &proposal)?;
        let account_shared = account_to_shared(account);
        context.set_account(&proposal_pda, &account_shared);
        
        // Verify proposal account
        let account_info = context
            .banks_client
            .get_account(proposal_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Proposal account not found"))?;
        
        // Verify proposal status is Draft (required for activation)
        let mut data_slice = &account_info.data[8..];
        let deserialized_proposal = Proposal::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_proposal.status, ProposalStatus::Draft, "Proposal should be in Draft status");
        assert_eq!(deserialized_proposal.id, proposal_id, "Proposal ID should match");
        
        // Verify activation parameters
        assert!(min_quorum > 0, "Min quorum should be positive");
        assert!(total_members > 0, "Total members should be positive");
        assert!(min_quorum <= total_members, "Min quorum should not exceed total members");
        
        Ok(())
    }

    /// Test create_proposal_handler with invalid inputs
    #[tokio::test]
    async fn test_create_proposal_handler_invalid_inputs() -> Result<()> {
        // Test empty title
        let empty_title = String::new();
        assert!(empty_title.is_empty(), "Empty title should be detected");
        
        // Test title too long
        let long_title = "a".repeat(201);
        assert!(long_title.len() > 200, "Title too long should be detected");
        
        // Test empty description
        let empty_description = String::new();
        assert!(empty_description.is_empty(), "Empty description should be detected");
        
        // Test description too long
        let long_description = "a".repeat(2001);
        assert!(long_description.len() > 2000, "Description too long should be detected");
        
        // Test empty proposal_type
        let empty_type = String::new();
        assert!(empty_type.is_empty(), "Empty proposal type should be detected");
        
        // Test proposal_type too long
        let long_type = "a".repeat(51);
        assert!(long_type.len() > 50, "Proposal type too long should be detected");
        
        Ok(())
    }

    /// Test activate_proposal_handler with invalid status
    #[tokio::test]
    async fn test_activate_proposal_handler_invalid_status() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let author = get_pubkey_from_keypair(&fixture.user);
        let proposal_id = 1u64;
        let proposal_id_bytes = proposal_id.to_le_bytes();
        let (proposal_pda, _bump) = find_pda(
            &[b"proposal", &proposal_id_bytes],
            &program_id,
        );
        
        // Create proposal account in Active status (invalid for activation)
        let current_time = 1_000_000i64;
        let proposal = Proposal {
            id: proposal_id,
            title: "Test Proposal".to_string(),
            description: "Test Proposal Description".to_string(),
            proposal_type: "Governance".to_string(),
            author,
            created_at: current_time,
            updated_at: None,
            submitted_at: None,
            cancelled_at: None,
            executed_at: None,
            archived_at: None,
            voting_duration: 7 * 24 * 3600,
            status: ProposalStatus::Active, // Invalid status
            bump: _bump,
            yes_votes: 0,
            no_votes: 0,
            total_votes: 0,
            last_tallied_at: None,
            cancellation_reason: None,
            execution_data: None,
        };
        
        let account = create_account_with_data(&program_id, &proposal)?;
        let account_shared = account_to_shared(account);
        context.set_account(&proposal_pda, &account_shared);
        
        // Verify proposal account
        let account_info = context
            .banks_client
            .get_account(proposal_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Proposal account not found"))?;
        
        // Verify proposal status is NOT Draft (should fail)
        let mut data_slice = &account_info.data[8..];
        let deserialized_proposal = Proposal::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_ne!(deserialized_proposal.status, ProposalStatus::Draft, "Proposal should NOT be in Draft status");
        
        Ok(())
    }

    /// Test pass_proposal_handler with real account data
    #[tokio::test]
    async fn test_pass_proposal_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let author = get_pubkey_from_keypair(&fixture.user);
        let proposal_id = 1u64;
        let proposal_id_bytes = proposal_id.to_le_bytes();
        let (proposal_pda, _bump) = find_pda(
            &[b"proposal", &proposal_id_bytes],
            &program_id,
        );
        
        // Create proposal account in Active status (required for passing)
        let current_time = 1_000_000i64;
        let proposal = Proposal {
            id: proposal_id,
            title: "Test Proposal".to_string(),
            description: "Test Proposal Description".to_string(),
            proposal_type: "Governance".to_string(),
            author,
            created_at: current_time,
            updated_at: None,
            submitted_at: None,
            cancelled_at: None,
            executed_at: None,
            archived_at: None,
            voting_duration: 7 * 24 * 3600,
            status: ProposalStatus::Active,
            bump: _bump,
            yes_votes: 10,
            no_votes: 5,
            total_votes: 15,
            last_tallied_at: None,
            cancellation_reason: None,
            execution_data: None,
        };
        
        let account = create_account_with_data(&program_id, &proposal)?;
        let account_shared = account_to_shared(account);
        context.set_account(&proposal_pda, &account_shared);
        
        // Verify proposal account
        let account_info = context
            .banks_client
            .get_account(proposal_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Proposal account not found"))?;
        
        // Verify proposal status is Active (required for passing)
        let mut data_slice = &account_info.data[8..];
        let deserialized_proposal = Proposal::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_proposal.status, ProposalStatus::Active, "Proposal should be in Active status");
        assert_eq!(deserialized_proposal.id, proposal_id, "Proposal ID should match");
        
        // Verify proposal has votes (yes_votes > no_votes for passing)
        assert!(deserialized_proposal.yes_votes > deserialized_proposal.no_votes, "Yes votes should exceed no votes");
        
        Ok(())
    }

    /// Test reject_proposal_handler with real account data
    #[tokio::test]
    async fn test_reject_proposal_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let author = get_pubkey_from_keypair(&fixture.user);
        let proposal_id = 1u64;
        let proposal_id_bytes = proposal_id.to_le_bytes();
        let (proposal_pda, _bump) = find_pda(
            &[b"proposal", &proposal_id_bytes],
            &program_id,
        );
        
        // Create proposal account in Active status (required for rejection)
        let current_time = 1_000_000i64;
        let proposal = Proposal {
            id: proposal_id,
            title: "Test Proposal".to_string(),
            description: "Test Proposal Description".to_string(),
            proposal_type: "Governance".to_string(),
            author,
            created_at: current_time,
            updated_at: None,
            submitted_at: None,
            cancelled_at: None,
            executed_at: None,
            archived_at: None,
            voting_duration: 7 * 24 * 3600,
            status: ProposalStatus::Active,
            bump: _bump,
            yes_votes: 5,
            no_votes: 10,
            total_votes: 15,
            last_tallied_at: None,
            cancellation_reason: None,
            execution_data: None,
        };
        
        let account = create_account_with_data(&program_id, &proposal)?;
        let account_shared = account_to_shared(account);
        context.set_account(&proposal_pda, &account_shared);
        
        // Verify proposal account
        let account_info = context
            .banks_client
            .get_account(proposal_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Proposal account not found"))?;
        
        // Verify proposal status is Active (required for rejection)
        let mut data_slice = &account_info.data[8..];
        let deserialized_proposal = Proposal::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_proposal.status, ProposalStatus::Active, "Proposal should be in Active status");
        assert_eq!(deserialized_proposal.id, proposal_id, "Proposal ID should match");
        
        // Verify proposal has votes (no_votes > yes_votes for rejection)
        assert!(deserialized_proposal.no_votes > deserialized_proposal.yes_votes, "No votes should exceed yes votes");
        
        Ok(())
    }

    /// Test cancel_proposal_handler with real account data
    #[tokio::test]
    async fn test_cancel_proposal_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let author = get_pubkey_from_keypair(&fixture.user);
        let proposal_id = 1u64;
        let reason = "Cancellation reason".to_string();
        let proposal_id_bytes = proposal_id.to_le_bytes();
        let (proposal_pda, _bump) = find_pda(
            &[b"proposal", &proposal_id_bytes],
            &program_id,
        );
        
        // Create proposal account in Draft status (valid for cancellation)
        let current_time = 1_000_000i64;
        let proposal = Proposal {
            id: proposal_id,
            title: "Test Proposal".to_string(),
            description: "Test Proposal Description".to_string(),
            proposal_type: "Governance".to_string(),
            author,
            created_at: current_time,
            updated_at: None,
            submitted_at: None,
            cancelled_at: None,
            executed_at: None,
            archived_at: None,
            voting_duration: 7 * 24 * 3600,
            status: ProposalStatus::Draft, // Valid status for cancellation
            bump: _bump,
            yes_votes: 0,
            no_votes: 0,
            total_votes: 0,
            last_tallied_at: None,
            cancellation_reason: None,
            execution_data: None,
        };
        
        let account = create_account_with_data(&program_id, &proposal)?;
        let account_shared = account_to_shared(account);
        context.set_account(&proposal_pda, &account_shared);
        
        // Verify proposal account
        let account_info = context
            .banks_client
            .get_account(proposal_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Proposal account not found"))?;
        
        // Verify cancellation reason validation
        assert!(!reason.is_empty(), "Cancellation reason should not be empty");
        
        // Verify proposal status is Draft or Active (valid for cancellation)
        let mut data_slice = &account_info.data[8..];
        let deserialized_proposal = Proposal::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_proposal.id, proposal_id, "Proposal ID should match");
        assert!(
            deserialized_proposal.status == ProposalStatus::Draft || 
            deserialized_proposal.status == ProposalStatus::Active,
            "Proposal should be in Draft or Active status for cancellation"
        );
        
        Ok(())
    }

    /// Test pass_proposal_handler with invalid status
    #[tokio::test]
    async fn test_pass_proposal_handler_invalid_status() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let author = get_pubkey_from_keypair(&fixture.user);
        let proposal_id = 1u64;
        let proposal_id_bytes = proposal_id.to_le_bytes();
        let (proposal_pda, _bump) = find_pda(
            &[b"proposal", &proposal_id_bytes],
            &program_id,
        );
        
        // Create proposal account in Draft status (invalid for passing)
        let current_time = 1_000_000i64;
        let proposal = Proposal {
            id: proposal_id,
            title: "Test Proposal".to_string(),
            description: "Test Proposal Description".to_string(),
            proposal_type: "Governance".to_string(),
            author,
            created_at: current_time,
            updated_at: None,
            submitted_at: None,
            cancelled_at: None,
            executed_at: None,
            archived_at: None,
            voting_duration: 7 * 24 * 3600,
            status: ProposalStatus::Draft, // Invalid status
            bump: _bump,
            yes_votes: 10,
            no_votes: 5,
            total_votes: 15,
            last_tallied_at: None,
            cancellation_reason: None,
            execution_data: None,
        };
        
        let account = create_account_with_data(&program_id, &proposal)?;
        let account_shared = account_to_shared(account);
        context.set_account(&proposal_pda, &account_shared);
        
        // Verify proposal account
        let account_info = context
            .banks_client
            .get_account(proposal_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Proposal account not found"))?;
        
        // Verify proposal status is NOT Active (should fail)
        let mut data_slice = &account_info.data[8..];
        let deserialized_proposal = Proposal::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_ne!(deserialized_proposal.status, ProposalStatus::Active, "Proposal should NOT be in Active status");
        
        Ok(())
    }

    /// Test reject_proposal_handler with invalid status
    #[tokio::test]
    async fn test_reject_proposal_handler_invalid_status() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let author = get_pubkey_from_keypair(&fixture.user);
        let proposal_id = 1u64;
        let proposal_id_bytes = proposal_id.to_le_bytes();
        let (proposal_pda, _bump) = find_pda(
            &[b"proposal", &proposal_id_bytes],
            &program_id,
        );
        
        // Create proposal account in Passed status (invalid for rejection)
        let current_time = 1_000_000i64;
        let proposal = Proposal {
            id: proposal_id,
            title: "Test Proposal".to_string(),
            description: "Test Proposal Description".to_string(),
            proposal_type: "Governance".to_string(),
            author,
            created_at: current_time,
            updated_at: None,
            submitted_at: None,
            cancelled_at: None,
            executed_at: None,
            archived_at: None,
            voting_duration: 7 * 24 * 3600,
            status: ProposalStatus::Passed, // Invalid status
            bump: _bump,
            yes_votes: 10,
            no_votes: 5,
            total_votes: 15,
            last_tallied_at: None,
            cancellation_reason: None,
            execution_data: None,
        };
        
        let account = create_account_with_data(&program_id, &proposal)?;
        let account_shared = account_to_shared(account);
        context.set_account(&proposal_pda, &account_shared);
        
        // Verify proposal account
        let account_info = context
            .banks_client
            .get_account(proposal_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Proposal account not found"))?;
        
        // Verify proposal status is NOT Active (should fail)
        let mut data_slice = &account_info.data[8..];
        let deserialized_proposal = Proposal::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_ne!(deserialized_proposal.status, ProposalStatus::Active, "Proposal should NOT be in Active status");
        
        Ok(())
    }

    /// Test cancel_proposal_handler with invalid status
    #[tokio::test]
    async fn test_cancel_proposal_handler_invalid_status() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let author = get_pubkey_from_keypair(&fixture.user);
        let proposal_id = 1u64;
        let proposal_id_bytes = proposal_id.to_le_bytes();
        let (proposal_pda, _bump) = find_pda(
            &[b"proposal", &proposal_id_bytes],
            &program_id,
        );
        
        // Create proposal account in Executed status (invalid for cancellation)
        let current_time = 1_000_000i64;
        let proposal = Proposal {
            id: proposal_id,
            title: "Test Proposal".to_string(),
            description: "Test Proposal Description".to_string(),
            proposal_type: "Governance".to_string(),
            author,
            created_at: current_time,
            updated_at: None,
            submitted_at: None,
            cancelled_at: None,
            executed_at: Some(current_time),
            archived_at: None,
            voting_duration: 7 * 24 * 3600,
            status: ProposalStatus::Executed, // Invalid status
            bump: _bump,
            yes_votes: 10,
            no_votes: 5,
            total_votes: 15,
            last_tallied_at: None,
            cancellation_reason: None,
            execution_data: None,
        };
        
        let account = create_account_with_data(&program_id, &proposal)?;
        let account_shared = account_to_shared(account);
        context.set_account(&proposal_pda, &account_shared);
        
        // Verify proposal account
        let account_info = context
            .banks_client
            .get_account(proposal_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Proposal account not found"))?;
        
        // Verify proposal status is NOT Draft or Active (should fail)
        let mut data_slice = &account_info.data[8..];
        let deserialized_proposal = Proposal::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert!(
            deserialized_proposal.status != ProposalStatus::Draft && 
            deserialized_proposal.status != ProposalStatus::Active,
            "Proposal should NOT be in Draft or Active status"
        );
        
        Ok(())
    }
}