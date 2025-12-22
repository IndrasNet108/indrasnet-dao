/// Member Management instruction accounts
///
/// Account structures for member management operations: leave_dao
use anchor_lang::prelude::{Account, Program, Signer, System};

/// Accounts for leaving DAO
#[derive(Accounts)]
pub struct LeaveDao<'info> {
    /// Member account to close
    /// CHECK: Account will be closed and rent returned to destination
    #[account(
        mut,
        close = destination,
        constraint = member.pubkey == member_pubkey.key() @ crate::error::IndrasError::Unauthorized
    )]
    pub member: Account<'info, crate::state::member::Member>,
    
    /// Destination account to receive rent
    /// CHECK: Must be signer to receive rent
    #[account(mut)]
    pub destination: Signer<'info>,
    
    /// Member pubkey (must match member.pubkey and be signer)
    /// CHECK: Must be signer and match member.pubkey
    pub member_pubkey: Signer<'info>,
    
    /// DAO config for validation
    /// CHECK: Read-only, used for validation
    pub dao_config: Account<'info, crate::state::DaoConfig>,
    
    /// System program for account closure
    pub system_program: Program<'info, System>,
}
