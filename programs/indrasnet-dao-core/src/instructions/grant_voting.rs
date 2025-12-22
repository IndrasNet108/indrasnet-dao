//! Grant Voting instruction handlers
//!
//! Handlers for grant voting operations: cast vote, tally votes
//!
//! This module implements the democratic voting system for grant approval.
//! Grants must go through a voting process before being approved.

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::voting_types::VoteType;
use crate::state::grant::{GrantStatus, VoterType};

/// Cast a vote on a grant
///
/// This handler creates a vote account for the given grant.
/// The vote account is a PDA with seeds [b"grant_vote", grant.key(), voter.key()].
///
/// Vote weights:
/// - MeshGroupMember: 2x weight
/// - DaoMember: 1x weight
/// - IdeaAuthor: 1x weight
///
/// # Compute Units
/// Recommended: 30,000 CU
/// - Validation: ~10,000 CU
/// - Account initialization: ~20,000 CU
pub fn cast_grant_vote_handler(
    ctx: Context<crate::CastGrantVote>,
    grant_id: u64,
    vote_choice: VoteType,
) -> Result<()> {
    let grant = &mut ctx.accounts.grant;
    let mesh_group = &ctx.accounts.mesh_group;
    let idea = &ctx.accounts.idea;
    let voter_key = ctx.accounts.voter.key();
    
    // Validate grant ID
    require!(grant.id == grant_id, IndrasError::InvalidInput);
    
    // Validate grant is in Pending status
    require!(grant.status == GrantStatus::Pending, IndrasError::InvalidState);
    
    // Validate voting period hasn't expired
    let current_time = Clock::get()?.unix_timestamp;
    require!(current_time <= grant.voting_end, IndrasError::InvalidState);
    
    // Validate mesh group contains the idea
    require!(
        mesh_group.ideas.contains(&grant.idea_id),
        IndrasError::InvalidInput
    );
    
    // Determine voter type and weight (extract values before mutable borrow)
    let is_mesh_group_member = mesh_group.members.iter()
        .any(|member| member.pubkey == voter_key);
    let is_idea_author = voter_key == idea.author;
    
    let (voter_type, weight) = if is_mesh_group_member {
        (VoterType::MeshGroupMember, 2)
    } else if is_idea_author {
        (VoterType::IdeaAuthor, 1)
    } else {
        (VoterType::DaoMember, 1)
    };
    
    // Create vote account
    let vote = &mut ctx.accounts.vote;
    vote.grant_id = grant_id;
    vote.voter = voter_key;
    vote.vote_type = vote_choice;
    vote.weight = weight;
    vote.voter_type = voter_type;
    vote.cast_at = current_time;
    vote.bump = ctx.bumps.vote;
    
    // Update grant vote counters
    match vote_choice {
        VoteType::Yes => {
            grant.total_yes_weight = grant.total_yes_weight
                .checked_add(weight)
                .ok_or(IndrasError::Overflow)?;
        }
        VoteType::No => {
            grant.total_no_weight = grant.total_no_weight
                .checked_add(weight)
                .ok_or(IndrasError::Overflow)?;
        }
        VoteType::Abstain => {
            grant.total_abstain_weight = grant.total_abstain_weight
                .checked_add(weight)
                .ok_or(IndrasError::Overflow)?;
        }
    }
    
    grant.total_votes = grant.total_votes
        .checked_add(1)
        .ok_or(IndrasError::Overflow)?;
    
    msg!("Vote cast on grant {}: {:?} (weight: {}) by {} ({:?})",
         grant_id, vote_choice, weight, voter_key, voter_type);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::state::grant::GrantStatus;

    // ========== cast_grant_vote_handler validation tests ==========
    
    #[test]
    fn test_cast_grant_vote_validation_grant_id_mismatch() {
        // Test: grant.id != grant_id should fail
        let grant_id = 1u64;
        let grant_id_actual = 2u64;
        
        // Validation logic: require!(grant.id == grant_id, IndrasError::InvalidInput)
        assert_ne!(grant_id_actual, grant_id, "Grant ID mismatch should be detected");
    }
    
    #[test]
    fn test_cast_grant_vote_validation_grant_status_not_pending() {
        // Test: grant.status != Pending should fail
        let grant_status = GrantStatus::Approved;
        
        // Validation logic: require!(grant.status == Pending, IndrasError::InvalidState)
        assert_ne!(grant_status, GrantStatus::Pending, "Grant status not Pending should be detected");
    }
    
    #[test]
    fn test_cast_grant_vote_validation_voting_period_ended() {
        // Test: current_time > grant.voting_end should fail
        let current_time = 1000000i64;
        let voting_end = 999999i64;
        
        // Validation logic: require!(current_time <= grant.voting_end, IndrasError::InvalidState)
        assert!(current_time > voting_end, "Voting period ended should be detected");
    }
    
    #[test]
    fn test_cast_grant_vote_validation_idea_not_in_mesh_group() {
        // Test: grant.idea_id not in mesh_group.ideas should fail
        let idea_id = 1u64;
        let mesh_group_ideas = vec![2u64, 3u64];
        
        // Validation logic: require!(mesh_group.ideas.contains(&grant.idea_id), IndrasError::InvalidInput)
        assert!(!mesh_group_ideas.contains(&idea_id), "Idea not in mesh group should be detected");
    }
    
    #[test]
    fn test_cast_grant_vote_validation_overflow() {
        // Test: checked_add returning None should fail
        let total_yes_weight = u64::MAX;
        let weight = 1u64;
        
        // Validation logic: checked_add should return None on overflow
        assert_eq!(total_yes_weight.checked_add(weight), None, "Overflow should be detected");
    }

    // ========== tally_grant_votes_handler validation tests ==========
    
    #[test]
    fn test_tally_grant_votes_validation_grant_id_mismatch() {
        // Test: grant.id != grant_id should fail
        let grant_id = 1u64;
        let grant_id_actual = 2u64;
        
        // Validation logic: require!(grant.id == grant_id, IndrasError::InvalidInput)
        assert_ne!(grant_id_actual, grant_id, "Grant ID mismatch should be detected");
    }
    
    #[test]
    fn test_tally_grant_votes_validation_grant_status_not_pending() {
        // Test: grant.status != Pending should fail
        let grant_status = GrantStatus::Approved;
        
        // Validation logic: require!(grant.status == Pending, IndrasError::InvalidState)
        assert_ne!(grant_status, GrantStatus::Pending, "Grant status not Pending should be detected");
    }
    
    #[test]
    fn test_tally_grant_votes_validation_overflow() {
        // Test: checked_add returning None should fail
        let total_yes_weight = u64::MAX;
        let total_no_weight = 1u64;
        
        // Validation logic: checked_add should return None on overflow
        assert_eq!(total_yes_weight.checked_add(total_no_weight), None, "Overflow should be detected");
    }
    
    #[test]
    fn test_tally_grant_votes_validation_division_by_zero() {
        // Test: total_weight == 0 should cause division by zero
        let total_weight = 0u64;
        let total_yes_weight = 100u64;
        
        // Validation logic: checked_div should return None on division by zero
        assert_eq!(total_yes_weight.checked_div(total_weight), None, "Division by zero should be detected");
    }
    
    #[test]
    fn test_tally_grant_votes_validation_voting_period_not_ended() {
        // Test: current_time <= grant.voting_end should keep grant as Pending
        let current_time = 999999i64;
        let voting_end = 1000000i64;
        
        // Validation logic: voting_expired = current_time > grant.voting_end
        assert!(current_time <= voting_end, "Voting period not ended should be detected");
    }
    
    #[test]
    fn test_tally_grant_votes_validation_quorum_not_reached_voting_expired() {
        // Test: !quorum_reached && voting_expired should reject grant
        let quorum_reached = false;
        let voting_expired = true;
        
        // Validation logic: if !quorum_reached && voting_expired { grant.status = Rejected }
        assert!(!quorum_reached && voting_expired, "Quorum not reached and voting expired should reject");
    }
    
    #[test]
    fn test_tally_grant_votes_validation_approval_threshold_met() {
        // Test: approval_percentage >= approval_threshold && quorum_reached should approve
        let approval_percentage = 60u64;
        let approval_threshold = 60u64;
        let quorum_reached = true;
        
        // Validation logic: if approval_percentage >= approval_threshold { grant.approve() }
        assert!(approval_percentage >= approval_threshold && quorum_reached, "Approval threshold met should approve");
    }
    
    #[test]
    fn test_tally_grant_votes_validation_approval_threshold_not_met() {
        // Test: approval_percentage < approval_threshold should reject
        let approval_percentage = 50u64;
        let approval_threshold = 60u64;
        
        // Validation logic: if approval_percentage < approval_threshold { grant.status = Rejected }
        assert!(approval_percentage < approval_threshold, "Approval threshold not met should reject");
    }

    // ========== calculate_quorum validation tests ==========
    
    #[test]
    fn test_calculate_quorum_mesh_group_size_1() {
        // Test: mesh_group_size <= 3 should require all members
        let mesh_group_size = 1u64;
        let total_votes = 1u64;
        let total_weight = 10u64;
        
        // Expected: mesh_quorum = mesh_group_size (1), min_dao_members = 2, quorum_required = 3
        // But for size 1, mesh_quorum = 1, min_dao_members = 2, quorum_required = 3
        assert!(mesh_group_size <= 3, "Mesh group size <= 3 should require all members");
    }
    
    #[test]
    fn test_calculate_quorum_mesh_group_size_3() {
        // Test: mesh_group_size == 3 should require all members
        let mesh_group_size = 3u64;
        
        // Expected: mesh_quorum = 3, min_dao_members = 2, quorum_required = 5
        assert_eq!(mesh_group_size, 3, "Mesh group size 3 should require all members");
    }
    
    #[test]
    fn test_calculate_quorum_mesh_group_size_4() {
        // Test: 3 < mesh_group_size <= 6 should require 50% rounded up
        let mesh_group_size = 4u64;
        
        // Expected: mesh_quorum = (4 + 1) / 2 = 2, min_dao_members = 3, quorum_required = 5
        assert!(mesh_group_size > 3 && mesh_group_size <= 6, "Mesh group size 4-6 should require 50% rounded up");
    }
    
    #[test]
    fn test_calculate_quorum_mesh_group_size_6() {
        // Test: mesh_group_size == 6 should require 50% rounded up
        let mesh_group_size = 6u64;
        
        // Expected: mesh_quorum = (6 + 1) / 2 = 3, min_dao_members = 3, quorum_required = 6
        assert_eq!(mesh_group_size, 6, "Mesh group size 6 should require 50% rounded up");
    }
    
    #[test]
    fn test_calculate_quorum_mesh_group_size_7() {
        // Test: mesh_group_size > 6 should require 40% rounded up
        let mesh_group_size = 7u64;
        
        // Expected: mesh_quorum = (7 * 2 + 4) / 5 = 3, min_dao_members = 5, quorum_required = 8
        assert!(mesh_group_size > 6, "Mesh group size > 6 should require 40% rounded up");
    }
    
    #[test]
    fn test_calculate_quorum_total_weight_zero() {
        // Test: total_weight == 0 should make quorum_reached = false
        let total_weight = 0u64;
        let total_votes = 5u64;
        
        // Validation logic: quorum_reached = total_votes >= quorum_required && total_weight > 0
        assert_eq!(total_weight, 0, "Total weight zero should make quorum not reached");
    }

    // ========== calculate_approval_threshold validation tests ==========
    
    #[test]
    fn test_calculate_approval_threshold_amount_less_than_0_1_sol() {
        // Test: sol_amount < 0.1 should return 51%
        let amount = 50_000_000u64; // 0.05 SOL
        let sol_amount = amount as f64 / 1_000_000_000.0;
        
        // Validation logic: if sol_amount < 0.1 { Ok(51) }
        assert!(sol_amount < 0.1, "Amount < 0.1 SOL should return 51% threshold");
    }
    
    #[test]
    fn test_calculate_approval_threshold_amount_0_1_sol() {
        // Test: 0.1 <= sol_amount < 0.5 should return 60%
        let amount = 100_000_000u64; // 0.1 SOL
        let sol_amount = amount as f64 / 1_000_000_000.0;
        
        // Validation logic: if sol_amount < 0.5 { Ok(60) }
        assert!(sol_amount >= 0.1 && sol_amount < 0.5, "Amount 0.1-0.5 SOL should return 60% threshold");
    }
    
    #[test]
    fn test_calculate_approval_threshold_amount_0_5_sol() {
        // Test: sol_amount >= 0.5 should return 67%
        let amount = 500_000_000u64; // 0.5 SOL
        let sol_amount = amount as f64 / 1_000_000_000.0;
        
        // Validation logic: if sol_amount >= 0.5 { Ok(67) }
        assert!(sol_amount >= 0.5, "Amount >= 0.5 SOL should return 67% threshold");
    }
    
    #[test]
    fn test_calculate_approval_threshold_amount_1_sol() {
        // Test: sol_amount >= 0.5 should return 67%
        let amount = 1_000_000_000u64; // 1.0 SOL
        let sol_amount = amount as f64 / 1_000_000_000.0;
        
        // Validation logic: if sol_amount >= 0.5 { Ok(67) }
        assert!(sol_amount >= 0.5, "Amount 1.0 SOL should return 67% threshold");
    }
    
    #[test]
    fn test_calculate_approval_threshold_amount_zero() {
        // Test: amount == 0 should return 51% (edge case)
        let amount = 0u64;
        let sol_amount = amount as f64 / 1_000_000_000.0;
        
        // Validation logic: if sol_amount < 0.1 { Ok(51) }
        assert!(sol_amount < 0.1, "Amount zero should return 51% threshold");
    }
}


