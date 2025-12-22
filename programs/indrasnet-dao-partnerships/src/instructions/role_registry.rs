//! Partnership role registry handlers

use anchor_lang::prelude::*;

pub fn initialize_partnership_role_registry_handler(
    ctx: Context<crate::InitializePartnershipRoleRegistry>,
) -> Result<()> {
    let registry = &mut ctx.accounts.role_registry;
    let authority = ctx.accounts.authority.key();

    registry.authority = authority;
    registry.partnership_admins = vec![authority];
    registry.revenue_admins = vec![authority];
    registry.metrics_admins = vec![authority];
    registry.bump = ctx.bumps.role_registry;

    msg!("Partnership role registry initialized by {}", authority);

    Ok(())
}

pub fn add_partnership_admin_handler(
    ctx: Context<crate::UpdatePartnershipRoleRegistry>,
    admin: Pubkey,
) -> Result<()> {
    ctx.accounts.role_registry.upsert_partnership_admin(admin);
    Ok(())
}

pub fn remove_partnership_admin_handler(
    ctx: Context<crate::UpdatePartnershipRoleRegistry>,
    admin: Pubkey,
) -> Result<()> {
    ctx.accounts.role_registry.remove_partnership_admin(&admin);
    Ok(())
}

pub fn add_revenue_admin_handler(
    ctx: Context<crate::UpdatePartnershipRoleRegistry>,
    admin: Pubkey,
) -> Result<()> {
    ctx.accounts.role_registry.upsert_revenue_admin(admin);
    Ok(())
}

pub fn remove_revenue_admin_handler(
    ctx: Context<crate::UpdatePartnershipRoleRegistry>,
    admin: Pubkey,
) -> Result<()> {
    ctx.accounts.role_registry.remove_revenue_admin(&admin);
    Ok(())
}

pub fn add_metrics_admin_handler(
    ctx: Context<crate::UpdatePartnershipRoleRegistry>,
    admin: Pubkey,
) -> Result<()> {
    ctx.accounts.role_registry.upsert_metrics_admin(admin);
    Ok(())
}

pub fn remove_metrics_admin_handler(
    ctx: Context<crate::UpdatePartnershipRoleRegistry>,
    admin: Pubkey,
) -> Result<()> {
    ctx.accounts.role_registry.remove_metrics_admin(&admin);
    Ok(())
}
