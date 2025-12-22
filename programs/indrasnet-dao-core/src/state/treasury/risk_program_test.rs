//! Real Solana Runtime Tests for state/treasury/risk.rs
//!
//! These tests use solana-program-test to test treasury risk management functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
#[allow(unused_imports, unused_variables, unused_mut)]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::{account_to_shared, create_account_with_data};
    use crate::state::treasury::risk::*;
    use crate::state::treasury::risk::onchain::*;
    use solana_sdk::{
        account::Account,
        pubkey::Pubkey as SdkPubkey,
        signature::{Signer, Keypair},
    };
    use anchor_lang::prelude::*;
    use anchor_lang::AccountDeserialize;
    use anyhow::Result;

    /// Test initialize_treasury_risk_management with real account data
    #[tokio::test]
    async fn test_initialize_treasury_risk_management_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let risk_id = 1u64;
        let treasury_id = 1u64;
        let risk_type = TreasuryRiskType::Market;
        let risk_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let risk_id_bytes = risk_id.to_le_bytes();
        let (risk_pda, _bump) = find_pda(
            &[b"treasury_risk", &risk_id_bytes],
            &program_id,
        );
        
        let mut risk = TreasuryRiskManagementMetadata {
            risk_id: 0,
            treasury_id: 0,
            risk_type: TreasuryRiskType::Market,
            status: TreasuryRiskStatus::Active,
            created_at: 0,
            risk_data_hash: [0u8; 32],
            bump: 0,
        };
        
        initialize_treasury_risk_management(
            &mut risk,
            risk_id,
            treasury_id,
            risk_type,
            risk_data_hash,
            current_time,
            bump,
        )?;
        
        let account = create_account_with_data(&program_id, &risk)?;
        let account_shared = account_to_shared(account);
        context.set_account(&risk_pda, &account_shared);
        
        // Verify risk account
        let account_info = context
            .banks_client
            .get_account(risk_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Risk account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_risk = TreasuryRiskManagementMetadata::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_risk.risk_id, risk_id);
        assert_eq!(deserialized_risk.treasury_id, treasury_id);
        assert_eq!(deserialized_risk.risk_type, risk_type);
        assert_eq!(deserialized_risk.status, TreasuryRiskStatus::Assessed);
        assert_eq!(deserialized_risk.created_at, current_time);
        
        Ok(())
    }

    /// Test initialize_treasury_risk_management with all risk types
    #[tokio::test]
    async fn test_initialize_treasury_risk_management_all_types() -> Result<()> {
        let risk_types = vec![
            TreasuryRiskType::Market,
            TreasuryRiskType::Credit,
            TreasuryRiskType::Liquidity,
            TreasuryRiskType::Custom,
        ];
        
        for risk_type in risk_types {
            let mut fixture = TestFixture::new().await?;
            let program_id = fixture.program_id;
            let context = fixture.context_mut();
            
            let risk_id = 1u64;
            let treasury_id = 1u64;
            let risk_data_hash = [1u8; 32];
            let current_time = 1_000_000i64;
            let bump = 255u8;
            
            let risk_id_bytes = risk_id.to_le_bytes();
            let (risk_pda, _bump) = find_pda(
                &[b"treasury_risk", &risk_id_bytes],
                &program_id,
            );
            
            let mut risk = TreasuryRiskManagementMetadata {
                risk_id: 0,
                treasury_id: 0,
                risk_type: TreasuryRiskType::Market,
                status: TreasuryRiskStatus::Active,
                created_at: 0,
                risk_data_hash: [0u8; 32],
                bump: 0,
            };
            
            initialize_treasury_risk_management(
                &mut risk,
                risk_id,
                treasury_id,
                risk_type,
                risk_data_hash,
                current_time,
                bump,
            )?;
            
            assert_eq!(risk.risk_type, risk_type);
            
            let account = create_account_with_data(&program_id, &risk)?;
            let account_shared = account_to_shared(account);
            context.set_account(&risk_pda, &account_shared);
        }
        
        Ok(())
    }

    /// Test initialize_treasury_risk_management status always Assessed
    #[tokio::test]
    async fn test_initialize_treasury_risk_management_status_always_assessed() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let risk_id = 1u64;
        let treasury_id = 1u64;
        let risk_type = TreasuryRiskType::Market;
        let risk_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let risk_id_bytes = risk_id.to_le_bytes();
        let (risk_pda, _bump) = find_pda(
            &[b"treasury_risk", &risk_id_bytes],
            &program_id,
        );
        
        let mut risk = TreasuryRiskManagementMetadata {
            risk_id: 0,
            treasury_id: 0,
            risk_type: TreasuryRiskType::Market,
            status: TreasuryRiskStatus::Active, // Will be reset
            created_at: 0,
            risk_data_hash: [0u8; 32],
            bump: 0,
        };
        
        initialize_treasury_risk_management(
            &mut risk,
            risk_id,
            treasury_id,
            risk_type,
            risk_data_hash,
            current_time,
            bump,
        )?;
        
        // Status should always be Assessed after initialization
        assert_eq!(risk.status, TreasuryRiskStatus::Assessed);
        
        let account = create_account_with_data(&program_id, &risk)?;
        let account_shared = account_to_shared(account);
        context.set_account(&risk_pda, &account_shared);
        
        Ok(())
    }
}
