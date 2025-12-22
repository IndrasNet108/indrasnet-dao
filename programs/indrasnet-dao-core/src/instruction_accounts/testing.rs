/// Accounts for testing-only instructions

#[derive(anchor_lang::prelude::Accounts)]
pub struct TestingUpdateProposalStatus<'info> {
    pub dao_config: anchor_lang::prelude::Account<'info, crate::state::dao_config::DaoConfig>,
    #[account(mut)]
    pub proposal: anchor_lang::prelude::Account<'info, crate::state::proposal::Proposal>,
    #[account(constraint = authority.key() == dao_config.authority @ crate::error::IndrasError::Unauthorized)]
    pub authority: anchor_lang::prelude::Signer<'info>,
}