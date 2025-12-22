//! Member account structures

use anchor_lang::prelude::*;
use crate::state::enums::MemberStatus;
// NOTE: MemberRole enum removed - use MemberRole struct from member::role instead

/// Member account structure
#[account]
#[derive(InitSpace)]
pub struct Member {
    pub pubkey: Pubkey,
    pub status: MemberStatus,
    // NOTE: role field removed - use MemberRole account (member::role::MemberRole) instead
    // pub role: MemberRole, // Deprecated - use separate MemberRole account
    pub reputation: u64,              // Member reputation
    pub joined_at: i64,               // Join date
    pub last_activity: i64,           // Last activity
    pub contributions_count: u32,     // Contribution count
    pub votes_cast: u32,              // Votes cast
    pub ideas_created: u32,           // Ideas created
    pub proposals_created: u32,       // Proposals created
    #[max_len(200)]
    pub suspension_reason: Option<String>, // Suspension reason
    pub suspension_until: Option<i64>,     // Suspended until
    pub created_by: Pubkey,           // Who added member
    pub bump: u8,
}

/// Member registry account structure
#[account]
#[derive(InitSpace)]
pub struct MemberRegistry {
    pub total_members: u32,
    pub active_members: u32,
    pub suspended_members: u32,
    pub banned_members: u32,
    pub total_reputation: u64,
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    #[test]
    fn test_member_structure() {
        let pubkey = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        
        let member = Member {
            pubkey,
            status: MemberStatus::Active,
            reputation: 100,
            joined_at: 1000,
            last_activity: 2000,
            contributions_count: 5,
            votes_cast: 10,
            ideas_created: 3,
            proposals_created: 2,
            suspension_reason: None,
            suspension_until: None,
            created_by,
            bump: 255,
        };
        
        assert_eq!(member.pubkey, pubkey);
        assert_eq!(member.status, MemberStatus::Active);
        assert_eq!(member.reputation, 100);
        assert_eq!(member.joined_at, 1000);
        assert_eq!(member.last_activity, 2000);
        assert_eq!(member.contributions_count, 5);
        assert_eq!(member.votes_cast, 10);
        assert_eq!(member.ideas_created, 3);
        assert_eq!(member.proposals_created, 2);
        assert_eq!(member.suspension_reason, None);
        assert_eq!(member.suspension_until, None);
        assert_eq!(member.created_by, created_by);
        assert_eq!(member.bump, 255);
    }

    #[test]
    fn test_member_with_suspension() {
        let pubkey = create_test_pubkey(3);
        let created_by = create_test_pubkey(4);
        
        let member = Member {
            pubkey,
            status: MemberStatus::Suspended,
            reputation: 50,
            joined_at: 1000,
            last_activity: 2000,
            contributions_count: 0,
            votes_cast: 0,
            ideas_created: 0,
            proposals_created: 0,
            suspension_reason: Some("Test reason".to_string()),
            suspension_until: Some(3000),
            created_by,
            bump: 128,
        };
        
        assert_eq!(member.status, MemberStatus::Suspended);
        assert_eq!(member.suspension_reason, Some("Test reason".to_string()));
        assert_eq!(member.suspension_until, Some(3000));
    }

    #[test]
    fn test_member_registry_structure() {
        let registry = MemberRegistry {
            total_members: 100,
            active_members: 80,
            suspended_members: 15,
            banned_members: 5,
            total_reputation: 10000,
            created_at: 1000,
            updated_at: 2000,
            bump: 255,
        };
        
        assert_eq!(registry.total_members, 100);
        assert_eq!(registry.active_members, 80);
        assert_eq!(registry.suspended_members, 15);
        assert_eq!(registry.banned_members, 5);
        assert_eq!(registry.total_reputation, 10000);
        assert_eq!(registry.created_at, 1000);
        assert_eq!(registry.updated_at, 2000);
        assert_eq!(registry.bump, 255);
    }

    #[test]
    fn test_member_registry_empty() {
        let registry = MemberRegistry {
            total_members: 0,
            active_members: 0,
            suspended_members: 0,
            banned_members: 0,
            total_reputation: 0,
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };
        
        assert_eq!(registry.total_members, 0);
        assert_eq!(registry.active_members, 0);
        assert_eq!(registry.suspended_members, 0);
        assert_eq!(registry.banned_members, 0);
        assert_eq!(registry.total_reputation, 0);
    }

    #[test]
    fn test_member_registry_large_values() {
        let registry = MemberRegistry {
            total_members: u32::MAX,
            active_members: u32::MAX - 100,
            suspended_members: 50,
            banned_members: 50,
            total_reputation: u64::MAX,
            created_at: i64::MAX,
            updated_at: i64::MAX,
            bump: 255,
        };
        
        assert_eq!(registry.total_members, u32::MAX);
        assert_eq!(registry.total_reputation, u64::MAX);
        assert_eq!(registry.created_at, i64::MAX);
    }

