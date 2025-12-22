//! Mesh group member management handlers

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::mesh_group::{GroupStatus, GroupRole};
use crate::state::member::role::role_permissions;

/// Join mesh group
///
/// Member joins mesh group after approval by leader or creator.
pub fn join_mesh_group_handler(
    ctx: Context<crate::JoinMeshGroup>,
    role: GroupRole,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let member = ctx.accounts.member.key();
    
    // Check that user is not already in group
    require!(!mesh_group.is_member(member), IndrasError::AlreadyMember);
    
    // Check member limit (maximum 7 for mesh group)
    require!(
        mesh_group.members.len() < mesh_group.max_members as usize, 
        IndrasError::GroupFull
    );
    
    // Add member
    let new_member = crate::state::mesh_group::GroupMember {
        pubkey: member,
        role: role.clone(),
        joined_at: Clock::get()?.unix_timestamp,
        contributions: 0,
        reputation: 0,
        is_active: true,
    };
    
    // Add member (updates last_contribution_at)
    mesh_group.add_member(new_member, Clock::get()?.unix_timestamp)?;
    
    // Group activates automatically when reaching min_members
    if mesh_group.status == GroupStatus::Forming && mesh_group.members.len() >= mesh_group.min_members as usize {
        mesh_group.status = GroupStatus::Active;
        if mesh_group.started_at.is_none() {
            mesh_group.started_at = Some(Clock::get()?.unix_timestamp);
        }
        msg!("Mesh Group {} activated with {} members", mesh_group.name, mesh_group.members.len());
    }
    
    msg!("Member {} joined Mesh Group {} as {:?} (approved by {})", 
         member, mesh_group.name, role, ctx.accounts.approver.key());
    
    Ok(())
}

/// Remove member from mesh group
///
/// Remove member from mesh group. Leader cannot be removed.
/// Creator (founder) can remove directly, voting required for others.
/// Remove member from mesh group
///
/// Remove member from mesh group. Leader cannot be removed.
/// Creator (founder) can remove directly, voting required for others.
pub fn remove_mesh_group_member_handler(
    ctx: Context<crate::RemoveMeshGroupMember>,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let member_to_remove = ctx.accounts.member_to_remove.key();
    let remover = ctx.accounts.remover.key();
    let dao_config = &ctx.accounts.dao_config;
    
    // Cannot remove leader
    require!(member_to_remove != mesh_group.leader, IndrasError::CannotRemoveLeader);
    
    // Check if member exists
    require!(mesh_group.is_member(member_to_remove), IndrasError::NotFound);
    
    // SECURITY: Founder (created_by) has priority - can remove directly without voting
    let is_authorized = remover == mesh_group.created_by || 
                       remover == mesh_group.leader || 
                       remover == dao_config.authority;
    
    if !is_authorized {
        if let Some(remover_role) = &ctx.accounts.remover_role {
            require!(
                remover_role.has_permission(role_permissions::CAN_MANAGE_MESH_GROUPS),
                IndrasError::Unauthorized
            );
        } else {
            // SECURITY: In production, role must exist - fail if not found
            require!(
                false,
                IndrasError::Unauthorized
            );
        }
    }
    
    // Remove member (updates last_contribution_at)
    mesh_group.remove_member(member_to_remove, Clock::get()?.unix_timestamp)?;
    
    msg!("Member {} removed from Mesh Group {} by {}", 
         member_to_remove, mesh_group.name, remover);
    
    // If group falls below minimum members, update status if needed
    if mesh_group.members.len() < mesh_group.min_members as usize && 
       mesh_group.status == GroupStatus::Active {
        mesh_group.status = GroupStatus::Forming;
        msg!("Mesh Group {} returned to Forming status (insufficient members)", mesh_group.name);
    }
    
    Ok(())
}

/// Start mesh group (Forming -> Active)
///
/// Activates mesh group to start work.
/// Add contribution to mesh group
///
/// Records member contribution to mesh group work.
/// Increases contribution counter and member reputation.
pub fn add_contribution_handler(
    ctx: Context<crate::AddContribution>,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let contributor = ctx.accounts.contributor.key();
    
    // Check that member is part of group
    require!(mesh_group.is_member(contributor), IndrasError::NotFound);
    
    // Add contribution
    // Add contribution (updates last_contribution_at)
    let current_time = Clock::get()?.unix_timestamp;
    mesh_group.add_contribution(contributor, current_time)?;
    
    msg!("Contribution added for member {} in Mesh Group {} (total contributions: {})", 
         contributor, 
         mesh_group.name,
         mesh_group.total_contributions);
    
    Ok(())
}
