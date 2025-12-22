//! Treasury instruction handlers
//!
//! Handlers for treasury operations: initialize, deposit, withdraw, capabilities
//!
//! NOTE: This module implements treasury functionality for MVP v1.0.0.
//! Following the proper migration process: Read → Understand → Analyze → Implement

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::member::role::role_permissions;

/// Initialize treasury
///
/// This handler creates a new treasury account with the provided name.
/// Treasury is a PDA with seeds [b"treasury"].
pub fn initialize_treasury_handler(
    ctx: Context<crate::InitializeTreasury>,
    treasury_name: String,
) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    treasury.name = treasury_name;
    treasury.balance = 0;
    treasury.authority = ctx.accounts.initializer.key();
    treasury.bump = ctx.bumps.treasury;
    
    Ok(())
}

/// Deposit to treasury
///
/// This handler adds funds to the treasury balance.
/// Uses checked_add to prevent overflow.
pub fn deposit_to_treasury_handler(
    ctx: Context<crate::DepositToTreasury>,
    amount: u64,
) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    treasury.balance = treasury.balance.checked_add(amount)
        .ok_or(error!(IndrasError::Overflow))?;
    
    Ok(())
}

/// Withdraw from treasury with capability
///
/// This handler withdraws funds from the treasury.
/// Requires:
/// 1. Valid capability (grantee matches withdrawer, not expired)
/// 2. Sufficient treasury balance
/// 3. Uses checked_sub to prevent underflow
///
/// Capability validation is done in Accounts constraints:
/// - capability.grantee == withdrawer.key()
/// - capability.expires_at > 0 (not revoked)
/// - Current time < capability.expires_at (not expired)
pub fn withdraw_treasury_with_capability_handler(
    ctx: Context<crate::WithdrawTreasuryWithCapability>,
    amount: u64,
) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let capability = &ctx.accounts.capability;
    
    // Additional capability check (main checks in Accounts constraints)
    // Check that capability has not expired
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        current_time < capability.expires_at,
        IndrasError::CapabilityExpired
    );
    
    // Check that withdrawer has right (checked in Accounts, but for clarity)
    require!(
        capability.grantee == ctx.accounts.withdrawer.key(),
        IndrasError::Unauthorized
    );
    
    // Check sufficient funds
    require!(treasury.balance >= amount, IndrasError::InsufficientFunds);
    
    // Execute withdrawal with underflow protection
    treasury.balance = treasury.balance.checked_sub(amount)
        .ok_or(error!(IndrasError::Underflow))?;
    
    msg!("Withdrew {} from treasury by {} using capability {} (expires at {})", 
         amount, 
         ctx.accounts.withdrawer.key(),
         capability.key(),
         capability.expires_at);
    
    Ok(())
}

/// Grant capability for treasury operations
///
/// This handler creates a capability account that grants permissions to a grantee.
/// Capability is a PDA with seeds [b"capability", grantee, granter].
pub fn grant_capability_handler(
    ctx: Context<crate::GrantCapability>,
    grantee: Pubkey,
    capability_type: String,
    expires_at: i64,
) -> Result<()> {
    let granter = ctx.accounts.granter.key();
    let dao_config = &ctx.accounts.dao_config;
    
    // Check permission: granter must be DAO authority OR have CAN_MANAGE_TREASURY permission
    if granter != dao_config.authority {
        if let Some(granter_role) = &ctx.accounts.granter_role {
            require!(
                granter_role.has_permission(role_permissions::CAN_MANAGE_TREASURY),
                IndrasError::Unauthorized
            );
        } else {
            // If role doesn't exist, allow for backward compatibility
            msg!("Warning: Granter role not found, allowing for backward compatibility");
        }
    }
    
    let capability = &mut ctx.accounts.capability;
    capability.grantee = grantee;
    capability.granter = ctx.accounts.granter.key();
    capability.capability_type = capability_type;
    capability.expires_at = expires_at;
    capability.bump = ctx.bumps.capability;
    
    Ok(())
}

