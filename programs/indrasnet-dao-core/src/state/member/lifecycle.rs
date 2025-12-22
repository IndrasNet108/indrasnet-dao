//! Member lifecycle methods

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use super::types::Member;
use crate::state::enums::MemberStatus;

impl Member {
    /// Create a new member with current time
    pub fn new(
        pubkey: Pubkey,
        created_by: Pubkey,
        bump: u8,
    ) -> Result<Self> {
        let current_time = Clock::get()?.unix_timestamp;
        Self::new_with_time(pubkey, created_by, bump, current_time)
    }

    /// Create a new member with specified time
    pub fn new_with_time(
        pubkey: Pubkey,
        created_by: Pubkey,
        bump: u8,
        current_time: i64,
    ) -> Result<Self> {
        Ok(Self {
            pubkey,
            status: MemberStatus::Active,
            // NOTE: role field removed - use MemberRole account instead
            reputation: 100, // Initial reputation
            joined_at: current_time,
            last_activity: current_time,
            contributions_count: 0,
            votes_cast: 0,
            ideas_created: 0,
            proposals_created: 0,
            suspension_reason: None,
            suspension_until: None,
            created_by,
            bump,
        })
    }

    /// Member leaves the DAO
    pub fn leave(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.leave_with_time(current_time)
    }

