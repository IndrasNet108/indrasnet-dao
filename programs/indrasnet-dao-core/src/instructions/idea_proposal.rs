//! Idea to Proposal conversion instruction handlers
//!
//! Handlers for converting Ideas to Proposals (rare case)

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::proposal::ProposalStatus;
use crate::state::member::role::role_permissions;

/// Convert an Idea to a Proposal
///
/// This handler converts an existing Idea into a Proposal for governance voting.
/// This is a rare case - normally Ideas are handled by Mesh Groups, while Proposals
/// are for general DAO governance.
///
/// # Security
/// - Validates idea exists and is in appropriate status
/// - Validates author has permission to convert
/// - Creates new proposal linked to the idea
pub fn convert_idea_to_proposal_handler(
    ctx: Context<crate::ConvertIdeaToProposal>,
    idea_id: u64,
    proposal_id: u64,
    title: Option<String>,
    description: Option<String>,
    voting_duration: Option<i64>,
) -> Result<()> {
    let idea = &ctx.accounts.idea;
    let proposal = &mut ctx.accounts.proposal;
    let author = ctx.accounts.author.key();
    
    // SECURITY: Validate idea ID matches
    require!(idea.id == idea_id, IndrasError::InvalidInput);
    
    // SECURITY: Validate author has permission
    if let Some(author_role) = &ctx.accounts.author_role {
        require!(
            author_role.has_permission(role_permissions::CAN_PROPOSE),
            IndrasError::Unauthorized
        );
    }
    
    // SECURITY: Only idea author or DAO authority can convert
    require!(
        author == idea.author || author == ctx.accounts.dao_config.authority,
        IndrasError::Unauthorized
    );
    
    let current_time = Clock::get()?.unix_timestamp;
    let bump = ctx.bumps.proposal;
    
    // Use provided title/description or derive from idea
    let proposal_title = title.unwrap_or_else(|| {
        if idea.title.len() <= 200 {
            idea.title.clone()
        } else {
            format!("{}...", &idea.title[..197])
        }
    });
    
    let proposal_description = description.unwrap_or_else(|| {
        if idea.description.len() <= 2000 {
            idea.description.clone()
        } else {
            format!("{}...\n\n(Converted from Idea {})", &idea.description[..1990], idea_id)
        }
    });
    
    // Validate inputs
    require!(!proposal_title.is_empty(), IndrasError::InvalidInput);
    require!(proposal_title.len() <= 200, IndrasError::InvalidInput);
    require!(!proposal_description.is_empty(), IndrasError::InvalidInput);
    require!(proposal_description.len() <= 2000, IndrasError::InvalidInput);
    
    // Initialize proposal fields
    proposal.id = proposal_id;
    proposal.title = proposal_title;
    proposal.description = proposal_description;
    proposal.proposal_type = "idea_conversion".to_string();
    proposal.author = author;
    proposal.created_at = current_time;
    proposal.updated_at = None;
    proposal.submitted_at = None;
    proposal.cancelled_at = None;
    proposal.executed_at = None;
    proposal.archived_at = None;
    proposal.voting_duration = voting_duration.unwrap_or(7 * 24 * 3600); // 7 days default
    proposal.status = ProposalStatus::Draft;
    proposal.bump = bump;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.total_votes = 0;
    proposal.last_tallied_at = None;
    proposal.cancellation_reason = None;
    proposal.execution_data = None;
    proposal.expires_at = None;
    proposal.idea_id = Some(idea_id); // Link to original idea
    proposal.treasury_operation = None;
    
    msg!("Idea {} converted to proposal {} by {}", idea_id, proposal_id, author);
    Ok(())
}
