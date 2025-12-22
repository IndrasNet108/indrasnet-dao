//! Mesh group idea management handlers

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::enums::IdeaStatus;

use super::helpers::verify_ai_analysis;

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
pub fn add_idea_to_mesh_group_handler(
    ctx: Context<crate::LinkIdeaToMeshGroup>,
    idea_id: u64,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let idea = &ctx.accounts.idea;
    
    // Validate idea ID matches
    require!(idea.id == idea_id, IndrasError::InvalidInput);
    
    // CRITICAL: Only Approved ideas can be added to mesh groups
    // This ensures AI has reviewed and approved the idea first
    require!(
        idea.status == IdeaStatus::Approved,
        IndrasError::InvalidState
    );
    
    // CRITICAL: AI analysis REQUIRED for adding idea to mesh group
    // Without AI analysis with decision == Approve, idea cannot enter mesh group
    // CRITICAL: Check not only that account exists, but also that it's not empty
    // With allow-missing-optionals, Anchor may create Some(UncheckedAccount) with empty data
    let analysis_account = ctx.accounts.ai_analysis.as_ref()
        .ok_or(IndrasError::InvalidInput)?;
    let analysis_record = ctx.accounts.ai_analysis_record.as_ref()
        .ok_or(IndrasError::InvalidInput)?;
    
    // Check that account is not empty (has data)
    require!(
        !analysis_account.data_is_empty(),
        IndrasError::InvalidInput
    );
    
    // Verify AI analysis - decision must be Approve
    verify_ai_analysis(analysis_account, idea_id, &idea.key(), analysis_record)?;
    
    msg!("Idea {} added to mesh group with verified AI analysis (decision=Approve)", idea_id);
    
    // Add idea to mesh group
    mesh_group.add_idea(idea_id)?;
    
    msg!("Idea {} (Approved) added to Mesh Group {} by {}", 
         idea_id, mesh_group.name, ctx.accounts.linker.key());
    Ok(())
}

/// Remove idea from mesh group
///
/// Removes idea from mesh group.
/// Remove idea from mesh group
///
/// Removes idea from mesh group.
pub fn remove_idea_from_mesh_group_handler(
    ctx: Context<crate::LinkIdeaToMeshGroup>,
    idea_id: u64,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    
    mesh_group.remove_idea(idea_id)?;
    
    msg!("Idea {} removed from Mesh Group {} by {}", 
         idea_id, mesh_group.name, ctx.accounts.linker.key());
    Ok(())
}

/// Add contribution to mesh group
///
/// Records member contribution to mesh group work.
/// Increases contribution counter and member reputation.
/// 2. Idea must be in Approved status (after AI analysis)
/// 3. Mesh group must be Active
/// 4. Anchoring can be performed by group leader, group creator, idea author, or authority
pub fn anchor_idea_in_mesh_group_handler(
    ctx: Context<crate::AnchorIdeaInMeshGroup>,
    idea_id: u64,
    content_hash: [u8; 32],
) -> Result<()> {
    let mesh_group = &ctx.accounts.mesh_group;
    let idea = &mut ctx.accounts.idea;
    let anchor_record = &mut ctx.accounts.anchor_record;
    
    // Validate idea ID matches
    require!(idea.id == idea_id, IndrasError::InvalidInput);
    
    // Check 1: Idea must be in mesh group
    require!(
        mesh_group.ideas.contains(&idea_id),
        IndrasError::IdeaNotInMeshGroup
    );
    
    // Check 2: Idea must be in Approved status (after AI analysis)
    require!(
        idea.status == IdeaStatus::Approved,
        IndrasError::InvalidState
    );
    
    // Check 3: Mesh group must be Active
    require!(
        mesh_group.status == crate::state::mesh_group::GroupStatus::Active,
        IndrasError::InvalidState
    );
    
    // Check 4: Idea not yet anchored (anchored_at not set)
    // NOTE: Current Idea structure has no anchored_at field, but we can check via AnchorRecord
    // If AnchorRecord already exists, idea is already anchored
    
    // Create AnchorRecord to fix idea in blockchain
    let current_time = Clock::get()?.unix_timestamp;
    anchor_record.idea_id = idea_id;
    
    // Form anchor_data with anchoring information
    let anchor_data = format!(
        "Idea anchored in mesh group {} at {}. Content hash: {}, Anchorer: {}, Author: {}",
        mesh_group.name,
        current_time,
        hex::encode(content_hash),
        ctx.accounts.anchorer.key(),
        idea.author
    );
    anchor_record.anchor_data = anchor_data;
    anchor_record.anchored_at = current_time;
    anchor_record.anchorer = ctx.accounts.anchorer.key();
    anchor_record.bump = ctx.bumps.anchor_record;
    
    // After anchoring, idea remains in Approved status
    // (status already Approved, as anchoring happens after AI analysis)
    
    msg!("Idea {} anchored in blockchain within mesh group {} by {} (content hash: {}, anchored at {})", 
         idea_id, 
         mesh_group.name,
         ctx.accounts.anchorer.key(),
         hex::encode(content_hash),
         current_time);
    msg!("After anchoring, author {} can now invite users to mesh group", idea.author);
    
    Ok(())
}
