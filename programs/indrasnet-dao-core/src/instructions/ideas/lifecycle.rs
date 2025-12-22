//! Idea lifecycle management handlers

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::enums::IdeaStatus;
use crate::state::proposal_execution::ExecutionStatus;

/// Complete idea work - transition from InProgress to Completed
///
/// Requires idea to be in InProgress status.
/// Optional mesh group validation is done in handler if provided.
pub fn complete_idea_handler(
    ctx: Context<crate::CompleteIdea>,
    idea_id: u64,
    completion_report: String,
) -> Result<()> {
    // SECURITY: Validate completion_report length to prevent DoS
    require!(
        completion_report.len() <= 2000,
        IndrasError::DataTooLarge
    );
    
    let idea = &mut ctx.accounts.idea;
    let dao_config = &ctx.accounts.dao_config;
    
    // Validate idea ID matches
    require!(idea.id == idea_id, IndrasError::InvalidInput);
    
    // Validate idea is in InProgress status
    require!(
        idea.status == IdeaStatus::InProgress,
        IndrasError::InvalidState
    );
    
    // Validate completion report
    require!(!completion_report.is_empty(), IndrasError::InvalidInput);
    require!(completion_report.len() <= 2000, IndrasError::StringTooLong);
    
    // Validate completer has permission (checked in Accounts struct, but double-check)
    require!(
        ctx.accounts.completer.key() == dao_config.authority || 
        ctx.accounts.completer.key() == idea.author,
        IndrasError::Unauthorized
    );
    
    // Update idea status to Completed
    idea.status = IdeaStatus::Completed;
    
    msg!("Idea {} completed by {} (report: {})", 
         idea_id, 
         ctx.accounts.completer.key(),
         completion_report);
    
    Ok(())
}

/// Archive idea - transition to Archived status
///
/// Can archive Completed, Executed, or Rejected ideas.
pub fn archive_idea_handler(
    ctx: Context<crate::ArchiveIdea>,
    idea_id: u64,
    reason: String,
) -> Result<()> {
    let idea = &mut ctx.accounts.idea;
    
    // Validate idea ID matches
    require!(idea.id == idea_id, IndrasError::InvalidInput);
    
    // Validate idea is in a valid state for archiving
    require!(
        idea.status == IdeaStatus::Completed ||
        idea.status == IdeaStatus::Executed ||
        idea.status == IdeaStatus::Rejected,
        IndrasError::InvalidState
    );
    
    // Validate reason
    require!(!reason.is_empty(), IndrasError::InvalidInput);
    require!(reason.len() <= 500, IndrasError::StringTooLong);
    
    // Update idea status to Archived
    idea.status = IdeaStatus::Archived;
    
    msg!("Idea {} archived by {} (reason: {})", 
         idea_id, 
         ctx.accounts.archiver.key(),
         reason);
    
    Ok(())
}

/// Resubmit rejected idea - transition from Rejected to Resubmitted
///
/// Allows updating title and description.
pub fn resubmit_idea_handler(
    ctx: Context<crate::ResubmitIdea>,
    idea_id: u64,
    updated_title: Option<String>,
    updated_description: Option<String>,
) -> Result<()> {
    let idea = &mut ctx.accounts.idea;
    
    // Validate idea ID matches
    require!(idea.id == idea_id, IndrasError::InvalidInput);
    
    // Validate idea is in Rejected status
    require!(
        idea.status == IdeaStatus::Rejected,
        IndrasError::InvalidState
    );
    
    // Update fields if provided
    if let Some(title) = updated_title {
        require!(!title.is_empty(), IndrasError::InvalidInput);
        require!(title.len() <= 100, IndrasError::StringTooLong);
        idea.title = title;
    }
    
    if let Some(description) = updated_description {
        // SECURITY: Validate description is not empty and within length limit
        require!(!description.is_empty(), IndrasError::InvalidInput);
        require!(description.len() <= 500, IndrasError::StringTooLong);
        idea.description = description;
    }
    
    // Transition: Rejected → Resubmitted (for new AI review)
    idea.status = IdeaStatus::Resubmitted;
    
    msg!("Idea {} resubmitted by {} (will be reviewed again)", 
         idea_id, 
         ctx.accounts.resubmitter.key());
    
    Ok(())
}

/// Execute idea - transition from Completed to Executed
///
/// Requires Completed status before Executed.
pub fn execute_idea_handler(
    ctx: Context<crate::ExecuteIdea>,
    idea_id: u64,
    execution_data: String,
) -> Result<()> {
    let idea = &mut ctx.accounts.idea;
    let dao_config = &ctx.accounts.dao_config;
    
    // Validate idea ID matches
    require!(idea.id == idea_id, IndrasError::InvalidInput);
    
    // Validate executor has permission (checked in Accounts struct, but double-check)
    require!(
        ctx.accounts.executor.key() == dao_config.authority || 
        ctx.accounts.executor.key() == idea.author,
        IndrasError::Unauthorized
    );
    
    // Requires Completed status before Executed
    require!(
        idea.status == IdeaStatus::Completed,
        IndrasError::InvalidState
    );
    
    // Validate execution data
    require!(!execution_data.is_empty(), IndrasError::InvalidInput);
    require!(execution_data.len() <= 1000, IndrasError::StringTooLong);
    
    // Update idea status
    idea.status = IdeaStatus::Executed;
    
    // Create execution record
    let idea_execution = &mut ctx.accounts.idea_execution;
    idea_execution.id = idea_id;
    idea_execution.idea_id = idea_id;
    idea_execution.executor = ctx.accounts.executor.key();
    idea_execution.execution_data = execution_data;
    idea_execution.status = ExecutionStatus::Completed;
    idea_execution.executed_at = Clock::get()?.unix_timestamp;
    idea_execution.bump = ctx.bumps.idea_execution;
    
    msg!("Idea {} executed by {}", idea_id, ctx.accounts.executor.key());
    
    Ok(())
}
