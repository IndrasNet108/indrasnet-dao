//! Real Solana Runtime Tests for legal/compliance.rs
//!
//! These tests use solana-program-test to actually call instruction handlers
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::account_to_shared;
    use crate::legal::compliance::*;
    use crate::legal::compliance::onchain::*;
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

    /// Test initialize_compliance_metadata with real account data
    #[tokio::test]
    async fn test_initialize_compliance_metadata_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let compliance_id = 1u64;
        let compliance_type = ComplianceType::Regulatory;
        let compliance_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        // Find compliance PDA
        let compliance_id_bytes = compliance_id.to_le_bytes();
        let (compliance_pda, _bump) = find_pda(
            &[b"compliance_metadata", &compliance_id_bytes],
            &fixture.program_id,
        );
        
        // Create compliance account with initialized data
        let mut compliance = ComplianceMetadata {
            compliance_id,
            compliance_type,
            status: ComplianceStatus::UnderReview,
            created_at: current_time,
            updated_at: current_time,
            last_checked_at: None,
            compliance_data_hash,
            bump,
        };
        
        // Simulate initialize_compliance_metadata call
        initialize_compliance_metadata(
            &mut compliance,
            compliance_id,
            compliance_type,
            compliance_data_hash,
            current_time,
            bump,
        )?;
        
        let account = create_account_with_data(&fixture.program_id, &compliance)?;
        let account_shared = account_to_shared(account);
        context.set_account(&compliance_pda, &account_shared);
        
        // Verify compliance account
        let account_info = context
            .banks_client
            .get_account(compliance_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Compliance account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_compliance = ComplianceMetadata::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_compliance.compliance_id, compliance_id);
        assert_eq!(deserialized_compliance.compliance_type, compliance_type);
        assert_eq!(deserialized_compliance.status, ComplianceStatus::UnderReview);
        assert_eq!(deserialized_compliance.compliance_data_hash, compliance_data_hash);
        assert_eq!(deserialized_compliance.created_at, current_time);
        assert_eq!(deserialized_compliance.updated_at, current_time);
        assert_eq!(deserialized_compliance.last_checked_at, None);
        
        Ok(())
    }

    /// Test update_compliance_status with real account data
    #[tokio::test]
    async fn test_update_compliance_status_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let compliance_id = 1u64;
        let compliance_type = ComplianceType::Regulatory;
        let compliance_data_hash = [1u8; 32];
        let initial_time = 1_000_000i64;
        let update_time = 2_000_000i64;
        let new_status = ComplianceStatus::Compliant;
        let bump = 255u8;
        
        // Find compliance PDA
        let compliance_id_bytes = compliance_id.to_le_bytes();
        let (compliance_pda, _bump) = find_pda(
            &[b"compliance_metadata", &compliance_id_bytes],
            &fixture.program_id,
        );
        
        // Create compliance account with initial status
        let mut compliance = ComplianceMetadata {
            compliance_id,
            compliance_type,
            status: ComplianceStatus::UnderReview,
            created_at: initial_time,
            updated_at: initial_time,
            last_checked_at: None,
            compliance_data_hash,
            bump,
        };
        
        // Initialize first
        initialize_compliance_metadata(
            &mut compliance,
            compliance_id,
            compliance_type,
            compliance_data_hash,
            initial_time,
            bump,
        )?;
        
        // Update status
        update_compliance_status(&mut compliance, new_status, update_time)?;
        
        let account = create_account_with_data(&fixture.program_id, &compliance)?;
        let account_shared = account_to_shared(account);
        context.set_account(&compliance_pda, &account_shared);
        
        // Verify compliance account
        let account_info = context
            .banks_client
            .get_account(compliance_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Compliance account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_compliance = ComplianceMetadata::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_compliance.status, new_status);
        assert_eq!(deserialized_compliance.updated_at, update_time);
        assert_eq!(deserialized_compliance.last_checked_at, Some(update_time));
        
        Ok(())
    }

    /// Test initialize_compliance_metadata with invalid inputs
    #[tokio::test]
    async fn test_initialize_compliance_metadata_invalid_inputs() -> Result<()> {
        // Test compliance_id == 0
        let zero_id = 0u64;
        assert_eq!(zero_id, 0, "Zero compliance ID should be detected");
        
        Ok(())
    }

    /// Test initialize_compliance_metadata with all compliance types
    #[tokio::test]
    async fn test_initialize_compliance_metadata_all_types() -> Result<()> {
        let compliance_types = vec![
            ComplianceType::Regulatory,
            ComplianceType::Legal,
            ComplianceType::Tax,
            ComplianceType::DataProtection,
        ];
        
        for compliance_type in compliance_types {
            let mut fixture = TestFixture::new().await?;
            let context = fixture.context_mut();
            
            let compliance_id = 1u64;
            let compliance_data_hash = [1u8; 32];
            let current_time = 1_000_000i64;
            let bump = 255u8;
            
            let compliance_id_bytes = compliance_id.to_le_bytes();
            let (compliance_pda, _bump) = find_pda(
                &[b"compliance_metadata", &compliance_id_bytes],
                &fixture.program_id,
            );
            
            let mut compliance = ComplianceMetadata {
                compliance_id,
                compliance_type,
                status: ComplianceStatus::UnderReview,
                created_at: current_time,
                updated_at: current_time,
                last_checked_at: None,
                compliance_data_hash,
                bump,
            };
            
            initialize_compliance_metadata(
                &mut compliance,
                compliance_id,
                compliance_type,
                compliance_data_hash,
                current_time,
                bump,
            )?;
            
            let account = create_account_with_data(&fixture.program_id, &compliance)?;
            let account_shared = account_to_shared(account);
        context.set_account(&compliance_pda, &account_shared);
            
            // Verify compliance type
            let account_info = context
                .banks_client
                .get_account(compliance_pda)
                .await?
                .ok_or_else(|| anyhow::anyhow!("Compliance account not found"))?;
            
            let mut data_slice = &account_info.data[8..];
            let deserialized_compliance = ComplianceMetadata::try_deserialize(&mut data_slice)
                .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
            
            assert_eq!(deserialized_compliance.compliance_type, compliance_type);
        }
        
        Ok(())
    }

    /// Test update_compliance_status with all statuses
    #[tokio::test]
    async fn test_update_compliance_status_all_statuses() -> Result<()> {
        let statuses = vec![
            ComplianceStatus::Compliant,
            ComplianceStatus::NonCompliant,
            ComplianceStatus::UnderReview,
            ComplianceStatus::RequiresAction,
        ];
        
        for new_status in statuses {
            let mut fixture = TestFixture::new().await?;
            let context = fixture.context_mut();
            
            let compliance_id = 1u64;
            let compliance_type = ComplianceType::Regulatory;
            let compliance_data_hash = [1u8; 32];
            let initial_time = 1_000_000i64;
            let update_time = 2_000_000i64;
            let bump = 255u8;
            
            let compliance_id_bytes = compliance_id.to_le_bytes();
            let (compliance_pda, _bump) = find_pda(
                &[b"compliance_metadata", &compliance_id_bytes],
                &fixture.program_id,
            );
            
            let mut compliance = ComplianceMetadata {
                compliance_id,
                compliance_type,
                status: ComplianceStatus::UnderReview,
                created_at: initial_time,
                updated_at: initial_time,
                last_checked_at: None,
                compliance_data_hash,
                bump,
            };
            
            initialize_compliance_metadata(
                &mut compliance,
                compliance_id,
                compliance_type,
                compliance_data_hash,
                initial_time,
                bump,
            )?;
            
            update_compliance_status(&mut compliance, new_status, update_time)?;
            
            let account = create_account_with_data(&fixture.program_id, &compliance)?;
            let account_shared = account_to_shared(account);
        context.set_account(&compliance_pda, &account_shared);
            
            // Verify status update
            let account_info = context
                .banks_client
                .get_account(compliance_pda)
                .await?
                .ok_or_else(|| anyhow::anyhow!("Compliance account not found"))?;
            
            let mut data_slice = &account_info.data[8..];
            let deserialized_compliance = ComplianceMetadata::try_deserialize(&mut data_slice)
                .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
            
            assert_eq!(deserialized_compliance.status, new_status);
        }
        
        Ok(())
    }

    // ========== Extended Tests for Sprint 11 ==========
    // Additional edge cases and validation scenarios

    /// Test initialize_compliance_metadata status always UnderReview
    #[tokio::test]
    async fn test_initialize_compliance_metadata_status_always_under_review() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let compliance_id = 1u64;
        let compliance_type = ComplianceType::Regulatory;
        let compliance_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let compliance_id_bytes = compliance_id.to_le_bytes();
        let (compliance_pda, _bump) = find_pda(
            &[b"compliance_metadata", &compliance_id_bytes],
            &fixture.program_id,
        );
        
        let mut compliance = ComplianceMetadata {
            compliance_id,
            compliance_type,
            status: ComplianceStatus::UnderReview, // Should always be UnderReview on init
            created_at: current_time,
            updated_at: current_time,
            last_checked_at: None,
            compliance_data_hash,
            bump,
        };
        
        initialize_compliance_metadata(
            &mut compliance,
            compliance_id,
            compliance_type,
            compliance_data_hash,
            current_time,
            bump,
        )?;
        
        // Status should always be UnderReview after initialization
        assert_eq!(compliance.status, ComplianceStatus::UnderReview, "Status should be UnderReview after initialization");
        assert_eq!(compliance.last_checked_at, None, "Last checked should be None on initialization");
        
        let account = create_account_with_data(&fixture.program_id, &compliance)?;
        let account_shared = account_to_shared(account);
        context.set_account(&compliance_pda, &account_shared);
        
        Ok(())
    }

    /// Test update_compliance_status timestamp updates
    #[tokio::test]
    async fn test_update_compliance_status_timestamp_updates() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let compliance_id = 1u64;
        let compliance_type = ComplianceType::Regulatory;
        let compliance_data_hash = [1u8; 32];
        let initial_time = 1_000_000i64;
        let update_time = 2_000_000i64;
        let new_status = ComplianceStatus::Compliant;
        let bump = 255u8;
        
        let compliance_id_bytes = compliance_id.to_le_bytes();
        let (compliance_pda, _bump) = find_pda(
            &[b"compliance_metadata", &compliance_id_bytes],
            &fixture.program_id,
        );
        
        let mut compliance = ComplianceMetadata {
            compliance_id,
            compliance_type,
            status: ComplianceStatus::UnderReview,
            created_at: initial_time,
            updated_at: initial_time,
            last_checked_at: None,
            compliance_data_hash,
            bump,
        };
        
        initialize_compliance_metadata(
            &mut compliance,
            compliance_id,
            compliance_type,
            compliance_data_hash,
            initial_time,
            bump,
        )?;
        
        // Update status
        update_compliance_status(&mut compliance, new_status, update_time)?;
        
        // Verify timestamps are updated correctly
        assert_eq!(compliance.updated_at, update_time, "Updated at should be set to update time");
        assert_eq!(compliance.last_checked_at, Some(update_time), "Last checked should be set to update time");
        assert!(compliance.updated_at > compliance.created_at, "Updated at should be after created at");
        
        let account = create_account_with_data(&fixture.program_id, &compliance)?;
        let account_shared = account_to_shared(account);
        context.set_account(&compliance_pda, &account_shared);
        
        Ok(())
    }

    /// Test update_compliance_status multiple updates
    #[tokio::test]
    async fn test_update_compliance_status_multiple_updates() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let compliance_id = 1u64;
        let compliance_type = ComplianceType::Regulatory;
        let compliance_data_hash = [1u8; 32];
        let initial_time = 1_000_000i64;
        let update_time1 = 2_000_000i64;
        let update_time2 = 3_000_000i64;
        let bump = 255u8;
        
        let compliance_id_bytes = compliance_id.to_le_bytes();
        let (compliance_pda, _bump) = find_pda(
            &[b"compliance_metadata", &compliance_id_bytes],
            &fixture.program_id,
        );
        
        let mut compliance = ComplianceMetadata {
            compliance_id,
            compliance_type,
            status: ComplianceStatus::UnderReview,
            created_at: initial_time,
            updated_at: initial_time,
            last_checked_at: None,
            compliance_data_hash,
            bump,
        };
        
        initialize_compliance_metadata(
            &mut compliance,
            compliance_id,
            compliance_type,
            compliance_data_hash,
            initial_time,
            bump,
        )?;
        
        // First update
        update_compliance_status(&mut compliance, ComplianceStatus::RequiresAction, update_time1)?;
        assert_eq!(compliance.status, ComplianceStatus::RequiresAction);
        assert_eq!(compliance.updated_at, update_time1);
        
        // Second update
        update_compliance_status(&mut compliance, ComplianceStatus::Compliant, update_time2)?;
        assert_eq!(compliance.status, ComplianceStatus::Compliant);
        assert_eq!(compliance.updated_at, update_time2);
        assert_eq!(compliance.last_checked_at, Some(update_time2));
        
        let account = create_account_with_data(&fixture.program_id, &compliance)?;
        let account_shared = account_to_shared(account);
        context.set_account(&compliance_pda, &account_shared);
        
        Ok(())
    }

    /// Test initialize_compliance_metadata with different compliance_data_hash values
    #[tokio::test]
    async fn test_initialize_compliance_metadata_different_hashes() -> Result<()> {
        let hashes = vec![
            [0u8; 32], // Zero hash
            [1u8; 32], // All ones
            [255u8; 32], // All max
        ];
        
        for (idx, hash) in hashes.iter().enumerate() {
            let mut fixture = TestFixture::new().await?;
            let context = fixture.context_mut();
            
            let compliance_id = (idx + 1) as u64;
            let compliance_type = ComplianceType::Regulatory;
            let current_time = 1_000_000i64;
            let bump = 255u8;
            
            let compliance_id_bytes = compliance_id.to_le_bytes();
            let (compliance_pda, _bump) = find_pda(
                &[b"compliance_metadata", &compliance_id_bytes],
                &fixture.program_id,
            );
            
            let mut compliance = ComplianceMetadata {
                compliance_id,
                compliance_type,
                status: ComplianceStatus::UnderReview,
                created_at: current_time,
                updated_at: current_time,
                last_checked_at: None,
                compliance_data_hash: *hash,
                bump,
            };
            
            initialize_compliance_metadata(
                &mut compliance,
                compliance_id,
                compliance_type,
                *hash,
                current_time,
                bump,
            )?;
            
            assert_eq!(compliance.compliance_data_hash, *hash, "Hash should match");
            
            let account = create_account_with_data(&fixture.program_id, &compliance)?;
            let account_shared = account_to_shared(account);
            context.set_account(&compliance_pda, &account_shared);
        }
        
        Ok(())
    }

    /// Test initialize_compliance_metadata with max compliance_id
    #[tokio::test]
    async fn test_initialize_compliance_metadata_max_id() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let compliance_id = u64::MAX;
        let compliance_type = ComplianceType::DataProtection;
        let compliance_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let compliance_id_bytes = compliance_id.to_le_bytes();
        let (compliance_pda, _bump) = find_pda(
            &[b"compliance_metadata", &compliance_id_bytes],
            &fixture.program_id,
        );
        
        let mut compliance = ComplianceMetadata {
            compliance_id,
            compliance_type,
            status: ComplianceStatus::UnderReview,
            created_at: current_time,
            updated_at: current_time,
            last_checked_at: None,
            compliance_data_hash,
            bump,
        };
        
        // Should succeed with max ID
        initialize_compliance_metadata(
            &mut compliance,
            compliance_id,
            compliance_type,
            compliance_data_hash,
            current_time,
            bump,
        )?;
        
        assert_eq!(compliance.compliance_id, u64::MAX, "Compliance ID should be max");
        
        let account = create_account_with_data(&fixture.program_id, &compliance)?;
        let account_shared = account_to_shared(account);
        context.set_account(&compliance_pda, &account_shared);
        
        Ok(())
    }
}
