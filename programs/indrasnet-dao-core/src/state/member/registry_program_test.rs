//! Real Solana Runtime Tests for state/member/registry.rs
//!
//! These tests use solana-program-test to test member registry functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
#[allow(unused_imports, unused_variables, unused_mut)]
mod tests {
    use crate::tests::fixtures::*;
    use crate::tests::fixtures::{account_to_shared, create_account_with_data};
    use crate::state::MemberRegistry;
    use solana_sdk::{
        account::Account,
        pubkey::Pubkey as SdkPubkey,
        signature::{Signer, Keypair},
    };
    use anchor_lang::prelude::*;
    use anchor_lang::AccountDeserialize;
    use anyhow::Result;

    /// Test new_with_time with real account data
    #[tokio::test]
    async fn test_member_registry_new_with_time_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let bump = 255u8;
        let current_time = 1_000_000i64;
        
        let (registry_pda, _bump) = find_pda(&[b"member_registry"], &program_id);
        
        let registry = MemberRegistry::new_with_time(bump, current_time)?;
        
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
        let deserialized_registry = MemberRegistry::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_registry.total_members, 0);
        assert_eq!(deserialized_registry.active_members, 0);
        assert_eq!(deserialized_registry.created_at, current_time);
        assert_eq!(deserialized_registry.updated_at, current_time);
        assert_eq!(deserialized_registry.bump, bump);
        
        Ok(())
    }

    /// Test add_member_with_time with real account data
    #[tokio::test]
    async fn test_member_registry_add_member_with_time_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let bump = 255u8;
        let current_time = 1_000_000i64;
        let update_time = 2_000_000i64;
        
        let (registry_pda, _bump) = find_pda(&[b"member_registry"], &program_id);
        
        let mut registry = MemberRegistry::new_with_time(bump, current_time)?;
        
        // Add member
        registry.add_member_with_time(update_time)?;
        
        let account = create_account_with_data(&program_id, &registry)?;
        let account_shared = account_to_shared(account);
        context.set_account(&registry_pda, &account_shared);
        
        // Verify member added
        let account_info = context
            .banks_client
            .get_account(registry_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Registry account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_registry = MemberRegistry::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_registry.total_members, 1);
        assert_eq!(deserialized_registry.active_members, 1);
        assert_eq!(deserialized_registry.updated_at, update_time);
        
        Ok(())
    }

    /// Test suspend_member_with_time with real account data
    #[tokio::test]
    async fn test_member_registry_suspend_member_with_time_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let bump = 255u8;
        let current_time = 1_000_000i64;
        let update_time = 2_000_000i64;
        
        let (registry_pda, _bump) = find_pda(&[b"member_registry"], &program_id);
        
        let mut registry = MemberRegistry::new_with_time(bump, current_time)?;
        registry.active_members = 5;
        registry.suspended_members = 2;
        
        // Suspend member
        registry.suspend_member_with_time(update_time)?;
        
        let account = create_account_with_data(&program_id, &registry)?;
        let account_shared = account_to_shared(account);
        context.set_account(&registry_pda, &account_shared);
        
        // Verify member suspended
        let account_info = context
            .banks_client
            .get_account(registry_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Registry account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_registry = MemberRegistry::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_registry.active_members, 4);
        assert_eq!(deserialized_registry.suspended_members, 3);
        assert_eq!(deserialized_registry.updated_at, update_time);
        
        Ok(())
    }

    /// Test activate_member_with_time with real account data
    #[tokio::test]
    async fn test_member_registry_activate_member_with_time_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let bump = 255u8;
        let current_time = 1_000_000i64;
        let update_time = 2_000_000i64;
        
        let (registry_pda, _bump) = find_pda(&[b"member_registry"], &program_id);
        
        let mut registry = MemberRegistry::new_with_time(bump, current_time)?;
        registry.active_members = 5;
        registry.suspended_members = 3;
        
        // Activate member
        registry.activate_member_with_time(update_time)?;
        
        let account = create_account_with_data(&program_id, &registry)?;
        let account_shared = account_to_shared(account);
        context.set_account(&registry_pda, &account_shared);
        
        // Verify member activated
        let account_info = context
            .banks_client
            .get_account(registry_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Registry account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_registry = MemberRegistry::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_registry.suspended_members, 2);
        assert_eq!(deserialized_registry.active_members, 6);
        assert_eq!(deserialized_registry.updated_at, update_time);
        
        Ok(())
    }

    /// Test ban_member_with_time with real account data
    #[tokio::test]
    async fn test_member_registry_ban_member_with_time_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let bump = 255u8;
        let current_time = 1_000_000i64;
        let update_time = 2_000_000i64;
        
        let (registry_pda, _bump) = find_pda(&[b"member_registry"], &program_id);
        
        let mut registry = MemberRegistry::new_with_time(bump, current_time)?;
        registry.active_members = 5;
        registry.banned_members = 1;
        
        // Ban member
        registry.ban_member_with_time(update_time)?;
        
        let account = create_account_with_data(&program_id, &registry)?;
        let account_shared = account_to_shared(account);
        context.set_account(&registry_pda, &account_shared);
        
        // Verify member banned
        let account_info = context
            .banks_client
            .get_account(registry_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Registry account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_registry = MemberRegistry::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_registry.active_members, 4);
        assert_eq!(deserialized_registry.banned_members, 2);
        assert_eq!(deserialized_registry.updated_at, update_time);
        
        Ok(())
    }

    /// Test update_reputation_with_time with real account data
    #[tokio::test]
    async fn test_member_registry_update_reputation_with_time_real() -> Result<()> {
        let mut fixture = TestFixture::new().await?;
        let program_id = fixture.program_id; // Get program_id before mutable borrow
        let context = fixture.context_mut();
        
        let bump = 255u8;
        let current_time = 1_000_000i64;
        let update_time = 2_000_000i64;
        let initial_reputation = 1_000u64;
        let old_reputation = 100u64;
        let new_reputation = 200u64;
        
        let (registry_pda, _bump) = find_pda(&[b"member_registry"], &program_id);
        
        let mut registry = MemberRegistry::new_with_time(bump, current_time)?;
        registry.total_reputation = initial_reputation;
        
        // Update reputation
        registry.update_reputation_with_time(old_reputation, new_reputation, update_time)?;
        
        let account = create_account_with_data(&program_id, &registry)?;
        let account_shared = account_to_shared(account);
        context.set_account(&registry_pda, &account_shared);
        
        // Verify reputation updated
        let account_info = context
            .banks_client
            .get_account(registry_pda)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Registry account not found"))?;
        
        let mut data_slice = &account_info.data[8..];
        let deserialized_registry = MemberRegistry::try_deserialize(&mut data_slice)
            .map_err(|e| anyhow::anyhow!("Deserialization failed: {:?}", e))?;
        
        assert_eq!(deserialized_registry.total_reputation, initial_reputation - old_reputation + new_reputation);
        assert_eq!(deserialized_registry.updated_at, update_time);
        
        Ok(())
    }
}
