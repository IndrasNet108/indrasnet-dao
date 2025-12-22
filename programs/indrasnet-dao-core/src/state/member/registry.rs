//! Member registry methods

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use super::types::MemberRegistry;

impl MemberRegistry {
    /// Create a new member registry
    pub fn new(bump: u8) -> Result<Self> {
        let now = Clock::get()?.unix_timestamp;
        Self::new_with_time(bump, now)
    }

    /// Create a new member registry with explicit timestamp (for testing)
    pub fn new_with_time(bump: u8, current_time: i64) -> Result<Self> {
        Ok(Self {
            total_members: 0,
            active_members: 0,
            suspended_members: 0,
            banned_members: 0,
            total_reputation: 0,
            created_at: current_time,
            updated_at: current_time,
            bump,
        })
    }

    /// Add a member to the registry
    pub fn add_member(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.add_member_with_time(current_time)
    }

    /// Add a member to the registry with explicit timestamp (for testing)
    pub fn add_member_with_time(&mut self, current_time: i64) -> Result<()> {
        self.total_members = self.total_members.checked_add(1)
            .ok_or(IndrasError::Overflow)?;
        self.active_members = self.active_members.checked_add(1)
            .ok_or(IndrasError::Overflow)?;
        self.updated_at = current_time;
        Ok(())
    }

    /// Remove a member from the registry
    pub fn remove_member(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.remove_member_with_time(current_time)
    }

    /// Remove a member from the registry with explicit timestamp (for testing)
    pub fn remove_member_with_time(&mut self, current_time: i64) -> Result<()> {
        self.total_members = self.total_members.saturating_sub(1);
        self.active_members = self.active_members.saturating_sub(1);
        self.updated_at = current_time;
        Ok(())
    }

    /// Suspend a member (move from active to suspended)
    pub fn suspend_member(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.suspend_member_with_time(current_time)
    }

    /// Suspend a member with explicit timestamp (for testing)
    pub fn suspend_member_with_time(&mut self, current_time: i64) -> Result<()> {
        self.active_members = self.active_members.saturating_sub(1);
        self.suspended_members = self.suspended_members.checked_add(1)
            .ok_or(IndrasError::Overflow)?;
        self.updated_at = current_time;
        Ok(())
    }

    /// Activate a member (move from suspended to active)
    pub fn activate_member(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.activate_member_with_time(current_time)
    }

    /// Activate a member with explicit timestamp (for testing)
    pub fn activate_member_with_time(&mut self, current_time: i64) -> Result<()> {
        self.suspended_members = self.suspended_members.saturating_sub(1);
        self.active_members = self.active_members.checked_add(1)
            .ok_or(IndrasError::Overflow)?;
        self.updated_at = current_time;
        Ok(())
    }

    /// Ban a member (move from active to banned)
    pub fn ban_member(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.ban_member_with_time(current_time)
    }

    /// Ban a member with explicit timestamp (for testing)
    pub fn ban_member_with_time(&mut self, current_time: i64) -> Result<()> {
        self.active_members = self.active_members.saturating_sub(1);
        self.banned_members = self.banned_members.checked_add(1)
            .ok_or(IndrasError::Overflow)?;
        self.updated_at = current_time;
        Ok(())
    }

