//! Member action methods

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use super::types::Member;
use crate::state::enums::MemberStatus;

impl Member {
    /// Add contribution (increases reputation by 10)
    pub fn add_contribution(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.add_contribution_with_time(current_time)
    }

    /// Add contribution with specified time
    pub fn add_contribution_with_time(&mut self, current_time: i64) -> Result<()> {
        self.contributions_count = self.contributions_count.checked_add(1)
            .ok_or(IndrasError::Overflow)?;
        self.reputation = self.reputation.checked_add(10)
            .ok_or(IndrasError::Overflow)?; // +10 reputation for contribution
        self.update_activity_with_time(current_time)?;
        Ok(())
    }

    /// Cast a vote
    pub fn cast_vote(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.cast_vote_with_time(current_time)
    }

    /// Cast a vote with specified time
    pub fn cast_vote_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(self.status == MemberStatus::Active, IndrasError::InvalidState);
        self.votes_cast = self.votes_cast.checked_add(1)
            .ok_or(IndrasError::Overflow)?;
        self.update_activity_with_time(current_time)?;
        Ok(())
    }

    /// Create an idea (increases reputation by 5)
    pub fn create_idea(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.create_idea_with_time(current_time)
    }

    /// Create an idea with specified time
    pub fn create_idea_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(self.status == MemberStatus::Active, IndrasError::InvalidState);
        self.ideas_created = self.ideas_created.checked_add(1)
            .ok_or(IndrasError::Overflow)?;
        self.reputation = self.reputation.checked_add(5)
            .ok_or(IndrasError::Overflow)?; // +5 reputation for idea
        self.update_activity_with_time(current_time)?;
        Ok(())
    }

    /// Create a proposal (increases reputation by 15)
    pub fn create_proposal(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.create_proposal_with_time(current_time)
    }

    /// Create a proposal with specified time
    pub fn create_proposal_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(self.status == MemberStatus::Active, IndrasError::InvalidState);
        self.proposals_created = self.proposals_created.checked_add(1)
            .ok_or(IndrasError::Overflow)?;
        self.reputation = self.reputation.checked_add(15)
            .ok_or(IndrasError::Overflow)?; // +15 reputation for proposal
        self.update_activity_with_time(current_time)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;
    use crate::state::enums::MemberStatus;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    fn create_test_member() -> Member {
        Member {
            pubkey: create_test_pubkey(1),
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
            created_by: create_test_pubkey(2),
            bump: 255,
        }
    }

    #[test]
    fn test_member_add_contribution_with_time() {
        let mut member = create_test_member();
        let initial_reputation = member.reputation;
        let initial_count = member.contributions_count;
        
        assert!(member.add_contribution_with_time(2000).is_ok());
        assert_eq!(member.contributions_count, initial_count + 1);
        assert_eq!(member.reputation, initial_reputation + 10);
        assert_eq!(member.last_activity, 2000);
    }

    #[test]
    fn test_member_add_contribution_overflow() {
        let mut member = create_test_member();
        member.contributions_count = u32::MAX;
        
        // Should fail on overflow
        assert!(member.add_contribution_with_time(2000).is_err());
    }

    #[test]
    fn test_member_cast_vote_with_time() {
        let mut member = create_test_member();
        let initial_votes = member.votes_cast;
        
        assert!(member.cast_vote_with_time(2000).is_ok());
        assert_eq!(member.votes_cast, initial_votes + 1);
        assert_eq!(member.last_activity, 2000);
    }

    #[test]
    fn test_member_cast_vote_inactive() {
        let mut member = create_test_member();
        member.status = MemberStatus::Suspended;
        
        // Cannot vote if not active
        assert!(member.cast_vote_with_time(2000).is_err());
    }

    #[test]
    fn test_member_create_idea_with_time() {
        let mut member = create_test_member();
        let initial_ideas = member.ideas_created;
        let initial_reputation = member.reputation;
        
        assert!(member.create_idea_with_time(2000).is_ok());
        assert_eq!(member.ideas_created, initial_ideas + 1);
        assert_eq!(member.reputation, initial_reputation + 5);
        assert_eq!(member.last_activity, 2000);
    }

    #[test]
    fn test_member_create_idea_inactive() {
        let mut member = create_test_member();
        member.status = MemberStatus::Suspended;
        
        // Cannot create idea if not active
        assert!(member.create_idea_with_time(2000).is_err());
    }

    #[test]
    fn test_member_create_proposal_with_time() {
        let mut member = create_test_member();
        let initial_proposals = member.proposals_created;
        let initial_reputation = member.reputation;
        
        assert!(member.create_proposal_with_time(2000).is_ok());
        assert_eq!(member.proposals_created, initial_proposals + 1);
        assert_eq!(member.reputation, initial_reputation + 15);
        assert_eq!(member.last_activity, 2000);
    }

    #[test]
    fn test_member_create_proposal_inactive() {
        let mut member = create_test_member();
        member.status = MemberStatus::Suspended;
        
        // Cannot create proposal if not active
        assert!(member.create_proposal_with_time(2000).is_err());
    }

    #[test]
    fn test_member_add_contribution_reputation_overflow() {
        let mut member = create_test_member();
        member.reputation = u64::MAX - 5; // Close to overflow
        
        // Should fail on reputation overflow
        assert!(member.add_contribution_with_time(2000).is_err());
    }

    #[test]
    fn test_member_add_contribution_multiple() {
        let mut member = create_test_member();
        let initial_reputation = member.reputation;
        
        // Add 3 contributions
        assert!(member.add_contribution_with_time(2000).is_ok());
        assert!(member.add_contribution_with_time(3000).is_ok());
        assert!(member.add_contribution_with_time(4000).is_ok());
        
        assert_eq!(member.contributions_count, 3);
        assert_eq!(member.reputation, initial_reputation + 30); // 3 * 10
        assert_eq!(member.last_activity, 4000);
    }

    #[test]
    fn test_member_cast_vote_overflow() {
        let mut member = create_test_member();
        member.votes_cast = u32::MAX;
        
        // Should fail on overflow
        assert!(member.cast_vote_with_time(2000).is_err());
    }

    #[test]
    fn test_member_cast_vote_multiple() {
        let mut member = create_test_member();
        
        // Cast 5 votes
        for i in 1..=5 {
            assert!(member.cast_vote_with_time(1000 + (i * 100) as i64).is_ok());
        }
        
        assert_eq!(member.votes_cast, 5);
        assert_eq!(member.last_activity, 1500);
    }

    #[test]
    fn test_member_create_idea_reputation_overflow() {
        let mut member = create_test_member();
        member.reputation = u64::MAX - 2; // Close to overflow
        
        // Should fail on reputation overflow
        assert!(member.create_idea_with_time(2000).is_err());
    }

    #[test]
    fn test_member_create_idea_multiple() {
        let mut member = create_test_member();
        let initial_reputation = member.reputation;
        
        // Create 3 ideas
        assert!(member.create_idea_with_time(2000).is_ok());
        assert!(member.create_idea_with_time(3000).is_ok());
        assert!(member.create_idea_with_time(4000).is_ok());
        
        assert_eq!(member.ideas_created, 3);
        assert_eq!(member.reputation, initial_reputation + 15); // 3 * 5
        assert_eq!(member.last_activity, 4000);
    }

    #[test]
    fn test_member_create_idea_overflow() {
        let mut member = create_test_member();
        member.ideas_created = u32::MAX;
        
        // Should fail on overflow
        assert!(member.create_idea_with_time(2000).is_err());
    }

    #[test]
    fn test_member_create_proposal_reputation_overflow() {
        let mut member = create_test_member();
        member.reputation = u64::MAX - 10; // Close to overflow
        
        // Should fail on reputation overflow
        assert!(member.create_proposal_with_time(2000).is_err());
    }

    #[test]
    fn test_member_create_proposal_multiple() {
        let mut member = create_test_member();
        let initial_reputation = member.reputation;
        
        // Create 2 proposals
        assert!(member.create_proposal_with_time(2000).is_ok());
        assert!(member.create_proposal_with_time(3000).is_ok());
        
        assert_eq!(member.proposals_created, 2);
        assert_eq!(member.reputation, initial_reputation + 30); // 2 * 15
        assert_eq!(member.last_activity, 3000);
    }

    #[test]
    fn test_member_create_proposal_overflow() {
        let mut member = create_test_member();
        member.proposals_created = u32::MAX;
        
        // Should fail on overflow
        assert!(member.create_proposal_with_time(2000).is_err());
    }

    #[test]
    fn test_member_combined_actions() {
        let mut member = create_test_member();
        let initial_reputation = member.reputation;
        
        // Perform various actions
        assert!(member.add_contribution_with_time(2000).is_ok());
        assert!(member.cast_vote_with_time(3000).is_ok());
        assert!(member.create_idea_with_time(4000).is_ok());
        assert!(member.create_proposal_with_time(5000).is_ok());
        
        assert_eq!(member.contributions_count, 1);
        assert_eq!(member.votes_cast, 1);
        assert_eq!(member.ideas_created, 1);
        assert_eq!(member.proposals_created, 1);
        assert_eq!(member.reputation, initial_reputation + 10 + 5 + 15); // 30 total
        assert_eq!(member.last_activity, 5000);
    }

    #[test]
    fn test_member_cast_vote_banned_status() {
        let mut member = create_test_member();
        member.status = MemberStatus::Banned;
        
        // Cannot vote if banned
        assert!(member.cast_vote_with_time(2000).is_err());
    }

    #[test]
    fn test_member_create_idea_banned_status() {
        let mut member = create_test_member();
        member.status = MemberStatus::Banned;
        
        // Cannot create idea if banned
        assert!(member.create_idea_with_time(2000).is_err());
    }

    #[test]
    fn test_member_add_contribution_reputation_exact_50() {
        let mut member = create_test_member();
        member.reputation = 40; // Will become 50 after contribution
        
        assert!(member.add_contribution_with_time(2000).is_ok());
        assert_eq!(member.reputation, 50);
        assert_eq!(member.contributions_count, 1);
    }

    #[test]
    fn test_member_add_contribution_reputation_exact_100() {
        let mut member = create_test_member();
        member.reputation = 90; // Will become 100 after contribution
        
        assert!(member.add_contribution_with_time(2000).is_ok());
        assert_eq!(member.reputation, 100);
    }

    #[test]
    fn test_member_create_idea_reputation_exact_100() {
        let mut member = create_test_member();
        member.reputation = 95; // Will become 100 after idea
        
        assert!(member.create_idea_with_time(2000).is_ok());
        assert_eq!(member.reputation, 100);
        assert_eq!(member.ideas_created, 1);
    }

    #[test]
    fn test_member_create_proposal_reputation_exact_100() {
        let mut member = create_test_member();
        member.reputation = 85; // Will become 100 after proposal
        
        assert!(member.create_proposal_with_time(2000).is_ok());
        assert_eq!(member.reputation, 100);
        assert_eq!(member.proposals_created, 1);
    }

    #[test]
    fn test_member_cast_vote_inactive_status() {
        let mut member = create_test_member();
        member.status = MemberStatus::Inactive;
        
        // Cannot vote if inactive
        assert!(member.cast_vote_with_time(2000).is_err());
    }

    #[test]
    fn test_member_create_idea_inactive_status() {
        let mut member = create_test_member();
        member.status = MemberStatus::Inactive;
        
        // Cannot create idea if inactive
        assert!(member.create_idea_with_time(2000).is_err());
    }

    #[test]
    fn test_member_create_proposal_inactive_status() {
        let mut member = create_test_member();
        member.status = MemberStatus::Inactive;
        
        // Cannot create proposal if inactive
        assert!(member.create_proposal_with_time(2000).is_err());
    }

    #[test]
    fn test_member_add_contribution_from_zero() {
        let mut member = create_test_member();
        member.contributions_count = 0;
        member.reputation = 0;
        
        assert!(member.add_contribution_with_time(2000).is_ok());
        assert_eq!(member.contributions_count, 1);
        assert_eq!(member.reputation, 10);
    }

    #[test]
    fn test_member_cast_vote_from_zero() {
        let mut member = create_test_member();
        member.votes_cast = 0;
        
        assert!(member.cast_vote_with_time(2000).is_ok());
        assert_eq!(member.votes_cast, 1);
    }

    #[test]
    fn test_member_create_idea_from_zero() {
        let mut member = create_test_member();
        member.ideas_created = 0;
        member.reputation = 0;
        
        assert!(member.create_idea_with_time(2000).is_ok());
        assert_eq!(member.ideas_created, 1);
        assert_eq!(member.reputation, 5);
    }

    #[test]
    fn test_member_create_proposal_from_zero() {
        let mut member = create_test_member();
        member.proposals_created = 0;
        member.reputation = 0;
        
        assert!(member.create_proposal_with_time(2000).is_ok());
        assert_eq!(member.proposals_created, 1);
        assert_eq!(member.reputation, 15);
    }

    #[test]
    fn test_member_add_contribution_activity_update() {
        let mut member = create_test_member();
        let initial_activity = member.last_activity;
        
        assert!(member.add_contribution_with_time(5000).is_ok());
        assert_eq!(member.last_activity, 5000);
        assert_ne!(member.last_activity, initial_activity);
    }

    #[test]
    fn test_member_cast_vote_activity_update() {
        let mut member = create_test_member();
        let initial_activity = member.last_activity;
        
        assert!(member.cast_vote_with_time(6000).is_ok());
        assert_eq!(member.last_activity, 6000);
        assert_ne!(member.last_activity, initial_activity);
    }

    #[test]
    fn test_member_create_idea_activity_update() {
        let mut member = create_test_member();
        let initial_activity = member.last_activity;
        
        assert!(member.create_idea_with_time(7000).is_ok());
        assert_eq!(member.last_activity, 7000);
        assert_ne!(member.last_activity, initial_activity);
    }

    #[test]
    fn test_member_create_proposal_activity_update() {
        let mut member = create_test_member();
        let initial_activity = member.last_activity;
        
        assert!(member.create_proposal_with_time(8000).is_ok());
        assert_eq!(member.last_activity, 8000);
        assert_ne!(member.last_activity, initial_activity);
    }

    #[test]
    fn test_member_add_contribution_preserves_other_fields() {
        let mut member = create_test_member();
        let original_pubkey = member.pubkey;
        let original_votes = member.votes_cast;
        let original_ideas = member.ideas_created;
        let original_proposals = member.proposals_created;
        
        assert!(member.add_contribution_with_time(2000).is_ok());
        
        assert_eq!(member.pubkey, original_pubkey);
        assert_eq!(member.status, MemberStatus::Active); // Status unchanged
        assert_eq!(member.votes_cast, original_votes);
        assert_eq!(member.ideas_created, original_ideas);
        assert_eq!(member.proposals_created, original_proposals);
    }

    #[test]
    fn test_member_cast_vote_preserves_other_fields() {
        let mut member = create_test_member();
        let original_pubkey = member.pubkey;
        let original_contributions = member.contributions_count;
        let original_ideas = member.ideas_created;
        let original_proposals = member.proposals_created;
        let original_reputation = member.reputation;
        
        assert!(member.cast_vote_with_time(2000).is_ok());
        
        assert_eq!(member.pubkey, original_pubkey);
        assert_eq!(member.status, MemberStatus::Active); // Status unchanged
        assert_eq!(member.contributions_count, original_contributions);
        assert_eq!(member.ideas_created, original_ideas);
        assert_eq!(member.proposals_created, original_proposals);
        assert_eq!(member.reputation, original_reputation);
    }

    #[test]
    fn test_member_create_idea_preserves_other_fields() {
        let mut member = create_test_member();
        let original_pubkey = member.pubkey;
        let original_contributions = member.contributions_count;
        let original_votes = member.votes_cast;
        let original_proposals = member.proposals_created;
        
        assert!(member.create_idea_with_time(2000).is_ok());
        
        assert_eq!(member.pubkey, original_pubkey);
        assert_eq!(member.status, MemberStatus::Active); // Status unchanged
        assert_eq!(member.contributions_count, original_contributions);
        assert_eq!(member.votes_cast, original_votes);
        assert_eq!(member.proposals_created, original_proposals);
    }

    #[test]
    fn test_member_create_proposal_preserves_other_fields() {
        let mut member = create_test_member();
        let original_pubkey = member.pubkey;
        let original_contributions = member.contributions_count;
        let original_votes = member.votes_cast;
        let original_ideas = member.ideas_created;
        
        assert!(member.create_proposal_with_time(2000).is_ok());
        
        assert_eq!(member.pubkey, original_pubkey);
        assert_eq!(member.status, MemberStatus::Active); // Status unchanged
        assert_eq!(member.contributions_count, original_contributions);
        assert_eq!(member.votes_cast, original_votes);
        assert_eq!(member.ideas_created, original_ideas);
    }

    #[test]
    fn test_member_add_contribution_reputation_increase() {
        let mut member = create_test_member();
        let initial_reputation = member.reputation;
        
        assert!(member.add_contribution_with_time(2000).is_ok());
        assert_eq!(member.reputation, initial_reputation + 10);
        assert_eq!(member.contributions_count, 1);
    }

    #[test]
    fn test_member_cast_vote_inactive_member() {
        let mut member = create_test_member();
        member.status = MemberStatus::Inactive;
        
        assert!(member.cast_vote_with_time(2000).is_err());
        assert_eq!(member.votes_cast, 0);
    }

    #[test]
    fn test_member_cast_vote_suspended_member() {
        let mut member = create_test_member();
        member.status = MemberStatus::Suspended;
        
        assert!(member.cast_vote_with_time(2000).is_err());
        assert_eq!(member.votes_cast, 0);
    }

    #[test]
    fn test_member_create_idea_inactive_member() {
        let mut member = create_test_member();
        member.status = MemberStatus::Inactive;
        
        assert!(member.create_idea_with_time(2000).is_err());
        assert_eq!(member.ideas_created, 0);
    }

    #[test]
    fn test_member_create_idea_ideas_count_overflow() {
        let mut member = create_test_member();
        member.ideas_created = u32::MAX;
        
        assert!(member.create_idea_with_time(2000).is_err());
    }

    #[test]
    fn test_member_create_proposal_inactive_member() {
        let mut member = create_test_member();
        member.status = MemberStatus::Inactive;
        
        assert!(member.create_proposal_with_time(2000).is_err());
        assert_eq!(member.proposals_created, 0);
    }

    #[test]
    fn test_member_create_proposal_proposals_count_overflow() {
        let mut member = create_test_member();
        member.proposals_created = u32::MAX;
        
        assert!(member.create_proposal_with_time(2000).is_err());
    }

    #[test]
    fn test_member_create_proposal_reputation_increase() {
        let mut member = create_test_member();
        let initial_reputation = member.reputation;
        
        assert!(member.create_proposal_with_time(2000).is_ok());
        assert_eq!(member.reputation, initial_reputation + 15);
        assert_eq!(member.proposals_created, 1);
    }

    #[test]
    fn test_member_add_contribution_contributions_count_overflow() {
        let mut member = create_test_member();
        member.contributions_count = u32::MAX;
        
        assert!(member.add_contribution_with_time(2000).is_err());
    }
}
