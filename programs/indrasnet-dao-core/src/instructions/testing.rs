use anchor_lang::prelude::*;
use crate::state::proposal::ProposalStatus;

/// Handler for `testing_update_proposal_status`
///
/// Updates the status of a proposal. This is a test-only function and should
/// only be callable by the DAO authority.
pub fn testing_update_proposal_status_handler(
    ctx: Context<crate::TestingUpdateProposalStatus>,
    new_status: ProposalStatus,
) -> Result<()> {
    msg!("Testing: Updating proposal {} to status {:?}", ctx.accounts.proposal.id, new_status);
    ctx.accounts.proposal.status = new_status;
    Ok(())
}