    /// Update reputation in registry
    pub fn update_reputation(&mut self, old_reputation: u64, new_reputation: u64) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.update_reputation_with_time(old_reputation, new_reputation, current_time)
    }

    /// Update reputation in registry with explicit timestamp (for testing)
    pub fn update_reputation_with_time(&mut self, old_reputation: u64, new_reputation: u64, current_time: i64) -> Result<()> {
        self.total_reputation = self.total_reputation.saturating_sub(old_reputation);
        self.total_reputation = self.total_reputation.checked_add(new_reputation)
            .ok_or(IndrasError::Overflow)?;
        self.updated_at = current_time;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_registry() -> MemberRegistry {
        MemberRegistry {
            total_members: 0,
            active_members: 0,
            suspended_members: 0,
            banned_members: 0,
            total_reputation: 0,
            created_at: 1000,
            updated_at: 1000,
            bump: 255,
        }
    }

    // Unit tests that call real methods using *_with_time variants
    // This ensures actual code execution and proper coverage
    #[test]
    fn test_member_registry_structure() {
        let registry = create_test_registry();
        assert_eq!(registry.total_members, 0);
        assert_eq!(registry.active_members, 0);
        assert_eq!(registry.suspended_members, 0);
        assert_eq!(registry.banned_members, 0);
        assert_eq!(registry.total_reputation, 0);
        assert_eq!(registry.created_at, 1000);
        assert_eq!(registry.updated_at, 1000);
        assert_eq!(registry.bump, 255);
    }

    #[test]
    fn test_member_registry_manual_update() {
        let mut registry = create_test_registry();
        
        // Manually test the logic without Clock::get()
        registry.total_members = 1;
        registry.active_members = 1;
        assert_eq!(registry.total_members, 1);
        assert_eq!(registry.active_members, 1);
        
        // Test suspend logic
        registry.active_members = registry.active_members.saturating_sub(1);
        registry.suspended_members = registry.suspended_members.checked_add(1).unwrap();
        assert_eq!(registry.active_members, 0);
        assert_eq!(registry.suspended_members, 1);
        
        // Test activate logic
        registry.suspended_members = registry.suspended_members.saturating_sub(1);
        registry.active_members = registry.active_members.checked_add(1).unwrap();
        assert_eq!(registry.suspended_members, 0);
        assert_eq!(registry.active_members, 1);
    }

    #[test]
    fn test_member_registry_reputation_calculation() {
        let mut registry = create_test_registry();
        registry.total_reputation = 1000;
        
        // Test reputation update logic
        registry.total_reputation = registry.total_reputation.saturating_sub(100);
        registry.total_reputation = registry.total_reputation.checked_add(200).unwrap();
        assert_eq!(registry.total_reputation, 1100);
    }

    #[test]
    fn test_member_registry_overflow_protection() {
        let mut registry = create_test_registry();
        registry.total_members = u32::MAX;
        registry.active_members = u32::MAX;
        
        // Test overflow protection
        let result = registry.total_members.checked_add(1);
        assert!(result.is_none());
    }

    #[test]
    fn test_member_registry_new_with_time() {
        let registry = MemberRegistry::new_with_time(255, 1000).unwrap();
        assert_eq!(registry.total_members, 0);
        assert_eq!(registry.active_members, 0);
        assert_eq!(registry.created_at, 1000);
        assert_eq!(registry.updated_at, 1000);
        assert_eq!(registry.bump, 255);
    }

    #[test]
    fn test_member_registry_add_member_with_time() {
        let mut registry = create_test_registry();
        
        // Call real method
        assert!(registry.add_member_with_time(2000).is_ok());
        assert_eq!(registry.total_members, 1);
        assert_eq!(registry.active_members, 1);
        assert_eq!(registry.updated_at, 2000);
    }

    #[test]
    fn test_member_registry_remove_member_with_time() {
        let mut registry = create_test_registry();
        registry.total_members = 5;
        registry.active_members = 5;
        
        // Call real method
        assert!(registry.remove_member_with_time(2000).is_ok());
        assert_eq!(registry.total_members, 4);
        assert_eq!(registry.active_members, 4);
        assert_eq!(registry.updated_at, 2000);
    }

    #[test]
    fn test_member_registry_remove_member_from_zero() {
        let mut registry = create_test_registry();
        
        // Call real method - saturating_sub prevents underflow
        assert!(registry.remove_member_with_time(2000).is_ok());
        assert_eq!(registry.total_members, 0);
        assert_eq!(registry.active_members, 0);
        assert_eq!(registry.updated_at, 2000);
    }

    #[test]
    fn test_member_registry_suspend_member_with_time() {
        let mut registry = create_test_registry();
        registry.active_members = 5;
        registry.suspended_members = 2;
        
        // Call real method
        assert!(registry.suspend_member_with_time(2000).is_ok());
        assert_eq!(registry.active_members, 4);
        assert_eq!(registry.suspended_members, 3);
        assert_eq!(registry.updated_at, 2000);
    }

    #[test]
    fn test_member_registry_activate_member_with_time() {
        let mut registry = create_test_registry();
        registry.active_members = 5;
        registry.suspended_members = 3;
        
        // Call real method
        assert!(registry.activate_member_with_time(2000).is_ok());
        assert_eq!(registry.suspended_members, 2);
        assert_eq!(registry.active_members, 6);
        assert_eq!(registry.updated_at, 2000);
    }

    #[test]
    fn test_member_registry_ban_member_with_time() {
        let mut registry = create_test_registry();
        registry.active_members = 5;
        registry.banned_members = 1;
        
        // Call real method
        assert!(registry.ban_member_with_time(2000).is_ok());
        assert_eq!(registry.active_members, 4);
        assert_eq!(registry.banned_members, 2);
        assert_eq!(registry.updated_at, 2000);
    }

    #[test]
    fn test_member_registry_update_reputation_with_time() {
        let mut registry = create_test_registry();
        registry.total_reputation = 1000;
        
        // Call real method
        assert!(registry.update_reputation_with_time(100, 200, 2000).is_ok());
        assert_eq!(registry.total_reputation, 1100);
        assert_eq!(registry.updated_at, 2000);
    }

    #[test]
    fn test_member_registry_update_reputation_overflow() {
        let mut registry = create_test_registry();
        registry.total_reputation = u64::MAX - 100;
        
        // Test overflow protection
        let old_reputation = 50;
        let new_reputation = 200; // Would cause overflow
        registry.total_reputation = registry.total_reputation.saturating_sub(old_reputation);
        let result = registry.total_reputation.checked_add(new_reputation);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_member_registry_suspend_member_overflow() {
        let mut registry = create_test_registry();
        registry.suspended_members = u32::MAX;
        
        // Test overflow protection
        let result = registry.suspended_members.checked_add(1);
        assert!(result.is_none());
    }

    #[test]
    fn test_member_registry_ban_member_overflow() {
        let mut registry = create_test_registry();
        registry.banned_members = u32::MAX;
        
        // Test overflow protection
        let result = registry.banned_members.checked_add(1);
        assert!(result.is_none());
    }

    #[test]
    fn test_member_registry_activate_from_zero_suspended() {
        let mut registry = create_test_registry();
        registry.active_members = 5;
        registry.suspended_members = 0;
        
        // Activate from zero suspended (saturating_sub prevents underflow)
        registry.suspended_members = registry.suspended_members.saturating_sub(1);
        registry.active_members = registry.active_members.checked_add(1).unwrap();
        
        assert_eq!(registry.suspended_members, 0);
        assert_eq!(registry.active_members, 6);
    }

    #[test]
    fn test_member_registry_add_member_multiple() {
        let mut registry = create_test_registry();
        
        // Call real method multiple times
        for i in 0..5 {
            assert!(registry.add_member_with_time(1000 + i).is_ok());
        }
        
        assert_eq!(registry.total_members, 5);
        assert_eq!(registry.active_members, 5);
        assert_eq!(registry.updated_at, 1004);
    }

    #[test]
    fn test_member_registry_add_member_overflow() {
        let mut registry = create_test_registry();
        registry.total_members = u32::MAX;
        registry.active_members = u32::MAX;
        
        // Call real method - should fail with overflow
        assert!(registry.add_member_with_time(2000).is_err());
    }

    #[test]
    fn test_member_registry_remove_member_multiple() {
        let mut registry = create_test_registry();
        registry.total_members = 10;
        registry.active_members = 10;
        
        // Call real method multiple times
        for i in 0..5 {
            assert!(registry.remove_member_with_time(1000 + i).is_ok());
        }
        
        assert_eq!(registry.total_members, 5);
        assert_eq!(registry.active_members, 5);
        assert_eq!(registry.updated_at, 1004);
    }

    #[test]
    fn test_member_registry_suspend_multiple_members() {
        let mut registry = create_test_registry();
        registry.active_members = 10;
        registry.suspended_members = 0;
        
        // Call real method multiple times
        for i in 0..3 {
            assert!(registry.suspend_member_with_time(1000 + i).is_ok());
        }
        
        assert_eq!(registry.active_members, 7);
        assert_eq!(registry.suspended_members, 3);
        assert_eq!(registry.updated_at, 1002);
    }

    #[test]
    fn test_member_registry_activate_multiple_members() {
        let mut registry = create_test_registry();
        registry.active_members = 5;
        registry.suspended_members = 5;
        
        // Call real method multiple times
        for i in 0..3 {
            assert!(registry.activate_member_with_time(1000 + i).is_ok());
        }
        
        assert_eq!(registry.suspended_members, 2);
        assert_eq!(registry.active_members, 8);
        assert_eq!(registry.updated_at, 1002);
    }

    #[test]
    fn test_member_registry_ban_multiple_members() {
        let mut registry = create_test_registry();
        registry.active_members = 10;
        registry.banned_members = 0;
        
        // Call real method multiple times
        for i in 0..2 {
            assert!(registry.ban_member_with_time(1000 + i).is_ok());
        }
        
        assert_eq!(registry.active_members, 8);
        assert_eq!(registry.banned_members, 2);
        assert_eq!(registry.updated_at, 1001);
    }

    #[test]
    fn test_member_registry_suspend_overflow() {
        let mut registry = create_test_registry();
        registry.active_members = 1;
        registry.suspended_members = u32::MAX;
        
        // Call real method - should fail with overflow
        assert!(registry.suspend_member_with_time(2000).is_err());
    }

    #[test]
    fn test_member_registry_activate_overflow() {
        let mut registry = create_test_registry();
        registry.active_members = u32::MAX;
        registry.suspended_members = 5;
        
        // Call real method - should fail with overflow
        assert!(registry.activate_member_with_time(2000).is_err());
    }

    #[test]
    fn test_member_registry_ban_overflow() {
        let mut registry = create_test_registry();
        registry.active_members = 1;
        registry.banned_members = u32::MAX;
        
        // Call real method - should fail with overflow
        assert!(registry.ban_member_with_time(2000).is_err());
    }

    #[test]
    fn test_member_registry_update_reputation_increase() {
        let mut registry = create_test_registry();
        registry.total_reputation = 1000;
        
        // Call real method
        assert!(registry.update_reputation_with_time(100, 500, 2000).is_ok());
        assert_eq!(registry.total_reputation, 1400);
        assert_eq!(registry.updated_at, 2000);
    }

    #[test]
    fn test_member_registry_update_reputation_decrease() {
        let mut registry = create_test_registry();
        registry.total_reputation = 1000;
        
        // Call real method
        assert!(registry.update_reputation_with_time(500, 100, 2000).is_ok());
        assert_eq!(registry.total_reputation, 600);
        assert_eq!(registry.updated_at, 2000);
    }

    #[test]
    fn test_member_registry_update_reputation_same() {
        let mut registry = create_test_registry();
        registry.total_reputation = 1000;
        
        // Call real method - same reputation (no change)
        assert!(registry.update_reputation_with_time(100, 100, 2000).is_ok());
        assert_eq!(registry.total_reputation, 1000);
        assert_eq!(registry.updated_at, 2000);
    }

    #[test]
    fn test_member_registry_update_reputation_from_zero() {
        let mut registry = create_test_registry();
        registry.total_reputation = 0;
        
        // Call real method - update from zero
        assert!(registry.update_reputation_with_time(0, 100, 2000).is_ok());
        assert_eq!(registry.total_reputation, 100);
        assert_eq!(registry.updated_at, 2000);
    }

    #[test]
    fn test_member_registry_update_reputation_to_zero() {
        let mut registry = create_test_registry();
        registry.total_reputation = 1000;
        
        // Call real method - update to zero
        assert!(registry.update_reputation_with_time(1000, 0, 2000).is_ok());
        assert_eq!(registry.total_reputation, 0);
        assert_eq!(registry.updated_at, 2000);
    }

    #[test]
    fn test_member_registry_complex_lifecycle() {
        let mut registry = create_test_registry();
        let mut time = 1000;
        
        // Call real methods - Add 5 members
        for _ in 0..5 {
            assert!(registry.add_member_with_time(time).is_ok());
            time += 1;
        }
        assert_eq!(registry.total_members, 5);
        assert_eq!(registry.active_members, 5);
        
        // Call real methods - Suspend 2
        for _ in 0..2 {
            assert!(registry.suspend_member_with_time(time).is_ok());
            time += 1;
        }
        assert_eq!(registry.active_members, 3);
        assert_eq!(registry.suspended_members, 2);
        
        // Call real method - Ban 1
        assert!(registry.ban_member_with_time(time).is_ok());
        time += 1;
        assert_eq!(registry.active_members, 2);
        assert_eq!(registry.banned_members, 1);
        
        // Call real method - Activate 1 suspended
        assert!(registry.activate_member_with_time(time).is_ok());
        assert_eq!(registry.active_members, 3);
        assert_eq!(registry.suspended_members, 1);
        
        // Final state
        assert_eq!(registry.total_members, 5);
        assert_eq!(registry.active_members, 3);
        assert_eq!(registry.suspended_members, 1);
        assert_eq!(registry.banned_members, 1);
    }

    #[test]
    fn test_member_registry_remove_all_members() {
        let mut registry = create_test_registry();
        registry.total_members = 10;
        registry.active_members = 10;
        
        // Call real method - Remove all members
        for i in 0..10 {
            assert!(registry.remove_member_with_time(1000 + i).is_ok());
        }
        
        assert_eq!(registry.total_members, 0);
        assert_eq!(registry.active_members, 0);
    }

    #[test]
    fn test_member_registry_suspend_all_active() {
        let mut registry = create_test_registry();
        registry.active_members = 5;
        registry.suspended_members = 0;
        
        // Call real method - Suspend all active members
        for i in 0..5 {
            assert!(registry.suspend_member_with_time(1000 + i).is_ok());
        }
        
        assert_eq!(registry.active_members, 0);
        assert_eq!(registry.suspended_members, 5);
    }

    #[test]
    fn test_member_registry_activate_all_suspended() {
        let mut registry = create_test_registry();
        registry.active_members = 0;
        registry.suspended_members = 5;
        
        // Call real method - Activate all suspended members
        for i in 0..5 {
            assert!(registry.activate_member_with_time(1000 + i).is_ok());
        }
        
        assert_eq!(registry.active_members, 5);
        assert_eq!(registry.suspended_members, 0);
    }

    #[test]
    fn test_member_registry_ban_all_active() {
        let mut registry = create_test_registry();
        registry.active_members = 5;
        registry.banned_members = 0;
        
        // Call real method - Ban all active members
        for i in 0..5 {
            assert!(registry.ban_member_with_time(1000 + i).is_ok());
        }
        
        assert_eq!(registry.active_members, 0);
        assert_eq!(registry.banned_members, 5);
    }

    #[test]
    fn test_member_registry_reputation_large_values() {
        let mut registry = create_test_registry();
        registry.total_reputation = 1_000_000;
        
        // Call real method - Update with large values
        assert!(registry.update_reputation_with_time(100_000, 200_000, 2000).is_ok());
        assert_eq!(registry.total_reputation, 1_100_000);
        assert_eq!(registry.updated_at, 2000);
    }

    #[test]
    fn test_member_registry_reputation_near_overflow() {
        let mut registry = create_test_registry();
        registry.total_reputation = u64::MAX - 1000;
        
        // Call real method - Should fail with overflow
        assert!(registry.update_reputation_with_time(500, 2000, 2000).is_err());
    }

    #[test]
    fn test_member_registry_new_with_time_all_fields() {
        let registry = MemberRegistry::new_with_time(128, 5000).unwrap();
        
        assert_eq!(registry.total_members, 0);
        assert_eq!(registry.active_members, 0);
        assert_eq!(registry.suspended_members, 0);
        assert_eq!(registry.banned_members, 0);
        assert_eq!(registry.total_reputation, 0);
        assert_eq!(registry.created_at, 5000);
        assert_eq!(registry.updated_at, 5000);
        assert_eq!(registry.bump, 128);
    }

    #[test]
    fn test_member_registry_add_member_preserves_other_fields() {
        let mut registry = create_test_registry();
        let original_suspended = registry.suspended_members;
        let original_banned = registry.banned_members;
        let original_reputation = registry.total_reputation;
        let original_bump = registry.bump;
        
        assert!(registry.add_member_with_time(2000).is_ok());
        
        assert_eq!(registry.suspended_members, original_suspended);
        assert_eq!(registry.banned_members, original_banned);
        assert_eq!(registry.total_reputation, original_reputation);
        assert_eq!(registry.bump, original_bump);
        assert_eq!(registry.total_members, 1);
        assert_eq!(registry.active_members, 1);
    }

    #[test]
    fn test_member_registry_remove_member_from_zero_saturating() {
        let mut registry = create_test_registry();
        
        // Remove from zero - should use saturating_sub, so no error
        assert!(registry.remove_member_with_time(2000).is_ok());
        assert_eq!(registry.total_members, 0);
        assert_eq!(registry.active_members, 0);
    }

    #[test]
    fn test_member_registry_suspend_member_preserves_other_fields() {
        let mut registry = create_test_registry();
        registry.active_members = 5;
        let original_total = registry.total_members;
        let original_banned = registry.banned_members;
        let original_reputation = registry.total_reputation;
        let original_bump = registry.bump;
        
        assert!(registry.suspend_member_with_time(2000).is_ok());
        
        assert_eq!(registry.total_members, original_total);
        assert_eq!(registry.banned_members, original_banned);
        assert_eq!(registry.total_reputation, original_reputation);
        assert_eq!(registry.bump, original_bump);
        assert_eq!(registry.active_members, 4);
        assert_eq!(registry.suspended_members, 1);
    }

    #[test]
    fn test_member_registry_activate_member_preserves_other_fields() {
        let mut registry = create_test_registry();
        registry.suspended_members = 3;
        let original_total = registry.total_members;
        let original_banned = registry.banned_members;
        let original_reputation = registry.total_reputation;
        let original_bump = registry.bump;
        
        assert!(registry.activate_member_with_time(2000).is_ok());
        
        assert_eq!(registry.total_members, original_total);
        assert_eq!(registry.banned_members, original_banned);
        assert_eq!(registry.total_reputation, original_reputation);
        assert_eq!(registry.bump, original_bump);
        assert_eq!(registry.suspended_members, 2);
        assert_eq!(registry.active_members, 1);
    }

    #[test]
    fn test_member_registry_ban_member_preserves_other_fields() {
        let mut registry = create_test_registry();
        registry.active_members = 5;
        let original_total = registry.total_members;
        let original_suspended = registry.suspended_members;
        let original_reputation = registry.total_reputation;
        let original_bump = registry.bump;
        
        assert!(registry.ban_member_with_time(2000).is_ok());
        
        assert_eq!(registry.total_members, original_total);
        assert_eq!(registry.suspended_members, original_suspended);
        assert_eq!(registry.total_reputation, original_reputation);
        assert_eq!(registry.bump, original_bump);
        assert_eq!(registry.active_members, 4);
        assert_eq!(registry.banned_members, 1);
    }

    #[test]
    fn test_member_registry_update_reputation_preserves_other_fields() {
        let mut registry = create_test_registry();
        registry.total_members = 10;
        registry.active_members = 8;
        registry.total_reputation = 1000;
        let original_suspended = registry.suspended_members;
        let original_banned = registry.banned_members;
        let original_bump = registry.bump;
        
        // update_reputation_with_time(old_reputation, new_reputation, current_time)
        assert!(registry.update_reputation_with_time(1000, 1500, 2000).is_ok());
        
        assert_eq!(registry.total_members, 10);
        assert_eq!(registry.active_members, 8);
        assert_eq!(registry.suspended_members, original_suspended);
        assert_eq!(registry.banned_members, original_banned);
        assert_eq!(registry.bump, original_bump);
        assert_eq!(registry.total_reputation, 1500);
    }
}
