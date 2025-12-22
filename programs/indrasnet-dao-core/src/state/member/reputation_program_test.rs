//! Real Solana Runtime Tests for state/member/reputation.rs
//!
//! These tests use solana-program-test to test member reputation functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
#[allow(unused_imports, unused_variables, unused_mut)]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::{account_to_shared, create_account_with_data};
    use crate::state::member::reputation::*;
    use crate::state::member::reputation::onchain::*;
    use solana_sdk::{
        account::Account,
        pubkey::Pubkey as SdkPubkey,
        signature::{Signer, Keypair},
    };
    use anchor_lang::prelude::*;
    use anchor_lang::AccountDeserialize;
    use anyhow::Result;

    /// Test initialize_member_reputation with real account data
    #[tokio::test]
    async fn test_initialize_member_reputation_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let reputation_id = 1u64;
        let member_id = 1u64;
        let reputation_factor = MemberReputationFactor::Contribution;
        let reputation_config_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let reputation_id_bytes = reputation_id.to_le_bytes();
        let (reputation_pda, _bump) = find_pda(
            &[b"member_reputation", &reputation_id_bytes],
            &program_id,
        );
        
        let mut reputation = MemberReputationMetadata {
            reputation_id: 0,
            member_id: 0,
            reputation_factor: MemberReputationFactor::Contribution,
            status: MemberReputationStatus::Disabled,
            created_at: 0,
            reputation_config_hash: [0u8; 32],
            bump: 0,
        };
        
        initialize_member_reputation(
            &mut reputation,
            reputation_id,
            member_id,
            reputation_factor,
            reputation_config_hash,
            current_time,
            bump,
        )?;
        
        let account = create_account_with_data(&program_id, &reputation)?;
        let account_shared = account_to_shared(account);
        context.set_account(&reputation_pda, &account_shared);
        
        // Verify reputation account
        let account_info = context
            .banks_client
            .get_account(reputation_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Reputation account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_reputation = MemberReputationMetadata::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_reputation.reputation_id, reputation_id);
        assert_eq!(deserialized_reputation.member_id, member_id);
        assert_eq!(deserialized_reputation.reputation_factor, reputation_factor);
        assert_eq!(deserialized_reputation.status, MemberReputationStatus::Active);
        assert_eq!(deserialized_reputation.created_at, current_time);
        
        Ok(())
    }

    /// Test initialize_member_reputation with all reputation factors
    #[tokio::test]
    async fn test_initialize_member_reputation_all_factors() -> Result<()> {
        let reputation_factors = vec![
            MemberReputationFactor::Contribution,
            MemberReputationFactor::Quality,
            MemberReputationFactor::Engagement,
            MemberReputationFactor::Custom,
        ];
        
        for reputation_factor in reputation_factors {
            let mut fixture = TestFixture::new().await?;
            let program_id = fixture.program_id; // Get program_id before mutable borrow
            let context = fixture.context_mut();
            
            let reputation_id = 1u64;
            let member_id = 1u64;
            let reputation_config_hash = [1u8; 32];
            let current_time = 1_000_000i64;
            let bump = 255u8;
            
            let reputation_id_bytes = reputation_id.to_le_bytes();
            let (reputation_pda, _bump) = find_pda(
                &[b"member_reputation", &reputation_id_bytes],
                &program_id,
            );
            
            let mut reputation = MemberReputationMetadata {
                reputation_id: 0,
                member_id: 0,
                reputation_factor: MemberReputationFactor::Contribution,
                status: MemberReputationStatus::Disabled,
                created_at: 0,
                reputation_config_hash: [0u8; 32],
                bump: 0,
            };
            
            initialize_member_reputation(
                &mut reputation,
                reputation_id,
                member_id,
                reputation_factor,
                reputation_config_hash,
                current_time,
                bump,
            )?;
            
            assert_eq!(reputation.reputation_factor, reputation_factor);
            
            let account = create_account_with_data(&program_id, &reputation)?;
            let account_shared = account_to_shared(account);
            context.set_account(&reputation_pda, &account_shared);
        }
        
        Ok(())
    }

    /// Test initialize_member_reputation status always Active
    #[tokio::test]
    async fn test_initialize_member_reputation_status_always_active() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let reputation_id = 1u64;
        let member_id = 1u64;
        let reputation_factor = MemberReputationFactor::Contribution;
        let reputation_config_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let reputation_id_bytes = reputation_id.to_le_bytes();
        let (reputation_pda, _bump) = find_pda(
            &[b"member_reputation", &reputation_id_bytes],
            &program_id,
        );
        
        let mut reputation = MemberReputationMetadata {
            reputation_id: 0,
            member_id: 0,
            reputation_factor: MemberReputationFactor::Contribution,
            status: MemberReputationStatus::Disabled, // Will be reset
            created_at: 0,
            reputation_config_hash: [0u8; 32],
            bump: 0,
        };
        
        initialize_member_reputation(
            &mut reputation,
            reputation_id,
            member_id,
            reputation_factor,
            reputation_config_hash,
            current_time,
            bump,
        )?;
        
        // Status should always be Active after initialization
        assert_eq!(reputation.status, MemberReputationStatus::Active);
        
        let account = create_account_with_data(&program_id, &reputation)?;
        let account_shared = account_to_shared(account);
        context.set_account(&reputation_pda, &account_shared);
        
        Ok(())
    }
}
