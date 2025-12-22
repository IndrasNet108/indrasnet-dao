//! Real Solana Runtime Tests for state/treasury/analytics.rs
//!
//! These tests use solana-program-test to test treasury analytics functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
#[allow(unused_imports, unused_variables, unused_mut)]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::{account_to_shared, create_account_with_data};
    use crate::state::treasury::analytics::*;
    use crate::state::treasury::analytics::onchain::*;
    use solana_sdk::{
        account::Account,
        pubkey::Pubkey as SdkPubkey,
        signature::{Signer, Keypair},
    };
    use anchor_lang::prelude::*;
    use anchor_lang::AccountDeserialize;
    use anyhow::Result;

    /// Test initialize_treasury_analytics with real account data
    #[tokio::test]
    async fn test_initialize_treasury_analytics_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let analytics_id = 1u64;
        let treasury_id = 1u64;
        let analytics_type = TreasuryAnalyticsType::Balance;
        let analytics_config_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let analytics_id_bytes = analytics_id.to_le_bytes();
        let (analytics_pda, _bump) = find_pda(
            &[b"treasury_analytics", &analytics_id_bytes],
            &program_id,
        );
        
        let mut analytics = TreasuryAnalyticsMetadata {
            analytics_id: 0,
            treasury_id: 0,
            analytics_type: TreasuryAnalyticsType::Balance,
            status: TreasuryAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        initialize_treasury_analytics(
            &mut analytics,
            analytics_id,
            treasury_id,
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
        let deserialized_analytics = TreasuryAnalyticsMetadata::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_analytics.analytics_id, analytics_id);
        assert_eq!(deserialized_analytics.treasury_id, treasury_id);
        assert_eq!(deserialized_analytics.analytics_type, analytics_type);
        assert_eq!(deserialized_analytics.status, TreasuryAnalyticsStatus::Active);
        assert_eq!(deserialized_analytics.created_at, current_time);
        
        Ok(())
    }

    /// Test initialize_treasury_analytics with all analytics types
    #[tokio::test]
    async fn test_initialize_treasury_analytics_all_types() -> Result<()> {
        let analytics_types = vec![
            TreasuryAnalyticsType::Balance,
            TreasuryAnalyticsType::Flow,
            TreasuryAnalyticsType::Performance,
            TreasuryAnalyticsType::Custom,
        ];
        
        for analytics_type in analytics_types {
            let mut fixture = TestFixture::new().await?;
            let program_id = fixture.program_id;
            let context = fixture.context_mut();
            
            let analytics_id = 1u64;
            let treasury_id = 1u64;
            let analytics_config_hash = [1u8; 32];
            let current_time = 1_000_000i64;
            let bump = 255u8;
            
            let analytics_id_bytes = analytics_id.to_le_bytes();
            let (analytics_pda, _bump) = find_pda(
                &[b"treasury_analytics", &analytics_id_bytes],
                &program_id,
            );
            
            let mut analytics = TreasuryAnalyticsMetadata {
                analytics_id: 0,
                treasury_id: 0,
                analytics_type: TreasuryAnalyticsType::Balance,
                status: TreasuryAnalyticsStatus::Disabled,
                created_at: 0,
                analytics_config_hash: [0u8; 32],
                bump: 0,
            };
            
            initialize_treasury_analytics(
                &mut analytics,
                analytics_id,
                treasury_id,
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

    /// Test initialize_treasury_analytics status always Active
    #[tokio::test]
    async fn test_initialize_treasury_analytics_status_always_active() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let analytics_id = 1u64;
        let treasury_id = 1u64;
        let analytics_type = TreasuryAnalyticsType::Balance;
        let analytics_config_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let analytics_id_bytes = analytics_id.to_le_bytes();
        let (analytics_pda, _bump) = find_pda(
            &[b"treasury_analytics", &analytics_id_bytes],
            &program_id,
        );
        
        let mut analytics = TreasuryAnalyticsMetadata {
            analytics_id: 0,
            treasury_id: 0,
            analytics_type: TreasuryAnalyticsType::Balance,
            status: TreasuryAnalyticsStatus::Disabled, // Will be reset
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        initialize_treasury_analytics(
            &mut analytics,
            analytics_id,
            treasury_id,
            analytics_type,
            analytics_config_hash,
            current_time,
            bump,
        )?;
        
        // Status should always be Active after initialization
        assert_eq!(analytics.status, TreasuryAnalyticsStatus::Active);
        
        let account = create_account_with_data(&program_id, &analytics)?;
        let account_shared = account_to_shared(account);
        context.set_account(&analytics_pda, &account_shared);
        
        Ok(())
    }
}
