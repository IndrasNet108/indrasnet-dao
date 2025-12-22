//! Real Solana Runtime Tests for state/member/contribution.rs
//!
//! These tests use solana-program-test to test member contribution functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
#[allow(unused_imports, unused_variables, unused_mut)]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::{account_to_shared, create_account_with_data};
    use crate::state::member::contribution::*;
    use crate::state::member::contribution::onchain::*;
    use solana_sdk::{
        account::Account,
        pubkey::Pubkey as SdkPubkey,
        signature::{Signer, Keypair},
    };
    use anchor_lang::prelude::*;
    use anchor_lang::AccountDeserialize;
    use anyhow::Result;

    /// Test initialize_member_contribution with real account data
    #[tokio::test]
    async fn test_initialize_member_contribution_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let contribution_id = 1u64;
        let member_id = 1u64;
        let contribution_type = MemberContributionType::Code;
        let contribution_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let contribution_id_bytes = contribution_id.to_le_bytes();
        let (contribution_pda, _bump) = find_pda(
            &[b"member_contribution", &contribution_id_bytes],
            &program_id,
        );
        
        let mut contribution = MemberContributionMetadata {
            contribution_id: 0,
            member_id: 0,
            contribution_type: MemberContributionType::Code,
            status: MemberContributionStatus::Accepted,
            created_at: 0,
            contribution_data_hash: [0u8; 32],
            bump: 0,
        };
        
        initialize_member_contribution(
            &mut contribution,
            contribution_id,
            member_id,
            contribution_type,
            contribution_data_hash,
            current_time,
            bump,
        )?;
        
        let account = create_account_with_data(&program_id, &contribution)?;
        let account_shared = account_to_shared(account);
        context.set_account(&contribution_pda, &account_shared);
        
        // Verify contribution account
        let account_info = context
            .banks_client
            .get_account(contribution_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Contribution account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_contribution = MemberContributionMetadata::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_contribution.contribution_id, contribution_id);
        assert_eq!(deserialized_contribution.member_id, member_id);
        assert_eq!(deserialized_contribution.contribution_type, contribution_type);
        assert_eq!(deserialized_contribution.status, MemberContributionStatus::Pending);
        assert_eq!(deserialized_contribution.created_at, current_time);
        
        Ok(())
    }

    /// Test initialize_member_contribution with all contribution types
    #[tokio::test]
    async fn test_initialize_member_contribution_all_types() -> Result<()> {
        let contribution_types = vec![
            MemberContributionType::Code,
            MemberContributionType::Documentation,
            MemberContributionType::Design,
            MemberContributionType::Custom,
        ];
        
        for contribution_type in contribution_types {
            let mut fixture = TestFixture::new().await?;
            let program_id = fixture.program_id; // Get program_id before mutable borrow
            let context = fixture.context_mut();
            
            let contribution_id = 1u64;
            let member_id = 1u64;
            let contribution_data_hash = [1u8; 32];
            let current_time = 1_000_000i64;
            let bump = 255u8;
            
            let contribution_id_bytes = contribution_id.to_le_bytes();
            let (contribution_pda, _bump) = find_pda(
                &[b"member_contribution", &contribution_id_bytes],
                &program_id,
            );
            
            let mut contribution = MemberContributionMetadata {
                contribution_id: 0,
                member_id: 0,
                contribution_type: MemberContributionType::Code,
                status: MemberContributionStatus::Accepted,
                created_at: 0,
                contribution_data_hash: [0u8; 32],
                bump: 0,
            };
            
            initialize_member_contribution(
                &mut contribution,
                contribution_id,
                member_id,
                contribution_type,
                contribution_data_hash,
                current_time,
                bump,
            )?;
            
            assert_eq!(contribution.contribution_type, contribution_type);
            
            let account = create_account_with_data(&program_id, &contribution)?;
            let account_shared = account_to_shared(account);
            context.set_account(&contribution_pda, &account_shared);
        }
        
        Ok(())
    }

    /// Test initialize_member_contribution status always Pending
    #[tokio::test]
    async fn test_initialize_member_contribution_status_always_pending() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let contribution_id = 1u64;
        let member_id = 1u64;
        let contribution_type = MemberContributionType::Code;
        let contribution_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let contribution_id_bytes = contribution_id.to_le_bytes();
        let (contribution_pda, _bump) = find_pda(
            &[b"member_contribution", &contribution_id_bytes],
            &program_id,
        );
        
        let mut contribution = MemberContributionMetadata {
            contribution_id: 0,
            member_id: 0,
            contribution_type: MemberContributionType::Code,
            status: MemberContributionStatus::Accepted, // Will be reset
            created_at: 0,
            contribution_data_hash: [0u8; 32],
            bump: 0,
        };
        
        initialize_member_contribution(
            &mut contribution,
            contribution_id,
            member_id,
            contribution_type,
            contribution_data_hash,
            current_time,
            bump,
        )?;
        
        // Status should always be Pending after initialization
        assert_eq!(contribution.status, MemberContributionStatus::Pending);
        
        let account = create_account_with_data(&program_id, &contribution)?;
        let account_shared = account_to_shared(account);
        context.set_account(&contribution_pda, &account_shared);
        
        Ok(())
    }
}
