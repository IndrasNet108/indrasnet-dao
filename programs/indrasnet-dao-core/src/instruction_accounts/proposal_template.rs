// Accounts structures for proposal template instructions
// NOTE: Types like DaoConfig, ProposalTemplate, Proposal, TemplateField and anchor_lang types are already imported in lib.rs before include!()

#[derive(Accounts)]
#[instruction(template_id: u64)]
pub struct CreateProposalTemplate<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + ProposalTemplate::INIT_SPACE,
        seeds = [b"proposal_template", dao_config.key().as_ref(), &template_id.to_le_bytes()],
        bump
    )]
    pub template: Account<'info, ProposalTemplate>,
    
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    /// SECURITY: Only DAO authority can create proposal templates
    #[account(
        mut,
        constraint = creator.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub creator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProposalTemplate<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub template: Account<'info, ProposalTemplate>,
    
    /// SECURITY: Only DAO authority can update proposal templates
    #[account(
        mut,
        constraint = updater.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub updater: Signer<'info>,
}

#[derive(Accounts)]
pub struct ManageProposalTemplate<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub template: Account<'info, ProposalTemplate>,
    
    /// SECURITY: Only DAO authority can manage proposal templates
    #[account(
        mut,
        constraint = manager.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub manager: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddTemplateField<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub template: Account<'info, ProposalTemplate>,
    
    /// SECURITY: Only DAO authority can manage proposal templates
    #[account(
        mut,
        constraint = manager.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub manager: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64, template_id: u64)]
pub struct CreateProposalFromTemplate<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    pub template: Account<'info, ProposalTemplate>,
    
    #[account(
        init,
        payer = author,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    /// Author's role (optional - for permission check)
    /// CHECK: If provided, must have CAN_PROPOSE permission (unless DAO authority)
    pub author_role: Option<Account<'info, crate::state::member::MemberRole>>,
    
    #[account(mut)]
    pub author: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
