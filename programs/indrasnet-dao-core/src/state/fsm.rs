//! Finite State Machine (FSM) for Idea and Grant state transitions
//!
//! This module provides formal validation of state transitions for ideas and grants.
//! Each state defines which states it can transition to, ensuring valid state changes.

use anchor_lang::prelude::*;
use crate::state::enums::IdeaStatus;
use crate::state::grant::GrantStatus;
use crate::error::IndrasError;

/// FSM implementation for IdeaStatus
impl IdeaStatus {
    /// Get all valid next states from current state
    pub fn next_states(&self) -> &'static [IdeaStatus] {
        use IdeaStatus::*;
        match self {
            Draft            => &[UnderReview, Voting],
            UnderReview      => &[Approved, Rejected, Voting],
            Voting           => &[Approved, Rejected],
            Approved         => &[InProgress, Paused],
            Rejected         => &[Archived, Resubmitted],
            InProgress       => &[Completed, Paused, Expired],
            Paused           => &[InProgress, Archived],
            Completed        => &[Executed, Archived],
            Executed         => &[Commercialization, Archived],
            Commercialization=> &[Archived],
            Archived         => &[Resubmitted],
            Resubmitted      => &[UnderReview, Voting],
            Expired          => &[Archived],
        }
    }
    
    /// Check if transition from current state to target state is valid
    pub fn can_transition_to(&self, target: IdeaStatus) -> bool {
        // Same state is always valid (no-op)
        if *self == target {
            return true;
        }
        
        self.next_states().contains(&target)
    }
    
    /// Validate transition and return error if invalid
    pub fn validate_transition(&self, target: IdeaStatus) -> Result<()> {
        require!(
            self.can_transition_to(target),
            IndrasError::InvalidState
        );
        Ok(())
    }
}

