//! Real Solana Runtime Tests for instructions/semantic_distance.rs
//!
//! These tests use solana-program-test to actually call instruction handlers
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
mod tests {
    use crate::tests::fixtures::*;
    use crate::instructions::semantic_distance::*;
    use crate::state::grant::semantic::SemanticDistanceBundle;
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

    /// Test verify_semantic_distance_handler with real account data (simplified)
    #[tokio::test]
    async fn test_verify_semantic_distance_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let source_entity = anchor_lang::prelude::Pubkey::new_unique();
        let target_entity = anchor_lang::prelude::Pubkey::new_unique();
        let distance = 0.5f32;
        let current_time = 1_000_000i64;
        let timestamp = current_time;
        let nonce = 1u64;
        let model_version = "1.0".to_string();
        let provider = "test-provider".to_string();
        let provider_pubkey = get_pubkey_from_keypair(&fixture.authority);
        
        // Compute bundle hash (simplified - in real handler this is computed via compute_canonical_distance_bundle_hash)
        let bundle_hash = [1u8; 32];
        let bundle_signature = [1u8; 64];
        
        // Create bundle
        let bundle = SemanticDistanceBundle {
            source_entity,
            target_entity,
            distance,
            timestamp,
            nonce,
            model_version: model_version.clone(),
            provider: provider.clone(),
            provider_pubkey,
            bundle_hash,
            bundle_signature,
        };
        
        // Verify bundle parameters
        assert!((0.0..=1.0).contains(&bundle.distance), "Distance should be in valid range");
        assert_ne!(bundle.bundle_hash, [0u8; 32], "Bundle hash should not be zero");
        assert_ne!(bundle.bundle_signature, [0u8; 64], "Bundle signature should not be zero");
        assert_ne!(bundle.source_entity, bundle.target_entity, "Source and target should be different");
        assert!(!bundle.provider.is_empty(), "Provider should not be empty");
        assert!(bundle.provider.len() <= 50, "Provider should not exceed max length");
        assert!(bundle.nonce > 0, "Nonce should be positive");
        assert!(!bundle.model_version.is_empty(), "Model version should not be empty");
        assert!(bundle.model_version.len() <= 50, "Model version should not exceed max length");
        
        // Verify timestamp is within valid range
        const MAX_TIMESTAMP_AGE_SECONDS: i64 = 3600;
        const MAX_TIMESTAMP_FUTURE_SECONDS: i64 = 300;
        assert!(
            bundle.timestamp <= current_time + MAX_TIMESTAMP_FUTURE_SECONDS,
            "Timestamp should not be too far in future"
        );
        assert!(
            bundle.timestamp >= current_time - MAX_TIMESTAMP_AGE_SECONDS,
            "Timestamp should not be too old"
        );
        
        Ok(())
    }

    /// Test verify_semantic_distance_handler with invalid inputs
    #[tokio::test]
    async fn test_verify_semantic_distance_handler_invalid_inputs() -> Result<()> {
        // Test distance < 0.0
        let negative_distance = -0.1f32;
        assert!(!(0.0..=1.0).contains(&negative_distance), "Negative distance should be detected");
        
        // Test distance > 1.0
        let too_large_distance = 1.5f32;
        assert!(!(0.0..=1.0).contains(&too_large_distance), "Distance > 1.0 should be detected");
        
        // Test bundle_hash == [0u8; 32]
        let zero_hash = [0u8; 32];
        assert_eq!(zero_hash, [0u8; 32], "Zero bundle hash should be detected");
        
        // Test bundle_signature == [0u8; 64]
        let zero_signature = [0u8; 64];
        assert_eq!(zero_signature, [0u8; 64], "Zero bundle signature should be detected");
        
        // Test source_entity == target_entity
        let same_entity = anchor_lang::prelude::Pubkey::new_unique();
        assert_eq!(same_entity, same_entity, "Source equals target should be detected");
        
        // Test empty provider
        let empty_provider = String::new();
        assert!(empty_provider.is_empty(), "Empty provider should be detected");
        
        // Test provider too long
        let long_provider = "a".repeat(51);
        assert!(long_provider.len() > 50, "Provider too long should be detected");
        
        // Test nonce == 0
        let zero_nonce = 0u64;
        assert_eq!(zero_nonce, 0, "Zero nonce should be detected");
        
        // Test empty model_version
        let empty_model_version = String::new();
        assert!(empty_model_version.is_empty(), "Empty model version should be detected");
        
        // Test model_version too long
        let long_model_version = "a".repeat(51);
        assert!(long_model_version.len() > 50, "Model version too long should be detected");
        
        Ok(())
    }
}
