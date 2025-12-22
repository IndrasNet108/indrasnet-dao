//! Proposal Template instruction handlers
//!
//! Handlers for proposal template operations: create, update, activate/deactivate, create from template

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::proposal::{ProposalTemplate, TemplateField};

/// Create a new proposal template
///
/// Creates a template without fields. Fields can be added later via add_template_field instruction.
pub fn create_proposal_template_handler(
    ctx: Context<crate::CreateProposalTemplate>,
    template_id: u64,
    name: String,
    description: String,
    proposal_type: String,
) -> Result<()> {
    let template = &mut ctx.accounts.template;
    let creator = ctx.accounts.creator.key();
    
    // Create template without fields initially
    let new_template = ProposalTemplate::new_with_time(
        template_id,
        name,
        description,
        proposal_type,
        vec![], // Start with empty fields
        creator,
        ctx.bumps.template,
        Clock::get()?.unix_timestamp,
    )?;
    
    // Copy fields to account
    template.template_id = new_template.template_id;
    template.name = new_template.name;
    template.description = new_template.description;
    template.proposal_type = new_template.proposal_type;
    template.fields = new_template.fields;
    template.created_by = new_template.created_by;
    template.created_at = new_template.created_at;
    template.updated_at = new_template.updated_at;
    template.is_active = new_template.is_active;
    template.bump = new_template.bump;
    
    msg!("Proposal template {} created by {} (fields can be added separately)", template_id, creator);
    Ok(())
}

/// Add a field to a proposal template
pub fn add_template_field_handler(
    ctx: Context<crate::AddTemplateField>,
    field_name: String,
    field_description: String,
    field_type: crate::state::proposal::TemplateFieldType,
    required: bool,
) -> Result<()> {
    let template = &mut ctx.accounts.template;
    
    require!(
        ctx.accounts.manager.key() == template.created_by || 
        ctx.accounts.manager.key() == ctx.accounts.dao_config.authority,
        IndrasError::Unauthorized
    );
    
    require!(template.fields.len() < 20, IndrasError::InvalidInput); // Max 20 fields
    require!(!field_name.is_empty(), IndrasError::InvalidInput);
    require!(field_name.len() <= 50, IndrasError::InvalidInput);
    require!(field_description.len() <= 200, IndrasError::InvalidInput);
    
    let new_field = TemplateField {
        name: field_name,
        description: field_description,
        field_type,
        required,
    };
    
    template.fields.push(new_field);
    template.updated_at = Some(Clock::get()?.unix_timestamp);
    
    msg!("Field added to template {}", template.template_id);
    Ok(())
}

/// Update proposal template
///
/// Note: To skip updating a field, pass empty string
pub fn update_proposal_template_handler(
    ctx: Context<crate::UpdateProposalTemplate>,
    name: String,
    description: String,
) -> Result<()> {
    let template = &mut ctx.accounts.template;
    
    require!(
        ctx.accounts.updater.key() == template.created_by || 
        ctx.accounts.updater.key() == ctx.accounts.dao_config.authority,
        IndrasError::Unauthorized
    );
    
    // Convert empty strings to None for update
    let name_opt = if name.is_empty() { None } else { Some(name) };
    let desc_opt = if description.is_empty() { None } else { Some(description) };
    
    template.update(name_opt, desc_opt, None)?; // Fields cannot be updated after creation
    
    msg!("Proposal template {} updated", template.template_id);
    Ok(())
}

/// Deactivate proposal template
pub fn deactivate_proposal_template_handler(
    ctx: Context<crate::ManageProposalTemplate>,
) -> Result<()> {
    let template = &mut ctx.accounts.template;
    
    require!(
        ctx.accounts.manager.key() == template.created_by || 
        ctx.accounts.manager.key() == ctx.accounts.dao_config.authority,
        IndrasError::Unauthorized
    );
    
    template.deactivate()?;
    
    msg!("Proposal template {} deactivated", template.template_id);
    Ok(())
}

/// Activate proposal template
pub fn activate_proposal_template_handler(
    ctx: Context<crate::ManageProposalTemplate>,
) -> Result<()> {
    let template = &mut ctx.accounts.template;
    
    require!(
        ctx.accounts.manager.key() == template.created_by || 
        ctx.accounts.manager.key() == ctx.accounts.dao_config.authority,
        IndrasError::Unauthorized
    );
    
    template.activate()?;
    
    msg!("Proposal template {} activated", template.template_id);
    Ok(())
}

/// Create proposal from template
///
/// This handler creates a new proposal using a template.
/// The values array should match the template fields in order.
pub fn create_proposal_from_template_handler(
    ctx: Context<crate::CreateProposalFromTemplate>,
    proposal_id: u64,
    template_id: u64,
    values: Vec<String>, // Values for template fields
    voting_duration: Option<i64>,
) -> Result<()> {
    let template = &ctx.accounts.template;
    let author = ctx.accounts.author.key();
    
    require!(template.template_id == template_id, IndrasError::InvalidInput);
    require!(template.is_active, IndrasError::InvalidState);
    require!(values.len() == template.fields.len(), IndrasError::InvalidInput);
    
    // Check permission: author must have CAN_PROPOSE permission
    if let Some(author_role) = &ctx.accounts.author_role {
        require!(
            author_role.has_permission(crate::state::member::role::role_permissions::CAN_PROPOSE),
            IndrasError::Unauthorized
        );
    }
    
    // Build title and description from template and values
    // For simplicity, we'll use template name as title and combine description with values
    let title = format!("{}: {}", template.name, 
        values.first().unwrap_or(&"".to_string()));
    
    let mut description = template.description.clone();
    description.push_str("\n\nTemplate Fields:\n");
    for (field, value) in template.fields.iter().zip(values.iter()) {
        description.push_str(&format!("{}: {}\n", field.name, value));
    }
    
    // Use template's proposal_type
    let proposal_type = template.proposal_type.clone();
    
    let proposal = &mut ctx.accounts.proposal;
    let bump = ctx.bumps.proposal;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate inputs
    require!(!title.is_empty(), IndrasError::InvalidInput);
    require!(title.len() <= 200, IndrasError::InvalidInput);
    require!(!description.is_empty(), IndrasError::InvalidInput);
    require!(description.len() <= 2000, IndrasError::InvalidInput);
    require!(!proposal_type.is_empty(), IndrasError::InvalidInput);
    require!(proposal_type.len() <= 50, IndrasError::InvalidInput);
    
    // Initialize proposal fields
    proposal.id = proposal_id;
    proposal.title = title;
    proposal.description = description;
    proposal.proposal_type = proposal_type;
    proposal.author = author;
    proposal.created_at = current_time;
    proposal.updated_at = None;
    proposal.submitted_at = None;
    proposal.cancelled_at = None;
    proposal.executed_at = None;
    proposal.archived_at = None;
    proposal.voting_duration = voting_duration.unwrap_or(7 * 24 * 3600); // 7 days default
    proposal.status = crate::state::proposal::ProposalStatus::Draft;
    proposal.bump = bump;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.total_votes = 0;
    proposal.last_tallied_at = None;
    proposal.cancellation_reason = None;
    proposal.execution_data = None;
    proposal.expires_at = None;
    proposal.idea_id = None;
    
    msg!("Proposal {} created from template {} by {}", proposal_id, template_id, author);
    Ok(())
}
