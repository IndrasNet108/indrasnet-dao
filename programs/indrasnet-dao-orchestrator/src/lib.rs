/*
 * Copyright (c) 2024 Captain Light. All rights reserved.
 * IndrasNet DAO Orchestrator - Coordinates complex operations between subprograms via CPI
 * Created by: Captain Light
 * Contact: info@indrasnet.ee
 * GitLab: https://gitlab.com/IndrasNet/indrasnet-dao-v3
 */

#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

// ===== PROGRAM ID =====
declare_id!("7xQ7pxcZhg9Lwngtu4199kc85eL94pc27XcFyVMZE6RC");

// ===== MODULE DECLARATIONS =====
pub mod error;
// pub mod constants; // Removed: empty module, not used
pub mod types;
pub mod events;
pub mod orchestrator;

// ===== RE-EXPORTS FOR CRATE-WIDE USE =====
pub use crate::error::IndrasError;

// CPI program imports (for CPI accounts)
use indrasnet_dao_core::program::IndrasnetDaoCore;
use indrasnet_dao_ai::program::IndrasnetDaoAi;
use indrasnet_dao_security::program::IndrasnetDaoSecurity;

// ===== ACCOUNTS STRUCTURES =====
// Accounts structures for workflow functions must be in crate root for Anchor 0.32.1

#[derive(Accounts)]
pub struct CreateIdeaWithAIAnalysisWorkflow<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    
    // Core program accounts
    #[account(mut)]
    pub idea: Account<'info, indrasnet_dao_core::state::Idea>,
    pub dao_config: Account<'info, indrasnet_dao_core::state::DaoConfig>,
    
    /// Author's role (optional - for permission check)
    /// CHECK: If provided, must have CAN_CREATE_IDEA permission (unless DAO authority)
    pub author_role: Option<Account<'info, indrasnet_dao_core::state::MemberRole>>,

    /// Rate limit tracker for idea creation (required in production)
    pub rate_limit_tracker: Account<'info, indrasnet_dao_core::state::rate_limit_tracker::RateLimitTracker>,
    
    // AI program accounts (optional - will be created during workflow)
    // Note: AI analysis account is created by AI program, not Orchestrator
    // For workflow coordination, we don't need it in accounts structure
    
    // Programs
    pub core_program: Program<'info, IndrasnetDaoCore>,
    pub ai_program: Program<'info, IndrasnetDaoAi>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePartnershipWithSecurityCheckWorkflow<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    // Security program accounts
    // Note: AuditEvent is exported from security::audit module
    // Using UncheckedAccount for now as Security program may not export it as Account type
    // #[account(mut)]
    // pub security_audit: UncheckedAccount<'info>,
    
    // Programs
    pub security_program: Program<'info, IndrasnetDaoSecurity>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RequestGrantWithFullValidationWorkflow<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    // Core program accounts
    #[account(mut)]
    pub grant: Account<'info, indrasnet_dao_core::state::Grant>,
    pub idea: Account<'info, indrasnet_dao_core::state::Idea>,
    pub mesh_group: Account<'info, indrasnet_dao_core::state::MeshGroup>,
    pub dao_config: Account<'info, indrasnet_dao_core::state::DaoConfig>,
    
    /// Creator's role (optional - for permission check)
    /// CHECK: If provided, must have CAN_CREATE_GRANT permission (unless DAO authority)
    pub creator_role: Option<Account<'info, indrasnet_dao_core::state::MemberRole>>,
    
    // AI program accounts (required for grant creation)
    // AI analysis account is UncheckedAccount in CreateGrant
    /// CHECK: AI analysis account is validated by the core program during grant creation CPI call
    pub ai_analysis: UncheckedAccount<'info>,

    /// Core-owned AI analysis registration record (required by Core)
    pub ai_analysis_record: Account<'info, indrasnet_dao_core::state::ai_analysis_record::AIAnalysisRecord>,
    
    // Programs
    pub core_program: Program<'info, IndrasnetDaoCore>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FullIdeaLifecycleWorkflow<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    
    // Core program accounts
    #[account(mut)]
    pub idea: Account<'info, indrasnet_dao_core::state::Idea>,
    #[account(mut)]
    pub mesh_group: Account<'info, indrasnet_dao_core::state::MeshGroup>,
    #[account(mut)]
    pub grant: Account<'info, indrasnet_dao_core::state::Grant>,
    pub dao_config: Account<'info, indrasnet_dao_core::state::DaoConfig>,
    
    /// Author's role (optional - for permission check)
    /// CHECK: If provided, must have appropriate permissions (unless DAO authority)
    pub author_role: Option<Account<'info, indrasnet_dao_core::state::MemberRole>>,
    
    /// Creator's role for mesh group and grant (optional - for permission check)
    /// CHECK: If provided, must have CAN_MANAGE_MESH_GROUPS and CAN_CREATE_GRANT permissions (unless DAO authority)
    pub creator_role: Option<Account<'info, indrasnet_dao_core::state::MemberRole>>,
    
    // AI program accounts
    /// CHECK: AI analysis account is validated by the core program during grant creation CPI call
    pub ai_analysis: UncheckedAccount<'info>, // Required for grant creation
    pub ai_analysis_record: Account<'info, indrasnet_dao_core::state::ai_analysis_record::AIAnalysisRecord>,
    #[account(mut)]
    pub phenomenon: Account<'info, indrasnet_dao_core::state::Phenomenon>, // Created during workflow
    
    // Anchor record for idea anchoring
    #[account(mut)]
    pub anchor_record: Account<'info, indrasnet_dao_core::AnchorRecord>,
    
    // Programs
    pub core_program: Program<'info, IndrasnetDaoCore>,
    pub ai_program: Program<'info, IndrasnetDaoAi>,
    pub system_program: Program<'info, System>,
}

