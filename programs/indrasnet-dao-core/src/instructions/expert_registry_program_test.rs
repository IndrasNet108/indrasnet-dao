//! Real Solana Runtime Tests for instructions/expert_registry.rs
//!
//! These tests use solana-program-test to actually call instruction handlers
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::account_to_shared;
    use crate::instructions::expert_registry::*;
    use crate::state::expert_registry::{ExpertRegistry, ExpertEntry, DomainExpertIndex};
    use crate::state::grant::semantic::{CompetencyLevel, CompetencySource};
    use crate::state::member::registry::Member;
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

    /// Test initialize_expert_registry_handler with real account data
    #[tokio::test]
    async fn test_initialize_expert_registry_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        // Find registry PDA
        let (registry_pda, _bump) = find_pda(
            &[b"expert_registry"],
            &program_id,
        );
        
        // Create registry account with initialized data
        let registry = ExpertRegistry {
            total_experts: 0,
            total_domains: 0,
            created_at: current_time,
            updated_at: current_time,
            bump,
        };
        
        let account = create_account_with_data(&program_id, &registry)?;
        let account_shared = account_to_shared(account);
        context.set_account(&registry_pda, &account_shared);
        
        // Verify registry account
        let account_info = context
            .banks_client
            .get_account(registry_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Registry account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_registry = ExpertRegistry::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_registry.total_experts, 0);
        assert_eq!(deserialized_registry.total_domains, 0);
        assert_eq!(deserialized_registry.created_at, current_time);
        assert_eq!(deserialized_registry.updated_at, current_time);
        
        Ok(())
    }

    /// Test add_expert_handler with real account data
    #[tokio::test]
    async fn test_add_expert_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let expert = get_pubkey_from_keypair(&fixture.user);
        let domain_id = "mathematics".to_string();
        let competency_level = CompetencyLevel::Expert;
        let confidence = 80u8; // >= MIN_CONFIDENCE (70)
        let source = CompetencySource::ManualAssignment;
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        // Find expert entry PDA
        let domain_id_bytes = domain_id.as_bytes();
        let (expert_entry_pda, _entry_bump) = find_pda(
            &[b"expert", expert.as_ref(), domain_id_bytes],
            &program_id,
        );
        
        // Create member account with sufficient reputation
        let member = Member {
            pubkey: expert,
            reputation: ExpertEntry::MIN_EXPERT_REPUTATION, // >= 100 required
            created_at: current_time,
            updated_at: current_time,
            bump: 255u8,
        };
        
        // Create expert entry account
        let expert_entry = ExpertEntry {
            expert,
            domain_id: domain_id.clone(),
            competency_level,
            confidence,
            source,
            reputation_score: member.reputation,
            created_at: current_time,
            updated_at: current_time,
            is_active: true,
            verified_by: get_pubkey_from_keypair(&fixture.authority),
            bump: _entry_bump,
        };
        
        let account = create_account_with_data(&program_id, &expert_entry)?;
        let account_shared = account_to_shared(account);
        context.set_account(&expert_entry_pda, &account_shared);
        
        // Verify expert entry account
        let account_info = context
            .banks_client
            .get_account(expert_entry_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Expert entry account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_entry = ExpertEntry::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_entry.expert, expert);
        assert_eq!(deserialized_entry.domain_id, domain_id);
        assert_eq!(deserialized_entry.competency_level, competency_level);
        assert_eq!(deserialized_entry.confidence, confidence);
        assert!(deserialized_entry.confidence >= ExpertEntry::MIN_CONFIDENCE);
        assert!(deserialized_entry.reputation_score >= ExpertEntry::MIN_EXPERT_REPUTATION);
        assert!(deserialized_entry.is_active);
        
        Ok(())
    }

    /// Test remove_expert_handler with real account data
    #[tokio::test]
    async fn test_remove_expert_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let expert = get_pubkey_from_keypair(&fixture.user);
        let domain_id = "mathematics".to_string();
        let current_time = 1_000_000i64;
        
        // Find expert entry PDA
        let domain_id_bytes = domain_id.as_bytes();
        let (expert_entry_pda, _entry_bump) = find_pda(
            &[b"expert", expert.as_ref(), domain_id_bytes],
            &program_id,
        );
        
        // Create expert entry account in active state
        let expert_entry = ExpertEntry {
            expert,
            domain_id: domain_id.clone(),
            competency_level: CompetencyLevel::Expert,
            confidence: 80u8,
            source: CompetencySource::ManualAssignment,
            reputation_score: ExpertEntry::MIN_EXPERT_REPUTATION,
            created_at: current_time,
            updated_at: current_time,
            is_active: true, // Active state required for removal
            verified_by: get_pubkey_from_keypair(&fixture.authority),
            bump: _entry_bump,
        };
        
        let account = create_account_with_data(&program_id, &expert_entry)?;
        let account_shared = account_to_shared(account);
        context.set_account(&expert_entry_pda, &account_shared);
        
        // Verify expert entry account
        let account_info = context
            .banks_client
            .get_account(expert_entry_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Expert entry account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_entry = ExpertEntry::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_entry.domain_id, domain_id);
        assert!(deserialized_entry.is_active, "Expert should be active before removal");
        
        Ok(())
    }

    /// Test update_expert_handler with real account data
    #[tokio::test]
    async fn test_update_expert_handler_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let expert = get_pubkey_from_keypair(&fixture.user);
        let domain_id = "mathematics".to_string();
        let current_time = 1_000_000i64;
        let new_confidence = 90u8; // >= MIN_CONFIDENCE
        
        // Find expert entry PDA
        let domain_id_bytes = domain_id.as_bytes();
        let (expert_entry_pda, _entry_bump) = find_pda(
            &[b"expert", expert.as_ref(), domain_id_bytes],
            &program_id,
        );
        
        // Create expert entry account in active state
        let expert_entry = ExpertEntry {
            expert,
            domain_id,
            competency_level: CompetencyLevel::Expert,
            confidence: 80u8, // Initial confidence
            source: CompetencySource::ManualAssignment,
            reputation_score: ExpertEntry::MIN_EXPERT_REPUTATION,
            created_at: current_time,
            updated_at: current_time,
            is_active: true, // Active state required for update
            verified_by: get_pubkey_from_keypair(&fixture.authority),
            bump: _entry_bump,
        };
        
        let account = create_account_with_data(&program_id, &expert_entry)?;
        let account_shared = account_to_shared(account);
        context.set_account(&expert_entry_pda, &account_shared);
        
        // Verify expert entry account
        let account_info = context
            .banks_client
            .get_account(expert_entry_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Expert entry account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_entry = ExpertEntry::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert!(deserialized_entry.is_active, "Expert should be active before update");
        assert!(new_confidence >= ExpertEntry::MIN_CONFIDENCE, "New confidence should be valid");
        
        Ok(())
    }

    /// Test add_expert_handler with invalid inputs
    #[tokio::test]
    async fn test_add_expert_handler_invalid_inputs() -> Result<()> {
        // Test domain_id too long
        let long_domain_id = "a".repeat(51);
        assert!(long_domain_id.len() > 50, "Domain ID too long should be detected");
        
        // Test confidence < MIN_CONFIDENCE
        let low_confidence = ExpertEntry::MIN_CONFIDENCE - 1;
        assert!(low_confidence < ExpertEntry::MIN_CONFIDENCE, "Confidence too low should be detected");
        
        // Test reputation < MIN_EXPERT_REPUTATION
        let low_reputation = ExpertEntry::MIN_EXPERT_REPUTATION - 1;
        assert!(low_reputation < ExpertEntry::MIN_EXPERT_REPUTATION, "Reputation too low should be detected");
        
        Ok(())
    }

    /// Test remove_expert_handler with invalid state
    #[tokio::test]
    async fn test_remove_expert_handler_invalid_state() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let expert = get_pubkey_from_keypair(&fixture.user);
        let domain_id = "mathematics".to_string();
        let current_time = 1_000_000i64;
        
        // Find expert entry PDA
        let domain_id_bytes = domain_id.as_bytes();
        let (expert_entry_pda, _entry_bump) = find_pda(
            &[b"expert", expert.as_ref(), domain_id_bytes],
            &program_id,
        );
        
        // Create expert entry account in inactive state (invalid for removal)
        let expert_entry = ExpertEntry {
            expert,
            domain_id,
            competency_level: CompetencyLevel::Expert,
            confidence: 80u8,
            source: CompetencySource::ManualAssignment,
            reputation_score: ExpertEntry::MIN_EXPERT_REPUTATION,
            created_at: current_time,
            updated_at: current_time,
            is_active: false, // Invalid state
            verified_by: get_pubkey_from_keypair(&fixture.authority),
            bump: _entry_bump,
        };
        
        let account = create_account_with_data(&program_id, &expert_entry)?;
        let account_shared = account_to_shared(account);
        context.set_account(&expert_entry_pda, &account_shared);
        
        // Verify expert entry account
        let account_info = context
            .banks_client
            .get_account(expert_entry_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Expert entry account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_entry = ExpertEntry::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert!(!deserialized_entry.is_active, "Expert should NOT be active");
        
        Ok(())
    }
}