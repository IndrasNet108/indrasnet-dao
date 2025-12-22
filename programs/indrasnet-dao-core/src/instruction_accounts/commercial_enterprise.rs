// Accounts structures for commercial enterprise instructions
// NOTE: Types like DaoConfig, Idea, Grant, MeshGroup, CommercialEnterprise, AnchorRecord and anchor_lang types are already imported in lib.rs before include!()

#[derive(Accounts)]
#[instruction(enterprise_id: u64)]
pub struct CreateCommercialEnterprise<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + CommercialEnterprise::INIT_SPACE,
        seeds = [b"commercial_enterprise", enterprise_id.to_le_bytes().as_ref()],
        bump
    )]
    pub enterprise: Account<'info, CommercialEnterprise>,
    
    #[account(
        mut,
        constraint = creator.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub creator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

/// Accounts for transferring idea to commercial enterprise
/// 
/// CRITICAL: Check intellectual property rights
/// - Idea author remains copyright owner
/// - e.V. received commercialization right via grant approval OR voluntary transfer
/// - e.V. is custodian of author's copyright
#[derive(Accounts)]
#[instruction(idea_id: u64, enterprise_id: u64)]
pub struct TransferIdeaToCommercialEnterprise<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut, constraint = idea.id == idea_id @ IndrasError::InvalidInput)]
    pub idea: Account<'info, Idea>,
    
    #[account(mut, constraint = enterprise.id == enterprise_id @ IndrasError::InvalidInput)]
    pub enterprise: Account<'info, CommercialEnterprise>,
    
    /// Mesh group that worked on this idea (optional, but recommended)
    /// CHECK: Validated in handler - must be closed/completed, MVP ready
    pub mesh_group: Option<UncheckedAccount<'info>>,
    
    /// Grant account - optional, validated in handler (for checking commercialization rights)
    /// CHECK: If grant provided, checks commercialization_right_transferred
    pub grant: Option<UncheckedAccount<'info>>,
    
    /// AnchorRecord to verify authorship (author is IP owner)
    /// CHECK: Validated in handler - anchor_record.anchorer == idea.author
    pub anchor_record: Account<'info, AnchorRecord>,
    
    /// Transferrer must be e.V. (dao_config.authority)
    /// e.V. has commercialization right (received from author via grant approval or voluntary transfer)
    #[account(
        constraint = transferrer.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub transferrer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(enterprise_id: u64)]
pub struct AddInvestor<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        constraint = enterprise.id == enterprise_id @ IndrasError::InvalidInput
    )]
    pub enterprise: Account<'info, CommercialEnterprise>,
    
    #[account(
        constraint = adder.key() == dao_config.authority || adder.key() == enterprise.enterprise_pubkey @ IndrasError::Unauthorized
    )]
    pub adder: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(enterprise_id: u64)]
pub struct UpdateProductionStatus<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        constraint = enterprise.id == enterprise_id @ IndrasError::InvalidInput
    )]
    pub enterprise: Account<'info, CommercialEnterprise>,
    
    #[account(
        constraint = updater.key() == dao_config.authority || updater.key() == enterprise.enterprise_pubkey @ IndrasError::Unauthorized
    )]
    pub updater: Signer<'info>,
}
