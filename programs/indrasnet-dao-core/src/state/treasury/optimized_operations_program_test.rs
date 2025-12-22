//! Real Solana Runtime Tests for state/treasury/optimized_operations.rs
//!
//! These tests use solana-program-test to test optimized treasury operations functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
#[allow(unused_imports, unused_variables, unused_mut)]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::{account_to_shared, create_account_with_data};
    use crate::state::treasury::optimized_operations::*;
    use crate::state::treasury::optimized_operations::onchain::*;
    use solana_sdk::{
        account::Account,
        pubkey::Pubkey as SdkPubkey,
        signature::{Signer, Keypair},
    };
    use anchor_lang::prelude::*;
    use anchor_lang::AccountDeserialize;
    use anyhow::Result;

    /// Test initialize_batch_operation with real account data
    #[tokio::test]
    async fn test_initialize_batch_operation_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let batch_id = 1u64;
        let treasury_id = 1u64;
        let operation_count = 5u32;
        let operations_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let batch_id_bytes = batch_id.to_le_bytes();
        let (batch_pda, _bump) = find_pda(
            &[b"treasury_batch", &batch_id_bytes],
            &program_id,
        );
        
        let mut batch = TreasuryBatchOperationMetadata {
            batch_id: 0,
            treasury_id: 0,
            operation_count: 0,
            status: BatchOperationStatus::Failed,
            created_at: 0,
            completed_at: Some(1000),
            updated_at: 0,
            operations_hash: [0u8; 32],
            bump: 0,
        };
        
        initialize_batch_operation(
            &mut batch,
            batch_id,
            treasury_id,
            operation_count,
            operations_hash,
            current_time,
            bump,
        )?;
        
        let account = create_account_with_data(&program_id, &batch)?;
        let account_shared = account_to_shared(account);
        context.set_account(&batch_pda, &account_shared);
        
        // Verify batch account
        let account_info = context
            .banks_client
            .get_account(batch_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Batch account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_batch = TreasuryBatchOperationMetadata::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_batch.batch_id, batch_id);
        assert_eq!(deserialized_batch.treasury_id, treasury_id);
        assert_eq!(deserialized_batch.operation_count, operation_count);
        assert_eq!(deserialized_batch.status, BatchOperationStatus::Pending);
        assert_eq!(deserialized_batch.created_at, current_time);
        assert_eq!(deserialized_batch.completed_at, None);
        assert_eq!(deserialized_batch.updated_at, current_time);
        
        Ok(())
    }

    /// Test update_batch_status with real account data
    #[tokio::test]
    async fn test_update_batch_status_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let batch_id = 1u64;
        let treasury_id = 1u64;
        let operation_count = 5u32;
        let operations_hash = [1u8; 32];
        let initial_time = 1_000_000i64;
        let update_time = 2_000_000i64;
        let new_status = BatchOperationStatus::Processing;
        let bump = 255u8;
        
        let batch_id_bytes = batch_id.to_le_bytes();
        let (batch_pda, _bump) = find_pda(
            &[b"treasury_batch", &batch_id_bytes],
            &program_id,
        );
        
        let mut batch = TreasuryBatchOperationMetadata {
            batch_id,
            treasury_id,
            operation_count,
            status: BatchOperationStatus::Pending,
            created_at: initial_time,
            completed_at: None,
            updated_at: initial_time,
            operations_hash,
            bump,
        };
        
        // Initialize first
        initialize_batch_operation(
            &mut batch,
            batch_id,
            treasury_id,
            operation_count,
            operations_hash,
            initial_time,
            bump,
        )?;
        
        // Update status
        update_batch_status(&mut batch, new_status, update_time)?;
        
        let account = create_account_with_data(&program_id, &batch)?;
        let account_shared = account_to_shared(account);
        context.set_account(&batch_pda, &account_shared);
        
        // Verify status update
        let account_info = context
            .banks_client
            .get_account(batch_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Batch account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_batch = TreasuryBatchOperationMetadata::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_batch.status, new_status);
        assert_eq!(deserialized_batch.updated_at, update_time);
        
        Ok(())
    }

    /// Test update_batch_status to Completed sets completed_at
    #[tokio::test]
    async fn test_update_batch_status_completed() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let batch_id = 1u64;
        let treasury_id = 1u64;
        let operation_count = 5u32;
        let operations_hash = [1u8; 32];
        let initial_time = 1_000_000i64;
        let complete_time = 2_000_000i64;
        let bump = 255u8;
        
        let batch_id_bytes = batch_id.to_le_bytes();
        let (batch_pda, _bump) = find_pda(
            &[b"treasury_batch", &batch_id_bytes],
            &program_id,
        );
        
        let mut batch = TreasuryBatchOperationMetadata {
            batch_id,
            treasury_id,
            operation_count,
            status: BatchOperationStatus::Pending,
            created_at: initial_time,
            completed_at: None,
            updated_at: initial_time,
            operations_hash,
            bump,
        };
        
        initialize_batch_operation(
            &mut batch,
            batch_id,
            treasury_id,
            operation_count,
            operations_hash,
            initial_time,
            bump,
        )?;
        
        // Update to Completed
        update_batch_status(&mut batch, BatchOperationStatus::Completed, complete_time)?;
        
        let account = create_account_with_data(&program_id, &batch)?;
        let account_shared = account_to_shared(account);
        context.set_account(&batch_pda, &account_shared);
        
        // Verify completed_at is set
        let account_info = context
            .banks_client
            .get_account(batch_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Batch account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_batch = TreasuryBatchOperationMetadata::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_batch.status, BatchOperationStatus::Completed);
        assert_eq!(deserialized_batch.completed_at, Some(complete_time));
        assert_eq!(deserialized_batch.updated_at, complete_time);
        
        Ok(())
    }

    /// Test initialize_batch_operation with all status transitions
    #[tokio::test]
    async fn test_update_batch_status_all_statuses() -> Result<()> {
        let statuses = vec![
            BatchOperationStatus::Processing,
            BatchOperationStatus::Completed,
            BatchOperationStatus::Failed,
        ];
        
        for new_status in statuses {
            let mut fixture = TestFixture::new().await?;
            let program_id = fixture.program_id;
            let context = fixture.context_mut();
            
            let batch_id = 1u64;
            let treasury_id = 1u64;
            let operation_count = 5u32;
            let operations_hash = [1u8; 32];
            let initial_time = 1_000_000i64;
            let update_time = 2_000_000i64;
            let bump = 255u8;
            
            let batch_id_bytes = batch_id.to_le_bytes();
            let (batch_pda, _bump) = find_pda(
                &[b"treasury_batch", &batch_id_bytes],
                &program_id,
            );
            
            let mut batch = TreasuryBatchOperationMetadata {
                batch_id,
                treasury_id,
                operation_count,
                status: BatchOperationStatus::Pending,
                created_at: initial_time,
                completed_at: None,
                updated_at: initial_time,
                operations_hash,
                bump,
            };
            
            initialize_batch_operation(
                &mut batch,
                batch_id,
                treasury_id,
                operation_count,
                operations_hash,
                initial_time,
                bump,
            )?;
            
            update_batch_status(&mut batch, new_status, update_time)?;
            
            let account = create_account_with_data(&program_id, &batch)?;
            let account_shared = account_to_shared(account);
            context.set_account(&batch_pda, &account_shared);
            
            // Verify status
            let account_info = context
                .banks_client
                .get_account(batch_pda)
                .await?
                .ok_or_else(|| anyhow::anyhow!("Batch account not found"))?;
            
            let mut data_slice = &account_info.data[8..];
            let deserialized_batch = TreasuryBatchOperationMetadata::try_deserialize(&mut data_slice)
                .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
            
            assert_eq!(deserialized_batch.status, new_status);
        }
        
        Ok(())
    }
}
