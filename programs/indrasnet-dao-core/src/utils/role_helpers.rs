//! Role validation helpers for Core program

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::member::MemberRole;

/// Strict role validation:
/// - PDA matches signer
/// - owner == core program
/// - member matches signer
/// - required permission present
pub fn assert_role(
    role_info: &AccountInfo,
    signer: &Pubkey,
    required_permission: u64,
    program_id: &Pubkey,
) -> Result<()> {
    require!(role_info.owner == program_id, IndrasError::InvalidProgram);

    let (expected_pda, _) =
        Pubkey::find_program_address(&[b"member_role", signer.as_ref()], program_id);
    require!(role_info.key() == expected_pda, IndrasError::InvalidProgram);

    let data = role_info.try_borrow_data()?;
    require!(data.len() >= 8, IndrasError::InvalidInput);
    let mut data_slice = &data[8..];
    let role = MemberRole::try_deserialize(&mut data_slice)
        .map_err(|_| IndrasError::InvalidInput)?;

    require!(role.member == *signer, IndrasError::Unauthorized);
    require!(
        role.has_permission(required_permission),
        IndrasError::Unauthorized
    );

    Ok(())
}