// ===== PROGRAM MODULE =====
#[program]
pub mod indrasnet_dao_orchestrator {
    use super::*;
    
    // Import account structures
    use crate::{
        CreateIdeaWithAIAnalysisWorkflow,
        CreatePartnershipWithSecurityCheckWorkflow,
        RequestGrantWithFullValidationWorkflow,
        // FullIdeaLifecycleWorkflow, // DISABLED: Workflow disabled due to stack overflow
    };

    // ===== ORCHESTRATOR FUNCTIONS ONLY =====
    // NOTE: All simple functions are in their respective programs (Core, AI, Security, Partnerships).
    // Orchestrator ONLY coordinates complex workflows via CPI.
    // For simple operations, call the programs directly.

    // ===== WORKFLOW FUNCTIONS =====
    // Workflow functions coordinate complex operations between subprograms via CPI
    
    /// Create idea with AI analysis workflow
    /// 
    /// Orchestrates the creation of an idea followed by AI analysis.
    pub fn create_idea_with_ai_analysis_workflow(
        ctx: Context<CreateIdeaWithAIAnalysisWorkflow>,
        idea_id: u64,
        title: String,
        description: String,
        metadata_uri: String,
    ) -> Result<()> {
        orchestrator::workflows::create_idea_with_ai_analysis_workflow(
            ctx,
            idea_id,
            title,
            description,
            metadata_uri,
        )
    }
    
    /// Create partnership with security check workflow
    /// 
    /// Orchestrates the creation of a partnership after security validation.
    pub fn create_partnership_with_security_check_workflow(
        ctx: Context<CreatePartnershipWithSecurityCheckWorkflow>,
        partnership_id: u64,
        partner_name: String,
        partnership_type: u8,
        description: String,
    ) -> Result<()> {
        orchestrator::workflows::create_partnership_with_security_check_workflow(
            ctx,
            partnership_id,
            partner_name,
            partnership_type,
            description,
        )
    }
    
    /// Request grant with full validation workflow
    /// 
    /// Orchestrates grant request with comprehensive validation.
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
        orchestrator::workflows::request_grant_with_full_validation_workflow(
            ctx,
            grant_id,
            idea_id,
            mesh_group_id,
            category,
            grant_type,
            base_amount,
            reputation_bonus,
            milestone_id,
        )
    }
    
    // DISABLED: full_idea_lifecycle_workflow - Stack overflow (5112 bytes > 4096 limit)
    // This workflow is too complex for a single instruction due to Solana stack limits.
    // Use separate instructions instead: create_idea_with_ai_analysis_workflow, 
    // create_partnership_with_security_check_workflow, request_grant_with_full_validation_workflow
    // 
    // pub fn full_idea_lifecycle_workflow(...) -> Result<()> {
    //     // DISABLED - Stack overflow
    // }
}