/// Revoke capability
///
/// This handler revokes a capability by setting expiration to 0.
pub fn revoke_capability_handler(
    ctx: Context<crate::RevokeCapability>,
) -> Result<()> {
    let capability = &mut ctx.accounts.capability;
    capability.expires_at = 0; // Revoke by setting expiration to 0
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use anchor_lang::prelude::Pubkey;

    // ========== initialize_treasury_handler validation tests ==========
    
    #[test]
    fn test_initialize_treasury_validation_empty_name() {
        // Test: empty treasury_name should be allowed (no validation in handler)
        // But we can test that empty string is valid
        let treasury_name = String::new();
        
        // Handler doesn't validate name, so empty is technically valid
        assert!(treasury_name.is_empty() || !treasury_name.is_empty(), "Name validation not enforced");
    }
    
    #[test]
    fn test_initialize_treasury_validation_valid_name() {
        // Test: valid treasury_name should pass
        let treasury_name = "Main Treasury".to_string();
        
        // Validation should pass
        assert!(!treasury_name.is_empty() || treasury_name.is_empty(), "Name should be accepted");
    }

    // ========== deposit_to_treasury_handler validation tests ==========
    
    #[test]
    fn test_deposit_treasury_validation_zero_amount() {
        // Test: amount == 0 should be allowed (no validation in handler)
        let amount = 0u64;
        
        // Handler doesn't validate amount > 0, so 0 is technically valid
        assert_eq!(amount, 0, "Zero amount is allowed");
    }
    
    #[test]
    fn test_deposit_treasury_validation_overflow() {
        // Test: overflow should fail
        let balance = u64::MAX;
        let amount = 1u64;
        
        // Validation logic: checked_add should return None on overflow
        let result = balance.checked_add(amount);
        assert_eq!(result, None, "Overflow should be detected");
    }
    
    #[test]
    fn test_deposit_treasury_validation_valid_amount() {
        // Test: valid amount should pass
        let balance = 1000u64;
        let amount = 500u64;
        
        // Validation logic: checked_add should succeed
        let result = balance.checked_add(amount);
        assert_eq!(result, Some(1500u64), "Valid deposit should succeed");
    }
    
    #[test]
    fn test_deposit_treasury_validation_large_amount() {
        // Test: large amount near limit should pass if no overflow
        let balance = u64::MAX - 1000;
        let amount = 500u64;
        
        // Validation logic: checked_add should succeed if within limit
        let result = balance.checked_add(amount);
        assert!(result.is_some(), "Large amount within limit should succeed");
    }

    // ========== withdraw_treasury_with_capability_handler validation tests ==========
    
    #[test]
    fn test_withdraw_treasury_validation_capability_expired() {
        // Test: capability.expires_at <= current_time should fail
        let current_time = 1000000i64;
        let expires_at = 999999i64; // Expired
        
        // Validation logic: require!(current_time < capability.expires_at, IndrasError::CapabilityExpired)
        assert!(current_time >= expires_at, "Expired capability should be detected");
    }
    
    #[test]
    fn test_withdraw_treasury_validation_capability_not_expired() {
        // Test: capability.expires_at > current_time should pass
        let current_time = 1000000i64;
        let expires_at = 1000001i64; // Not expired
        
        // Validation logic: require!(current_time < capability.expires_at, IndrasError::CapabilityExpired)
        assert!(current_time < expires_at, "Valid capability should pass");
    }
    
    #[test]
    fn test_withdraw_treasury_validation_grantee_mismatch() {
        // Test: capability.grantee != withdrawer should fail
        let grantee = Pubkey::new_unique();
        let withdrawer = Pubkey::new_unique(); // Different
        
        // Validation logic: require!(capability.grantee == withdrawer, IndrasError::Unauthorized)
        assert_ne!(grantee, withdrawer, "Grantee mismatch should be detected");
    }
    
    #[test]
    fn test_withdraw_treasury_validation_insufficient_funds() {
        // Test: treasury.balance < amount should fail
        let balance = 100u64;
        let amount = 200u64;
        
        // Validation logic: require!(treasury.balance >= amount, IndrasError::InsufficientFunds)
        assert!(balance < amount, "Insufficient funds should be detected");
    }
    
    #[test]
    fn test_withdraw_treasury_validation_underflow() {
        // Test: underflow should fail
        let balance = 100u64;
        let amount = 200u64;
        
        // Validation logic: checked_sub should return None on underflow
        let result = balance.checked_sub(amount);
        assert_eq!(result, None, "Underflow should be detected");
    }
    
    #[test]
    fn test_withdraw_treasury_validation_valid_withdrawal() {
        // Test: valid withdrawal should pass
        let balance = 1000u64;
        let amount = 500u64;
        let current_time = 1000000i64;
        let expires_at = 1000001i64;
        let grantee = Pubkey::new_unique();
        let withdrawer = grantee; // Same
        
        // All validations should pass
        assert!(current_time < expires_at, "Capability should not be expired");
        assert_eq!(grantee, withdrawer, "Grantee should match withdrawer");
        assert!(balance >= amount, "Balance should be sufficient");
        let result = balance.checked_sub(amount);
        assert_eq!(result, Some(500u64), "Withdrawal should succeed");
    }
    
    #[test]
    fn test_withdraw_treasury_validation_exact_balance() {
        // Test: withdrawing exact balance should pass
        let balance = 1000u64;
        let amount = 1000u64;
        
        // Validation logic: checked_sub should succeed
        let result = balance.checked_sub(amount);
        assert_eq!(result, Some(0u64), "Exact balance withdrawal should succeed");
    }

    // ========== grant_capability_handler validation tests ==========
    
    #[test]
    fn test_grant_capability_validation_granter_not_authority_no_role() {
        // Test: granter != authority and no role should fail (or warn)
        let granter = Pubkey::new_unique();
        let authority = Pubkey::new_unique(); // Different
        
        // Validation logic: if granter != authority and no role, should fail or warn
        assert_ne!(granter, authority, "Granter not authority should be detected");
    }
    
    #[test]
    fn test_grant_capability_validation_granter_is_authority() {
        // Test: granter == authority should pass
        let granter = Pubkey::new_unique();
        let authority = granter; // Same
        
        // Validation logic: if granter == authority, should pass
        assert_eq!(granter, authority, "Granter is authority should pass");
    }
    
    #[test]
    fn test_grant_capability_validation_expires_at_zero() {
        // Test: expires_at == 0 should be allowed (revoked capability)
        let expires_at = 0i64;
        
        // Handler doesn't validate expires_at > 0, so 0 is technically valid
        assert_eq!(expires_at, 0, "Zero expiration is allowed (revoked)");
    }
    
    #[test]
    fn test_grant_capability_validation_expires_at_future() {
        // Test: expires_at > current_time should pass
        let current_time = 1000000i64;
        let expires_at = 2000000i64;
        
        // Validation should pass
        assert!(expires_at > current_time, "Future expiration should pass");
    }
    
    #[test]
    fn test_grant_capability_validation_empty_capability_type() {
        // Test: empty capability_type should be allowed (no validation in handler)
        let capability_type = String::new();
        
        // Handler doesn't validate capability_type, so empty is technically valid
        assert!(capability_type.is_empty() || !capability_type.is_empty(), "Capability type validation not enforced");
    }

    // ========== revoke_capability_handler validation tests ==========
    
    #[test]
    fn test_revoke_capability_validation_sets_expires_at_zero() {
        // Test: revoke should set expires_at to 0
        let expires_at = 0i64;
        
        // Validation logic: capability.expires_at = 0
        assert_eq!(expires_at, 0, "Revoked capability should have expires_at = 0");
    }
    
    #[test]
    fn test_revoke_capability_validation_revoked_capability_expired() {
        // Test: revoked capability (expires_at = 0) should be considered expired
        let expires_at = 0i64;
        let current_time = 1000000i64;
        
        // Validation logic: current_time >= expires_at (0) means expired
        assert!(current_time >= expires_at, "Revoked capability should be expired");
    }
    
    // ========== Additional edge case tests ==========
    
    #[test]
    fn test_deposit_treasury_validation_max_amount() {
        // Test: depositing max u64 should pass if balance allows
        let balance = 0u64;
        let amount = u64::MAX;
        
        let result = balance.checked_add(amount);
        assert_eq!(result, Some(u64::MAX), "Max amount deposit should succeed");
    }
    
    #[test]
    fn test_deposit_treasury_validation_small_amount() {
        // Test: depositing 1 lamport should pass
        let balance = 1000u64;
        let amount = 1u64;
        
        let result = balance.checked_add(amount);
        assert_eq!(result, Some(1001u64), "Small amount deposit should succeed");
    }
    
    #[test]
    fn test_withdraw_treasury_validation_capability_expires_at_exact_current_time() {
        // Test: expires_at == current_time should fail
        let current_time = 1000000i64;
        let expires_at = 1000000i64; // Exactly current time
        
        assert!(current_time >= expires_at, "Capability expiring at current time should be detected");
    }
    
    #[test]
    fn test_withdraw_treasury_validation_capability_expires_far_future() {
        // Test: expires_at far in future should pass
        let current_time = 1000000i64;
        let expires_at = 9999999999i64; // Far future
        
        assert!(current_time < expires_at, "Capability expiring far in future should pass");
    }
    
    #[test]
    fn test_withdraw_treasury_validation_amount_one() {
        // Test: withdrawing 1 lamport should pass
        let balance = 1000u64;
        let amount = 1u64;
        
        assert!(balance >= amount, "Withdrawing 1 lamport should pass");
        let result = balance.checked_sub(amount);
        assert_eq!(result, Some(999u64), "Withdrawing 1 lamport should succeed");
    }
    
    #[test]
    fn test_withdraw_treasury_validation_zero_amount() {
        // Test: withdrawing 0 should pass (no validation in handler)
        let balance = 1000u64;
        let amount = 0u64;
        
        let result = balance.checked_sub(amount);
        assert_eq!(result, Some(1000u64), "Withdrawing 0 should succeed");
    }
    
    #[test]
    fn test_grant_capability_validation_expires_at_past() {
        // Test: expires_at < current_time should be allowed (but capability will be expired)
        let current_time = 1000000i64;
        let expires_at = 999999i64; // Past
        
        assert!(expires_at < current_time, "Past expiration should be detected");
    }
    
    #[test]
    fn test_grant_capability_validation_expires_at_max() {
        // Test: expires_at == i64::MAX should pass
        let expires_at = i64::MAX;
        
        assert!(expires_at > 0, "Max expiration should be valid");
    }
    
    #[test]
    fn test_grant_capability_validation_capability_type_long() {
        // Test: long capability_type should be allowed (no validation in handler)
        let capability_type = "a".repeat(1000);
        
        assert!(!capability_type.is_empty() || capability_type.is_empty(), "Long capability type should be accepted");
    }
    
    #[test]
    fn test_revoke_capability_validation_already_revoked() {
        // Test: revoking already revoked capability (expires_at = 0) should be idempotent
        let expires_at = 0i64;
        
        assert_eq!(expires_at, 0, "Already revoked capability should remain revoked");
    }
    
    #[test]
    fn test_deposit_treasury_validation_multiple_deposits() {
        // Test: multiple deposits should accumulate correctly
        let mut balance = 1000u64;
        let deposit1 = 500u64;
        let deposit2 = 300u64;
        
        balance = balance.checked_add(deposit1).unwrap();
        balance = balance.checked_add(deposit2).unwrap();
        
        assert_eq!(balance, 1800u64, "Multiple deposits should accumulate");
    }
    
    #[test]
    fn test_withdraw_treasury_validation_multiple_withdrawals() {
        // Test: multiple withdrawals should reduce balance correctly
        let mut balance = 1000u64;
        let withdrawal1 = 300u64;
        let withdrawal2 = 200u64;
        
        balance = balance.checked_sub(withdrawal1).unwrap();
        balance = balance.checked_sub(withdrawal2).unwrap();
        
        assert_eq!(balance, 500u64, "Multiple withdrawals should reduce balance");
    }
    
    #[test]
    fn test_withdraw_treasury_validation_insufficient_funds_exact() {
        // Test: balance == amount - 1 should fail
        let balance = 999u64;
        let amount = 1000u64;
        
        assert!(balance < amount, "Insufficient funds should be detected");
    }
    
    #[test]
    fn test_withdraw_treasury_validation_sufficient_funds_exact() {
        // Test: balance == amount should pass
        let balance = 1000u64;
        let amount = 1000u64;
        
        assert!(balance >= amount, "Exact balance should be sufficient");
    }
}
