//! Group Member History
//!
//! SEC-INV-15: Tracks former members for cooldown period enforcement
//! Used to prevent Sybil attacks by requiring cooldown before rejoining

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Group Member History Entry
/// 
/// Tracks when a member left a group for cooldown enforcement
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, InitSpace)]
pub struct MemberHistoryEntry {
    pub member_pubkey: Pubkey,
    pub left_at: i64,
    pub reason: MemberLeaveReason,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub enum MemberLeaveReason {
    Removed,      // Removed by leader
    Left,         // Voluntarily left
    Disbanded,    // Group was disbanded
}

/// Group Member History
/// 
/// PDA: [b"member_history", mesh_group.key()]
/// Tracks former members to enforce cooldown periods (SEC-INV-15)
#[account]
#[derive(InitSpace)]
pub struct GroupMemberHistory {
    pub mesh_group: Pubkey,
    #[max_len(100)]
    pub entries: Vec<MemberHistoryEntry>,
    pub bump: u8,
}

impl GroupMemberHistory {
    /// Add a member to history (when they leave)
    pub fn add_member_exit(
        &mut self,
        member_pubkey: Pubkey,
        current_time: i64,
        reason: MemberLeaveReason,
    ) -> Result<()> {
        // Remove any existing entry for this member (update timestamp)
        self.entries.retain(|e| e.member_pubkey != member_pubkey);
        
        // Add new entry
        self.entries.push(MemberHistoryEntry {
            member_pubkey,
            left_at: current_time,
            reason,
        });
        
        Ok(())
    }
    
    /// Check if member can rejoin (cooldown period passed)
    /// 
    /// SEC-INV-15: Returns Ok(()) if cooldown passed, Err(CooldownPeriodActive) if not
    pub fn check_cooldown(
        &self,
        member_pubkey: &Pubkey,
        current_time: i64,
        cooldown_days: u16,
    ) -> Result<()> {
        if let Some(entry) = self.entries.iter().find(|e| e.member_pubkey == *member_pubkey) {
            let days_since_left = (current_time - entry.left_at) / (24 * 60 * 60);
            require!(
                days_since_left >= cooldown_days as i64,
                IndrasError::CooldownPeriodActive
            );
        }
        Ok(())
    }
    
