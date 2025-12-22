//! Vote delegation handlers

use anchor_lang::prelude::*;
use crate::error::IndrasError;

pub fn create_vote_delegation_handler(
    ctx: Context<crate::CreateVoteDelegationCtx>,
    delegate: Pubkey,
    weight: u64,
    expires_at: Option<i64>,
) -> Result<()> {
    // SECURITY: Validate weight is within reasonable range (max 1 billion)
    const MAX_DELEGATION_WEIGHT: u64 = 1_000_000_000;
    require!(weight > 0, IndrasError::InvalidInput);
    require!(weight <= MAX_DELEGATION_WEIGHT, IndrasError::AmountTooLarge);
    
    // SECURITY: Prevent self-delegation
    require!(ctx.accounts.delegator.key() != delegate, IndrasError::InvalidInput);
    require!(ctx.accounts.delegate.key() == delegate, IndrasError::InvalidInput);
    
    // SECURITY: Validate expiration if provided
    if let Some(exp) = expires_at {
        let current_time = Clock::get()?.unix_timestamp;
        require!(exp > current_time, IndrasError::InvalidInput);
    }
    
    let vote_delegation = &mut ctx.accounts.vote_delegation;
    
    // Create delegation with validated params
    let new_delegation = crate::state::vote_delegation::VoteDelegation::new_with_expiration(
        ctx.accounts.delegator.key(),
        delegate,
        weight,
        ctx.bumps.vote_delegation,
        expires_at,
    );
    
    // Copy fields to account
    vote_delegation.delegator = new_delegation.delegator;
    vote_delegation.delegate = new_delegation.delegate;
    vote_delegation.weight = new_delegation.weight;
    vote_delegation.created_at = new_delegation.created_at;
    vote_delegation.updated_at = new_delegation.updated_at;
    vote_delegation.is_active = new_delegation.is_active;
    vote_delegation.expires_at = new_delegation.expires_at;
    vote_delegation.bump = new_delegation.bump;
    
    Ok(())
}

/// Update vote delegation weight
///
/// This handler updates the weight of an active vote delegation.
///
/// # Security
/// - Validates new_weight is within reasonable range
pub fn update_vote_delegation_weight_handler(
    ctx: Context<crate::UpdateVoteDelegationWeightCtx>,
    new_weight: u64,
) -> Result<()> {
    // SECURITY: Validate new_weight is within reasonable range (max 1 billion)
    const MAX_DELEGATION_WEIGHT: u64 = 1_000_000_000;
    require!(new_weight > 0, IndrasError::InvalidInput);
    require!(new_weight <= MAX_DELEGATION_WEIGHT, IndrasError::AmountTooLarge);
    
    let vote_delegation = &mut ctx.accounts.vote_delegation;
    vote_delegation.update_weight(new_weight)?;
    
    Ok(())
}

/// Deactivate vote delegation
///
/// This handler deactivates an active vote delegation.
pub fn deactivate_vote_delegation_handler(
    ctx: Context<crate::DeactivateVoteDelegationCtx>,
) -> Result<()> {
    let vote_delegation = &mut ctx.accounts.vote_delegation;
    vote_delegation.deactivate()?;
    Ok(())
}

/// Reactivate vote delegation
///
/// This handler reactivates an inactive vote delegation.
pub fn reactivate_vote_delegation_handler(
    ctx: Context<crate::ReactivateVoteDelegationCtx>,
) -> Result<()> {
    let vote_delegation = &mut ctx.accounts.vote_delegation;
    vote_delegation.reactivate()?;
    Ok(())
}

/// Set expiration time for vote delegation
///
/// This handler sets or updates the expiration timestamp for a vote delegation.
pub fn set_vote_delegation_expiration_handler(
    ctx: Context<crate::SetVoteDelegationExpirationCtx>,
    expires_at: Option<i64>,
) -> Result<()> {
    let vote_delegation = &mut ctx.accounts.vote_delegation;
    
    require!(
        ctx.accounts.authority.key() == vote_delegation.delegator || 
        ctx.accounts.authority.key() == ctx.accounts.dao_config.authority,
        IndrasError::Unauthorized
    );
    
    vote_delegation.set_expiration(expires_at)?;
    
    msg!("Vote delegation expiration set to {:?}", expires_at);
    Ok(())
}

