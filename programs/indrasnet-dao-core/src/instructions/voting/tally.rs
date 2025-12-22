//! Vote tallying handlers

use anchor_lang::prelude::*;
use crate::state::proposal::ProposalStatus;
use crate::error::IndrasError;

/// Tally votes for a proposal
///
/// This handler tallies votes and updates the proposal status based on the results.
/// Simple logic: if yes_votes > no_votes → Passed, else if no_votes > yes_votes → Rejected, else → Tied.
///
/// # Security
/// - Validates proposal is active
/// - Validates voting period has ended
/// - Only proposal author or DAO authority can tally
///
/// # Compute Units
/// Recommended: 15,000 CU
/// - Vote calculation: ~5,000 CU
/// - State update: ~10,000 CU
///
/// # Notes
/// - This is a simplified tally. The actual vote counting should be done by reading
///   all vote accounts and aggregating them. This handler just updates the status based on
///   the proposal's yes_votes and no_votes fields.
pub fn tally_votes_handler(
    ctx: Context<crate::TallyVotes>,
    proposal_id: u64,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let dao_config = &ctx.accounts.dao_config;
    
    // SECURITY: Validate proposal ID matches
    require!(
        proposal.id == proposal_id,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate proposal is active
    require!(
        proposal.status == ProposalStatus::Active,
        IndrasError::VotingNotActive
    );
    
    // SECURITY: Validate voting period has ended
    let current_time = Clock::get()?.unix_timestamp;
    // Use submitted_at if available (when proposal was activated), otherwise created_at
    let voting_start = proposal.submitted_at.unwrap_or(proposal.created_at);
    let voting_end = voting_start
        .checked_add(proposal.voting_duration)
        .ok_or(error!(IndrasError::Overflow))?;
    require!(
        current_time >= voting_end,
        IndrasError::VotingNotActive
    );
    
    // SECURITY: Validate author is proposal author or DAO authority (checked in Accounts)
    require!(
        ctx.accounts.author.key() == proposal.author || 
        ctx.accounts.author.key() == dao_config.authority,
        IndrasError::Unauthorized
    );
    
    // Update last_tallied_at
    proposal.last_tallied_at = Some(current_time);
    
    // Simple tally logic - votes are already counted in cast_vote_handler
    // Just determine the result based on vote counts
    if proposal.yes_votes > proposal.no_votes {
        proposal.status = ProposalStatus::Passed;
    } else if proposal.no_votes > proposal.yes_votes {
        proposal.status = ProposalStatus::Rejected;
    } else {
        proposal.status = ProposalStatus::Tied;
    }
    
    msg!("Votes tallied for proposal {}: yes={}, no={}, total={}, status={:?}", 
         proposal_id, proposal.yes_votes, proposal.no_votes, proposal.total_votes, proposal.status);
    
    Ok(())
}
