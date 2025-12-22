//! Proposal lifecycle methods

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use super::types::{Proposal, ProposalStatus};

impl Proposal {
    /// Create a new proposal with current time
    pub fn new(
        id: u64,
        title: String,
        description: String,
        proposal_type: String,
        author: Pubkey,
        bump: u8,
    ) -> Result<Self> {
        Self::new_with_time(id, title, description, proposal_type, author, bump, Clock::get()?.unix_timestamp)
    }

    /// Create a new proposal with specified time
    pub fn new_with_time(
        id: u64,
        title: String,
        description: String,
        proposal_type: String,
        author: Pubkey,
        bump: u8,
        current_time: i64,
    ) -> Result<Self> {
        require!(!title.is_empty(), IndrasError::InvalidInput);
        require!(title.len() <= 200, IndrasError::InvalidInput);
        require!(!description.is_empty(), IndrasError::InvalidInput);
        require!(description.len() <= 2000, IndrasError::InvalidInput);
        require!(!proposal_type.is_empty(), IndrasError::InvalidInput);
        require!(proposal_type.len() <= 50, IndrasError::InvalidInput);

        Ok(Self {
            id,
            title,
            description,
            proposal_type,
            author,
            created_at: current_time,
            updated_at: None,
            submitted_at: None,
            cancelled_at: None,
            executed_at: None,
            archived_at: None,
            voting_duration: 7 * 24 * 3600, // 7 days default
            status: ProposalStatus::Draft,
            bump,
            yes_votes: 0,
            no_votes: 0,
            total_votes: 0,
            last_tallied_at: None,
            cancellation_reason: None,
            execution_data: None,
            expires_at: None,
            idea_id: None,
            treasury_operation: None,
        })
    }

    /// Activate proposal (move from Draft to Active)
    pub fn activate(&mut self, min_quorum: u64, total_members: u64) -> Result<()> {
        self.activate_with_time(min_quorum, total_members, Clock::get()?.unix_timestamp)
    }

    /// Activate proposal with specified time
    pub fn activate_with_time(&mut self, min_quorum: u64, total_members: u64, current_time: i64) -> Result<()> {
        require!(self.status == ProposalStatus::Draft, IndrasError::InvalidInput);
        require!(total_members >= min_quorum, IndrasError::InsufficientMembers);
        require!(min_quorum > 0, IndrasError::InvalidInput);
        require!(total_members > 0, IndrasError::InvalidInput);
        
        self.status = ProposalStatus::Active;
        self.submitted_at = Some(current_time);
        Ok(())
    }

    /// Pass proposal (move from Active to Passed)
    pub fn pass(&mut self) -> Result<()> {
        self.pass_with_time(Clock::get()?.unix_timestamp)
    }

    /// Pass proposal with specified time
    pub fn pass_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(self.status == ProposalStatus::Active, IndrasError::InvalidInput);
        
        // Check that voting is completed
        let voting_end = self.created_at + self.voting_duration;
        require!(current_time >= voting_end, IndrasError::VotingNotActive);
        