/// FSM implementation for GrantStatus
impl GrantStatus {
    /// Get all valid next states from current state
    pub fn next_states(&self) -> &'static [GrantStatus] {
        use GrantStatus::*;
        match self {
            Pending     => &[Approved, Rejected],
            Approved    => &[Active, Suspended],
            Active      => &[Completed, Cancelled, Suspended, Expired],
            Suspended   => &[Active, Cancelled],
            Completed   => &[Archived],
            Cancelled   => &[Archived],
            Rejected    => &[Archived],
            Expired     => &[Archived],
            Archived    => &[], // Terminal state
        }
    }
    
    /// Check if transition from current state to target state is valid
    pub fn can_transition_to(&self, target: GrantStatus) -> bool {
        // Same state is always valid (no-op)
        if *self == target {
            return true;
        }
        
        self.next_states().contains(&target)
    }
    
    /// Validate transition and return error if invalid
    pub fn validate_transition(&self, target: GrantStatus) -> Result<()> {
        require!(
            self.can_transition_to(target),
            IndrasError::InvalidState
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_idea_fsm_transitions() {
        // Test Draft -> UnderReview
        assert!(IdeaStatus::Draft.can_transition_to(IdeaStatus::UnderReview));
        assert!(IdeaStatus::Draft.can_transition_to(IdeaStatus::Voting));
        assert!(!IdeaStatus::Draft.can_transition_to(IdeaStatus::Completed));
        
        // Test Approved -> InProgress
        assert!(IdeaStatus::Approved.can_transition_to(IdeaStatus::InProgress));
        assert!(IdeaStatus::Approved.can_transition_to(IdeaStatus::Paused));
        assert!(!IdeaStatus::Approved.can_transition_to(IdeaStatus::Draft));
        
        // Test same state (no-op)
        assert!(IdeaStatus::Draft.can_transition_to(IdeaStatus::Draft));
    }
    
    #[test]
    fn test_grant_fsm_transitions() {
        // Test Pending -> Approved
        assert!(GrantStatus::Pending.can_transition_to(GrantStatus::Approved));
        assert!(GrantStatus::Pending.can_transition_to(GrantStatus::Rejected));
        assert!(!GrantStatus::Pending.can_transition_to(GrantStatus::Active));
        
        // Test Active -> Completed
        assert!(GrantStatus::Active.can_transition_to(GrantStatus::Completed));
        assert!(GrantStatus::Active.can_transition_to(GrantStatus::Cancelled));
        assert!(GrantStatus::Active.can_transition_to(GrantStatus::Suspended));
        assert!(GrantStatus::Active.can_transition_to(GrantStatus::Expired));
        
        // Test Archived is terminal
        assert_eq!(GrantStatus::Archived.next_states().len(), 0);
    }

    #[test]
    fn test_idea_fsm_validate_transition() {
        // Valid transition
        assert!(IdeaStatus::Draft.validate_transition(IdeaStatus::UnderReview).is_ok());
        
        // Invalid transition
        assert!(IdeaStatus::Draft.validate_transition(IdeaStatus::Completed).is_err());
        
        // Same state (no-op)
        assert!(IdeaStatus::Draft.validate_transition(IdeaStatus::Draft).is_ok());
    }

    #[test]
    fn test_grant_fsm_validate_transition() {
        // Valid transition
        assert!(GrantStatus::Pending.validate_transition(GrantStatus::Approved).is_ok());
        
        // Invalid transition
        assert!(GrantStatus::Pending.validate_transition(GrantStatus::Active).is_err());
        
        // Same state (no-op)
        assert!(GrantStatus::Pending.validate_transition(GrantStatus::Pending).is_ok());
    }

    #[test]
    fn test_idea_fsm_next_states() {
        // Test all states have next states defined
        let draft_states = IdeaStatus::Draft.next_states();
        assert!(draft_states.contains(&IdeaStatus::UnderReview));
        assert!(draft_states.contains(&IdeaStatus::Voting));
        
        let approved_states = IdeaStatus::Approved.next_states();
        assert!(approved_states.contains(&IdeaStatus::InProgress));
        assert!(approved_states.contains(&IdeaStatus::Paused));
    }

    #[test]
    fn test_grant_fsm_next_states() {
        // Test all states have next states defined
        let pending_states = GrantStatus::Pending.next_states();
        assert!(pending_states.contains(&GrantStatus::Approved));
        assert!(pending_states.contains(&GrantStatus::Rejected));
        
        let active_states = GrantStatus::Active.next_states();
        assert!(active_states.contains(&GrantStatus::Completed));
        assert!(active_states.contains(&GrantStatus::Cancelled));
        assert!(active_states.contains(&GrantStatus::Suspended));
        assert!(active_states.contains(&GrantStatus::Expired));
    }

    #[test]
    fn test_idea_fsm_all_draft_transitions() {
        let next = IdeaStatus::Draft.next_states();
        assert!(next.contains(&IdeaStatus::UnderReview));
        assert!(next.contains(&IdeaStatus::Voting));
        assert_eq!(next.len(), 2);
    }

    #[test]
    fn test_idea_fsm_all_approved_transitions() {
        let next = IdeaStatus::Approved.next_states();
        assert!(next.contains(&IdeaStatus::InProgress));
        assert!(next.contains(&IdeaStatus::Paused));
        assert_eq!(next.len(), 2);
    }

    #[test]
    fn test_idea_fsm_all_in_progress_transitions() {
        let next = IdeaStatus::InProgress.next_states();
        assert!(next.contains(&IdeaStatus::Completed));
        assert!(next.contains(&IdeaStatus::Paused));
        assert!(next.contains(&IdeaStatus::Expired));
        assert_eq!(next.len(), 3);
    }

    #[test]
    fn test_idea_fsm_archived_transitions() {
        let next = IdeaStatus::Archived.next_states();
        assert!(next.contains(&IdeaStatus::Resubmitted));
        assert_eq!(next.len(), 1);
    }

    #[test]
    fn test_idea_fsm_commercialization_transitions() {
        let next = IdeaStatus::Commercialization.next_states();
        assert!(next.contains(&IdeaStatus::Archived));
        assert_eq!(next.len(), 1);
    }

    #[test]
    fn test_grant_fsm_all_pending_transitions() {
        let next = GrantStatus::Pending.next_states();
        assert!(next.contains(&GrantStatus::Approved));
        assert!(next.contains(&GrantStatus::Rejected));
        assert_eq!(next.len(), 2);
    }

    #[test]
    fn test_grant_fsm_all_active_transitions() {
        let next = GrantStatus::Active.next_states();
        assert!(next.contains(&GrantStatus::Completed));
        assert!(next.contains(&GrantStatus::Cancelled));
        assert!(next.contains(&GrantStatus::Suspended));
        assert!(next.contains(&GrantStatus::Expired));
        assert_eq!(next.len(), 4);
    }

    #[test]
    fn test_grant_fsm_suspended_transitions() {
        let next = GrantStatus::Suspended.next_states();
        assert!(next.contains(&GrantStatus::Active));
        assert!(next.contains(&GrantStatus::Cancelled));
        assert_eq!(next.len(), 2);
    }

    #[test]
    fn test_grant_fsm_completed_transitions() {
        let next = GrantStatus::Completed.next_states();
        assert!(next.contains(&GrantStatus::Archived));
        assert_eq!(next.len(), 1);
    }

    #[test]
    fn test_grant_fsm_archived_is_terminal() {
        let next = GrantStatus::Archived.next_states();
        assert_eq!(next.len(), 0);
        // Cannot transition from Archived
        assert!(!GrantStatus::Archived.can_transition_to(GrantStatus::Active));
    }

    #[test]
    fn test_idea_fsm_validate_all_valid_transitions() {
        // Test multiple valid transitions
        assert!(IdeaStatus::UnderReview.validate_transition(IdeaStatus::Approved).is_ok());
        assert!(IdeaStatus::UnderReview.validate_transition(IdeaStatus::Rejected).is_ok());
        assert!(IdeaStatus::UnderReview.validate_transition(IdeaStatus::Voting).is_ok());
    }

    #[test]
    fn test_grant_fsm_validate_all_valid_transitions() {
        // Test multiple valid transitions
        assert!(GrantStatus::Approved.validate_transition(GrantStatus::Active).is_ok());
        assert!(GrantStatus::Approved.validate_transition(GrantStatus::Suspended).is_ok());
    }

    #[test]
    fn test_idea_fsm_rejected_transitions() {
        let next = IdeaStatus::Rejected.next_states();
        assert!(next.contains(&IdeaStatus::Archived));
        assert!(next.contains(&IdeaStatus::Resubmitted));
        assert_eq!(next.len(), 2);
    }

    #[test]
    fn test_idea_fsm_expired_transitions() {
        let next = IdeaStatus::Expired.next_states();
        assert!(next.contains(&IdeaStatus::Archived));
        assert_eq!(next.len(), 1);
    }

    #[test]
    fn test_idea_fsm_all_status_transitions() {
        // Test all IdeaStatus variants have valid transitions
        let all_statuses = vec![
            IdeaStatus::Draft,
            IdeaStatus::UnderReview,
            IdeaStatus::Voting,
            IdeaStatus::Approved,
            IdeaStatus::Rejected,
            IdeaStatus::InProgress,
            IdeaStatus::Paused,
            IdeaStatus::Completed,
            IdeaStatus::Executed,
            IdeaStatus::Commercialization,
            IdeaStatus::Archived,
            IdeaStatus::Resubmitted,
            IdeaStatus::Expired,
        ];
        
        for status in all_statuses {
            let next = status.next_states();
            assert!(!next.is_empty() || status == IdeaStatus::Archived || status == IdeaStatus::Expired);
        }
    }

    #[test]
    fn test_grant_fsm_all_status_transitions() {
        // Test all GrantStatus variants have valid transitions
        let all_statuses = vec![
            GrantStatus::Pending,
            GrantStatus::Approved,
            GrantStatus::Active,
            GrantStatus::Suspended,
            GrantStatus::Completed,
            GrantStatus::Cancelled,
            GrantStatus::Rejected,
            GrantStatus::Expired,
            GrantStatus::Archived,
        ];
        
        for status in all_statuses {
            let next = status.next_states();
            // Archived is terminal, others should have transitions
            if status == GrantStatus::Archived {
                assert_eq!(next.len(), 0);
            } else {
                assert!(!next.is_empty());
            }
        }
    }

    #[test]
    fn test_idea_fsm_under_review_transitions() {
        let next = IdeaStatus::UnderReview.next_states();
        assert!(next.contains(&IdeaStatus::Approved));
        assert!(next.contains(&IdeaStatus::Rejected));
        assert!(next.contains(&IdeaStatus::Voting));
        assert_eq!(next.len(), 3);
    }

    #[test]
    fn test_idea_fsm_voting_transitions() {
        let next = IdeaStatus::Voting.next_states();
        assert!(next.contains(&IdeaStatus::Approved));
        assert!(next.contains(&IdeaStatus::Rejected));
        assert_eq!(next.len(), 2);
    }

    #[test]
    fn test_idea_fsm_paused_transitions() {
        let next = IdeaStatus::Paused.next_states();
        assert!(next.contains(&IdeaStatus::InProgress));
        assert!(next.contains(&IdeaStatus::Archived));
        assert_eq!(next.len(), 2);
    }

    #[test]
    fn test_idea_fsm_completed_transitions() {
        let next = IdeaStatus::Completed.next_states();
        assert!(next.contains(&IdeaStatus::Executed));
        assert!(next.contains(&IdeaStatus::Archived));
        assert_eq!(next.len(), 2);
    }

    #[test]
    fn test_idea_fsm_executed_transitions() {
        let next = IdeaStatus::Executed.next_states();
        assert!(next.contains(&IdeaStatus::Commercialization));
        assert!(next.contains(&IdeaStatus::Archived));
        assert_eq!(next.len(), 2);
    }

    #[test]
    fn test_idea_fsm_resubmitted_transitions() {
        let next = IdeaStatus::Resubmitted.next_states();
        assert!(next.contains(&IdeaStatus::UnderReview));
        assert!(next.contains(&IdeaStatus::Voting));
        assert_eq!(next.len(), 2);
    }

    #[test]
    fn test_grant_fsm_approved_transitions() {
        let next = GrantStatus::Approved.next_states();
        assert!(next.contains(&GrantStatus::Active));
        assert!(next.contains(&GrantStatus::Suspended));
        assert_eq!(next.len(), 2);
    }

    #[test]
    fn test_grant_fsm_rejected_transitions() {
        let next = GrantStatus::Rejected.next_states();
        assert!(next.contains(&GrantStatus::Archived));
        assert_eq!(next.len(), 1);
    }

    #[test]
    fn test_grant_fsm_cancelled_transitions() {
        let next = GrantStatus::Cancelled.next_states();
        assert!(next.contains(&GrantStatus::Archived));
        assert_eq!(next.len(), 1);
    }

    #[test]
    fn test_grant_fsm_expired_transitions() {
        let next = GrantStatus::Expired.next_states();
        assert!(next.contains(&GrantStatus::Archived));
        assert_eq!(next.len(), 1);
    }

    #[test]
    fn test_idea_fsm_invalid_transitions() {
        // Test some invalid transitions
        assert!(!IdeaStatus::Draft.can_transition_to(IdeaStatus::Completed));
        assert!(!IdeaStatus::Draft.can_transition_to(IdeaStatus::Executed));
        assert!(!IdeaStatus::Approved.can_transition_to(IdeaStatus::Draft));
        assert!(!IdeaStatus::Completed.can_transition_to(IdeaStatus::Draft));
        assert!(!IdeaStatus::Archived.can_transition_to(IdeaStatus::Draft));
    }

    #[test]
    fn test_grant_fsm_invalid_transitions() {
        // Test some invalid transitions
        assert!(!GrantStatus::Pending.can_transition_to(GrantStatus::Active));
        assert!(!GrantStatus::Pending.can_transition_to(GrantStatus::Completed));
        assert!(!GrantStatus::Archived.can_transition_to(GrantStatus::Active));
        assert!(!GrantStatus::Rejected.can_transition_to(GrantStatus::Active));
    }

    #[test]
    fn test_idea_fsm_validate_invalid_transitions() {
        // Test validation of invalid transitions
        assert!(IdeaStatus::Draft.validate_transition(IdeaStatus::Completed).is_err());
        assert!(IdeaStatus::Approved.validate_transition(IdeaStatus::Draft).is_err());
        assert!(IdeaStatus::Archived.validate_transition(IdeaStatus::Draft).is_err());
    }

    #[test]
    fn test_grant_fsm_validate_invalid_transitions() {
        // Test validation of invalid transitions
        assert!(GrantStatus::Pending.validate_transition(GrantStatus::Active).is_err());
        assert!(GrantStatus::Archived.validate_transition(GrantStatus::Active).is_err());
        assert!(GrantStatus::Rejected.validate_transition(GrantStatus::Active).is_err());
    }

    #[test]
    fn test_idea_fsm_complete_cycle() {
        // Test a complete cycle: Draft -> UnderReview -> Approved -> InProgress -> Completed -> Executed -> Commercialization -> Archived -> Resubmitted
        assert!(IdeaStatus::Draft.can_transition_to(IdeaStatus::UnderReview));
        assert!(IdeaStatus::UnderReview.can_transition_to(IdeaStatus::Approved));
        assert!(IdeaStatus::Approved.can_transition_to(IdeaStatus::InProgress));
        assert!(IdeaStatus::InProgress.can_transition_to(IdeaStatus::Completed));
        assert!(IdeaStatus::Completed.can_transition_to(IdeaStatus::Executed));
        assert!(IdeaStatus::Executed.can_transition_to(IdeaStatus::Commercialization));
        assert!(IdeaStatus::Commercialization.can_transition_to(IdeaStatus::Archived));
        assert!(IdeaStatus::Archived.can_transition_to(IdeaStatus::Resubmitted));
        assert!(IdeaStatus::Resubmitted.can_transition_to(IdeaStatus::UnderReview));
    }

    #[test]
    fn test_grant_fsm_complete_cycle() {
        // Test a complete cycle: Pending -> Approved -> Active -> Completed -> Archived
        assert!(GrantStatus::Pending.can_transition_to(GrantStatus::Approved));
        assert!(GrantStatus::Approved.can_transition_to(GrantStatus::Active));
        assert!(GrantStatus::Active.can_transition_to(GrantStatus::Completed));
        assert!(GrantStatus::Completed.can_transition_to(GrantStatus::Archived));
    }

    #[test]
    fn test_idea_fsm_rejection_cycle() {
        // Test rejection cycle: Draft -> UnderReview -> Rejected -> Archived -> Resubmitted
        assert!(IdeaStatus::Draft.can_transition_to(IdeaStatus::UnderReview));
        assert!(IdeaStatus::UnderReview.can_transition_to(IdeaStatus::Rejected));
        assert!(IdeaStatus::Rejected.can_transition_to(IdeaStatus::Archived));
        assert!(IdeaStatus::Archived.can_transition_to(IdeaStatus::Resubmitted));
    }

    #[test]
    fn test_grant_fsm_suspension_cycle() {
        // Test suspension cycle: Approved -> Active -> Suspended -> Active
        assert!(GrantStatus::Approved.can_transition_to(GrantStatus::Active));
        assert!(GrantStatus::Active.can_transition_to(GrantStatus::Suspended));
        assert!(GrantStatus::Suspended.can_transition_to(GrantStatus::Active));
    }

    #[test]
    fn test_idea_fsm_all_states_same_state_transition() {
        // All states should allow transition to themselves (no-op)
        let all_statuses = vec![
            IdeaStatus::Draft,
            IdeaStatus::UnderReview,
            IdeaStatus::Voting,
            IdeaStatus::Approved,
            IdeaStatus::Rejected,
            IdeaStatus::InProgress,
            IdeaStatus::Paused,
            IdeaStatus::Completed,
            IdeaStatus::Executed,
            IdeaStatus::Commercialization,
            IdeaStatus::Archived,
            IdeaStatus::Resubmitted,
            IdeaStatus::Expired,
        ];
        
        for status in all_statuses {
            assert!(status.can_transition_to(status));
            assert!(status.validate_transition(status).is_ok());
        }
    }

    #[test]
    fn test_grant_fsm_all_states_same_state_transition() {
        // All states should allow transition to themselves (no-op)
        let all_statuses = vec![
            GrantStatus::Pending,
            GrantStatus::Approved,
            GrantStatus::Active,
            GrantStatus::Suspended,
            GrantStatus::Completed,
            GrantStatus::Cancelled,
            GrantStatus::Rejected,
            GrantStatus::Expired,
            GrantStatus::Archived,
        ];
        
        for status in all_statuses {
            assert!(status.can_transition_to(status));
            assert!(status.validate_transition(status).is_ok());
        }
    }

    #[test]
    fn test_idea_fsm_next_states_immutable() {
        // Test that next_states() returns consistent results
        let states1 = IdeaStatus::Draft.next_states();
        let states2 = IdeaStatus::Draft.next_states();
        assert_eq!(states1.len(), states2.len());
        for state in states1 {
            assert!(states2.contains(state));
        }
    }

    #[test]
    fn test_grant_fsm_next_states_immutable() {
        // Test that next_states() returns consistent results
        let states1 = GrantStatus::Pending.next_states();
        let states2 = GrantStatus::Pending.next_states();
        assert_eq!(states1.len(), states2.len());
        for state in states1 {
            assert!(states2.contains(state));
        }
    }

    #[test]
    fn test_idea_fsm_validate_transition_all_valid() {
        // Test validate_transition for all valid transitions from Draft
        let valid_targets = IdeaStatus::Draft.next_states();
        for target in valid_targets {
            assert!(IdeaStatus::Draft.validate_transition(*target).is_ok());
        }
    }

    #[test]
    fn test_grant_fsm_validate_transition_all_valid() {
        // Test validate_transition for all valid transitions from Pending
        let valid_targets = GrantStatus::Pending.next_states();
        for target in valid_targets {
            assert!(GrantStatus::Pending.validate_transition(*target).is_ok());
        }
    }

    #[test]
    fn test_idea_fsm_no_cycles_to_archived() {
        // Archived should not be able to transition back to active states
        assert!(!IdeaStatus::Archived.can_transition_to(IdeaStatus::Draft));
        assert!(!IdeaStatus::Archived.can_transition_to(IdeaStatus::UnderReview));
        assert!(!IdeaStatus::Archived.can_transition_to(IdeaStatus::Approved));
        assert!(!IdeaStatus::Archived.can_transition_to(IdeaStatus::InProgress));
    }

    #[test]
    fn test_grant_fsm_no_cycles_from_archived() {
        // Archived should not be able to transition to any state
        assert!(!GrantStatus::Archived.can_transition_to(GrantStatus::Pending));
        assert!(!GrantStatus::Archived.can_transition_to(GrantStatus::Active));
        assert!(!GrantStatus::Archived.can_transition_to(GrantStatus::Approved));
    }

    #[test]
    fn test_idea_fsm_expired_only_to_archived() {
        // Expired can only go to Archived
        let next = IdeaStatus::Expired.next_states();
        assert_eq!(next.len(), 1);
        assert!(next.contains(&IdeaStatus::Archived));
        assert!(!IdeaStatus::Expired.can_transition_to(IdeaStatus::Draft));
        assert!(!IdeaStatus::Expired.can_transition_to(IdeaStatus::InProgress));
    }

    #[test]
    fn test_grant_fsm_all_terminal_states_to_archived() {
        // All terminal states (Completed, Cancelled, Rejected, Expired) should go to Archived
        assert!(GrantStatus::Completed.can_transition_to(GrantStatus::Archived));
        assert!(GrantStatus::Cancelled.can_transition_to(GrantStatus::Archived));
        assert!(GrantStatus::Rejected.can_transition_to(GrantStatus::Archived));
        assert!(GrantStatus::Expired.can_transition_to(GrantStatus::Archived));
    }

    #[test]
    fn test_idea_fsm_all_states_self_transition() {
        // All states should allow transition to themselves (no-op)
        let states = vec![
            IdeaStatus::Draft, IdeaStatus::UnderReview, IdeaStatus::Voting,
            IdeaStatus::Approved, IdeaStatus::Rejected, IdeaStatus::InProgress,
            IdeaStatus::Paused, IdeaStatus::Completed, IdeaStatus::Executed,
            IdeaStatus::Commercialization, IdeaStatus::Archived,
            IdeaStatus::Resubmitted, IdeaStatus::Expired,
        ];
        
        for state in states {
            assert!(state.can_transition_to(state), "State {:?} should allow self-transition", state);
            assert!(state.validate_transition(state).is_ok(), "State {:?} should validate self-transition", state);
        }
    }

    #[test]
    fn test_grant_fsm_all_states_self_transition() {
        // All states should allow transition to themselves (no-op)
        let states = vec![
            GrantStatus::Pending, GrantStatus::Approved, GrantStatus::Active,
            GrantStatus::Suspended, GrantStatus::Completed, GrantStatus::Cancelled,
            GrantStatus::Rejected, GrantStatus::Expired, GrantStatus::Archived,
        ];
        
        for state in states {
            assert!(state.can_transition_to(state), "State {:?} should allow self-transition", state);
            assert!(state.validate_transition(state).is_ok(), "State {:?} should validate self-transition", state);
        }
    }

    #[test]
    fn test_idea_fsm_validate_transition_error_message() {
        // Test that invalid transitions return proper error
        let result = IdeaStatus::Draft.validate_transition(IdeaStatus::Completed);
        assert!(result.is_err());
    }

    #[test]
    fn test_grant_fsm_validate_transition_error_message() {
        // Test that invalid transitions return proper error
        let result = GrantStatus::Pending.validate_transition(GrantStatus::Completed);
        assert!(result.is_err());
    }

    #[test]
    fn test_idea_fsm_all_next_states_valid() {
        // Test that all next states for each state are valid transitions
        let states = vec![
            IdeaStatus::Draft, IdeaStatus::UnderReview, IdeaStatus::Voting,
            IdeaStatus::Approved, IdeaStatus::Rejected, IdeaStatus::InProgress,
            IdeaStatus::Paused, IdeaStatus::Completed, IdeaStatus::Executed,
            IdeaStatus::Commercialization, IdeaStatus::Archived,
            IdeaStatus::Resubmitted, IdeaStatus::Expired,
        ];
        
        for state in states {
            let next_states = state.next_states();
            for next_state in next_states {
                assert!(state.can_transition_to(*next_state), 
                    "State {:?} should be able to transition to {:?}", state, next_state);
            }
        }
    }

    #[test]
    fn test_grant_fsm_all_next_states_valid() {
        // Test that all next states for each state are valid transitions
        let states = vec![
            GrantStatus::Pending, GrantStatus::Approved, GrantStatus::Active,
            GrantStatus::Suspended, GrantStatus::Completed, GrantStatus::Cancelled,
            GrantStatus::Rejected, GrantStatus::Expired, GrantStatus::Archived,
        ];
        
        for state in states {
            let next_states = state.next_states();
            for next_state in next_states {
                assert!(state.can_transition_to(*next_state), 
                    "State {:?} should be able to transition to {:?}", state, next_state);
            }
        }
    }
}
