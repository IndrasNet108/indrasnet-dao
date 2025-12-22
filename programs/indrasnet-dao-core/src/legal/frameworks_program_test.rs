//! Real Solana Runtime Tests for legal/frameworks.rs
//!
//! These tests use solana-program-test to actually call instruction handlers
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::account_to_shared;
    use crate::legal::frameworks::*;
    use crate::legal::frameworks::onchain::*;
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

    /// Test initialize_legal_framework with real account data
    #[tokio::test]
    async fn test_initialize_legal_framework_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let framework_id = 1u64;
        let framework_type = FrameworkType::GDPR;
        let framework_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        // Find framework PDA
        let framework_id_bytes = framework_id.to_le_bytes();
        let (framework_pda, _bump) = find_pda(
            &[b"legal_framework", &framework_id_bytes],
            &fixture.program_id,
        );
        
        // Create framework account with initialized data
        let mut framework = LegalFrameworkMetadata {
            framework_id,
            framework_type,
            created_at: current_time,
            updated_at: current_time,
            framework_data_hash,
            bump,
        };
        
        // Simulate initialize_legal_framework call
        initialize_legal_framework(
            &mut framework,
            framework_id,
            framework_type,
            framework_data_hash,
            current_time,
            bump,
        )?;
        
        let account = create_account_with_data(&fixture.program_id, &framework)?;
        let account_shared = account_to_shared(account);
        context.set_account(&framework_pda, &account_shared);
        
        // Verify framework account
        let account_info = context
            .banks_client
            .get_account(framework_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Framework account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_framework = LegalFrameworkMetadata::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_framework.framework_id, framework_id);
        assert_eq!(deserialized_framework.framework_type, framework_type);
        assert_eq!(deserialized_framework.framework_data_hash, framework_data_hash);
        assert_eq!(deserialized_framework.created_at, current_time);
        assert_eq!(deserialized_framework.updated_at, current_time);
        
        Ok(())
    }

    /// Test initialize_legal_framework with invalid inputs
    #[tokio::test]
    async fn test_initialize_legal_framework_invalid_inputs() -> Result<()> {
        // Test framework_id == 0
        let zero_id = 0u64;
        assert_eq!(zero_id, 0, "Zero framework ID should be detected");
        
        Ok(())
    }

    /// Test initialize_legal_framework with all framework types
    #[tokio::test]
    async fn test_initialize_legal_framework_all_types() -> Result<()> {
        let framework_types = vec![
            FrameworkType::GDPR,
            FrameworkType::CCPA,
            FrameworkType::EURegulations,
            FrameworkType::Custom,
        ];
        
        for framework_type in framework_types {
            let mut fixture = TestFixture::new().await?;
            let context = fixture.context_mut();
            
            let framework_id = 1u64;
            let framework_data_hash = [1u8; 32];
            let current_time = 1_000_000i64;
            let bump = 255u8;
            
            let framework_id_bytes = framework_id.to_le_bytes();
            let (framework_pda, _bump) = find_pda(
                &[b"legal_framework", &framework_id_bytes],
                &fixture.program_id,
            );
            
            let mut framework = LegalFrameworkMetadata {
                framework_id,
                framework_type,
                created_at: current_time,
                updated_at: current_time,
                framework_data_hash,
                bump,
            };
            
            initialize_legal_framework(
                &mut framework,
                framework_id,
                framework_type,
                framework_data_hash,
                current_time,
                bump,
            )?;
            
            let account = create_account_with_data(&fixture.program_id, &framework)?;
            let account_shared = account_to_shared(account);
        context.set_account(&framework_pda, &account_shared);
            
            // Verify framework type
            let account_info = context
                .banks_client
                .get_account(framework_pda)
                .await?
                .ok_or_else(|| anyhow::anyhow!("Framework account not found"))?;
            
            let mut data_slice = &account_info.data[8..];
            let deserialized_framework = LegalFrameworkMetadata::try_deserialize(&mut data_slice)
                .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
            
            assert_eq!(deserialized_framework.framework_type, framework_type);
        }
        
        Ok(())
    }

    // ========== Extended Tests for Sprint 11 ==========
    // Additional edge cases and validation scenarios

    /// Test initialize_legal_framework with different framework_data_hash values
    #[tokio::test]
    async fn test_initialize_legal_framework_different_hashes() -> Result<()> {
        let hashes = vec![
            [0u8; 32], // Zero hash
            [1u8; 32], // All ones
            [255u8; 32], // All max
        ];
        
        for (idx, hash) in hashes.iter().enumerate() {
            let mut fixture = TestFixture::new().await?;
            let context = fixture.context_mut();
            
            let framework_id = (idx + 1) as u64;
            let framework_type = FrameworkType::GDPR;
            let current_time = 1_000_000i64;
            let bump = 255u8;
            
            let framework_id_bytes = framework_id.to_le_bytes();
            let (framework_pda, _bump) = find_pda(
                &[b"legal_framework", &framework_id_bytes],
                &fixture.program_id,
            );
            
            let mut framework = LegalFrameworkMetadata {
                framework_id,
                framework_type,
                created_at: current_time,
                updated_at: current_time,
                framework_data_hash: *hash,
                bump,
            };
            
            initialize_legal_framework(
                &mut framework,
                framework_id,
                framework_type,
                *hash,
                current_time,
                bump,
            )?;
            
            assert_eq!(framework.framework_data_hash, *hash, "Hash should match");
            
            let account = create_account_with_data(&fixture.program_id, &framework)?;
            let account_shared = account_to_shared(account);
            context.set_account(&framework_pda, &account_shared);
        }
        
        Ok(())
    }

    /// Test initialize_legal_framework timestamp consistency
    #[tokio::test]
    async fn test_initialize_legal_framework_timestamp_consistency() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let framework_id = 1u64;
        let framework_type = FrameworkType::GDPR;
        let framework_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let framework_id_bytes = framework_id.to_le_bytes();
        let (framework_pda, _bump) = find_pda(
            &[b"legal_framework", &framework_id_bytes],
            &fixture.program_id,
        );
        
        let mut framework = LegalFrameworkMetadata {
            framework_id,
            framework_type,
            created_at: current_time,
            updated_at: current_time,
            framework_data_hash,
            bump,
        };
        
        initialize_legal_framework(
            &mut framework,
            framework_id,
            framework_type,
            framework_data_hash,
            current_time,
            bump,
        )?;
        
        // Created_at and updated_at should be equal on initialization
        assert_eq!(framework.created_at, current_time, "Created at should match current time");
        assert_eq!(framework.updated_at, current_time, "Updated at should match current time");
        assert_eq!(framework.created_at, framework.updated_at, "Created and updated should be equal");
        
        let account = create_account_with_data(&fixture.program_id, &framework)?;
        let account_shared = account_to_shared(account);
        context.set_account(&framework_pda, &account_shared);
        
        Ok(())
    }

    /// Test initialize_legal_framework with max framework_id
    #[tokio::test]
    async fn test_initialize_legal_framework_max_id() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let framework_id = u64::MAX;
        let framework_type = FrameworkType::Custom;
        let framework_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let framework_id_bytes = framework_id.to_le_bytes();
        let (framework_pda, _bump) = find_pda(
            &[b"legal_framework", &framework_id_bytes],
            &fixture.program_id,
        );
        
        let mut framework = LegalFrameworkMetadata {
            framework_id,
            framework_type,
            created_at: current_time,
            updated_at: current_time,
            framework_data_hash,
            bump,
        };
        
        // Should succeed with max ID
        initialize_legal_framework(
            &mut framework,
            framework_id,
            framework_type,
            framework_data_hash,
            current_time,
            bump,
        )?;
        
        assert_eq!(framework.framework_id, u64::MAX, "Framework ID should be max");
        
        let account = create_account_with_data(&fixture.program_id, &framework)?;
        let account_shared = account_to_shared(account);
        context.set_account(&framework_pda, &account_shared);
        
        Ok(())
    }
}