        self.status = ProposalStatus::Passed;
        Ok(())
    }

    /// Reject proposal (move from Active to Rejected)
    pub fn reject(&mut self) -> Result<()> {
        self.reject_with_time(Clock::get()?.unix_timestamp)
    }

    /// Reject proposal with specified time
    pub fn reject_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(self.status == ProposalStatus::Active, IndrasError::InvalidInput);
        
        // Check that voting is completed
        let voting_end = self.created_at + self.voting_duration;
        require!(current_time >= voting_end, IndrasError::VotingNotActive);
        
        self.status = ProposalStatus::Rejected;
        Ok(())
    }

    /// Execute proposal (move from Passed to Executed)
    pub fn execute(&mut self) -> Result<()> {
        self.execute_with_time(Clock::get()?.unix_timestamp)
    }

    /// Execute proposal with specified time
    pub fn execute_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(self.status == ProposalStatus::Passed, IndrasError::InvalidInput);
        require!(self.executed_at.is_none(), IndrasError::InvalidState);
        
        self.status = ProposalStatus::Executed;
        self.executed_at = Some(current_time);
        Ok(())
    }

    /// Cancel proposal (move from Draft or Active to Cancelled)
    pub fn cancel(&mut self, reason: String) -> Result<()> {
        self.cancel_with_time(reason, Clock::get()?.unix_timestamp)
    }

    /// Cancel proposal with specified time
    pub fn cancel_with_time(&mut self, reason: String, current_time: i64) -> Result<()> {
        require!(
            self.status == ProposalStatus::Draft || 
            self.status == ProposalStatus::Active,
            IndrasError::InvalidInput
        );
        self.status = ProposalStatus::Cancelled;
        self.cancelled_at = Some(current_time);
        self.cancellation_reason = Some(reason);
        Ok(())
    }

    /// Archive proposal (move from Executed, Rejected, or Cancelled to Archived)
    pub fn archive(&mut self) -> Result<()> {
        self.archive_with_time(Clock::get()?.unix_timestamp)
    }

    /// Archive proposal with specified time
    pub fn archive_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(
            self.status == ProposalStatus::Executed ||
            self.status == ProposalStatus::Rejected ||
            self.status == ProposalStatus::Cancelled,
            IndrasError::InvalidInput
        );
        self.status = ProposalStatus::Archived;
        self.archived_at = Some(current_time);
        Ok(())
    }

    /// Check if proposal has expired and auto-archive if needed
    /// Returns true if proposal was archived, false otherwise
    pub fn check_and_auto_archive(&mut self, current_time: i64) -> Result<bool> {
        if let Some(expires_at) = self.expires_at {
            if current_time >= expires_at {
                // Only auto-archive if in a finalizable state
                if self.status == ProposalStatus::Executed ||
                   self.status == ProposalStatus::Rejected ||
                   self.status == ProposalStatus::Cancelled {
                    self.archive_with_time(current_time)?;
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Set expiration time for proposal
    pub fn set_expiration(&mut self, expires_at: Option<i64>) -> Result<()> {
        if let Some(exp) = expires_at {
            require!(exp > self.created_at, IndrasError::InvalidInput);
        }
        self.expires_at = expires_at;
        Ok(())
    }

    /// Automatically transition Active proposal to Passed/Rejected based on votes
    /// This checks voting period end and vote counts
    pub fn auto_transition_after_voting(&mut self, current_time: i64) -> Result<bool> {
        if self.status != ProposalStatus::Active {
            return Ok(false);
        }

        // Check if voting period has ended
        // Use submitted_at if available (when proposal was activated), otherwise created_at
        let voting_start = self.submitted_at.unwrap_or(self.created_at);
        let voting_end = voting_start
            .checked_add(self.voting_duration)
            .ok_or(IndrasError::Overflow)?;

        if current_time >= voting_end {
            // Determine result based on votes
            if self.yes_votes > self.no_votes {
                self.pass_with_time(current_time)?;
                return Ok(true);
            } else if self.no_votes > self.yes_votes {
                self.reject_with_time(current_time)?;
                return Ok(true);
            } else {
                // Tied - set status to Tied
                self.status = ProposalStatus::Tied;
                self.last_tallied_at = Some(current_time);
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Check if proposal can be auto-activated (for future use)
    /// Currently returns false - activation requires manual call
    pub fn can_auto_activate(&self) -> bool {
        // Future: could check conditions like minimum support, time since creation, etc.
        false
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
    fn test_proposal_new_with_time() {
        let author = create_test_pubkey(1);
        let proposal = Proposal::new_with_time(
            1,
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        assert_eq!(proposal.id, 1);
        assert_eq!(proposal.title, "Test Proposal");
        assert_eq!(proposal.author, author);
        assert_eq!(proposal.status, ProposalStatus::Draft);
        assert_eq!(proposal.created_at, 1000);
    }

    #[test]
    fn test_proposal_new_validation_empty_title() {
        let author = create_test_pubkey(1);
        let result = Proposal::new_with_time(
            1,
            String::new(), // Invalid: empty
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_proposal_activate_with_time() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        assert!(proposal.activate_with_time(10, 20, 2000).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Active);
        assert_eq!(proposal.submitted_at, Some(2000));
    }

    #[test]
    fn test_proposal_activate_insufficient_members() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        // total_members < min_quorum
        assert!(proposal.activate_with_time(20, 10, 2000).is_err());
    }

    #[test]
    fn test_proposal_pass_with_time() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        
        // Pass after voting duration
        let voting_end = proposal.created_at + proposal.voting_duration;
        assert!(proposal.pass_with_time(voting_end + 1).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Passed);
    }

    #[test]
    fn test_proposal_pass_before_voting_end() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        
        // Try to pass before voting ends - should fail
        assert!(proposal.pass_with_time(2000).is_err());
    }

    #[test]
    fn test_proposal_reject_with_time() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        
        assert!(proposal.reject_with_time(voting_end + 1).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Rejected);
    }

    #[test]
    fn test_proposal_cancel_with_time() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        
        assert!(proposal.cancel_with_time("Changed mind".to_string(), 3000).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Cancelled);
        assert_eq!(proposal.cancellation_reason, Some("Changed mind".to_string()));
        assert_eq!(proposal.cancelled_at, Some(3000));
    }

    #[test]
    fn test_proposal_execute_with_time() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.pass_with_time(voting_end + 1).unwrap();
        
        assert!(proposal.execute_with_time(5000).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Executed);
        assert_eq!(proposal.executed_at, Some(5000));
        // execution_data is set separately in real usage
    }

    #[test]
    fn test_proposal_archive_with_time_executed() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.pass_with_time(voting_end + 1).unwrap();
        proposal.execute_with_time(3000).unwrap();
        
        // Can archive executed proposal
        assert!(proposal.archive_with_time(4000).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Archived);
        assert_eq!(proposal.archived_at, Some(4000));
    }

    #[test]
    fn test_proposal_archive_with_time_rejected() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.reject_with_time(voting_end + 1).unwrap();
        
        // Can archive rejected proposal
        assert!(proposal.archive_with_time(4000).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Archived);
    }

    #[test]
    fn test_proposal_archive_with_time_cancelled() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        proposal.cancel_with_time("Changed mind".to_string(), 3000).unwrap();
        
        // Can archive cancelled proposal
        assert!(proposal.archive_with_time(4000).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Archived);
    }

    #[test]
    fn test_proposal_archive_invalid_status() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        // Cannot archive Draft or Active proposal
        assert!(proposal.archive_with_time(4000).is_err());
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        assert!(proposal.archive_with_time(4000).is_err());
    }

    #[test]
    fn test_proposal_new_with_time_all_fields() {
        let author = create_test_pubkey(5);
        let proposal = Proposal::new_with_time(
            999,
            "Title".to_string(),
            "Description".to_string(),
            "type".to_string(),
            author,
            128,
            5000,
        ).unwrap();
        
        assert_eq!(proposal.id, 999);
        assert_eq!(proposal.title, "Title");
        assert_eq!(proposal.description, "Description");
        assert_eq!(proposal.proposal_type, "type");
        assert_eq!(proposal.author, author);
        assert_eq!(proposal.created_at, 5000);
        assert_eq!(proposal.status, ProposalStatus::Draft);
        assert_eq!(proposal.bump, 128);
        assert_eq!(proposal.voting_duration, 7 * 24 * 3600);
    }

    #[test]
    fn test_proposal_activate_with_time_zero_quorum() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        // Zero quorum should fail
        assert!(proposal.activate_with_time(0, 10, 2000).is_err());
    }

    #[test]
    fn test_proposal_activate_with_time_zero_total_members() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        // Zero total members should fail
        assert!(proposal.activate_with_time(10, 0, 2000).is_err());
    }

    #[test]
    fn test_proposal_pass_with_time_exact_voting_end() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        
        // Pass exactly at voting end
        let voting_end = proposal.created_at + proposal.voting_duration;
        assert!(proposal.pass_with_time(voting_end).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Passed);
    }

    #[test]
    fn test_proposal_execute_with_time_already_executed() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.pass_with_time(voting_end + 1).unwrap();
        proposal.execute_with_time(5000).unwrap();
        
        // Try to execute again - should fail
        assert!(proposal.execute_with_time(6000).is_err());
    }

    #[test]
    fn test_proposal_cancel_with_time_draft() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        // Cancel from Draft
        assert!(proposal.cancel_with_time("Reason".to_string(), 2000).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Cancelled);
        assert_eq!(proposal.cancellation_reason, Some("Reason".to_string()));
    }

    #[test]
    fn test_proposal_cancel_with_time_invalid_status() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.pass_with_time(voting_end + 1).unwrap();
        
        // Cannot cancel Passed proposal
        assert!(proposal.cancel_with_time("Reason".to_string(), 3000).is_err());
    }

    // ========== New lifecycle methods tests ==========

    #[test]
    fn test_proposal_set_expiration() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        // Set expiration in the future
        assert!(proposal.set_expiration(Some(5000)).is_ok());
        assert_eq!(proposal.expires_at, Some(5000));
        
        // Clear expiration
        assert!(proposal.set_expiration(None).is_ok());
        assert_eq!(proposal.expires_at, None);
    }

    #[test]
    fn test_proposal_set_expiration_invalid() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        // Expiration before creation should fail
        assert!(proposal.set_expiration(Some(500)).is_err());
    }

    #[test]
    fn test_proposal_check_and_auto_archive_expired() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.reject_with_time(voting_end + 1).unwrap();
        
        // Set expiration in the past
        proposal.expires_at = Some(5000);
        
        // Should auto-archive
        assert!(proposal.check_and_auto_archive(6000).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Archived);
    }

    #[test]
    fn test_proposal_check_and_auto_archive_not_expired() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.reject_with_time(voting_end + 1).unwrap();
        
        // Set expiration in the future
        proposal.expires_at = Some(10000);
        
        // Should not archive
        assert!(!proposal.check_and_auto_archive(6000).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Rejected);
    }

    #[test]
    fn test_proposal_auto_transition_after_voting_yes_wins() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        proposal.yes_votes = 100;
        proposal.no_votes = 50;
        
        let voting_end = proposal.submitted_at.unwrap() + proposal.voting_duration;
        
        // Should auto-transition to Passed
        assert!(proposal.auto_transition_after_voting(voting_end + 1).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Passed);
    }

    #[test]
    fn test_proposal_auto_transition_after_voting_no_wins() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        proposal.yes_votes = 50;
        proposal.no_votes = 100;
        
        let voting_end = proposal.submitted_at.unwrap() + proposal.voting_duration;
        
        // Should auto-transition to Rejected
        assert!(proposal.auto_transition_after_voting(voting_end + 1).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Rejected);
    }

    #[test]
    fn test_proposal_auto_transition_after_voting_tied() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        proposal.yes_votes = 100;
        proposal.no_votes = 100;
        
        let voting_end = proposal.submitted_at.unwrap() + proposal.voting_duration;
        
        // Should auto-transition to Tied
        assert!(proposal.auto_transition_after_voting(voting_end + 1).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Tied);
    }

    #[test]
    fn test_proposal_auto_transition_before_voting_end() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();
        
        proposal.activate_with_time(10, 20, 2000).unwrap();
        proposal.yes_votes = 100;
        proposal.no_votes = 50;
        
        // Try to auto-transition before voting ends - should not transition
        assert!(!proposal.auto_transition_after_voting(2000).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Active);
    }
}
