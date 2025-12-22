//! Real Solana Runtime Tests for state/treasury/allocation.rs
//!
//! These tests use solana-program-test to test treasury allocation functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
#[allow(unused_imports, unused_variables, unused_mut)]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::{account_to_shared, create_account_with_data};
    use crate::state::treasury::allocation::*;
    use crate::state::treasury::allocation::onchain::*;
    use solana_sdk::{
        account::Account,
        pubkey::Pubkey as SdkPubkey,
        signature::{Signer, Keypair},
    };
    use anchor_lang::prelude::*;
    use anchor_lang::AccountDeserialize;
    use anyhow::Result;

    /// Test initialize_treasury_allocation with real account data
    #[tokio::test]
    async fn test_initialize_treasury_allocation_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let allocation_id = 1u64;
        let treasury_id = 1u64;
        let allocation_strategy = TreasuryAllocationStrategy::Equal;
        let allocation_config_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let allocation_id_bytes = allocation_id.to_le_bytes();
        let (allocation_pda, _bump) = find_pda(
            &[b"treasury_allocation", &allocation_id_bytes],
            &program_id,
        );
        
        let mut allocation = TreasuryAllocationStrategyMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocation_strategy: TreasuryAllocationStrategy::Equal,
            status: TreasuryAllocationStatus::Completed,
            created_at: 0,
            allocation_config_hash: [0u8; 32],
            bump: 0,
        };
        
        initialize_treasury_allocation(
            &mut allocation,
            allocation_id,
            treasury_id,
            allocation_strategy,
            allocation_config_hash,
            current_time,
            bump,
        )?;
        
        let account = create_account_with_data(&program_id, &allocation)?;
        let account_shared = account_to_shared(account);
        context.set_account(&allocation_pda, &account_shared);
        
        // Verify allocation account
        let account_info = context
            .banks_client
            .get_account(allocation_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Allocation account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_allocation = TreasuryAllocationStrategyMetadata::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_allocation.allocation_id, allocation_id);
        assert_eq!(deserialized_allocation.treasury_id, treasury_id);
        assert_eq!(deserialized_allocation.allocation_strategy, allocation_strategy);
        assert_eq!(deserialized_allocation.status, TreasuryAllocationStatus::Active);
        assert_eq!(deserialized_allocation.created_at, current_time);
        
        Ok(())
    }

    /// Test initialize_treasury_allocation with all strategies
    #[tokio::test]
    async fn test_initialize_treasury_allocation_all_strategies() -> Result<()> {
        let strategies = vec![
            TreasuryAllocationStrategy::Equal,
            TreasuryAllocationStrategy::Proportional,
            TreasuryAllocationStrategy::PriorityBased,
            TreasuryAllocationStrategy::Custom,
        ];
        
        for strategy in strategies {
            let mut fixture = TestFixture::new().await?;
            let program_id = fixture.program_id;
            let context = fixture.context_mut();
            
            let allocation_id = 1u64;
            let treasury_id = 1u64;
            let allocation_config_hash = [1u8; 32];
            let current_time = 1_000_000i64;
            let bump = 255u8;
            
            let allocation_id_bytes = allocation_id.to_le_bytes();
            let (allocation_pda, _bump) = find_pda(
                &[b"treasury_allocation", &allocation_id_bytes],
                &program_id,
            );
            
            let mut allocation = TreasuryAllocationStrategyMetadata {
                allocation_id: 0,
                treasury_id: 0,
                allocation_strategy: TreasuryAllocationStrategy::Equal,
                status: TreasuryAllocationStatus::Completed,
                created_at: 0,
                allocation_config_hash: [0u8; 32],
                bump: 0,
            };
            
            initialize_treasury_allocation(
                &mut allocation,
                allocation_id,
                treasury_id,
                strategy,
                allocation_config_hash,
                current_time,
                bump,
            )?;
            
            assert_eq!(allocation.allocation_strategy, strategy);
            
            let account = create_account_with_data(&program_id, &allocation)?;
            let account_shared = account_to_shared(account);
            context.set_account(&allocation_pda, &account_shared);
        }
        
        Ok(())
    }

    /// Test initialize_treasury_allocation status always Active
    #[tokio::test]
    async fn test_initialize_treasury_allocation_status_always_active() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id;
        let context = fixture.context_mut();
        
        let allocation_id = 1u64;
        let treasury_id = 1u64;
        let allocation_strategy = TreasuryAllocationStrategy::Equal;
        let allocation_config_hash = [1u8; 32];
        let current_time = 1_000_000i64;
        let bump = 255u8;
        
        let allocation_id_bytes = allocation_id.to_le_bytes();
        let (allocation_pda, _bump) = find_pda(
            &[b"treasury_allocation", &allocation_id_bytes],
            &program_id,
        );
        
        let mut allocation = TreasuryAllocationStrategyMetadata {
            allocation_id: 0,
            treasury_id: 0,
            allocation_strategy: TreasuryAllocationStrategy::Equal,
            status: TreasuryAllocationStatus::Completed, // Will be reset
            created_at: 0,
            allocation_config_hash: [0u8; 32],
            bump: 0,
        };
        
        initialize_treasury_allocation(
            &mut allocation,
            allocation_id,
            treasury_id,
            allocation_strategy,
            allocation_config_hash,
            current_time,
            bump,
        )?;
        
        // Status should always be Active after initialization
        assert_eq!(allocation.status, TreasuryAllocationStatus::Active);
        
        let account = create_account_with_data(&program_id, &allocation)?;
        let account_shared = account_to_shared(account);
        context.set_account(&allocation_pda, &account_shared);
        
        Ok(())
    }
}
