//! Real Solana Runtime Tests for state/member/analytics.rs
//!
//! These tests use solana-program-test to test member analytics functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
#[allow(unused_imports, unused_variables, unused_mut)]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::{account_to_shared, create_account_with_data};
    use crate::state::member::analytics::*;
    use crate::state::member::analytics::onchain::*;
    use solana_sdk::{
        account::Account,
        pubkey::Pubkey as SdkPubkey,
        signature::{Signer, Keypair},
    };
    use anchor_lang::prelude::*;
    use anchor_lang::AccountDeserialize;
    use anyhow::Result;

    /// Test initialize_member_analytics with real account data
    #[tokio::test]
    async fn test_initialize_member_analytics_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let analytics_id = 1u64;
        let member_id = 1u64;
        let analytics_type = MemberAnalyticsType::Activity;
        let analytics_config_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let analytics_id_bytes = analytics_id.to_le_bytes();
        let (analytics_pda, _bump) = find_pda(
            &[b"member_analytics", &analytics_id_bytes],
            &program_id,
        );
        
        let mut analytics = MemberAnalyticsMetadata {
            analytics_id: 0,
            member_id: 0,
            analytics_type: MemberAnalyticsType::Activity,
            status: MemberAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        initialize_member_analytics(
            &mut analytics,
            analytics_id,
            member_id,
            analytics_type,
            analytics_config_hash,
            current_time,
            bump,
        )?;
        
        let account = create_account_with_data(&program_id, &analytics)?;
        let account_shared = account_to_shared(account);
        context.set_account(&analytics_pda, &account_shared);
        
        // Verify analytics account
        let account_info = context
            .banks_client
            .get_account(analytics_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Analytics account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_analytics = MemberAnalyticsMetadata::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_analytics.analytics_id, analytics_id);
        assert_eq!(deserialized_analytics.member_id, member_id);
        assert_eq!(deserialized_analytics.analytics_type, analytics_type);
        assert_eq!(deserialized_analytics.status, MemberAnalyticsStatus::Active);
        assert_eq!(deserialized_analytics.created_at, current_time);
        
        Ok(())
    }

    /// Test initialize_member_analytics with all analytics types
    #[tokio::test]
    async fn test_initialize_member_analytics_all_types() -> Result<()> {
        let analytics_types = vec![
            MemberAnalyticsType::Activity,
            MemberAnalyticsType::Contribution,
            MemberAnalyticsType::Engagement,
            MemberAnalyticsType::Custom,
        ];
        
        for analytics_type in analytics_types {
            let mut fixture = TestFixture::new().await?;
            let program_id = fixture.program_id; // Get program_id before mutable borrow
            let context = fixture.context_mut();
            
            let analytics_id = 1u64;
            let member_id = 1u64;
            let analytics_config_hash = [1u8; 32];
            let current_time = 1_000_000i64;
            let bump = 255u8;
            
            let analytics_id_bytes = analytics_id.to_le_bytes();
            let (analytics_pda, _bump) = find_pda(
                &[b"member_analytics", &analytics_id_bytes],
                &program_id,
            );
            
            let mut analytics = MemberAnalyticsMetadata {
                analytics_id: 0,
                member_id: 0,
                analytics_type: MemberAnalyticsType::Activity,
                status: MemberAnalyticsStatus::Disabled,
                created_at: 0,
                analytics_config_hash: [0u8; 32],
                bump: 0,
            };
            
            initialize_member_analytics(
                &mut analytics,
                analytics_id,
                member_id,
                analytics_type,
                analytics_config_hash,
                current_time,
                bump,
            )?;
            
            assert_eq!(analytics.analytics_type, analytics_type);
            
            let account = create_account_with_data(&program_id, &analytics)?;
            let account_shared = account_to_shared(account);
            context.set_account(&analytics_pda, &account_shared);
        }
        
        Ok(())
    }

    /// Test initialize_member_analytics status always Active
    #[tokio::test]
    async fn test_initialize_member_analytics_status_always_active() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let analytics_id = 1u64;
        let member_id = 1u64;
        let analytics_type = MemberAnalyticsType::Activity;
        let analytics_config_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let analytics_id_bytes = analytics_id.to_le_bytes();
        let (analytics_pda, _bump) = find_pda(
            &[b"member_analytics", &analytics_id_bytes],
            &program_id,
        );
        
        let mut analytics = MemberAnalyticsMetadata {
            analytics_id: 0,
            member_id: 0,
            analytics_type: MemberAnalyticsType::Activity,
            status: MemberAnalyticsStatus::Disabled, // Will be reset
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        initialize_member_analytics(
            &mut analytics,
            analytics_id,
            member_id,
            analytics_type,
            analytics_config_hash,
            current_time,
            bump,
        )?;
        
        // Status should always be Active after initialization
        assert_eq!(analytics.status, MemberAnalyticsStatus::Active);
        
        let account = create_account_with_data(&program_id, &analytics)?;
        let account_shared = account_to_shared(account);
        context.set_account(&analytics_pda, &account_shared);
        
        Ok(())
    }
}
