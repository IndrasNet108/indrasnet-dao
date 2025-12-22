/*
 * Copyright (c) 2024 Captain Light. All rights reserved.
 * IndrasNet DAO Security - Security Monitoring, Operations, Compliance
 * Created by: Captain Light
 * Contact: info@indrasnet.ee
 * GitLab: https://gitlab.com/IndrasNet/indrasnet-dao-v3
 */

#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

// ===== PROGRAM ID =====
declare_id!("7qnYTjKghAhyJv8iRRbBp9p5bqZ4JVswz7xeDhVdyaZj");

// ===== MODULE DECLARATIONS =====
// Modules will be added during migration
pub mod error;
// pub mod constants; // Removed: empty module, not used
pub mod types;
pub mod events;
pub mod state;
// pub mod utils; // Removed: empty module, not used
pub mod security;
pub mod voting_types;
pub mod voting_functions;
pub mod handlers;
pub mod instructions;
pub mod instruction_accounts;

// ===== RE-EXPORTS FOR CRATE-WIDE USE =====
pub use crate::error::IndrasError;

// Re-export account structures for use in program (before #[program])
pub use instruction_accounts::security_handlers::*;
pub use instruction_accounts::security_roles::*;
include!("instruction_accounts/nis2_compliance.rs");

// CPI program import (for CPI accounts)
// use indrasnet_dao_core::program::IndrasnetDaoCore; // Will be used when CPI is implemented

// ===== PROGRAM MODULE =====
#[program]
pub mod indrasnet_dao_security {
    use super::*;

    // NOTE: IoT security, IR team, SSO, network monitoring, cloud monitoring
    // moved to off-chain service (services/offchain-security-service/)
    // These instructions are now handled off-chain and results are recorded on-chain

    // ===== SECURITY ANALYTICS INSTRUCTIONS =====

    /// Create security analytics report
    ///
    /// Creates a security analytics report with metadata
    ///
    /// # Compute Units
    /// Recommended: 40,000 CU
    pub fn create_security_analytics_report(
        ctx: Context<CreateSecurityAnalyticsReport>,
        report_id: u64,
        idea_id: u64,
        report_type: security::security_analytics::AnalyticsReportType,
        title: String,
        metadata_uri: String,
        report_hash: [u8; 32],
        summary: String,
    ) -> Result<()> {
        handlers::create_security_analytics_report_handler(
            ctx,
            report_id,
            idea_id,
            report_type,
            title,
            metadata_uri,
            report_hash,
            summary,
        )
    }

    // ===== COMPLIANCE CHECKING INSTRUCTIONS =====

    /// Create compliance check
    ///
    /// Creates a compliance check record for a specific framework
    ///
    /// # Compute Units
    /// Recommended: 40,000 CU
    pub fn create_compliance_check(
        ctx: Context<CreateComplianceCheck>,
        check_id: u64,
        framework: security::compliance_checking::ComplianceFramework,
        name: String,
        description: String,
        status: security::compliance_checking::ComplianceCheckStatus,
        result_hash: [u8; 32],
        metadata_uri: String,
    ) -> Result<()> {
        handlers::create_compliance_check_handler(
            ctx,
            check_id,
            framework,
            name,
            description,
            status,
            result_hash,
            metadata_uri,
        )
    }

    /// Update compliance check status
    pub fn update_compliance_check_status(
        ctx: Context<UpdateComplianceCheckStatus>,
        status: security::compliance_checking::ComplianceCheckStatus,
        result_hash: Option<[u8; 32]>,
    ) -> Result<()> {
        handlers::update_compliance_check_status_handler(ctx, status, result_hash)
    }

    /// Run NIS2 compliance check
    ///
    /// Runs a series of automated checks for NIS2 compliance.
    ///
    /// # Compute Units
    /// Recommended: 50,000 CU
    pub fn run_nis2_compliance_check(
        ctx: Context<RunNis2ComplianceCheck>,
        check_id: u64,
    ) -> Result<()> {
        handlers::run_nis2_compliance_check_handler(ctx, check_id)
    }

    // ===== SECURITY ROLE REGISTRY INSTRUCTIONS =====

    /// Initialize Security role registry
    pub fn initialize_security_role_registry(
        ctx: Context<InitializeSecurityRoleRegistry>,
    ) -> Result<()> {
        handlers::initialize_security_role_registry_handler(ctx)
    }

    /// Add security admin
    pub fn add_security_admin(
        ctx: Context<UpdateSecurityRoleRegistry>,
        admin: Pubkey,
    ) -> Result<()> {
        handlers::add_security_admin_handler(ctx, admin)
    }

    /// Remove security admin
    pub fn remove_security_admin(
        ctx: Context<UpdateSecurityRoleRegistry>,
        admin: Pubkey,
    ) -> Result<()> {
        handlers::remove_security_admin_handler(ctx, admin)
    }

    /// Add compliance admin
    pub fn add_compliance_admin(
        ctx: Context<UpdateSecurityRoleRegistry>,
        admin: Pubkey,
    ) -> Result<()> {
        handlers::add_compliance_admin_handler(ctx, admin)
    }

    /// Remove compliance admin
    pub fn remove_compliance_admin(
        ctx: Context<UpdateSecurityRoleRegistry>,
        admin: Pubkey,
    ) -> Result<()> {
        handlers::remove_compliance_admin_handler(ctx, admin)
    }

    /// Add analytics admin
    pub fn add_analytics_admin(
        ctx: Context<UpdateSecurityRoleRegistry>,
        admin: Pubkey,
    ) -> Result<()> {
        handlers::add_analytics_admin_handler(ctx, admin)
    }

    /// Remove analytics admin
    pub fn remove_analytics_admin(
        ctx: Context<UpdateSecurityRoleRegistry>,
        admin: Pubkey,
    ) -> Result<()> {
        handlers::remove_analytics_admin_handler(ctx, admin)
    }

    // ===== DAO SECURITY INTEGRATION INSTRUCTIONS =====

    /// Check proposal security (CPI callable from DAO)
    ///
    /// This instruction can be called from DAO program via CPI
    /// to check security of a proposal before voting
    ///
    /// NOTE: Emergency response, IoT security, IR team, SSO, network monitoring, cloud monitoring
    /// moved to off-chain service (services/offchain-security-service/)
    ///
    /// # Compute Units
    /// Recommended: 50,000 CU
    pub fn check_proposal_security_cpi(
        ctx: Context<CheckProposalSecurity>,
        proposal_title: String,
        proposal_description: String,
    ) -> Result<types::SecurityCheckResult> {
        handlers::check_proposal_security_cpi_handler(
            ctx,
            proposal_title,
            proposal_description,
        )
    }
    
    // NOTE: IoT security, IR team, SSO, network monitoring, cloud monitoring, emergency response
    // moved to off-chain service (services/offchain-security-service/)

    // NOTE: update_security_check_result moved to off-chain service
    // Off-chain service uses create_security_analytics_report to record results
}
