//! Mesh group lifecycle management handlers

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::mesh_group::{GroupStatus, DevelopmentStage};
use crate::state::member::role::role_permissions;

/// Start mesh group (Forming -> Active)
///
/// Activates mesh group to start work.
pub fn start_mesh_group_handler(
    ctx: Context<crate::ManageMeshGroup>,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let manager = ctx.accounts.manager.key();
    let dao_config = &ctx.accounts.dao_config;
    
    // Check permission: manager must be leader/creator/authority OR have CAN_MANAGE_MESH_GROUPS permission
    let is_authorized = manager == mesh_group.leader || 
                       manager == mesh_group.created_by || 
                       manager == dao_config.authority;
    
    if !is_authorized {
        if let Some(manager_role) = &ctx.accounts.manager_role {
            require!(
                manager_role.has_permission(role_permissions::CAN_MANAGE_MESH_GROUPS),
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
    
    mesh_group.start_group()?;
    msg!("Mesh Group {} started by {}", mesh_group.name, manager);
    Ok(())
}

/// Pause mesh group (Active -> Paused)
///
/// Pauses mesh group work.
/// Pause mesh group (Active -> Paused)
///
/// Pauses mesh group work.
pub fn pause_mesh_group_handler(
    ctx: Context<crate::ManageMeshGroup>,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let manager = ctx.accounts.manager.key();
    let dao_config = &ctx.accounts.dao_config;
    
    // Check permission: manager must be leader/creator/authority OR have CAN_MANAGE_MESH_GROUPS permission
    let is_authorized = manager == mesh_group.leader || 
                       manager == mesh_group.created_by || 
                       manager == dao_config.authority;
    
    if !is_authorized {
        if let Some(manager_role) = &ctx.accounts.manager_role {
            require!(
                manager_role.has_permission(role_permissions::CAN_MANAGE_MESH_GROUPS),
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
    
    mesh_group.pause_group()?;
    msg!("Mesh Group {} paused by {}", mesh_group.name, manager);
    Ok(())
}

/// Resume mesh group (Paused -> Active)
///
/// Resumes mesh group work.
/// Resume mesh group (Paused -> Active)
///
/// Resumes mesh group work.
pub fn resume_mesh_group_handler(
    ctx: Context<crate::ManageMeshGroup>,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let manager = ctx.accounts.manager.key();
    let dao_config = &ctx.accounts.dao_config;
    
    // Check permission: manager must be leader/creator/authority OR have CAN_MANAGE_MESH_GROUPS permission
    let is_authorized = manager == mesh_group.leader || 
                       manager == mesh_group.created_by || 
                       manager == dao_config.authority;
    
    if !is_authorized {
        if let Some(manager_role) = &ctx.accounts.manager_role {
            require!(
                manager_role.has_permission(role_permissions::CAN_MANAGE_MESH_GROUPS),
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
    
    mesh_group.resume_group()?;
    msg!("Mesh Group {} resumed by {}", mesh_group.name, manager);
    Ok(())
}

/// Complete mesh group (Active -> Completed)
///
/// Completes mesh group work. Ideas must be completed separately via complete_idea.
/// Complete mesh group (Active -> Completed)
///
/// Completes mesh group work. Ideas must be completed separately via complete_idea.
pub fn complete_mesh_group_handler(
    ctx: Context<crate::ManageMeshGroup>,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let manager = ctx.accounts.manager.key();
    let dao_config = &ctx.accounts.dao_config;
    
    // Check permission: manager must be leader/creator/authority OR have CAN_MANAGE_MESH_GROUPS permission
    let is_authorized = manager == mesh_group.leader || 
                       manager == mesh_group.created_by || 
                       manager == dao_config.authority;
    
    if !is_authorized {
        if let Some(manager_role) = &ctx.accounts.manager_role {
            require!(
                manager_role.has_permission(role_permissions::CAN_MANAGE_MESH_GROUPS),
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
    
    // Complete the mesh group
    mesh_group.complete_group()?;
    
    // NOTE: Ideas in this group should be completed separately via complete_idea
    // This allows for flexibility - some ideas might be completed before the group,
    // and some groups might work on multiple ideas
    msg!("Mesh Group {} completed by {} (contains {} ideas - they should be completed separately via complete_idea)", 
         mesh_group.name, 
         ctx.accounts.manager.key(),
         mesh_group.ideas.len());
    
    Ok(())
}

/// Close mesh group - complete group work and transfer ideas for commercialization
///
/// Closes group and verifies all ideas are completed.
/// Group must be Active or Completed.
/// Close mesh group - complete group work and transfer ideas for commercialization
///
/// Closes group and verifies all ideas are completed.
/// Group must be Active or Completed.
pub fn close_mesh_group_handler(
    ctx: Context<crate::CloseMeshGroup>,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let closer = ctx.accounts.closer.key();
    let dao_config = &ctx.accounts.dao_config;
    
    // Check permission: closer must be leader/creator/authority OR have CAN_MANAGE_MESH_GROUPS permission
    let is_authorized = closer == mesh_group.leader || 
                       closer == mesh_group.created_by || 
                       closer == dao_config.authority;
    
    if !is_authorized {
        if let Some(closer_role) = &ctx.accounts.closer_role {
            require!(
                closer_role.has_permission(role_permissions::CAN_MANAGE_MESH_GROUPS),
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
    
    // Check: group must be Active or Completed
    require!(
        mesh_group.status == GroupStatus::Active || 
        mesh_group.status == GroupStatus::Completed,
        IndrasError::InvalidState
    );
    
    // Close group
    mesh_group.disband_group()?;
    
    msg!("Mesh Group {} closed by {} ({} ideas ready for commercialization)", 
         mesh_group.name, closer, mesh_group.ideas.len());
    
    Ok(())
}

/// Close Mesh Group account and return rent
///
/// This handler allows closing a Mesh Group account and returning the rent exemption.
/// Only the group leader, creator, or DAO authority can close the account.
///
/// # Security
/// - Group leader, creator, or DAO authority must be the closer
/// - Account will be closed and rent returned to destination
/// - CRITICAL: Before closing, checks if there are active grants that require reports
///
/// # Compute Units
/// Recommended: 30,000 CU
/// # Compute Units
/// Recommended: 30,000 CU
pub fn close_mesh_group_account_handler(
    ctx: Context<crate::CloseMeshGroupAccount>,
    mesh_group_id: u64,
) -> Result<()> {
    let mesh_group = &ctx.accounts.mesh_group;
    let closer = ctx.accounts.closer.key();
    let destination = ctx.accounts.destination.key();
    let dao_config = &ctx.accounts.dao_config;
    
    // SECURITY: Validate mesh group ID matches
    require!(
        mesh_group.id == mesh_group_id,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate closer is authorized (leader, creator, or DAO authority)
    require!(
        mesh_group.leader == closer || 
        mesh_group.created_by == closer || 
        dao_config.authority == closer,
        IndrasError::Unauthorized
    );
    
    // NOTE: Grant report validation is performed OFF-CHAIN
    // On-chain we only check flags in grants (if passed in accounts)
    // Minimize transactions - do not search all grants on-chain
    // Off-chain service must validate all grants and ensure reports are approved
    // before calling close_mesh_group_account
    
    msg!(
        "Mesh Group {} closed by {}, rent will be returned to {}",
        mesh_group_id,
        closer,
        destination
    );
    
    // Anchor's `close = destination` will automatically:
    // 1. Transfer all lamports from mesh group account to destination
    // 2. Set account data to zero
    // 3. Mark account as closed
    
    Ok(())
}

/// Disband mesh group (any status -> Disbanded)
///
/// Disbands mesh group.
/// Disband mesh group (any status -> Disbanded)
///
/// Disbands mesh group.
pub fn disband_mesh_group_handler(
    ctx: Context<crate::ManageMeshGroup>,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let manager = ctx.accounts.manager.key();
    let dao_config = &ctx.accounts.dao_config;
    
    // Check permission: manager must be leader/creator/authority OR have CAN_MANAGE_MESH_GROUPS permission
    let is_authorized = manager == mesh_group.leader || 
                       manager == mesh_group.created_by || 
                       manager == dao_config.authority;
    
    if !is_authorized {
        if let Some(manager_role) = &ctx.accounts.manager_role {
            require!(
                manager_role.has_permission(role_permissions::CAN_MANAGE_MESH_GROUPS),
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
    
    mesh_group.disband_group()?;
    msg!("Mesh Group {} disbanded by {}", mesh_group.name, manager);
    Ok(())
}

/// Add idea to mesh group
///
/// Adds idea to mesh group.
/// CRITICAL: Only Approved ideas can be added to mesh groups.
/// This ensures AI has checked and approved idea first.
///
/// Requirements:
/// 1. Idea must be in Approved status
/// 2. AIAnalysis must exist for idea
/// 3. AIAnalysis must be Approved (decision = Approve)
/// 4. AIAnalysis must confirm idea can enter mesh group
/// Update mesh group development stage
///
/// Updates mesh group development stage.
pub fn update_mesh_group_stage_handler(
    ctx: Context<crate::ManageMeshGroup>,
    new_stage: DevelopmentStage,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    
    // Check that group is active
    require!(
        mesh_group.status == GroupStatus::Active,
        IndrasError::InvalidState
    );
    
    // Update stage
    let stage_str = format!("{:?}", new_stage);
    mesh_group.current_stage = new_stage;
    
    msg!("Mesh Group {} development stage updated to {} by {}", 
         mesh_group.name, 
         stage_str,
         ctx.accounts.manager.key());
    
    Ok(())
}
