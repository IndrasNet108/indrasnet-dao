//! Real Solana Runtime Tests for state/member/actions.rs
//!
//! These tests use solana-program-test to test member action functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
#[allow(unused_imports, unused_variables, unused_mut)]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::{account_to_shared, get_pubkey_from_keypair, create_account_with_data};
    use crate::state::member::types::Member;
    use crate::state::enums::MemberStatus;
    use solana_sdk::{
        account::Account,
        pubkey::Pubkey as SdkPubkey,
        signature::{Signer, Keypair},
    };
    use anchor_lang::prelude::*;
    use anchor_lang::AccountDeserialize;
    use anyhow::Result;

    /// Helper to create test member
    fn create_test_member(pubkey: anchor_lang::prelude::Pubkey, created_by: anchor_lang::prelude::Pubkey) -> Member {
        Member {
            pubkey,
            status: MemberStatus::Active,
            reputation: 100,
            joined_at: 1_000_000i64,
            last_activity: 1_000_000i64,
            contributions_count: 0,
            votes_cast: 0,
            ideas_created: 0,
            proposals_created: 0,
            suspension_reason: None,
            suspension_until: None,
            created_by,
            bump: 255,
        }
    }

    /// Test add_contribution_with_time with real account data
    #[tokio::test]
    async fn test_add_contribution_with_time_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let member_pubkey = get_pubkey_from_keypair(&fixture.user);
        let created_by = get_pubkey_from_keypair(&fixture.authority);
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        let initial_reputation = 100u64;
        let initial_contributions = 0u32;
        let current_time = 2_000_000i64;
        
        let (member_pda, _bump) = find_pda(
            &[b"member", member_pubkey.as_ref()],
            &program_id, // Use local program_id
        );
        
        let mut member = create_test_member(member_pubkey, created_by);
        member.reputation = initial_reputation;
        member.contributions_count = initial_contributions;
        
        // Add contribution
        member.add_contribution_with_time(current_time)?;
        
        let account = create_account_with_data(&program_id, &member)?; // Use local program_id
        let account_shared = account_to_shared(account);
        context.set_account(&member_pda, &account_shared);
        
        // Verify contribution added
        let account_info = context
            .banks_client
            .get_account(member_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Member account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_member = Member::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_member.contributions_count, initial_contributions + 1);
        assert_eq!(deserialized_member.reputation, initial_reputation + 10); // +10 reputation
        assert_eq!(deserialized_member.last_activity, current_time);
        
        Ok(())
    }

    /// Test cast_vote_with_time with real account data
    #[tokio::test]
    async fn test_cast_vote_with_time_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let member_pubkey = get_pubkey_from_keypair(&fixture.user);
        let created_by = get_pubkey_from_keypair(&fixture.authority);
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        let initial_votes = 0u32;
        let current_time = 2_000_000i64;
        
        let (member_pda, _bump) = find_pda(
            &[b"member", member_pubkey.as_ref()],
            &program_id, // Use local program_id
        );
        
        let mut member = create_test_member(member_pubkey, created_by);
        member.status = MemberStatus::Active;
        member.votes_cast = initial_votes;
        
        // Cast vote
        member.cast_vote_with_time(current_time)?;
        
        let account = create_account_with_data(&program_id, &member)?; // Use local program_id
        let account_shared = account_to_shared(account);
        context.set_account(&member_pda, &account_shared);
        
        // Verify vote cast
        let account_info = context
            .banks_client
            .get_account(member_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Member account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_member = Member::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_member.votes_cast, initial_votes + 1);
        assert_eq!(deserialized_member.last_activity, current_time);
        
        Ok(())
    }

    /// Test create_idea_with_time with real account data
    #[tokio::test]
    async fn test_create_idea_with_time_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let member_pubkey = get_pubkey_from_keypair(&fixture.user);
        let created_by = get_pubkey_from_keypair(&fixture.authority);
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        let initial_reputation = 100u64;
        let initial_ideas = 0u32;
        let current_time = 2_000_000i64;
        
        let (member_pda, _bump) = find_pda(
            &[b"member", member_pubkey.as_ref()],
            &program_id, // Use local program_id
        );
        
        let mut member = create_test_member(member_pubkey, created_by);
        member.status = MemberStatus::Active;
        member.reputation = initial_reputation;
        member.ideas_created = initial_ideas;
        
        // Create idea
        member.create_idea_with_time(current_time)?;
        
        let account = create_account_with_data(&program_id, &member)?; // Use local program_id
        let account_shared = account_to_shared(account);
        context.set_account(&member_pda, &account_shared);
        
        // Verify idea created
        let account_info = context
            .banks_client
            .get_account(member_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Member account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_member = Member::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_member.ideas_created, initial_ideas + 1);
        assert_eq!(deserialized_member.reputation, initial_reputation + 5); // +5 reputation
        assert_eq!(deserialized_member.last_activity, current_time);
        
        Ok(())
    }

    /// Test create_proposal_with_time with real account data
    #[tokio::test]
    async fn test_create_proposal_with_time_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let member_pubkey = get_pubkey_from_keypair(&fixture.user);
        let created_by = get_pubkey_from_keypair(&fixture.authority);
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        let initial_reputation = 100u64;
        let initial_proposals = 0u32;
        let current_time = 2_000_000i64;
        
        let (member_pda, _bump) = find_pda(
            &[b"member", member_pubkey.as_ref()],
            &program_id, // Use local program_id
        );
        
        let mut member = create_test_member(member_pubkey, created_by);
        member.status = MemberStatus::Active;
        member.reputation = initial_reputation;
        member.proposals_created = initial_proposals;
        
        // Create proposal
        member.create_proposal_with_time(current_time)?;
        
        let account = create_account_with_data(&program_id, &member)?; // Use local program_id
        let account_shared = account_to_shared(account);
        context.set_account(&member_pda, &account_shared);
        
        // Verify proposal created
        let account_info = context
            .banks_client
            .get_account(member_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Member account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_member = Member::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_member.proposals_created, initial_proposals + 1);
        assert_eq!(deserialized_member.reputation, initial_reputation + 15); // +15 reputation
        assert_eq!(deserialized_member.last_activity, current_time);
        
        Ok(())
    }

    /// Test multiple actions
    #[tokio::test]
    async fn test_member_multiple_actions() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let member_pubkey = get_pubkey_from_keypair(&fixture.user);
        let created_by = get_pubkey_from_keypair(&fixture.authority);
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        let initial_reputation = 100u64;
        let mut current_time = 2_000_000i64;
        
        let (member_pda, _bump) = find_pda(
            &[b"member", member_pubkey.as_ref()],
            &program_id, // Use local program_id
        );
        
        let mut member = create_test_member(member_pubkey, created_by);
        member.status = MemberStatus::Active;
        member.reputation = initial_reputation;
        
        // Add contribution (using impl Member from actions.rs)
        member.add_contribution_with_time(current_time)?;
        current_time += 1;
        
        // Cast vote (using impl Member from actions.rs)
        member.cast_vote_with_time(current_time)?;
        current_time += 1;
        
        // Create idea (using impl Member from actions.rs)
        member.create_idea_with_time(current_time)?;
        current_time += 1;
        
        // Create proposal (using impl Member from actions.rs)
        member.create_proposal_with_time(current_time)?;
        
        let account = create_account_with_data(&program_id, &member)?; // Use local program_id
        let account_shared = account_to_shared(account);
        context.set_account(&member_pda, &account_shared);
        
        // Verify all actions
        let account_info = context
            .banks_client
            .get_account(member_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Member account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_member = Member::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_member.contributions_count, 1);
        assert_eq!(deserialized_member.votes_cast, 1);
        assert_eq!(deserialized_member.ideas_created, 1);
        assert_eq!(deserialized_member.proposals_created, 1);
        assert_eq!(deserialized_member.reputation, initial_reputation + 10 + 5 + 15); // +30 total
        assert_eq!(deserialized_member.last_activity, current_time);
        
        Ok(())
    }
}
