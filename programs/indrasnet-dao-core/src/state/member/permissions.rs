//! Member permission check methods

use super::types::Member;
use crate::state::enums::MemberStatus;

impl Member {
    /// Check if member can vote (active and reputation >= 50)
    pub fn can_vote(&self) -> bool {
        self.status == MemberStatus::Active && self.reputation >= 50
    }

    /// Check if member can propose (active and reputation >= 100)
    pub fn can_propose(&self) -> bool {
        self.status == MemberStatus::Active && self.reputation >= 100
    }

    /// Check if member can create ideas (active)
    pub fn can_create_ideas(&self) -> bool {
        self.status == MemberStatus::Active
    }

    /// Check if member is suspended
    pub fn is_suspended(&self) -> bool {
        self.status == MemberStatus::Suspended
    }

    /// Check if member is banned
    pub fn is_banned(&self) -> bool {
        self.status == MemberStatus::Banned
    }

    /// Check if member is active
    pub fn is_active(&self) -> bool {
        self.status == MemberStatus::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_member(status: MemberStatus, reputation: u64) -> Member {
        Member {
            pubkey: Pubkey::new_unique(),
            status,
            reputation,
            joined_at: 1000,
            last_activity: 1000,
            contributions_count: 0,
            votes_cast: 0,
            ideas_created: 0,
            proposals_created: 0,
            suspension_reason: None,
            suspension_until: None,
            created_by: Pubkey::new_unique(),
            bump: 255,
        }
    }

    #[test]
    fn test_member_can_vote_active_high_reputation() {
        let member = create_test_member(MemberStatus::Active, 100);
        assert!(member.can_vote());
    }

    #[test]
    fn test_member_can_vote_active_low_reputation() {
        let member = create_test_member(MemberStatus::Active, 30);
        assert!(!member.can_vote());
    }

    #[test]
    fn test_member_can_vote_suspended() {
        let member = create_test_member(MemberStatus::Suspended, 100);
        assert!(!member.can_vote());
    }

    #[test]
    fn test_member_can_propose_active_high_reputation() {
        let member = create_test_member(MemberStatus::Active, 150);
        assert!(member.can_propose());
    }

    #[test]
    fn test_member_can_propose_active_low_reputation() {
        let member = create_test_member(MemberStatus::Active, 50);
        assert!(!member.can_propose());
    }

    #[test]
    fn test_member_can_create_ideas_active() {
        let member = create_test_member(MemberStatus::Active, 10);
        assert!(member.can_create_ideas());
    }

    #[test]
    fn test_member_can_create_ideas_suspended() {
        let member = create_test_member(MemberStatus::Suspended, 100);
        assert!(!member.can_create_ideas());
    }

    #[test]
    fn test_member_is_suspended() {
        let member = create_test_member(MemberStatus::Suspended, 100);
        assert!(member.is_suspended());
        assert!(!member.is_banned());
        assert!(!member.is_active());
    }

    #[test]
    fn test_member_is_banned() {
        let member = create_test_member(MemberStatus::Banned, 100);
        assert!(member.is_banned());
        assert!(!member.is_suspended());
        assert!(!member.is_active());
    }

    #[test]
    fn test_member_is_active() {
        let member = create_test_member(MemberStatus::Active, 100);
        assert!(member.is_active());
        assert!(!member.is_suspended());
        assert!(!member.is_banned());
    }

    #[test]
    fn test_member_can_vote_exact_threshold() {
        let member = create_test_member(MemberStatus::Active, 50);
        assert!(member.can_vote()); // Exactly 50, should be >= 50
    }

    #[test]
    fn test_member_can_vote_just_below_threshold() {
        let member = create_test_member(MemberStatus::Active, 49);
        assert!(!member.can_vote()); // Just below 50
    }

    #[test]
    fn test_member_can_propose_exact_threshold() {
        let member = create_test_member(MemberStatus::Active, 100);
        assert!(member.can_propose()); // Exactly 100, should be >= 100
    }

    #[test]
    fn test_member_can_propose_just_below_threshold() {
        let member = create_test_member(MemberStatus::Active, 99);
        assert!(!member.can_propose()); // Just below 100
    }

    #[test]
    fn test_member_can_create_ideas_inactive() {
        let member = create_test_member(MemberStatus::Inactive, 100);
        assert!(!member.can_create_ideas());
    }

    #[test]
    fn test_member_can_create_ideas_banned() {
        let member = create_test_member(MemberStatus::Banned, 100);
        assert!(!member.can_create_ideas());
    }

    #[test]
    fn test_member_can_vote_banned() {
        let member = create_test_member(MemberStatus::Banned, 100);
        assert!(!member.can_vote());
    }

    #[test]
    fn test_member_can_propose_banned() {
        let member = create_test_member(MemberStatus::Banned, 150);
        assert!(!member.can_propose());
    }

    #[test]
    fn test_member_can_vote_inactive() {
        let member = create_test_member(MemberStatus::Inactive, 100);
        assert!(!member.can_vote());
    }

    #[test]
    fn test_member_can_propose_inactive() {
        let member = create_test_member(MemberStatus::Inactive, 150);
        assert!(!member.can_propose());
    }

    #[test]
    fn test_member_status_checks_comprehensive() {
        // Test all status checks for Active member
        let active_member = create_test_member(MemberStatus::Active, 100);
        assert!(active_member.is_active());
        assert!(!active_member.is_suspended());
        assert!(!active_member.is_banned());
        assert!(active_member.can_vote());
        assert!(active_member.can_propose());
        assert!(active_member.can_create_ideas());
        
        // Test all status checks for Suspended member
        let suspended_member = create_test_member(MemberStatus::Suspended, 100);
        assert!(!suspended_member.is_active());
        assert!(suspended_member.is_suspended());
        assert!(!suspended_member.is_banned());
        assert!(!suspended_member.can_vote());
        assert!(!suspended_member.can_propose());
        assert!(!suspended_member.can_create_ideas());
        
        // Test all status checks for Banned member
        let banned_member = create_test_member(MemberStatus::Banned, 100);
        assert!(!banned_member.is_active());
        assert!(!banned_member.is_suspended());
        assert!(banned_member.is_banned());
        assert!(!banned_member.can_vote());
        assert!(!banned_member.can_propose());
        assert!(!banned_member.can_create_ideas());
    }

    #[test]
    fn test_member_can_vote_all_statuses() {
        // Test Active
        let member = create_test_member(MemberStatus::Active, 100);
        assert!(member.can_vote());
        
        // Test Inactive
        let member = create_test_member(MemberStatus::Inactive, 100);
        assert!(!member.can_vote());
        
        // Test Suspended
        let member = create_test_member(MemberStatus::Suspended, 100);
        assert!(!member.can_vote());
        
        // Test Banned
        let member = create_test_member(MemberStatus::Banned, 100);
        assert!(!member.can_vote());
    }

    #[test]
    fn test_member_can_propose_all_statuses() {
        // Test Active
        let member = create_test_member(MemberStatus::Active, 150);
        assert!(member.can_propose());
        
        // Test Inactive
        let member = create_test_member(MemberStatus::Inactive, 150);
        assert!(!member.can_propose());
        
        // Test Suspended
        let member = create_test_member(MemberStatus::Suspended, 150);
        assert!(!member.can_propose());
        
        // Test Banned
        let member = create_test_member(MemberStatus::Banned, 150);
        assert!(!member.can_propose());
    }

    #[test]
    fn test_member_can_create_ideas_all_statuses() {
        // Test Active
        let member = create_test_member(MemberStatus::Active, 100);
        assert!(member.can_create_ideas());
        
        // Test Inactive
        let member = create_test_member(MemberStatus::Inactive, 100);
        assert!(!member.can_create_ideas());
        
        // Test Suspended
        let member = create_test_member(MemberStatus::Suspended, 100);
        assert!(!member.can_create_ideas());
        
        // Test Banned
        let member = create_test_member(MemberStatus::Banned, 100);
        assert!(!member.can_create_ideas());
    }
}
