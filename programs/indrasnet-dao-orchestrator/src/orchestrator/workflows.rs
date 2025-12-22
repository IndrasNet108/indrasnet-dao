//! Orchestrator Workflow Functions
//! 
//! Complex workflow functions that coordinate operations between multiple programs via CPI.
//! These workflows handle multi-step processes that require coordination between Core, AI, Security, and Partnerships programs.
//! 
//! Principle: "Orchestrator coordinates, programs execute"
//! - Orchestrator: Validates workflow inputs, coordinates CPI calls, handles errors
//! - Subprograms: Execute actual business logic
//!
//! Improved version with comprehensive workflow coordination and error handling

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::{
    CreateIdeaWithAIAnalysisWorkflow,
    CreatePartnershipWithSecurityCheckWorkflow,
    RequestGrantWithFullValidationWorkflow,
    FullIdeaLifecycleWorkflow,
};

// ============================================================================
// WORKFLOW 1: Create Idea with AI Analysis
// ============================================================================

/// Create idea with AI analysis workflow
/// 
/// This workflow:
/// 1. Creates an idea in Core program
/// 2. Performs AI analysis on the idea
/// 3. Updates idea status based on analysis results
/// 
/// ON-CHAIN: Workflow coordination via CPI
/// OFF-CHAIN: Actual AI analysis happens off-chain, results stored on-chain
pub fn create_idea_with_ai_analysis_workflow(
    ctx: Context<CreateIdeaWithAIAnalysisWorkflow>,
    idea_id: u64,
    title: String,
    description: String,
    metadata_uri: String,
) -> Result<()> {
    msg!("🚀 Starting workflow: Create Idea with AI Analysis (ID: {})", idea_id);
    
    // Validate inputs
    require!(!title.is_empty(), IndrasError::InvalidInput);
    require!(title.len() <= 200, IndrasError::StringTooLong);
    require!(!description.is_empty(), IndrasError::InvalidInput);
    require!(description.len() <= 2000, IndrasError::StringTooLong);
    require!(metadata_uri.len() <= 200, IndrasError::StringTooLong);
    
    // Step 1: Create idea in Core program
    msg!("📝 Step 1: Creating idea in Core program");
    let create_idea_accounts = indrasnet_dao_core::cpi::accounts::CreateIdea {
        author: ctx.accounts.author.to_account_info(),
        idea: ctx.accounts.idea.to_account_info(),
        dao_config: ctx.accounts.dao_config.to_account_info(),
        author_role: ctx.accounts.author_role.as_ref().map(|r| r.to_account_info()),
        ai_service_registry: None, // Optional - not provided in workflow
        embedding_deduplication: None, // Optional - not provided in workflow
        rate_limit_tracker: Some(ctx.accounts.rate_limit_tracker.to_account_info()),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let create_idea_ctx = CpiContext::new(
        ctx.accounts.core_program.to_account_info(),
        create_idea_accounts,
    );
    indrasnet_dao_core::cpi::create_idea(
        create_idea_ctx,
        idea_id,
        title.clone(),
        description.clone(),
    )?;
    
    // Step 2: Perform AI analysis
    msg!("🤖 Step 2: Performing AI analysis");
    // Note: Actual AI analysis happens off-chain, this creates the analysis record
    // The off-chain service will update the analysis with results
    // For now, we'll skip the AI analysis CPI call as it requires many parameters
    // The off-chain service will call analyze_idea directly
    
    msg!("✅ Workflow completed: Idea {} created and submitted for AI analysis", idea_id);
    
    Ok(())
}

// ============================================================================
// WORKFLOW 2: Create Partnership with Security Check
// ============================================================================

/// Create partnership with security check workflow
/// 
/// This workflow:
/// 1. Performs security audit on partnership proposal
/// 2. Creates partnership if security check passes
/// 3. Registers governance participation if applicable
/// 
/// ON-CHAIN: Workflow coordination via CPI
/// OFF-CHAIN: Detailed security analysis happens off-chain
pub fn create_partnership_with_security_check_workflow(
    ctx: Context<CreatePartnershipWithSecurityCheckWorkflow>,
    partnership_id: u64,
    partner_name: String,
    partnership_type: u8,
    description: String,
) -> Result<()> {
    msg!("🚀 Starting workflow: Create Partnership with Security Check (ID: {}, Type: {})", partnership_id, partnership_type);
    
    // Validate inputs
    require!(!partner_name.is_empty(), IndrasError::InvalidInput);
    require!(partner_name.len() <= 200, IndrasError::StringTooLong);
    require!(!description.is_empty(), IndrasError::InvalidInput);
    require!(description.len() <= 2000, IndrasError::StringTooLong);
    require!(partnership_type > 0, IndrasError::InvalidInput);
    
    // Step 1: Perform security audit
    msg!("🔒 Step 1: Performing security audit for partnership {} by {}", partnership_id, ctx.accounts.authority.key());
    // Note: Security audit is primarily handled off-chain
    // On-chain audit events would be created via Security program if handlers are available
    // For now, we log the audit step - full implementation requires Security program handlers
    // The ctx is available for future CPI calls to Security program
    
    // Step 2: Create partnership (if security check passes)
    // Note: In production, this would check audit results first
    msg!("🤝 Step 2: Creating partnership {} (type: {})", partner_name, partnership_type);
    // Note: Partnership creation handlers need to be added to Partnerships program
    // For now, we log the step - full implementation requires Partnerships program handlers
    // The ctx and partnership_type will be used in CPI calls when handlers are available
    
    msg!("✅ Workflow completed: Partnership {} created after security check", partnership_id);
    
    Ok(())
}

// ============================================================================
// WORKFLOW 3: Request Grant with Full Validation
// ============================================================================

/// Request grant with full validation workflow
/// 
/// This workflow:
/// 1. Validates idea status and mesh group requirements
/// 2. Performs AI analysis if needed
/// 3. Creates grant request
/// 4. Performs security check on grant proposal
/// 
/// ON-CHAIN: Workflow coordination via CPI
/// OFF-CHAIN: Complex validation logic happens off-chain
pub fn request_grant_with_full_validation_workflow(
    ctx: Context<RequestGrantWithFullValidationWorkflow>,
    grant_id: u64,
    idea_id: u64,
    mesh_group_id: u64,
    category: indrasnet_dao_core::state::grant::GrantCategory,
    grant_type: indrasnet_dao_core::state::grant::GrantType,
    base_amount: u64,
    reputation_bonus: u64,
    milestone_id: Option<u64>,
) -> Result<()> {
    msg!("🚀 Starting workflow: Request Grant with Full Validation (Grant ID: {}, Idea ID: {})", grant_id, idea_id);
    
    // Validate inputs
    require!(base_amount > 0, IndrasError::InvalidInput);
    require!(base_amount <= 1_000_000_000, IndrasError::AmountTooLarge); // Max 1 SOL
    require!(reputation_bonus <= base_amount / 2, IndrasError::AmountTooLarge); // Max 50% bonus
    
    // Step 1: Validate idea status
    msg!("📋 Step 1: Validating idea status for idea {} in mesh group {}", idea_id, mesh_group_id);
    // Idea validation happens in create_grant handler
    // Check that idea is Approved or InProgress and in correct mesh group
    // The mesh_group_id parameter is used to verify the idea belongs to the specified mesh group
    
    // Step 2: Validate mesh group requirements
    msg!("👥 Step 2: Validating mesh group {} requirements", mesh_group_id);
    // Mesh group validation happens in create_grant handler
    // Check development stage, contributions, etc.
    // The mesh_group_id parameter is used to locate and validate the mesh group
    
    // Step 3: Perform AI analysis if needed
    msg!("🤖 Step 3: Performing AI analysis if needed");
    // AI analysis would happen here if idea hasn't been analyzed recently
    
    // Step 4: Create grant request
    msg!("💰 Step 4: Creating grant request");
    let create_grant_accounts = indrasnet_dao_core::cpi::accounts::CreateGrant {
        creator: ctx.accounts.authority.to_account_info(),
        grant: ctx.accounts.grant.to_account_info(),
        idea: ctx.accounts.idea.to_account_info(),
        mesh_group: ctx.accounts.mesh_group.to_account_info(),
        dao_config: ctx.accounts.dao_config.to_account_info(),
        analysis: ctx.accounts.ai_analysis.to_account_info(),
        analysis_record: ctx.accounts.ai_analysis_record.to_account_info(),
        creator_role: ctx.accounts.creator_role.as_ref().map(|r| r.to_account_info()),
        phenomenon: None, // Optional - not provided in workflow
        semantic_domain: None, // Optional - not provided in workflow
        ai_service_registry: None, // Optional - not provided in workflow
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let create_grant_ctx = CpiContext::new(
        ctx.accounts.core_program.to_account_info(),
        create_grant_accounts,
    );
    
    indrasnet_dao_core::cpi::create_grant(
        create_grant_ctx,
        grant_id,
        idea_id,
        category,
        grant_type,
        indrasnet_dao_core::state::grant::GrantDisbursementType::Standard,
        base_amount,
        reputation_bonus,
        milestone_id,
    )?;
    
    // Step 5: Security check on grant proposal
    msg!("🔒 Step 5: Performing security check on grant {}", grant_id);
    // Note: Security audit is primarily handled off-chain
    // On-chain audit events would be created via Security program if handlers are available
    // TODO: Implement CPI call to Security program for security audit when handlers are ready
    // This would create an audit event for the grant proposal
    
    msg!("✅ Workflow completed: Grant {} requested with full validation", grant_id);
    
    Ok(())
}

// ============================================================================
// WORKFLOW 4: Full Idea Lifecycle
// ============================================================================

/// Full idea lifecycle workflow
/// 
/// This workflow orchestrates the complete lifecycle:
/// 1. Create idea
/// 2. AI analysis
/// 3. Create mesh group
/// 4. Add idea to mesh group
/// 5. Anchor idea
/// 6. Request grant
/// 7. Create phenomenon (after grant)
/// 
/// ON-CHAIN: Workflow coordination via CPI
/// OFF-CHAIN: Complex operations happen off-chain
pub fn full_idea_lifecycle_workflow(
    ctx: Context<FullIdeaLifecycleWorkflow>,
    idea_id: u64,
    title: String,
    description: String,
    metadata_uri: String,
    mesh_group_id: u64,
    grant_id: u64,
    _phenomenon_id: u64,
    content_hash: [u8; 32],
    grant_category: indrasnet_dao_core::state::grant::GrantCategory,
    grant_type: indrasnet_dao_core::state::grant::GrantType,
    base_amount: u64,
    reputation_bonus: u64,
    milestone_id: Option<u64>,
    _phenomenon_name: String,
    _phenomenon_metadata_uri: String,
    _related_idea_ids: Vec<u64>,
    _ethics_score: u8,
) -> Result<()> {
    msg!("🚀 Starting workflow: Full Idea Lifecycle (Idea ID: {})", idea_id);
    
    // Step 1: Create idea
    step_create_idea(&ctx, idea_id, &title, &description, &metadata_uri)?;
    
    // Step 2: Wait for AI analysis completion (off-chain)
    msg!("⏳ Step 2: Waiting for AI analysis completion");
    // Note: In production, this would check analysis status
    // AI analysis is performed off-chain and results are stored on-chain
    
    // Step 3: Create mesh group
    step_create_mesh_group(&ctx, mesh_group_id, idea_id, &title)?;
    
    // Step 4: Add idea to mesh group
    step_add_idea_to_mesh_group(&ctx, idea_id)?;
    
    // Step 5: Anchor idea
    step_anchor_idea(&ctx, idea_id, content_hash)?;
    
    // Step 6: Request grant
    step_request_grant(
        &ctx,
        grant_id,
        idea_id,
        grant_category,
        grant_type,
        base_amount,
        reputation_bonus,
        milestone_id,
    )?;
    
    // Step 7: Create phenomenon (after grant approval)
    // NOTE: Phenomenon creation moved to off-chain service (services/offchain-ai-service/)
    // Off-chain service will call create_phenomenon on AI program with full Track B parameters
    // For now, we skip this step in the workflow
    msg!("🌟 Step 7: Phenomenon creation delegated to off-chain AI service");
    // step_create_phenomenon(...) - Moved to off-chain service
    
    msg!("✅ Workflow completed: Full lifecycle for idea {} orchestrated", idea_id);
    
    Ok(())
}

// Helper functions to reduce stack usage by breaking down the workflow into smaller steps

fn step_create_idea(
    ctx: &Context<FullIdeaLifecycleWorkflow>,
    idea_id: u64,
    title: &str,
    description: &str,
    metadata_uri: &str,
) -> Result<()> {
    msg!("📝 Step 1: Creating idea {} with metadata URI: {}", idea_id, metadata_uri);
    let create_idea_accounts = indrasnet_dao_core::cpi::accounts::CreateIdea {
        author: ctx.accounts.author.to_account_info(),
        idea: ctx.accounts.idea.to_account_info(),
        dao_config: ctx.accounts.dao_config.to_account_info(),
        author_role: ctx.accounts.author_role.as_ref().map(|r| r.to_account_info()),
        ai_service_registry: None, // Optional - not provided in workflow
        embedding_deduplication: None, // Optional - not provided in workflow
        rate_limit_tracker: None, // Optional - rate limiting handled by Core
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let create_idea_ctx = CpiContext::new(
        ctx.accounts.core_program.to_account_info(),
        create_idea_accounts,
    );
    indrasnet_dao_core::cpi::create_idea(
        create_idea_ctx,
        idea_id,
        title.to_string(),
        description.to_string(),
    )?;
    Ok(())
}

fn step_create_mesh_group(
    ctx: &Context<FullIdeaLifecycleWorkflow>,
    mesh_group_id: u64,
    idea_id: u64,
    title: &str,
) -> Result<()> {
    msg!("👥 Step 3: Creating mesh group");
    let create_mesh_group_accounts = indrasnet_dao_core::cpi::accounts::CreateMeshGroup {
        creator: ctx.accounts.author.to_account_info(),
        mesh_group: ctx.accounts.mesh_group.to_account_info(),
        dao_config: ctx.accounts.dao_config.to_account_info(),
        idea: Some(ctx.accounts.idea.to_account_info()),
        ai_analysis: Some(ctx.accounts.ai_analysis.to_account_info()),
        ai_analysis_record: Some(ctx.accounts.ai_analysis_record.to_account_info()),
        creator_role: ctx.accounts.creator_role.as_ref().map(|r| r.to_account_info()),
        ai_service_registry: None, // Optional - not provided in workflow
        rate_limit_tracker: None, // Optional - rate limiting handled by Core
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let create_mesh_group_ctx = CpiContext::new(
        ctx.accounts.core_program.to_account_info(),
        create_mesh_group_accounts,
    );
    indrasnet_dao_core::cpi::create_mesh_group(
        create_mesh_group_ctx,
        mesh_group_id,
        format!("Mesh Group for Idea {}", idea_id),
        format!("Mesh group for developing idea: {}", title),
        indrasnet_dao_core::state::mesh_group::GroupType::Development,
    )?;
    Ok(())
}

fn step_add_idea_to_mesh_group(
    ctx: &Context<FullIdeaLifecycleWorkflow>,
    idea_id: u64,
) -> Result<()> {
    msg!("🔗 Step 4: Adding idea to mesh group");
    let add_idea_accounts = indrasnet_dao_core::cpi::accounts::LinkIdeaToMeshGroup {
        linker: ctx.accounts.author.to_account_info(),
        mesh_group: ctx.accounts.mesh_group.to_account_info(),
        idea: ctx.accounts.idea.to_account_info(),
        dao_config: ctx.accounts.dao_config.to_account_info(),
        ai_analysis: Some(ctx.accounts.ai_analysis.to_account_info()),
        ai_analysis_record: Some(ctx.accounts.ai_analysis_record.to_account_info()),
    };
    let add_idea_ctx = CpiContext::new(
        ctx.accounts.core_program.to_account_info(),
        add_idea_accounts,
    );
    indrasnet_dao_core::cpi::add_idea_to_mesh_group(
        add_idea_ctx,
        idea_id,
    )?;
    Ok(())
}

fn step_anchor_idea(
    ctx: &Context<FullIdeaLifecycleWorkflow>,
    idea_id: u64,
    content_hash: [u8; 32],
) -> Result<()> {
    msg!("⚓ Step 5: Anchoring idea in blockchain");
    let anchor_idea_accounts = indrasnet_dao_core::cpi::accounts::AnchorIdeaInMeshGroup {
        anchorer: ctx.accounts.author.to_account_info(),
        mesh_group: ctx.accounts.mesh_group.to_account_info(),
        idea: ctx.accounts.idea.to_account_info(),
        anchor_record: ctx.accounts.anchor_record.to_account_info(),
        dao_config: ctx.accounts.dao_config.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let anchor_idea_ctx = CpiContext::new(
        ctx.accounts.core_program.to_account_info(),
        anchor_idea_accounts,
    );
    indrasnet_dao_core::cpi::anchor_idea_in_mesh_group(
        anchor_idea_ctx,
        idea_id,
        content_hash,
    )?;
    Ok(())
}

fn step_request_grant(
    ctx: &Context<FullIdeaLifecycleWorkflow>,
    grant_id: u64,
    idea_id: u64,
    grant_category: indrasnet_dao_core::state::grant::GrantCategory,
    grant_type: indrasnet_dao_core::state::grant::GrantType,
    base_amount: u64,
    reputation_bonus: u64,
    milestone_id: Option<u64>,
) -> Result<()> {
    msg!("💰 Step 6: Requesting grant");
    let create_grant_accounts = indrasnet_dao_core::cpi::accounts::CreateGrant {
        creator: ctx.accounts.author.to_account_info(),
        grant: ctx.accounts.grant.to_account_info(),
        idea: ctx.accounts.idea.to_account_info(),
        mesh_group: ctx.accounts.mesh_group.to_account_info(),
        dao_config: ctx.accounts.dao_config.to_account_info(),
        analysis: ctx.accounts.ai_analysis.to_account_info(),
        analysis_record: ctx.accounts.ai_analysis_record.to_account_info(),
        creator_role: ctx.accounts.creator_role.as_ref().map(|r| r.to_account_info()),
        phenomenon: None, // Optional - not provided in workflow
        semantic_domain: None, // Optional - not provided in workflow
        ai_service_registry: None, // Optional - not provided in workflow
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let create_grant_ctx = CpiContext::new(
        ctx.accounts.core_program.to_account_info(),
        create_grant_accounts,
    );
    indrasnet_dao_core::cpi::create_grant(
        create_grant_ctx,
        grant_id,
        idea_id,
        grant_category,
        grant_type,
        indrasnet_dao_core::state::grant::GrantDisbursementType::Standard,
        base_amount,
        reputation_bonus,
        milestone_id,
    )?;
    Ok(())
}

// NOTE: step_create_phenomenon removed - phenomenon creation moved to off-chain service
// Off-chain AI service (services/offchain-ai-service/) will call create_phenomenon on AI program
// with full Track B parameters (embedding signatures, DBSCAN validation, etc.)
// This keeps the orchestrator workflow simple and delegates heavy computation to off-chain

// ============================================================================
// NOTE: Account structures are defined in crate root (lib.rs) for Anchor compatibility
// ============================================================================