    /// Get when member left (if exists)
    pub fn get_member_left_at(&self, member_pubkey: &Pubkey) -> Option<i64> {
        self.entries
            .iter()
            .find(|e| e.member_pubkey == *member_pubkey)
            .map(|e| e.left_at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    fn create_test_history() -> GroupMemberHistory {
        GroupMemberHistory {
            mesh_group: create_test_pubkey(1),
            entries: Vec::new(),
            bump: 255,
        }
    }

    #[test]
    fn test_group_member_history_add_member_exit() {
        let mut history = create_test_history();
        let member = create_test_pubkey(2);
        
        assert!(history.add_member_exit(member, 1000, MemberLeaveReason::Left).is_ok());
        assert_eq!(history.entries.len(), 1);
        assert_eq!(history.entries[0].member_pubkey, member);
        assert_eq!(history.entries[0].left_at, 1000);
        assert_eq!(history.entries[0].reason, MemberLeaveReason::Left);
    }

    #[test]
    fn test_group_member_history_add_member_exit_update_existing() {
        let mut history = create_test_history();
        let member = create_test_pubkey(2);
        
        history.add_member_exit(member, 1000, MemberLeaveReason::Left).unwrap();
        assert_eq!(history.entries.len(), 1);
        
        // Add again - should update existing entry
        history.add_member_exit(member, 2000, MemberLeaveReason::Removed).unwrap();
        assert_eq!(history.entries.len(), 1); // Still 1 entry
        assert_eq!(history.entries[0].left_at, 2000);
        assert_eq!(history.entries[0].reason, MemberLeaveReason::Removed);
    }

    #[test]
    fn test_group_member_history_check_cooldown_passed() {
        let mut history = create_test_history();
        let member = create_test_pubkey(2);
        
        // Member left 100 days ago, cooldown is 30 days
        history.add_member_exit(member, 1000, MemberLeaveReason::Left).unwrap();
        let current_time = 1000 + 100 * 24 * 60 * 60;
        
        assert!(history.check_cooldown(&member, current_time, 30).is_ok());
    }

    #[test]
    fn test_group_member_history_check_cooldown_active() {
        let mut history = create_test_history();
        let member = create_test_pubkey(2);
        
        // Member left 10 days ago, cooldown is 30 days
        history.add_member_exit(member, 1000, MemberLeaveReason::Left).unwrap();
        let current_time = 1000 + 10 * 24 * 60 * 60;
        
        assert!(history.check_cooldown(&member, current_time, 30).is_err());
    }

    #[test]
    fn test_group_member_history_check_cooldown_no_entry() {
        let history = create_test_history();
        let member = create_test_pubkey(2);
        
        // Member never left - should pass
        assert!(history.check_cooldown(&member, 1000, 30).is_ok());
    }

    #[test]
    fn test_group_member_history_get_member_left_at() {
        let mut history = create_test_history();
        let member = create_test_pubkey(2);
        
        history.add_member_exit(member, 1000, MemberLeaveReason::Left).unwrap();
        
        assert_eq!(history.get_member_left_at(&member), Some(1000));
        
        let other_member = create_test_pubkey(3);
        assert_eq!(history.get_member_left_at(&other_member), None);
    }

    #[test]
    fn test_member_leave_reason_variants() {
        assert_eq!(MemberLeaveReason::Removed, MemberLeaveReason::Removed);
        assert_eq!(MemberLeaveReason::Left, MemberLeaveReason::Left);
        assert_eq!(MemberLeaveReason::Disbanded, MemberLeaveReason::Disbanded);
    }

    #[test]
    fn test_member_leave_reason_all_variants_unique() {
        let reasons = vec![
            MemberLeaveReason::Removed,
            MemberLeaveReason::Left,
            MemberLeaveReason::Disbanded,
        ];
        
        for i in 0..reasons.len() {
            for j in (i + 1)..reasons.len() {
                assert_ne!(reasons[i], reasons[j], "Duplicate reason found");
            }
        }
    }

    #[test]
    fn test_group_member_history_check_cooldown_exact_threshold() {
        let mut history = create_test_history();
        let member = create_test_pubkey(2);
        
        // Member left exactly 30 days ago, cooldown is 30 days
        history.add_member_exit(member, 1000, MemberLeaveReason::Left).unwrap();
        let current_time = 1000 + 30 * 24 * 60 * 60;
        
        assert!(history.check_cooldown(&member, current_time, 30).is_ok());
    }

    #[test]
    fn test_group_member_history_check_cooldown_one_day_before() {
        let mut history = create_test_history();
        let member = create_test_pubkey(2);
        
        // Member left 29 days ago, cooldown is 30 days
        history.add_member_exit(member, 1000, MemberLeaveReason::Left).unwrap();
        let current_time = 1000 + 29 * 24 * 60 * 60;
        
        assert!(history.check_cooldown(&member, current_time, 30).is_err());
    }

    #[test]
    fn test_group_member_history_check_cooldown_all_reasons() {
        let reasons = vec![
            MemberLeaveReason::Removed,
            MemberLeaveReason::Left,
            MemberLeaveReason::Disbanded,
        ];
        
        for reason in reasons {
            let mut history = create_test_history();
            let member = create_test_pubkey(2);
            
            history.add_member_exit(member, 1000, reason).unwrap();
            let current_time = 1000 + 100 * 24 * 60 * 60;
            
            // All reasons should work the same for cooldown
            assert!(history.check_cooldown(&member, current_time, 30).is_ok());
        }
    }

    #[test]
    fn test_group_member_history_add_multiple_members() {
        let mut history = create_test_history();
        let member1 = create_test_pubkey(2);
        let member2 = create_test_pubkey(3);
        let member3 = create_test_pubkey(4);
        
        history.add_member_exit(member1, 1000, MemberLeaveReason::Left).unwrap();
        history.add_member_exit(member2, 2000, MemberLeaveReason::Removed).unwrap();
        history.add_member_exit(member3, 3000, MemberLeaveReason::Disbanded).unwrap();
        
        assert_eq!(history.entries.len(), 3);
        assert_eq!(history.get_member_left_at(&member1), Some(1000));
        assert_eq!(history.get_member_left_at(&member2), Some(2000));
        assert_eq!(history.get_member_left_at(&member3), Some(3000));
    }

    #[test]
    fn test_group_member_history_get_member_left_at_multiple_entries() {
        let mut history = create_test_history();
        let member1 = create_test_pubkey(2);
        let member2 = create_test_pubkey(3);
        
        history.add_member_exit(member1, 1000, MemberLeaveReason::Left).unwrap();
        history.add_member_exit(member2, 2000, MemberLeaveReason::Removed).unwrap();
        
        assert_eq!(history.get_member_left_at(&member1), Some(1000));
        assert_eq!(history.get_member_left_at(&member2), Some(2000));
        assert_eq!(history.get_member_left_at(&create_test_pubkey(99)), None);
    }

    #[test]
    fn test_member_history_entry_structure() {
        let member = create_test_pubkey(5);
        let entry = MemberHistoryEntry {
            member_pubkey: member,
            left_at: 5000,
            reason: MemberLeaveReason::Removed,
        };
        
        assert_eq!(entry.member_pubkey, member);
        assert_eq!(entry.left_at, 5000);
        assert_eq!(entry.reason, MemberLeaveReason::Removed);
    }

    #[test]
    fn test_group_member_history_structure() {
        let mesh_group = create_test_pubkey(1);
        let history = GroupMemberHistory {
            mesh_group,
            entries: vec![],
            bump: 128,
        };
        
        assert_eq!(history.mesh_group, mesh_group);
        assert_eq!(history.entries.len(), 0);
        assert_eq!(history.bump, 128);
    }

    #[test]
    fn test_member_leave_reason_equality() {
        assert_eq!(MemberLeaveReason::Removed, MemberLeaveReason::Removed);
        assert_ne!(MemberLeaveReason::Removed, MemberLeaveReason::Left);
        assert_eq!(MemberLeaveReason::Left, MemberLeaveReason::Left);
        assert_ne!(MemberLeaveReason::Left, MemberLeaveReason::Disbanded);
        assert_eq!(MemberLeaveReason::Disbanded, MemberLeaveReason::Disbanded);
    }

    #[test]
    fn test_member_leave_reason_copy() {
        let reason1 = MemberLeaveReason::Removed;
        let reason2 = reason1; // Copy trait
        assert_eq!(reason1, reason2);
    }

    #[test]
    fn test_group_member_history_check_cooldown_zero_days() {
        let mut history = create_test_history();
        let member = create_test_pubkey(2);
        
        history.add_member_exit(member, 1000, MemberLeaveReason::Left).unwrap();
        
        // Zero cooldown = should always pass
        assert!(history.check_cooldown(&member, 1000, 0).is_ok());
        assert!(history.check_cooldown(&member, 1001, 0).is_ok());
    }

    #[test]
    fn test_group_member_history_check_cooldown_negative_time() {
        let mut history = create_test_history();
        let member = create_test_pubkey(2);
        
        history.add_member_exit(member, 2000, MemberLeaveReason::Left).unwrap();
        
        // Current time before left_at (negative days) = should fail
        assert!(history.check_cooldown(&member, 1000, 30).is_err());
    }

    #[test]
    fn test_member_leave_reason_space() {
        assert_eq!(<MemberLeaveReason as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_group_member_history_add_member_exit_preserves_other_fields() {
        let mut history = create_test_history();
        let original_mesh_group = history.mesh_group;
        let original_bump = history.bump;
        let member = create_test_pubkey(2);
        
        assert!(history.add_member_exit(member, 1000, MemberLeaveReason::Left).is_ok());
        
        assert_eq!(history.mesh_group, original_mesh_group);
        assert_eq!(history.bump, original_bump);
    }

    #[test]
    fn test_group_member_history_check_cooldown_large_cooldown() {
        let mut history = create_test_history();
        let member = create_test_pubkey(2);
        
        history.add_member_exit(member, 1000, MemberLeaveReason::Left).unwrap();
        
        // Very large cooldown (1000 days) - should fail
        let current_time = 1000 + 100 * 24 * 60 * 60; // 100 days later
        assert!(history.check_cooldown(&member, current_time, 1000).is_err());
    }

    #[test]
    fn test_group_member_history_multiple_exits_same_member() {
        let mut history = create_test_history();
        let member = create_test_pubkey(2);
        
        // First exit
        history.add_member_exit(member, 1000, MemberLeaveReason::Left).unwrap();
        assert_eq!(history.entries.len(), 1);
        
        // Second exit (should update, not add)
        history.add_member_exit(member, 2000, MemberLeaveReason::Removed).unwrap();
        assert_eq!(history.entries.len(), 1);
        assert_eq!(history.entries[0].left_at, 2000);
        assert_eq!(history.entries[0].reason, MemberLeaveReason::Removed);
    }

    #[test]
    fn test_member_history_entry_all_fields() {
        let member = create_test_pubkey(10);
        let entry = MemberHistoryEntry {
            member_pubkey: member,
            left_at: 5000,
            reason: MemberLeaveReason::Disbanded,
        };
        
        assert_eq!(entry.member_pubkey, member);
        assert_eq!(entry.left_at, 5000);
        assert_eq!(entry.reason, MemberLeaveReason::Disbanded);
    }
}
