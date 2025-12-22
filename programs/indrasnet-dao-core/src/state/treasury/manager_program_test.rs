//! Real Solana Runtime Tests for state/treasury/manager.rs
//!
//! These tests use solana-program-test to test treasury manager functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
#[allow(unused_imports, unused_variables, unused_mut)]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::{account_to_shared, get_pubkey_from_keypair, create_account_with_data};
    use crate::state::treasury::manager::*;
    use crate::state::treasury::manager::onchain::*;
    use solana_sdk::{
        account::Account,
        pubkey::Pubkey as SdkPubkey,
        signature::{Signer, Keypair},
    };
    use anchor_lang::prelude::*;
    use anchor_lang::AccountDeserialize;
    use anyhow::Result;

    /// Test initialize_treasury with real account data
    #[tokio::test]
    async fn test_initialize_treasury_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let authority = get_pubkey_from_keypair(&fixture.authority);
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let name = "Test Treasury".to_string();
        let bump = 255u8;
        
        let (treasury_pda, _bump) = find_pda(&[b"treasury"], &program_id);
        
        let mut treasury = Treasury {
            name: String::new(),
            balance: 999,
            authority: anchor_lang::prelude::Pubkey::default(),
            bump: 0,
        };
        
        initialize_treasury(&mut treasury, name.clone(), authority, bump)?;
        
        let account = create_account_with_data(&program_id, &treasury)?;
        let account_shared = account_to_shared(account);
        context.set_account(&treasury_pda, &account_shared);
        
        // Verify treasury account
        let account_info = context
            .banks_client
            .get_account(treasury_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Treasury account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_treasury = Treasury::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_treasury.name, name);
        assert_eq!(deserialized_treasury.balance, 0);
        assert_eq!(deserialized_treasury.authority, authority);
        assert_eq!(deserialized_treasury.bump, bump);
        
        Ok(())
    }

    /// Test deposit_to_treasury with real account data
    #[tokio::test]
    async fn test_deposit_to_treasury_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let authority = get_pubkey_from_keypair(&fixture.authority);
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let initial_balance = 1_000_000u64;
        let deposit_amount = 500_000u64;
        
        let (treasury_pda, _bump) = find_pda(&[b"treasury"], &program_id);
        
        let mut treasury = Treasury {
            name: "Test Treasury".to_string(),
            balance: initial_balance,
            authority,
            bump: 255,
        };
        
        deposit_to_treasury(&mut treasury, deposit_amount)?;
        
        let account = create_account_with_data(&program_id, &treasury)?;
        let account_shared = account_to_shared(account);
        context.set_account(&treasury_pda, &account_shared);
        
        // Verify balance increased
        let account_info = context
            .banks_client
            .get_account(treasury_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Treasury account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_treasury = Treasury::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_treasury.balance, initial_balance + deposit_amount);
        
        Ok(())
    }

    /// Test withdraw_from_treasury with real account data
    #[tokio::test]
    async fn test_withdraw_from_treasury_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let authority = get_pubkey_from_keypair(&fixture.authority);
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let initial_balance = 1_000_000u64;
        let withdrawal_amount = 300_000u64;
        
        let (treasury_pda, _bump) = find_pda(&[b"treasury"], &program_id);
        
        let mut treasury = Treasury {
            name: "Test Treasury".to_string(),
            balance: initial_balance,
            authority,
            bump: 255,
        };
        
        withdraw_from_treasury(&mut treasury, withdrawal_amount)?;
        
        let account = create_account_with_data(&program_id, &treasury)?;
        let account_shared = account_to_shared(account);
        context.set_account(&treasury_pda, &account_shared);
        
        // Verify balance decreased
        let account_info = context
            .banks_client
            .get_account(treasury_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Treasury account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_treasury = Treasury::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_treasury.balance, initial_balance - withdrawal_amount);
        
        Ok(())
    }

    /// Test multiple deposits and withdrawals
    #[tokio::test]
    async fn test_treasury_multiple_operations() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let authority = get_pubkey_from_keypair(&fixture.authority);
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let initial_balance = 1_000_000u64;
        
        let (treasury_pda, _bump) = find_pda(&[b"treasury"], &program_id);
        
        let mut treasury = Treasury {
            name: "Test Treasury".to_string(),
            balance: initial_balance,
            authority,
            bump: 255,
        };
        
        // Multiple deposits
        deposit_to_treasury(&mut treasury, 100_000u64)?;
        deposit_to_treasury(&mut treasury, 200_000u64)?;
        deposit_to_treasury(&mut treasury, 300_000u64)?;
        
        // Multiple withdrawals
        withdraw_from_treasury(&mut treasury, 150_000u64)?;
        withdraw_from_treasury(&mut treasury, 250_000u64)?;
        
        let account = create_account_with_data(&program_id, &treasury)?;
        let account_shared = account_to_shared(account);
        context.set_account(&treasury_pda, &account_shared);
        
        // Verify final balance
        let account_info = context
            .banks_client
            .get_account(treasury_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Treasury account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_treasury = Treasury::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        let expected_balance = initial_balance + 100_000 + 200_000 + 300_000 - 150_000 - 250_000;
        assert_eq!(deserialized_treasury.balance, expected_balance);
        
        Ok(())
    }

    /// Test initialize_treasury with max name length
    #[tokio::test]
    async fn test_initialize_treasury_max_name_length() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let authority = get_pubkey_from_keypair(&fixture.authority);
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let name = "a".repeat(100); // Max length
        let bump = 255u8;
        
        let (treasury_pda, _bump) = find_pda(&[b"treasury"], &program_id);
        
        let mut treasury = Treasury {
            name: String::new(),
            balance: 0,
            authority: anchor_lang::prelude::Pubkey::default(),
            bump: 0,
        };
        
        initialize_treasury(&mut treasury, name.clone(), authority, bump)?;
        
        assert_eq!(treasury.name.len(), 100, "Name should be max length");
        
        let account = create_account_with_data(&program_id, &treasury)?;
        let account_shared = account_to_shared(account);
        context.set_account(&treasury_pda, &account_shared);
        
        Ok(())
    }
}
