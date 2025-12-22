//! Handlers for Security role registry

use anchor_lang::prelude::*;
use crate::instruction_accounts::security_roles::*;

pub fn initialize_security_role_registry_handler(
    ctx: Context<InitializeSecurityRoleRegistry>,
) -> Result<()> {
    let registry = &mut ctx.accounts.role_registry;
    let authority = ctx.accounts.authority.key();

    registry.authority = authority;
    registry.security_admins = vec![authority];
    registry.compliance_admins = vec![authority];
    registry.analytics_admins = vec![authority];
    registry.bump = ctx.bumps.role_registry;

    msg!("Security role registry initialized by {}", authority);

    Ok(())
}

pub fn add_security_admin_handler(
    ctx: Context<UpdateSecurityRoleRegistry>,
    admin: Pubkey,
) -> Result<()> {
    let registry = &mut ctx.accounts.role_registry;
    registry.upsert_security_admin(admin);
    Ok(())
}

pub fn remove_security_admin_handler(
    ctx: Context<UpdateSecurityRoleRegistry>,
    admin: Pubkey,
) -> Result<()> {
    let registry = &mut ctx.accounts.role_registry;
    registry.remove_security_admin(&admin);
    Ok(())
}

pub fn add_compliance_admin_handler(
    ctx: Context<UpdateSecurityRoleRegistry>,
    admin: Pubkey,
) -> Result<()> {
    let registry = &mut ctx.accounts.role_registry;
    registry.upsert_compliance_admin(admin);
    Ok(())
}

pub fn remove_compliance_admin_handler(
    ctx: Context<UpdateSecurityRoleRegistry>,
    admin: Pubkey,
) -> Result<()> {
    let registry = &mut ctx.accounts.role_registry;
    registry.remove_compliance_admin(&admin);
    Ok(())
}

pub fn add_analytics_admin_handler(
    ctx: Context<UpdateSecurityRoleRegistry>,
    admin: Pubkey,
) -> Result<()> {
    let registry = &mut ctx.accounts.role_registry;
    registry.upsert_analytics_admin(admin);
    Ok(())
}

pub fn remove_analytics_admin_handler(
    ctx: Context<UpdateSecurityRoleRegistry>,
    admin: Pubkey,
) -> Result<()> {
    let registry = &mut ctx.accounts.role_registry;
    registry.remove_analytics_admin(&admin);
    Ok(())
}
