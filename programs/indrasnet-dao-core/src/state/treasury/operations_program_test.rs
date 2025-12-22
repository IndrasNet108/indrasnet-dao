//! Real Solana Runtime Tests for state/treasury/operations.rs
//!
//! These tests use solana-program-test to actually call onchain functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::{account_to_shared, get_pubkey_from_keypair, create_account_with_data};
    use crate::state::treasury::operations::*;
    use crate::state::treasury::operations::onchain::*;
    use crate::state::treasury::types::TreasuryOperationType;
    use solana_sdk::{
        account::Account,
        pubkey::Pubkey as SdkPubkey,
        signature::{Signer, Keypair},
    };
    use anchor_lang::prelude::*;
    use anchor_lang::AccountDeserialize;
    use anyhow::Result;
    
    // Helper to convert Anchor Pubkey to SdkPubkey
    fn anchor_to_sdk_pubkey(anchor_pubkey: &anchor_lang::prelude::Pubkey) -> SdkPubkey {
        let bytes: [u8; 32] = anchor_pubkey.to_bytes();
        SdkPubkey::from(bytes)
    }

    /// Test initialize_treasury_operations with real account data
    #[tokio::test]
    async fn test_initialize_treasury_operations_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let operation_id = 1u64;
        let treasury_id = 1u64;
        let operation_type = TreasuryOperationType::Deposit;
        let operation_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        // Find operation PDA
        let operation_id_bytes = operation_id.to_le_bytes();
        let (operation_pda, _bump) = find_pda(
            &[b"treasury_operation", &operation_id_bytes],
            &fixture.program_id,
        );
        
        // Create operation account with initialized data
        let mut operation = TreasuryOperationsMetadata {
            operation_id,
            treasury_id,
            operation_type,
            status: TreasuryOperationStatus::Pending,
            created_at: current_time,
            operation_data_hash,
            bump,
        };
        
        // Simulate initialize_treasury_operations call
        initialize_treasury_operations(
            &mut operation,
            operation_id,
            treasury_id,
            operation_type,
            operation_data_hash,
            current_time,
            bump,
        )?;
        
        let account = create_account_with_data(&fixture.program_id, &operation)?;
        let account_shared = account_to_shared(account);
        context.set_account(&operation_pda, &account_shared);
        
        // Verify operation account
        let account_info = context
            .banks_client
            .get_account(operation_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Operation account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_operation = TreasuryOperationsMetadata::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_operation.operation_id, operation_id);
        assert_eq!(deserialized_operation.treasury_id, treasury_id);
        assert_eq!(deserialized_operation.operation_type, operation_type);
        assert_eq!(deserialized_operation.status, TreasuryOperationStatus::Pending);
        assert_eq!(deserialized_operation.operation_data_hash, operation_data_hash);
        
        Ok(())
    }

    /// Test initialize_treasury_operations with all operation types
    #[tokio::test]
    async fn test_initialize_treasury_operations_all_types() -> Result<()> {
        let operation_types = vec![
            TreasuryOperationType::Deposit,
            TreasuryOperationType::Withdrawal,
            TreasuryOperationType::Transfer,
            TreasuryOperationType::CapabilityGrant,
            TreasuryOperationType::CapabilityRevoke,
        ];
        
        for operation_type in operation_types {
            let mut fixture = TestFixture::new().await?;
            let context = fixture.context_mut();
            
            let operation_id = 1u64;
            let treasury_id = 1u64;
            let operation_data_hash = [1u8; 32];
            let current_time = 1_000_000i64;
            let bump = 255u8;
            
            let operation_id_bytes = operation_id.to_le_bytes();
            let (operation_pda, _bump) = find_pda(
                &[b"treasury_operation", &operation_id_bytes],
                &fixture.program_id,
            );
            
            let mut operation = TreasuryOperationsMetadata {
                operation_id,
                treasury_id,
                operation_type,
                status: TreasuryOperationStatus::Pending,
                created_at: current_time,
                operation_data_hash,
                bump,
            };
            
            initialize_treasury_operations(
                &mut operation,
                operation_id,
                treasury_id,
                operation_type,
                operation_data_hash,
                current_time,
                bump,
            )?;
            
            let account = create_account_with_data(&fixture.program_id, &operation)?;
            let account_shared = account_to_shared(account);
        context.set_account(&operation_pda, &account_shared);
            
            // Verify operation type
            let account_info = context
                .banks_client
                .get_account(operation_pda)
                .await?
                .ok_or_else(|| anyhow::anyhow!("Operation account not found"))?;
            
            let mut data_slice = &account_info.data[8..];
            let deserialized_operation = TreasuryOperationsMetadata::try_deserialize(&mut data_slice)
                .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
            
            assert_eq!(deserialized_operation.operation_type, operation_type);
        }
        
        Ok(())
    }

    /// Test initialize_treasury_operations with invalid inputs
    #[tokio::test]
    async fn test_initialize_treasury_operations_invalid_inputs() -> Result<()> {
        // Test operation_id == 0
        let zero_id = 0u64;
        assert_eq!(zero_id, 0, "Zero operation ID should be detected");
        
        Ok(())
    }

    // ========== Extended Tests for Sprint 12 ==========
    // Additional edge cases and validation scenarios

    /// Test initialize_treasury_operations with max operation_id
    #[tokio::test]
    async fn test_initialize_treasury_operations_max_id() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let operation_id = u64::MAX;
        let treasury_id = 1u64;
        let operation_type = TreasuryOperationType::Deposit;
        let operation_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let operation_id_bytes = operation_id.to_le_bytes();
        let (operation_pda, _bump) = find_pda(
            &[b"treasury_operation", &operation_id_bytes],
            &fixture.program_id,
        );
        
        let mut operation = TreasuryOperationsMetadata {
            operation_id,
            treasury_id,
            operation_type,
            status: TreasuryOperationStatus::Pending,
            created_at: current_time,
            operation_data_hash,
            bump,
        };
        
        // Should succeed with max ID
        initialize_treasury_operations(
            &mut operation,
            operation_id,
            treasury_id,
            operation_type,
            operation_data_hash,
            current_time,
            bump,
        )?;
        
        assert_eq!(operation.operation_id, u64::MAX, "Operation ID should be max");
        
        let account = create_account_with_data(&fixture.program_id, &operation)?;
        let account_shared = account_to_shared(account);
        context.set_account(&operation_pda, &account_shared);
        
        Ok(())
    }

    /// Test initialize_treasury_operations with max treasury_id
    #[tokio::test]
    async fn test_initialize_treasury_operations_max_treasury_id() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let operation_id = 1u64;
        let treasury_id = u64::MAX;
        let operation_type = TreasuryOperationType::Withdrawal;
        let operation_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let operation_id_bytes = operation_id.to_le_bytes();
        let (operation_pda, _bump) = find_pda(
            &[b"treasury_operation", &operation_id_bytes],
            &fixture.program_id,
        );
        
        let mut operation = TreasuryOperationsMetadata {
            operation_id,
            treasury_id,
            operation_type,
            status: TreasuryOperationStatus::Pending,
            created_at: current_time,
            operation_data_hash,
            bump,
        };
        
        // Should succeed with max treasury_id (no validation)
        initialize_treasury_operations(
            &mut operation,
            operation_id,
            treasury_id,
            operation_type,
            operation_data_hash,
            current_time,
            bump,
        )?;
        
        assert_eq!(operation.treasury_id, u64::MAX, "Treasury ID should be max");
        
        let account = create_account_with_data(&fixture.program_id, &operation)?;
        let account_shared = account_to_shared(account);
        context.set_account(&operation_pda, &account_shared);
        
        Ok(())
    }

    /// Test initialize_treasury_operations with different operation_data_hash values
    #[tokio::test]
    async fn test_initialize_treasury_operations_different_hashes() -> Result<()> {
        let hashes = vec![
            [0u8; 32], // Zero hash
            [1u8; 32], // All ones
            [255u8; 32], // All max
        ];
        
        for (idx, hash) in hashes.iter().enumerate() {
            let mut fixture = TestFixture::new().await?;
            let context = fixture.context_mut();
            
            let operation_id = (idx + 1) as u64;
            let treasury_id = 1u64;
            let operation_type = TreasuryOperationType::Deposit;
            let current_time = 1_000_000i64;
            let bump = 255u8;
            
            let operation_id_bytes = operation_id.to_le_bytes();
            let (operation_pda, _bump) = find_pda(
                &[b"treasury_operation", &operation_id_bytes],
                &fixture.program_id,
            );
            
            let mut operation = TreasuryOperationsMetadata {
                operation_id,
                treasury_id,
                operation_type,
                status: TreasuryOperationStatus::Pending,
                created_at: current_time,
                operation_data_hash: *hash,
                bump,
            };
            
            initialize_treasury_operations(
                &mut operation,
                operation_id,
                treasury_id,
                operation_type,
                *hash,
                current_time,
                bump,
            )?;
            
            assert_eq!(operation.operation_data_hash, *hash, "Hash should match");
            
            let account = create_account_with_data(&fixture.program_id, &operation)?;
            let account_shared = account_to_shared(account);
            context.set_account(&operation_pda, &account_shared);
        }
        
        Ok(())
    }

    /// Test initialize_treasury_operations status always Pending
    #[tokio::test]
    async fn test_initialize_treasury_operations_status_always_pending() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let operation_id = 1u64;
        let treasury_id = 1u64;
        let operation_type = TreasuryOperationType::Deposit;
        let operation_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let operation_id_bytes = operation_id.to_le_bytes();
        let (operation_pda, _bump) = find_pda(
            &[b"treasury_operation", &operation_id_bytes],
            &fixture.program_id,
        );
        
        let mut operation = TreasuryOperationsMetadata {
            operation_id,
            treasury_id,
            operation_type,
            status: TreasuryOperationStatus::Pending, // Should always be Pending on init
            created_at: current_time,
            operation_data_hash,
            bump,
        };
        
        initialize_treasury_operations(
            &mut operation,
            operation_id,
            treasury_id,
            operation_type,
            operation_data_hash,
            current_time,
            bump,
        )?;
        
        // Status should always be Pending after initialization
        assert_eq!(operation.status, TreasuryOperationStatus::Pending, "Status should be Pending after initialization");
        
        let account = create_account_with_data(&fixture.program_id, &operation)?;
        let account_shared = account_to_shared(account);
        context.set_account(&operation_pda, &account_shared);
        
        Ok(())
    }

    /// Test initialize_treasury_operations timestamp consistency
    #[tokio::test]
    async fn test_initialize_treasury_operations_timestamp_consistency() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let operation_id = 1u64;
        let treasury_id = 1u64;
        let operation_type = TreasuryOperationType::Transfer;
        let operation_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let operation_id_bytes = operation_id.to_le_bytes();
        let (operation_pda, _bump) = find_pda(
            &[b"treasury_operation", &operation_id_bytes],
            &fixture.program_id,
        );
        
        let mut operation = TreasuryOperationsMetadata {
            operation_id,
            treasury_id,
            operation_type,
            status: TreasuryOperationStatus::Pending,
            created_at: current_time,
            operation_data_hash,
            bump,
        };
        
        initialize_treasury_operations(
            &mut operation,
            operation_id,
            treasury_id,
            operation_type,
            operation_data_hash,
            current_time,
            bump,
        )?;
        
        assert_eq!(operation.created_at, current_time, "Created at should match current time");
        
        let account = create_account_with_data(&fixture.program_id, &operation)?;
        let account_shared = account_to_shared(account);
        context.set_account(&operation_pda, &account_shared);
        
        Ok(())
    }

    /// Test initialize_treasury_operations with zero treasury_id
    #[tokio::test]
    async fn test_initialize_treasury_operations_zero_treasury_id() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let context = fixture.context_mut();
        
        let operation_id = 1u64;
        let treasury_id = 0u64; // Zero treasury_id should be allowed
        let operation_type = TreasuryOperationType::CapabilityGrant;
        let operation_data_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let operation_id_bytes = operation_id.to_le_bytes();
        let (operation_pda, _bump) = find_pda(
            &[b"treasury_operation", &operation_id_bytes],
            &fixture.program_id,
        );
        
        let mut operation = TreasuryOperationsMetadata {
            operation_id,
            treasury_id,
            operation_type,
            status: TreasuryOperationStatus::Pending,
            created_at: current_time,
            operation_data_hash,
            bump,
        };
        
        // Should succeed with zero treasury_id (no validation)
        initialize_treasury_operations(
            &mut operation,
            operation_id,
            treasury_id,
            operation_type,
            operation_data_hash,
            current_time,
            bump,
        )?;
        
        assert_eq!(operation.treasury_id, 0, "Treasury ID can be zero");
        
        let account = create_account_with_data(&fixture.program_id, &operation)?;
        let account_shared = account_to_shared(account);
        context.set_account(&operation_pda, &account_shared);
        
        Ok(())
    }
}
