//! Vote casting handlers

use anchor_lang::prelude::*;
use crate::voting_types::VoteType;
use crate::error::IndrasError;

/// Cast a vote on a proposal
///
/// This handler creates a vote account for the given proposal.
/// The vote account is a PDA with seeds [b"vote", proposal.key(), voter.key()].
///
/// # Security
/// - Validates proposal is active
/// - Prevents duplicate voting (PDA seeds ensure uniqueness)
/// - Validates proposal ID matches
///
/// # Compute Units
/// Recommended: 25,000 CU
/// - Validation: ~5,000 CU
/// - Account initialization: ~20,000 CU
pub fn cast_vote_handler(
    ctx: Context<crate::CastVote>,
    proposal_id: u64,
    vote_choice: VoteType,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    
    // SECURITY: Validate proposal ID matches
    require!(
        proposal.id == proposal_id,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate proposal is active (checked in Accounts, but double-check)
    require!(
        proposal.status == crate::state::proposal::ProposalStatus::Active,
        IndrasError::VotingNotActive
    );
    
    // SECURITY: Validate voting period hasn't ended
    let current_time = Clock::get()?.unix_timestamp;
    let voting_start = proposal.submitted_at.unwrap_or(proposal.created_at);
    let voting_end = voting_start
        .checked_add(proposal.voting_duration)
        .ok_or(error!(IndrasError::Overflow))?;
    require!(
        current_time <= voting_end,
        IndrasError::VotingNotActive
    );
    
    let vote = &mut ctx.accounts.vote;
    vote.idea_id = proposal_id;
    vote.voter = ctx.accounts.voter.key();
    vote.vote_type = vote_choice.clone();
    vote.weight = 1;
    vote.cast_at = current_time;
    vote.bump = ctx.bumps.vote;
    
    // Update proposal vote counts
    match vote_choice {
        VoteType::Yes => {
            proposal.yes_votes = proposal.yes_votes
                .checked_add(1)
                .ok_or(IndrasError::Overflow)?;
        },
        VoteType::No => {
            proposal.no_votes = proposal.no_votes
                .checked_add(1)
                .ok_or(IndrasError::Overflow)?;
        },
        VoteType::Abstain => {
            // Abstain votes don't count towards yes/no, but count towards total
        },
    }
    
    proposal.total_votes = proposal.total_votes
        .checked_add(1)
        .ok_or(IndrasError::Overflow)?;
    
    msg!("Vote cast on proposal {} by {}: {:?} (yes={}, no={}, total={})", 
         proposal_id, ctx.accounts.voter.key(), vote_choice, 
         proposal.yes_votes, proposal.no_votes, proposal.total_votes);
    
    Ok(())
}