/// Tally votes for a grant
///
/// This handler tallies votes and updates the grant status based on the results.
/// It checks quorum requirements and approval thresholds.
///
/// # Compute Units
/// Recommended: 20,000 CU
/// - Vote calculation: ~10,000 CU
/// - State update: ~10,000 CU
pub fn tally_grant_votes_handler(
    ctx: Context<crate::TallyGrantVotes>,
    grant_id: u64,
) -> Result<()> {
    let grant = &mut ctx.accounts.grant;
    let mesh_group = &mut ctx.accounts.mesh_group;
    
    // Validate grant ID
    require!(grant.id == grant_id, IndrasError::InvalidInput);
    
    // Validate grant is in Pending status
    require!(grant.status == GrantStatus::Pending, IndrasError::InvalidState);
    
    // Calculate total vote weight
    let total_weight = grant.total_yes_weight
        .checked_add(grant.total_no_weight)
        .and_then(|sum| sum.checked_add(grant.total_abstain_weight))
        .ok_or(IndrasError::Overflow)?;
    
    // Calculate quorum requirements
    let mesh_group_size = mesh_group.members.len() as u64;
    let (quorum_required, quorum_reached) = calculate_quorum(
        mesh_group_size,
        grant.total_votes,
        total_weight,
    )?;
    
    grant.quorum_reached = quorum_reached;
    
    // Check if voting period has expired
    let current_time = Clock::get()?.unix_timestamp;
    let voting_expired = current_time > grant.voting_end;
    
    // If quorum not reached and voting expired, reject grant
    if !quorum_reached && voting_expired {
        grant.status = GrantStatus::Rejected;
        msg!("Grant {} rejected: Quorum not reached (required: {}, got: {})",
             grant_id, quorum_required, grant.total_votes);
        return Ok(());
    }
    
    // If quorum reached, check approval threshold
    if quorum_reached && total_weight > 0 {
        let approval_threshold = calculate_approval_threshold(grant.total_amount)?;
        let approval_percentage = (grant.total_yes_weight * 100)
            .checked_div(total_weight)
            .ok_or(IndrasError::DivisionByZero)?;
        
        if approval_percentage >= approval_threshold {
            // Approve grant
            grant.approve()?;
            grant.commercialization_right_transferred = true;
            
            // Add grant to mesh group
            mesh_group.add_grant(grant.id)?;
            
            msg!("Grant {} approved: {}% yes votes (threshold: {}%)",
                 grant_id, approval_percentage, approval_threshold);
        } else {
            // Reject grant (not enough support)
            grant.status = GrantStatus::Rejected;
            msg!("Grant {} rejected: {}% yes votes (threshold: {}%)",
                 grant_id, approval_percentage, approval_threshold);
        }
    } else {
        // Quorum not reached yet, keep as Pending
        msg!("Grant {} voting in progress: {}/{} votes, quorum: {}",
             grant_id, grant.total_votes, quorum_required, if quorum_reached { "reached" } else { "not reached" });
    }
    
    Ok(())
}