    /// Member leaves the DAO with specified time
    pub fn leave_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(self.status == MemberStatus::Active, IndrasError::InvalidState);
        self.status = MemberStatus::Inactive;
        self.last_activity = current_time;
        Ok(())
    }

    /// Suspend member
    pub fn suspend(&mut self, reason: String, duration_hours: u64) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.suspend_with_time(reason, duration_hours, current_time)
    }

    /// Suspend member with specified time
    pub fn suspend_with_time(&mut self, reason: String, duration_hours: u64, current_time: i64) -> Result<()> {
        require!(self.status == MemberStatus::Active, IndrasError::InvalidState);
        require!(!reason.is_empty(), IndrasError::InvalidInput);
        
        self.status = MemberStatus::Suspended;
        self.suspension_reason = Some(reason);
        let duration_seconds = duration_hours.checked_mul(3600)
            .ok_or(IndrasError::Overflow)?;
        self.suspension_until = Some(current_time.checked_add(duration_seconds as i64)
            .ok_or(IndrasError::Overflow)?);
        self.last_activity = current_time;
        Ok(())
    }

    /// Activate suspended member
    pub fn activate(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.activate_with_time(current_time)
    }

    /// Activate suspended member with specified time
    pub fn activate_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(self.status == MemberStatus::Suspended, IndrasError::InvalidState);
        
        // Check if suspension has expired
        if let Some(until) = self.suspension_until {
            require!(current_time >= until, IndrasError::SuspensionNotExpired);
        }
        
        self.status = MemberStatus::Active;
        self.suspension_reason = None;
        self.suspension_until = None;
        self.last_activity = current_time;
        Ok(())
    }

    /// Ban member
    pub fn ban(&mut self, reason: String) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.ban_with_time(reason, current_time)
    }

    /// Ban member with specified time
    pub fn ban_with_time(&mut self, reason: String, current_time: i64) -> Result<()> {
        require!(self.status != MemberStatus::Banned, IndrasError::InvalidState);
        require!(!reason.is_empty(), IndrasError::InvalidInput);
        
        self.status = MemberStatus::Banned;
        self.suspension_reason = Some(reason);
        self.suspension_until = None; // Ban is permanent
        self.last_activity = current_time;
        Ok(())
    }

    /// Update activity timestamp
    pub fn update_activity(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.update_activity_with_time(current_time)
    }

    /// Update activity timestamp with specified time
    pub fn update_activity_with_time(&mut self, current_time: i64) -> Result<()> {
        self.last_activity = current_time;
        Ok(())
    }

    /// Update reputation
    pub fn update_reputation(&mut self, new_reputation: u64) {
        self.reputation = new_reputation;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    #[test]
    fn test_member_new_with_time() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        
        let member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        assert_eq!(member.pubkey, pubkey);
        assert_eq!(member.status, MemberStatus::Active);
        assert_eq!(member.reputation, 100);
        assert_eq!(member.joined_at, 1000);
        assert_eq!(member.created_by, created_by);
    }

    #[test]
    fn test_member_leave_with_time() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        assert!(member.leave_with_time(2000).is_ok());
        assert_eq!(member.status, MemberStatus::Inactive);
        assert_eq!(member.last_activity, 2000);
    }

    #[test]
    fn test_member_leave_invalid_state() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        member.status = MemberStatus::Suspended;
        
        // Cannot leave if not active
        assert!(member.leave_with_time(2000).is_err());
    }

    #[test]
    fn test_member_suspend_with_time() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        assert!(member.suspend_with_time("Test reason".to_string(), 24, 2000).is_ok());
        assert_eq!(member.status, MemberStatus::Suspended);
        assert_eq!(member.suspension_reason, Some("Test reason".to_string()));
        assert!(member.suspension_until.is_some());
    }

    #[test]
    fn test_member_suspend_empty_reason() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        // Empty reason should fail
        assert!(member.suspend_with_time(String::new(), 24, 2000).is_err());
    }

    #[test]
    fn test_member_activate_with_time() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        // Suspend first
        member.suspend_with_time("Test".to_string(), 24, 2000).unwrap();
        let suspension_until = member.suspension_until.unwrap();
        
        // Activate after suspension expires
        assert!(member.activate_with_time(suspension_until + 1).is_ok());
        assert_eq!(member.status, MemberStatus::Active);
        assert_eq!(member.suspension_reason, None);
    }

    #[test]
    fn test_member_activate_before_expiry() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        member.suspend_with_time("Test".to_string(), 24, 2000).unwrap();
        let suspension_until = member.suspension_until.unwrap();
        
        // Try to activate before expiry - should fail
        assert!(member.activate_with_time(suspension_until - 1).is_err());
    }

    #[test]
    fn test_member_ban_with_time() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        assert!(member.ban_with_time("Violation".to_string(), 2000).is_ok());
        assert_eq!(member.status, MemberStatus::Banned);
        assert_eq!(member.last_activity, 2000);
    }

    #[test]
    fn test_member_update_activity_with_time() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        assert!(member.update_activity_with_time(2000).is_ok());
        assert_eq!(member.last_activity, 2000);
    }

    #[test]
    fn test_member_update_reputation() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        member.update_reputation(200);
        assert_eq!(member.reputation, 200);
    }

    #[test]
    fn test_member_new_with_time_initial_values() {
        let pubkey = create_test_pubkey(3);
        let created_by = create_test_pubkey(4);
        let member = Member::new_with_time(pubkey, created_by, 128, 5000).unwrap();
        
        assert_eq!(member.status, MemberStatus::Active);
        assert_eq!(member.reputation, 100);
        assert_eq!(member.contributions_count, 0);
        assert_eq!(member.votes_cast, 0);
        assert_eq!(member.ideas_created, 0);
        assert_eq!(member.proposals_created, 0);
        assert_eq!(member.suspension_reason, None);
        assert_eq!(member.suspension_until, None);
    }

    #[test]
    fn test_member_leave_updates_timestamp() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        assert!(member.leave_with_time(5000).is_ok());
        assert_eq!(member.last_activity, 5000);
    }

    #[test]
    fn test_member_suspend_duration_calculation() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        // Suspend for 48 hours
        assert!(member.suspend_with_time("Reason".to_string(), 48, 2000).is_ok());
        let suspension_until = member.suspension_until.unwrap();
        // 2000 + (48 * 3600) = 2000 + 172800 = 174800
        assert_eq!(suspension_until, 2000 + (48 * 3600) as i64);
    }

    #[test]
    fn test_member_suspend_invalid_state() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        member.status = MemberStatus::Suspended;
        
        // Cannot suspend if already suspended
        assert!(member.suspend_with_time("Reason".to_string(), 24, 2000).is_err());
    }

    #[test]
    fn test_member_suspend_overflow() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        // Very large duration_hours that would cause overflow
        let huge_hours = u64::MAX / 3600 + 1;
        assert!(member.suspend_with_time("Reason".to_string(), huge_hours, 2000).is_err());
    }

    #[test]
    fn test_member_activate_invalid_state() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        // Member is Active, not Suspended
        
        // Cannot activate if not suspended
        assert!(member.activate_with_time(5000).is_err());
    }

    #[test]
    fn test_member_activate_exact_expiry_time() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        member.suspend_with_time("Test".to_string(), 24, 2000).unwrap();
        let suspension_until = member.suspension_until.unwrap();
        
        // Activate exactly at expiry time - should succeed
        assert!(member.activate_with_time(suspension_until).is_ok());
        assert_eq!(member.status, MemberStatus::Active);
    }

    #[test]
    fn test_member_ban_empty_reason() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        // Empty reason should fail
        assert!(member.ban_with_time(String::new(), 2000).is_err());
    }

    #[test]
    fn test_member_ban_already_banned() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        member.status = MemberStatus::Banned;
        
        // Cannot ban if already banned
        assert!(member.ban_with_time("Reason".to_string(), 2000).is_err());
    }

    #[test]
    fn test_member_ban_clears_suspension_until() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        // First suspend
        member.suspend_with_time("Suspension".to_string(), 24, 2000).unwrap();
        assert!(member.suspension_until.is_some());
        
        // Then ban (should clear suspension_until)
        assert!(member.ban_with_time("Ban reason".to_string(), 3000).is_ok());
        assert_eq!(member.suspension_until, None);
    }

    #[test]
    fn test_member_update_activity_multiple_times() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        // Update activity multiple times
        assert!(member.update_activity_with_time(2000).is_ok());
        assert_eq!(member.last_activity, 2000);
        
        assert!(member.update_activity_with_time(3000).is_ok());
        assert_eq!(member.last_activity, 3000);
        
        assert!(member.update_activity_with_time(4000).is_ok());
        assert_eq!(member.last_activity, 4000);
    }

    #[test]
    fn test_member_suspend_sets_reason() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        let reason = "Violation of rules".to_string();
        assert!(member.suspend_with_time(reason.clone(), 48, 2000).is_ok());
        assert_eq!(member.suspension_reason, Some(reason));
    }

    #[test]
    fn test_member_suspend_zero_duration() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        // Suspend for 0 hours (should still work, but suspension_until = current_time)
        assert!(member.suspend_with_time("Reason".to_string(), 0, 2000).is_ok());
        assert_eq!(member.suspension_until, Some(2000));
    }

    #[test]
    fn test_member_suspend_one_hour() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        // Suspend for 1 hour
        assert!(member.suspend_with_time("Reason".to_string(), 1, 2000).is_ok());
        assert_eq!(member.suspension_until, Some(2000 + 3600));
    }

    #[test]
    fn test_member_suspend_max_duration() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        // Suspend for maximum safe duration (avoiding overflow)
        let max_safe_hours = (i64::MAX - 2000) / 3600;
        assert!(member.suspend_with_time("Reason".to_string(), max_safe_hours as u64, 2000).is_ok());
    }

    #[test]
    fn test_member_activate_no_suspension_until() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        member.status = MemberStatus::Suspended;
        member.suspension_until = None; // No expiration set
        
        // Should be able to activate if no expiration set
        assert!(member.activate_with_time(5000).is_ok());
        assert_eq!(member.status, MemberStatus::Active);
    }

    #[test]
    fn test_member_ban_from_suspended() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        // First suspend
        member.suspend_with_time("Suspension".to_string(), 24, 2000).unwrap();
        assert_eq!(member.status, MemberStatus::Suspended);
        
        // Then ban from suspended state
        assert!(member.ban_with_time("Ban reason".to_string(), 3000).is_ok());
        assert_eq!(member.status, MemberStatus::Banned);
    }

    #[test]
    fn test_member_ban_from_inactive() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        member.status = MemberStatus::Inactive;
        
        // Can ban from inactive state
        assert!(member.ban_with_time("Ban reason".to_string(), 3000).is_ok());
        assert_eq!(member.status, MemberStatus::Banned);
    }

    #[test]
    fn test_member_update_reputation_zero() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        member.update_reputation(0);
        assert_eq!(member.reputation, 0);
    }

    #[test]
    fn test_member_update_reputation_large() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        member.update_reputation(u64::MAX);
        assert_eq!(member.reputation, u64::MAX);
    }

    #[test]
    fn test_member_update_reputation_multiple() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        member.update_reputation(200);
        assert_eq!(member.reputation, 200);
        
        member.update_reputation(300);
        assert_eq!(member.reputation, 300);
        
        member.update_reputation(150);
        assert_eq!(member.reputation, 150);
    }

    #[test]
    fn test_member_complete_lifecycle() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        // Initial state
        assert_eq!(member.status, MemberStatus::Active);
        assert_eq!(member.reputation, 100);
        
        // Suspend
        assert!(member.suspend_with_time("Test".to_string(), 24, 2000).is_ok());
        assert_eq!(member.status, MemberStatus::Suspended);
        
        // Activate after expiry
        let suspension_until = member.suspension_until.unwrap();
        assert!(member.activate_with_time(suspension_until + 1).is_ok());
        assert_eq!(member.status, MemberStatus::Active);
        
        // Update reputation
        member.update_reputation(200);
        assert_eq!(member.reputation, 200);
        
        // Update activity
        assert!(member.update_activity_with_time(5000).is_ok());
        assert_eq!(member.last_activity, 5000);
        
        // Leave
        assert!(member.leave_with_time(6000).is_ok());
        assert_eq!(member.status, MemberStatus::Inactive);
    }

    #[test]
    fn test_member_suspend_timestamp_update() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        assert!(member.suspend_with_time("Reason".to_string(), 24, 5000).is_ok());
        assert_eq!(member.last_activity, 5000);
    }

    #[test]
    fn test_member_activate_timestamp_update() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        member.suspend_with_time("Test".to_string(), 24, 2000).unwrap();
        let suspension_until = member.suspension_until.unwrap();
        
        assert!(member.activate_with_time(suspension_until + 1).is_ok());
        assert_eq!(member.last_activity, suspension_until + 1);
    }

    #[test]
    fn test_member_ban_timestamp_update() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        assert!(member.ban_with_time("Reason".to_string(), 7000).is_ok());
        assert_eq!(member.last_activity, 7000);
    }

    #[test]
    fn test_member_new_with_time_all_fields() {
        let pubkey = create_test_pubkey(10);
        let created_by = create_test_pubkey(20);
        let member = Member::new_with_time(pubkey, created_by, 128, 5000).unwrap();
        
        assert_eq!(member.pubkey, pubkey);
        assert_eq!(member.created_by, created_by);
        assert_eq!(member.status, MemberStatus::Active);
        assert_eq!(member.reputation, 100);
        assert_eq!(member.joined_at, 5000);
        assert_eq!(member.last_activity, 5000);
        assert_eq!(member.contributions_count, 0);
        assert_eq!(member.votes_cast, 0);
        assert_eq!(member.ideas_created, 0);
        assert_eq!(member.proposals_created, 0);
        assert_eq!(member.suspension_reason, None);
        assert_eq!(member.suspension_until, None);
        assert_eq!(member.bump, 128);
    }

    #[test]
    fn test_member_leave_preserves_other_fields() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        let original_pubkey = member.pubkey;
        let original_reputation = member.reputation;
        let original_contributions = member.contributions_count;
        
        assert!(member.leave_with_time(2000).is_ok());
        
        assert_eq!(member.pubkey, original_pubkey);
        assert_eq!(member.reputation, original_reputation);
        assert_eq!(member.contributions_count, original_contributions);
        assert_eq!(member.status, MemberStatus::Inactive);
    }

    #[test]
    fn test_member_suspend_preserves_other_fields() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        let original_pubkey = member.pubkey;
        let original_reputation = member.reputation;
        let original_contributions = member.contributions_count;
        
        assert!(member.suspend_with_time("Reason".to_string(), 24, 2000).is_ok());
        
        assert_eq!(member.pubkey, original_pubkey);
        assert_eq!(member.reputation, original_reputation);
        assert_eq!(member.contributions_count, original_contributions);
        assert_eq!(member.status, MemberStatus::Suspended);
    }

    #[test]
    fn test_member_activate_preserves_other_fields() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        member.suspend_with_time("Test".to_string(), 24, 2000).unwrap();
        let suspension_until = member.suspension_until.unwrap();
        let original_pubkey = member.pubkey;
        let original_reputation = member.reputation;
        let original_contributions = member.contributions_count;
        
        assert!(member.activate_with_time(suspension_until + 1).is_ok());
        
        assert_eq!(member.pubkey, original_pubkey);
        assert_eq!(member.reputation, original_reputation);
        assert_eq!(member.contributions_count, original_contributions);
        assert_eq!(member.status, MemberStatus::Active);
        assert_eq!(member.suspension_reason, None);
        assert_eq!(member.suspension_until, None);
    }

    #[test]
    fn test_member_ban_preserves_other_fields() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        let original_pubkey = member.pubkey;
        let original_reputation = member.reputation;
        let original_contributions = member.contributions_count;
        
        assert!(member.ban_with_time("Reason".to_string(), 2000).is_ok());
        
        assert_eq!(member.pubkey, original_pubkey);
        assert_eq!(member.reputation, original_reputation);
        assert_eq!(member.contributions_count, original_contributions);
        assert_eq!(member.status, MemberStatus::Banned);
        assert_eq!(member.suspension_until, None);
    }

    #[test]
    fn test_member_update_activity_preserves_other_fields() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        let original_pubkey = member.pubkey;
        let original_reputation = member.reputation;
        
        assert!(member.update_activity_with_time(2000).is_ok());
        
        assert_eq!(member.pubkey, original_pubkey);
        assert_eq!(member.status, MemberStatus::Active); // Status unchanged
        assert_eq!(member.reputation, original_reputation);
        assert_eq!(member.last_activity, 2000);
    }

    #[test]
    fn test_member_suspend_duration_overflow_timestamp() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        
        // Duration that would cause timestamp overflow
        let huge_hours = ((i64::MAX - 2000) / 3600 + 1) as u64;
        assert!(member.suspend_with_time("Reason".to_string(), huge_hours, 2000).is_err());
    }

    #[test]
    fn test_member_ban_preserves_other_fields_comprehensive() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        let original_pubkey = member.pubkey;
        let original_reputation = member.reputation;
        let original_contributions = member.contributions_count;
        let original_bump = member.bump;
        
        assert!(member.ban_with_time("Ban reason".to_string(), 2000).is_ok());
        
        assert_eq!(member.pubkey, original_pubkey);
        assert_eq!(member.reputation, original_reputation);
        assert_eq!(member.contributions_count, original_contributions);
        assert_eq!(member.bump, original_bump);
        assert_eq!(member.status, MemberStatus::Banned);
    }

    #[test]
    fn test_member_activate_before_suspension_expires() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        member.suspend_with_time("Reason".to_string(), 24, 1000).unwrap();
        // suspension_until = 1000 + 24*3600 = 1000 + 86400 = 87400
        
        // Try to activate before suspension expires - should fail
        assert!(member.activate_with_time(50000).is_err());
        assert_eq!(member.status, MemberStatus::Suspended);
    }

    #[test]
    fn test_member_activate_exactly_when_suspension_expires() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let mut member = Member::new_with_time(pubkey, created_by, 255, 1000).unwrap();
        member.suspend_with_time("Reason".to_string(), 24, 1000).unwrap();
        let suspension_until = member.suspension_until.unwrap();
        
        // Activate exactly when suspension expires
        assert!(member.activate_with_time(suspension_until).is_ok());
        assert_eq!(member.status, MemberStatus::Active);
    }
}