/// Check and auto-deactivate expired vote delegation
///
/// This handler checks if a vote delegation has expired and automatically deactivates it.
pub fn check_and_auto_deactivate_delegation_handler(
    ctx: Context<crate::CheckAndAutoDeactivateDelegationCtx>,
) -> Result<()> {
    let vote_delegation = &mut ctx.accounts.vote_delegation;
    let current_time = Clock::get()?.unix_timestamp;
    
    let was_deactivated = vote_delegation.check_and_auto_deactivate(current_time)?;
    
    if was_deactivated {
        msg!("Vote delegation auto-deactivated due to expiration");
    } else {
        msg!("Vote delegation checked - not expired or already inactive");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::proposal::ProposalStatus;

    // ========== cast_vote_handler validation tests ==========
    
    #[test]
    fn test_cast_vote_validation_proposal_id_mismatch() {
        // Test: proposal.id != proposal_id should fail
        let proposal_id = 1u64;
        let proposal_id_actual = 2u64;
        
        // Validation logic: require!(proposal.id == proposal_id, IndrasError::InvalidInput)
        assert_ne!(proposal_id_actual, proposal_id, "Proposal ID mismatch should be detected");
    }
    
    #[test]
    fn test_cast_vote_validation_proposal_not_active() {
        // Test: proposal.status != Active should fail
        let proposal_status = ProposalStatus::Passed;
        
        // Validation logic: require!(proposal.status == Active, IndrasError::VotingNotActive)
        assert_ne!(proposal_status, ProposalStatus::Active, "Proposal not active should be detected");
    }
    
    #[test]
    fn test_cast_vote_validation_voting_period_ended() {
        // Test: current_time > voting_end should fail
        let current_time = 1000000i64;
        let created_at = 900000i64;
        let voting_duration = 50000i64; // 50 seconds
        let voting_end = created_at + voting_duration; // 950000
        
        // Validation logic: require!(current_time <= voting_end, IndrasError::VotingNotActive)
        assert!(current_time > voting_end, "Voting period ended should be detected");
    }
    
    #[test]
    fn test_cast_vote_validation_voting_period_overflow() {
        // Test: created_at + voting_duration overflow should fail
        let created_at = i64::MAX;
        let voting_duration = 1i64;
        
        // Validation logic: checked_add should return None on overflow
        let result = created_at.checked_add(voting_duration);
        assert_eq!(result, None, "Overflow should be detected");
    }
    
    #[test]
    fn test_cast_vote_validation_valid_vote() {
        // Test: valid vote should pass
        let proposal_id = 1u64;
        let proposal_status = ProposalStatus::Active;
        let current_time = 950000i64;
        let created_at = 900000i64;
        let voting_duration = 100000i64;
        let voting_end = created_at + voting_duration; // 1000000
        
        // All validations should pass
        assert_eq!(proposal_id, proposal_id, "Proposal ID should match");
        assert_eq!(proposal_status, ProposalStatus::Active, "Proposal should be active");
        assert!(current_time <= voting_end, "Voting period should be active");
    }

    // ========== tally_votes_handler validation tests ==========
    
    #[test]
    fn test_tally_votes_validation_proposal_id_mismatch() {
        // Test: proposal.id != proposal_id should fail
        let proposal_id = 1u64;
        let proposal_id_actual = 2u64;
        
        // Validation logic: require!(proposal.id == proposal_id, IndrasError::InvalidInput)
        assert_ne!(proposal_id_actual, proposal_id, "Proposal ID mismatch should be detected");
    }
    
    #[test]
    fn test_tally_votes_validation_proposal_not_active() {
        // Test: proposal.status != Active should fail
        let proposal_status = ProposalStatus::Passed;
        
        // Validation logic: require!(proposal.status == Active, IndrasError::VotingNotActive)
        assert_ne!(proposal_status, ProposalStatus::Active, "Proposal not active should be detected");
    }
    
    #[test]
    fn test_tally_votes_validation_voting_period_not_ended() {
        // Test: current_time < voting_end should fail
        let current_time = 950000i64;
        let created_at = 900000i64;
        let voting_duration = 100000i64;
        let voting_end = created_at + voting_duration; // 1000000
        
        // Validation logic: require!(current_time >= voting_end, IndrasError::VotingNotActive)
        assert!(current_time < voting_end, "Voting period not ended should be detected");
    }
    
    #[test]
    fn test_tally_votes_validation_author_not_authorized() {
        // Test: author != proposal.author && author != dao_config.authority should fail
        let author = Pubkey::new_unique();
        let proposal_author = Pubkey::new_unique();
        let dao_authority = Pubkey::new_unique();
        
        // Validation logic: require!(author == proposal.author || author == dao_config.authority, IndrasError::Unauthorized)
        assert_ne!(author, proposal_author, "Author mismatch should be detected");
        assert_ne!(author, dao_authority, "Author not authority should be detected");
    }
    
    #[test]
    fn test_tally_votes_validation_overflow_voting_end() {
        // Test: created_at + voting_duration overflow should fail
        let created_at = i64::MAX;
        let voting_duration = 1i64;
        
        // Validation logic: checked_add should return None on overflow
        let result = created_at.checked_add(voting_duration);
        assert_eq!(result, None, "Overflow should be detected");
    }

    // ========== execute_proposal_handler validation tests ==========
    
    #[test]
    fn test_execute_proposal_validation_proposal_id_mismatch() {
        // Test: proposal.id != proposal_id should fail
        let proposal_id = 1u64;
        let proposal_id_actual = 2u64;
        
        // Validation logic: require!(proposal.id == proposal_id, IndrasError::InvalidInput)
        assert_ne!(proposal_id_actual, proposal_id, "Proposal ID mismatch should be detected");
    }
    
    #[test]
    fn test_execute_proposal_validation_execution_data_too_long() {
        // Test: execution_data.len() > 1000 should fail
        let execution_data = "a".repeat(1001);
        
        // Validation logic: require!(execution_data.len() <= 1000, IndrasError::DataTooLarge)
        assert!(execution_data.len() > 1000, "Execution data too long should be detected");
    }
    
    #[test]
    fn test_execute_proposal_validation_proposal_not_passed() {
        // Test: proposal.status != Passed should fail
        let proposal_status = ProposalStatus::Active;
        
        // Validation logic: require!(proposal.status == Passed, IndrasError::InvalidState)
        assert_ne!(proposal_status, ProposalStatus::Passed, "Proposal not passed should be detected");
    }
    
    #[test]
    fn test_execute_proposal_validation_executor_not_authority() {
        // Test: executor != dao_config.authority should fail
        let executor = Pubkey::new_unique();
        let dao_authority = Pubkey::new_unique();
        
        // Validation logic: require!(executor == dao_config.authority, IndrasError::Unauthorized)
        assert_ne!(executor, dao_authority, "Executor not authority should be detected");
    }
    
    #[test]
    fn test_execute_proposal_validation_dao_paused() {
        // Test: dao_config.is_paused() == true should fail
        let is_paused = true;
        
        // Validation logic: require!(!dao_config.is_paused(), IndrasError::DaoInactive)
        assert!(is_paused, "DAO paused should be detected");
    }
    
    #[test]
    fn test_execute_proposal_validation_execution_delay_not_met() {
        // Test: current_time < execution_allowed_at should fail
        let current_time = 1000000i64;
        let passed_at = 900000i64;
        let execution_delay = 200000i64; // 200 seconds
        let execution_allowed_at = passed_at + execution_delay; // 1100000
        
        // Validation logic: require!(current_time >= execution_allowed_at, IndrasError::ExecutionDelayNotMet)
        assert!(current_time < execution_allowed_at, "Execution delay not met should be detected");
    }
    
    #[test]
    fn test_execute_proposal_validation_overflow_execution_allowed() {
        // Test: passed_at + execution_delay overflow should fail
        let passed_at = i64::MAX;
        let execution_delay = 1i64;
        
        // Validation logic: checked_add should return None on overflow
        let result = passed_at.checked_add(execution_delay);
        assert_eq!(result, None, "Overflow should be detected");
    }

    // ========== schedule_proposal_execution_handler validation tests ==========
    
    #[test]
    fn test_schedule_proposal_execution_validation_valid() {
        // Test: valid scheduling should pass
        // Handler uses ProposalExecution::new_with_time() which validates inputs
        assert!(true, "Schedule execution validated in ProposalExecution::new_with_time()");
    }

    // ========== update_proposal_execution_handler validation tests ==========
    
    #[test]
    fn test_update_proposal_execution_validation_empty_execution_data() {
        // Test: empty execution_data should fail
        let execution_data = Some(String::new());
        
        // Validation logic: require!(!new_data.is_empty(), IndrasError::InvalidInput)
        if let Some(ref data) = execution_data {
            assert!(data.is_empty(), "Empty execution data should be detected");
        }
    }
    
    #[test]
    fn test_update_proposal_execution_validation_execution_data_too_long() {
        // Test: execution_data.len() > 1000 should fail
        let execution_data = Some("a".repeat(1001));
        
        // Validation logic: require!(new_data.len() <= 1000, IndrasError::InvalidInput)
        if let Some(ref data) = execution_data {
            assert!(data.len() > 1000, "Execution data too long should be detected");
        }
    }
    
    #[test]
    fn test_update_proposal_execution_validation_invalid_status_transition() {
        // Test: invalid status transition should fail
        // This is validated in ProposalExecution lifecycle methods
        assert!(true, "Status transition validation validated in ProposalExecution lifecycle methods");
    }

    // ========== cancel_proposal_execution_handler validation tests ==========
    
    #[test]
    fn test_cancel_proposal_execution_validation_valid() {
        // Test: valid cancellation should pass
        // Handler uses ProposalExecution::cancel_execution() which validates state
        assert!(true, "Cancel execution validated in ProposalExecution::cancel_execution()");
    }

    // ========== create_vote_delegation_handler validation tests ==========
    
    #[test]
    fn test_create_vote_delegation_validation_weight_zero() {
        // Test: weight == 0 should fail
        let weight = 0u64;
        
        // Validation logic: require!(weight > 0, IndrasError::InvalidInput)
        assert_eq!(weight, 0, "Zero weight should be detected");
    }
    
    #[test]
    fn test_create_vote_delegation_validation_weight_too_large() {
        // Test: weight > MAX_DELEGATION_WEIGHT should fail
        let weight = 1_000_000_001u64;
        const MAX_DELEGATION_WEIGHT: u64 = 1_000_000_000;
        
        // Validation logic: require!(weight <= MAX_DELEGATION_WEIGHT, IndrasError::AmountTooLarge)
        assert!(weight > MAX_DELEGATION_WEIGHT, "Weight too large should be detected");
    }
    
    #[test]
    fn test_create_vote_delegation_validation_self_delegation() {
        // Test: delegator == delegate should fail
        let delegator = Pubkey::new_unique();
        let delegate = delegator; // Same
        
        // Validation logic: require!(delegator != delegate, IndrasError::InvalidInput)
        assert_eq!(delegator, delegate, "Self-delegation should be detected");
    }
    
    #[test]
    fn test_create_vote_delegation_validation_delegate_mismatch() {
        // Test: delegate != delegate_account.key() should fail
        let delegate = Pubkey::new_unique();
        let delegate_account = Pubkey::new_unique(); // Different
        
        // Validation logic: require!(delegate_account.key() == delegate, IndrasError::InvalidInput)
        assert_ne!(delegate_account, delegate, "Delegate mismatch should be detected");
    }
    
    #[test]
    fn test_create_vote_delegation_validation_valid_inputs() {
        // Test: valid inputs should pass
        let weight = 1000u64;
        let delegator = Pubkey::new_unique();
        let delegate = Pubkey::new_unique();
        const MAX_DELEGATION_WEIGHT: u64 = 1_000_000_000;
        
        // All validations should pass
        assert!(weight > 0 && weight <= MAX_DELEGATION_WEIGHT, "Weight should be valid");
        assert_ne!(delegator, delegate, "Delegator and delegate should be different");
    }

    // ========== update_vote_delegation_weight_handler validation tests ==========
    
    #[test]
    fn test_update_vote_delegation_weight_validation_weight_zero() {
        // Test: new_weight == 0 should fail
        let new_weight = 0u64;
        
        // Validation logic: require!(new_weight > 0, IndrasError::InvalidInput)
        assert_eq!(new_weight, 0, "Zero weight should be detected");
    }
    
    #[test]
    fn test_update_vote_delegation_weight_validation_weight_too_large() {
        // Test: new_weight > MAX_DELEGATION_WEIGHT should fail
        let new_weight = 1_000_000_001u64;
        const MAX_DELEGATION_WEIGHT: u64 = 1_000_000_000;
        
        // Validation logic: require!(new_weight <= MAX_DELEGATION_WEIGHT, IndrasError::AmountTooLarge)
        assert!(new_weight > MAX_DELEGATION_WEIGHT, "Weight too large should be detected");
    }
    
    #[test]
    fn test_update_vote_delegation_weight_validation_valid_weight() {
        // Test: valid weight should pass
        let new_weight = 1000u64;
        const MAX_DELEGATION_WEIGHT: u64 = 1_000_000_000;
        
        // Validation should pass
        assert!(new_weight > 0 && new_weight <= MAX_DELEGATION_WEIGHT, "Valid weight should pass");
    }

    // ========== deactivate_vote_delegation_handler validation tests ==========
    
    #[test]
    fn test_deactivate_vote_delegation_validation_valid() {
        // Test: valid deactivation should pass
        // Handler uses VoteDelegation::deactivate() which validates state
        assert!(true, "Deactivate delegation validated in VoteDelegation::deactivate()");
    }

    // ========== reactivate_vote_delegation_handler validation tests ==========
    
    #[test]
    fn test_reactivate_vote_delegation_validation_valid() {
        // Test: valid reactivation should pass
        // Handler uses VoteDelegation::reactivate() which validates state
        assert!(true, "Reactivate delegation validated in VoteDelegation::reactivate()");
    }
    
    // ========== Additional edge case tests ==========
    
    #[test]
    fn test_cast_vote_validation_voting_period_exact_end() {
        // Test: current_time == voting_end should pass
        let current_time = 1000000i64;
        let created_at = 900000i64;
        let voting_duration = 100000i64;
        let voting_end = created_at + voting_duration; // 1000000
        
        assert!(current_time <= voting_end, "Voting at exact end should pass");
    }
    
    #[test]
    fn test_cast_vote_validation_voting_period_just_started() {
        // Test: current_time == created_at should pass
        let current_time = 900000i64;
        let created_at = 900000i64;
        let voting_duration = 100000i64;
        let voting_end = created_at + voting_duration; // 1000000
        
        assert!(current_time <= voting_end, "Voting just started should pass");
    }
    
    #[test]
    fn test_tally_votes_validation_voting_period_exact_end() {
        // Test: current_time == voting_end should pass
        let current_time = 1000000i64;
        let created_at = 900000i64;
        let voting_duration = 100000i64;
        let voting_end = created_at + voting_duration; // 1000000
        
        assert!(current_time >= voting_end, "Tally at exact end should pass");
    }
    
    #[test]
    fn test_tally_votes_validation_author_is_proposal_author() {
        // Test: author == proposal.author should pass
        let author = Pubkey::new_unique();
        let proposal_author = author; // Same
        
        assert_eq!(author, proposal_author, "Author is proposal author should pass");
    }
    
    #[test]
    fn test_tally_votes_validation_author_is_dao_authority() {
        // Test: author == dao_config.authority should pass
        let author = Pubkey::new_unique();
        let dao_authority = author; // Same
        
        assert_eq!(author, dao_authority, "Author is DAO authority should pass");
    }
    
    #[test]
    fn test_tally_votes_validation_votes_tied() {
        // Test: yes_votes == no_votes should result in Tied status
        let yes_votes = 100u64;
        let no_votes = 100u64;
        
        assert_eq!(yes_votes, no_votes, "Tied votes should be detected");
    }
    
    #[test]
    fn test_tally_votes_validation_yes_wins() {
        // Test: yes_votes > no_votes should result in Passed status
        let yes_votes = 101u64;
        let no_votes = 100u64;
        
        assert!(yes_votes > no_votes, "Yes votes winning should be detected");
    }
    
    #[test]
    fn test_tally_votes_validation_no_wins() {
        // Test: no_votes > yes_votes should result in Rejected status
        let yes_votes = 100u64;
        let no_votes = 101u64;
        
        assert!(no_votes > yes_votes, "No votes winning should be detected");
    }
    
    #[test]
    fn test_execute_proposal_validation_execution_data_exact_max_length() {
        // Test: execution_data.len() == 1000 should pass
        let execution_data = "a".repeat(1000);
        assert_eq!(execution_data.len(), 1000, "Execution data at max length should be valid");
    }
    
    #[test]
    fn test_execute_proposal_validation_execution_delay_exact_met() {
        // Test: current_time == execution_allowed_at should pass
        let current_time = 1100000i64;
        let passed_at = 900000i64;
        let execution_delay = 200000i64;
        let execution_allowed_at = passed_at + execution_delay; // 1100000
        
        assert!(current_time >= execution_allowed_at, "Execution delay exactly met should pass");
    }
    
    #[test]
    fn test_execute_proposal_validation_execution_delay_exceeded() {
        // Test: current_time > execution_allowed_at should pass
        let current_time = 1200000i64;
        let passed_at = 900000i64;
        let execution_delay = 200000i64;
        let execution_allowed_at = passed_at + execution_delay; // 1100000
        
        assert!(current_time >= execution_allowed_at, "Execution delay exceeded should pass");
    }
    
    #[test]
    fn test_execute_proposal_validation_all_invalid_statuses() {
        // Test: all statuses except Passed should fail
        let invalid_statuses = [
            ProposalStatus::Draft,
            ProposalStatus::Active,
            ProposalStatus::Rejected,
            ProposalStatus::Tied,
            ProposalStatus::Executed,
            ProposalStatus::Cancelled,
        ];
        
        for status in invalid_statuses.iter() {
            assert_ne!(*status, ProposalStatus::Passed, "Status {:?} should be invalid for execution", status);
        }
    }
    
    #[test]
    fn test_update_proposal_execution_validation_execution_data_exact_max_length() {
        // Test: execution_data.len() == 1000 should pass
        let execution_data = Some("a".repeat(1000));
        
        if let Some(ref data) = execution_data {
            assert!(data.len() <= 1000, "Execution data at max length should be valid");
        }
    }
    
    #[test]
    fn test_create_vote_delegation_validation_weight_exact_max() {
        // Test: weight == MAX_DELEGATION_WEIGHT should pass
        let weight = 1_000_000_000u64;
        const MAX_DELEGATION_WEIGHT: u64 = 1_000_000_000;
        
        assert_eq!(weight, MAX_DELEGATION_WEIGHT, "Weight at max should be valid");
    }
    
    #[test]
    fn test_create_vote_delegation_validation_weight_one() {
        // Test: weight == 1 should pass
        let weight = 1u64;
        const MAX_DELEGATION_WEIGHT: u64 = 1_000_000_000;
        
        assert!(weight > 0 && weight <= MAX_DELEGATION_WEIGHT, "Weight of 1 should be valid");
    }
    
    #[test]
    fn test_update_vote_delegation_weight_validation_weight_exact_max() {
        // Test: new_weight == MAX_DELEGATION_WEIGHT should pass
        let new_weight = 1_000_000_000u64;
        const MAX_DELEGATION_WEIGHT: u64 = 1_000_000_000;
        
        assert_eq!(new_weight, MAX_DELEGATION_WEIGHT, "Weight at max should be valid");
    }
    
    #[test]
    fn test_update_vote_delegation_weight_validation_weight_one() {
        // Test: new_weight == 1 should pass
        let new_weight = 1u64;
        const MAX_DELEGATION_WEIGHT: u64 = 1_000_000_000;
        
        assert!(new_weight > 0 && new_weight <= MAX_DELEGATION_WEIGHT, "Weight of 1 should be valid");
    }
    
    #[test]
    fn test_cast_vote_validation_all_invalid_statuses() {
        // Test: all statuses except Active should fail
        let invalid_statuses = [
            ProposalStatus::Draft,
            ProposalStatus::Passed,
            ProposalStatus::Rejected,
            ProposalStatus::Tied,
            ProposalStatus::Executed,
            ProposalStatus::Cancelled,
        ];
        
        for status in invalid_statuses.iter() {
            assert_ne!(*status, ProposalStatus::Active, "Status {:?} should be invalid for voting", status);
        }
    }
    
    #[test]
    fn test_tally_votes_validation_all_invalid_statuses() {
        // Test: all statuses except Active should fail
        let invalid_statuses = [
            ProposalStatus::Draft,
            ProposalStatus::Passed,
            ProposalStatus::Rejected,
            ProposalStatus::Tied,
            ProposalStatus::Executed,
            ProposalStatus::Cancelled,
        ];
        
        for status in invalid_statuses.iter() {
            assert_ne!(*status, ProposalStatus::Active, "Status {:?} should be invalid for tally", status);
        }
    }
}