    #[test]
    fn test_member_all_statuses() {
        let pubkey = create_test_pubkey(5);
        let created_by = create_test_pubkey(6);
        
        let statuses = vec![
            MemberStatus::Active,
            MemberStatus::Inactive,
            MemberStatus::Suspended,
            MemberStatus::Banned,
        ];
        
        for status in &statuses {
            let member = Member {
                pubkey,
                status: status.clone(), // MemberStatus implements Clone
                reputation: 100,
                joined_at: 1000,
                last_activity: 2000,
                contributions_count: 0,
                votes_cast: 0,
                ideas_created: 0,
                proposals_created: 0,
                suspension_reason: None,
                suspension_until: None,
                created_by,
                bump: 255,
            };
            
            assert_eq!(member.status, status.clone());
        }
    }

    #[test]
    fn test_member_suspension_reason_max_length() {
        let pubkey = create_test_pubkey(7);
        let created_by = create_test_pubkey(8);
        
        // Test with long suspension reason (max 200 chars)
        let long_reason = "a".repeat(200);
        let member = Member {
            pubkey,
            status: MemberStatus::Suspended,
            reputation: 0,
            joined_at: 1000,
            last_activity: 2000,
            contributions_count: 0,
            votes_cast: 0,
            ideas_created: 0,
            proposals_created: 0,
            suspension_reason: Some(long_reason.clone()),
            suspension_until: Some(3000),
            created_by,
            bump: 255,
        };
        
        assert_eq!(member.suspension_reason, Some(long_reason));
    }

    #[test]
    fn test_member_initial_counters() {
        let pubkey = create_test_pubkey(9);
        let created_by = create_test_pubkey(10);
        
        let member = Member {
            pubkey,
            status: MemberStatus::Active,
            reputation: 100,
            joined_at: 1000,
            last_activity: 1000,
            contributions_count: 0,
            votes_cast: 0,
            ideas_created: 0,
            proposals_created: 0,
            suspension_reason: None,
            suspension_until: None,
            created_by,
            bump: 255,
        };
        
        assert_eq!(member.contributions_count, 0);
        assert_eq!(member.votes_cast, 0);
        assert_eq!(member.ideas_created, 0);
        assert_eq!(member.proposals_created, 0);
    }

    #[test]
    fn test_member_registry_sum_consistency() {
        // Test that total_members >= active + suspended + banned
        let registry = MemberRegistry {
            total_members: 100,
            active_members: 80,
            suspended_members: 15,
            banned_members: 5,
            total_reputation: 0,
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };
        
        let sum = registry.active_members as u64 
            + registry.suspended_members as u64 
            + registry.banned_members as u64;
        
        // Total should be at least the sum (some members might be inactive)
        assert!(registry.total_members as u64 >= sum);
    }

    #[test]
    fn test_member_all_fields() {
        let pubkey = create_test_pubkey(11);
        let created_by = create_test_pubkey(12);
        
        let member = Member {
            pubkey,
            status: MemberStatus::Active,
            reputation: 500,
            joined_at: 1000,
            last_activity: 2000,
            contributions_count: 10,
            votes_cast: 20,
            ideas_created: 5,
            proposals_created: 3,
            suspension_reason: None,
            suspension_until: None,
            created_by,
            bump: 128,
        };
        
        assert_eq!(member.pubkey, pubkey);
        assert_eq!(member.status, MemberStatus::Active);
        assert_eq!(member.reputation, 500);
        assert_eq!(member.joined_at, 1000);
        assert_eq!(member.last_activity, 2000);
        assert_eq!(member.contributions_count, 10);
        assert_eq!(member.votes_cast, 20);
        assert_eq!(member.ideas_created, 5);
        assert_eq!(member.proposals_created, 3);
        assert_eq!(member.created_by, created_by);
        assert_eq!(member.bump, 128);
    }

    #[test]
    fn test_member_registry_all_fields() {
        let registry = MemberRegistry {
            total_members: 200,
            active_members: 150,
            suspended_members: 30,
            banned_members: 20,
            total_reputation: 50000,
            created_at: 1000,
            updated_at: 2000,
            bump: 128,
        };
        
        assert_eq!(registry.total_members, 200);
        assert_eq!(registry.active_members, 150);
        assert_eq!(registry.suspended_members, 30);
        assert_eq!(registry.banned_members, 20);
        assert_eq!(registry.total_reputation, 50000);
        assert_eq!(registry.created_at, 1000);
        assert_eq!(registry.updated_at, 2000);
        assert_eq!(registry.bump, 128);
    }
}
