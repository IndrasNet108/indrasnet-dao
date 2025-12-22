// Accounts structures for expert registry instructions

use crate::state::expert_registry::{ExpertRegistry, ExpertEntry, DomainExpertIndex};
use crate::state::member::{Member, MemberRole};

/// Initialize expert registry
#[derive(Accounts)]
pub struct InitializeExpertRegistry<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ExpertRegistry::INIT_SPACE,
        seeds = [b"expert_registry"],
        bump
    )]
    pub registry: Account<'info, ExpertRegistry>,
    
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    /// SECURITY: Only DAO authority can initialize expert registry
    #[account(
        mut,
        constraint = authority.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

/// Add expert to registry
#[derive(Accounts)]
#[instruction(domain_id: String)]
pub struct AddExpert<'info> {
    #[account(
        seeds = [b"expert_registry"],
        bump = registry.bump
    )]
    pub registry: Account<'info, ExpertRegistry>,
    
    /// Expert entry - PDA with expert and domain in seeds
    #[account(
        init,
        payer = authority,
        space = 8 + ExpertEntry::INIT_SPACE,
        seeds = [b"expert", expert.key().as_ref(), domain_id.as_bytes()],
        bump
    )]
    pub expert_entry: Account<'info, ExpertEntry>,
    
    /// Domain expert index - PDA with domain_id in seeds
    /// CHECK: May not exist yet (will be created if needed)
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + DomainExpertIndex::INIT_SPACE,
        seeds = [b"domain_experts", domain_id.as_bytes()],
        bump
    )]
    pub domain_index: Account<'info, DomainExpertIndex>,
    
    /// Member account for expert (to check reputation)
    #[account(
        constraint = member.pubkey == expert.key() @ IndrasError::InvalidInput
    )]
    pub member: Account<'info, Member>,
    
    /// Expert's public key (not a signer - added by authority)
    /// CHECK: Validated against member.pubkey
    pub expert: UncheckedAccount<'info>,
    
    /// SECURITY: Authority must be DAO authority OR have CAN_MANAGE_EXPERTS permission
    /// Note: Permission check is done in handler, but we require role account to exist
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Authority's role (to check permissions)
    /// CHECK: If provided, must have CAN_MANAGE_EXPERTS permission (unless DAO authority)
    #[account(
        seeds = [b"member_role", authority.key().as_ref()],
        bump = authority_role.bump,
        constraint = authority_role.member == authority.key() @ IndrasError::InvalidInput
    )]
    pub authority_role: Account<'info, MemberRole>,
    
    pub system_program: Program<'info, System>,
}

/// Remove expert from registry
#[derive(Accounts)]
#[instruction(domain_id: String)]
pub struct RemoveExpert<'info> {
    #[account(
        seeds = [b"expert_registry"],
        bump = registry.bump
    )]
    pub registry: Account<'info, ExpertRegistry>,
    
    /// Expert entry to deactivate
    #[account(
        mut,
        seeds = [b"expert", expert.key().as_ref(), domain_id.as_bytes()],
        bump = expert_entry.bump,
        constraint = expert_entry.expert == expert.key() @ IndrasError::InvalidInput,
        constraint = expert_entry.domain_id == domain_id @ IndrasError::InvalidInput
    )]
    pub expert_entry: Account<'info, ExpertEntry>,
    
    /// Domain expert index
    #[account(
        mut,
        seeds = [b"domain_experts", domain_id.as_bytes()],
        bump = domain_index.bump,
        constraint = domain_index.domain_id == domain_id @ IndrasError::InvalidInput
    )]
    pub domain_index: Account<'info, DomainExpertIndex>,
    
    /// Expert's public key
    /// CHECK: Validated in expert_entry constraint
    pub expert: UncheckedAccount<'info>,
    
    /// SECURITY: Authority must be DAO authority OR have CAN_MANAGE_EXPERTS permission
    /// Note: Permission check is done in handler, but we require role account to exist
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Authority's role (to check permissions)
    /// CHECK: If provided, must have CAN_MANAGE_EXPERTS permission (unless DAO authority)
    #[account(
        seeds = [b"member_role", authority.key().as_ref()],
        bump = authority_role.bump,
        constraint = authority_role.member == authority.key() @ IndrasError::InvalidInput
    )]
    pub authority_role: Account<'info, MemberRole>,
}

/// Update expert entry
#[derive(Accounts)]
pub struct UpdateExpert<'info> {
    /// Expert entry to update
    #[account(mut)]
    pub expert_entry: Account<'info, ExpertEntry>,
    
    /// SECURITY: Authority must be DAO authority OR have CAN_MANAGE_EXPERTS permission
    /// Note: Permission check is done in handler
    pub authority: Signer<'info>,
    
    /// Authority's role (to check permissions)
    /// CHECK: If provided, must have CAN_MANAGE_EXPERTS permission (unless DAO authority)
    #[account(
        seeds = [b"member_role", authority.key().as_ref()],
        bump = authority_role.bump,
        constraint = authority_role.member == authority.key() @ IndrasError::InvalidInput
    )]
    pub authority_role: Account<'info, MemberRole>,
}