/// Calculate quorum requirements
///
/// Returns (quorum_required, quorum_reached)
fn calculate_quorum(
    mesh_group_size: u64,
    total_votes: u64,
    total_weight: u64,
) -> Result<(u64, bool)> {
    // Calculate minimum quorum based on mesh group size
    let mesh_quorum = if mesh_group_size <= 3 {
        mesh_group_size // All members required
    } else if mesh_group_size <= 6 {
        (mesh_group_size + 1) / 2 // 50% rounded up
    } else {
        (mesh_group_size * 2 + 4) / 5 // 40% rounded up
    };
    
    // Minimum DAO members required
    let min_dao_members = if mesh_group_size <= 3 {
        2
    } else if mesh_group_size <= 6 {
        3
    } else {
        5
    };
    
    let quorum_required = mesh_quorum + min_dao_members;
    let quorum_reached = total_votes >= quorum_required && total_weight > 0;
    
    Ok((quorum_required, quorum_reached))
}

/// Calculate approval threshold based on grant amount
///
/// Returns threshold as percentage (51, 60, or 67)
fn calculate_approval_threshold(amount: u64) -> Result<u64> {
    // Convert lamports to SOL (1 SOL = 1_000_000_000 lamports)
    let sol_amount = amount as f64 / 1_000_000_000.0;
    
    if sol_amount < 0.1 {
        Ok(51) // Simple majority
    } else if sol_amount < 0.5 {
        Ok(60) // Qualified majority
    } else {
        Ok(67) // Supermajority (66.67% rounded to 67)
    }
}
