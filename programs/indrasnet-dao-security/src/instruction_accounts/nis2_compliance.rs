use crate::security::compliance_checking::ComplianceCheck;
use crate::state::SecurityRoleRegistry;

#[derive(Accounts)]
pub struct RunNis2ComplianceCheck<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ComplianceCheck::INIT_SPACE,
        seeds = [b"compliance", subject.key().as_ref()],
        bump
    )]
    pub check: Account<'info, ComplianceCheck>,

    /// CHECK: Subject of compliance record (key used for PDA seeds)
    pub subject: AccountInfo<'info>,

    #[account(
        seeds = [b"security_roles"],
        bump = role_registry.bump
    )]
    pub role_registry: Account<'info, SecurityRoleRegistry>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
