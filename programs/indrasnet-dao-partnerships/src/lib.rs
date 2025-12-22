/*
 * Copyright (c) 2024 Captain Light. All rights reserved.
 * IndrasNet DAO Partnerships - Partnership Management, Analytics, Revenue Sharing
 * Created by: Captain Light
 * Contact: info@indrasnet.ee
 * GitLab: https://gitlab.com/IndrasNet/indrasnet-dao-v3
 */

#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

// ===== PROGRAM ID =====
declare_id!("FpDydZgGo9tezmKwCstSdzbZVv2JiGAnQhQLeT23gHHA");

// ===== MODULE DECLARATIONS =====
// Modules will be added during migration
pub mod error;
pub mod constants;
pub mod types;
pub mod events;
pub mod state;
// pub mod utils; // Removed: empty module, not used
pub mod partnerships;
pub mod security;
pub mod voting_types;
pub mod instructions;

// ===== RE-EXPORTS FOR CRATE-WIDE USE =====
pub use crate::error::IndrasError;

// CPI program imports (for CPI accounts)
// use indrasnet_dao_core::program::IndrasnetDaoCore; // Will be used when CPI is implemented
// use indrasnet_dao_security::program::IndrasnetDaoSecurity; // Will be used when CPI is implemented

// ===== INCLUDE ACCOUNTS STRUCTURES =====
include!("instruction_accounts/partnership.rs");
include!("instruction_accounts/revenue_sharing.rs");
include!("instruction_accounts/metrics.rs");
include!("instruction_accounts/role_registry.rs");

// ===== PROGRAM MODULE =====
#[program]
pub mod indrasnet_dao_partnerships {
    use super::*;

    // ===== PARTNERSHIP INSTRUCTIONS =====

    /// Create partnership
    ///
    /// Creates a new partnership record
    ///
    /// # Compute Units
    /// Recommended: 50,000 CU
    pub fn create_partnership(
        ctx: Context<CreatePartnership>,
        partnership_id: u64,
        partner_name: String,
        partnership_type: partnerships::types::PartnershipType,
        tier: partnerships::types::PartnershipTier,
        description: String,
        terms: partnerships::types::PartnershipTerms,
        metadata_uri: String,
    ) -> Result<()> {
        instructions::create_partnership_handler(
            ctx,
            partnership_id,
            partner_name,
            partnership_type,
            tier,
            description,
            terms,
            metadata_uri,
        )
    }

    /// Initialize partnership configuration
    ///
    /// Initializes global partnership configuration
    pub fn initialize_partnership_config(
        ctx: Context<InitializePartnershipConfig>,
        max_partnerships: u64,
        min_partnership_value: u64,
        max_partnership_value: u64,
        default_revenue_share: u64,
        governance_threshold: u64,
        auto_renewal_enabled: bool,
        risk_threshold: u64,
    ) -> Result<()> {
        instructions::initialize_partnership_config_handler(
            ctx,
            max_partnerships,
            min_partnership_value,
            max_partnership_value,
            default_revenue_share,
            governance_threshold,
            auto_renewal_enabled,
            risk_threshold,
        )
    }

    /// Initialize partnership role registry
    pub fn initialize_partnership_role_registry(
        ctx: Context<InitializePartnershipRoleRegistry>,
    ) -> Result<()> {
        instructions::initialize_partnership_role_registry_handler(ctx)
    }

    /// Add partnership admin
    pub fn add_partnership_admin(
        ctx: Context<UpdatePartnershipRoleRegistry>,
        admin: Pubkey,
    ) -> Result<()> {
        instructions::add_partnership_admin_handler(ctx, admin)
    }

    /// Remove partnership admin
    pub fn remove_partnership_admin(
        ctx: Context<UpdatePartnershipRoleRegistry>,
        admin: Pubkey,
    ) -> Result<()> {
        instructions::remove_partnership_admin_handler(ctx, admin)
    }

    /// Add revenue admin
    pub fn add_revenue_admin(
        ctx: Context<UpdatePartnershipRoleRegistry>,
        admin: Pubkey,
    ) -> Result<()> {
        instructions::add_revenue_admin_handler(ctx, admin)
    }

    /// Remove revenue admin
    pub fn remove_revenue_admin(
        ctx: Context<UpdatePartnershipRoleRegistry>,
        admin: Pubkey,
    ) -> Result<()> {
        instructions::remove_revenue_admin_handler(ctx, admin)
    }

    /// Add metrics admin
    pub fn add_metrics_admin(
        ctx: Context<UpdatePartnershipRoleRegistry>,
        admin: Pubkey,
    ) -> Result<()> {
        instructions::add_metrics_admin_handler(ctx, admin)
    }

    /// Remove metrics admin
    pub fn remove_metrics_admin(
        ctx: Context<UpdatePartnershipRoleRegistry>,
        admin: Pubkey,
    ) -> Result<()> {
        instructions::remove_metrics_admin_handler(ctx, admin)
    }

    /// Update partnership
    ///
    /// Updates an existing partnership
    ///
    /// # Compute Units
    /// Recommended: 30,000 CU
    pub fn update_partnership(
        ctx: Context<UpdatePartnership>,
        partner_name: Option<String>,
        description: Option<String>,
        terms: Option<partnerships::types::PartnershipTerms>,
        status: Option<partnerships::types::PartnershipStatus>,
    ) -> Result<()> {
        instructions::update_partnership_handler(
            ctx,
            partner_name,
            description,
            terms,
            status,
        )
    }

    // ===== REVENUE SHARING INSTRUCTIONS =====

    /// Create revenue share configuration
    ///
    /// Creates a revenue share configuration for a partnership
    ///
    /// # Compute Units
    /// Recommended: 40,000 CU
    pub fn create_revenue_share_config(
        ctx: Context<CreateRevenueShareConfig>,
        config_id: u64,
        partnership_id: u64,
        share_type: partnerships::revenue_sharing::RevenueShareType,
        base_share_percentage: u64,
        performance_multiplier: u64,
        min_share_percentage: u64,
        max_share_percentage: u64,
    ) -> Result<()> {
        instructions::create_revenue_share_config_handler(
            ctx,
            config_id,
            partnership_id,
            share_type,
            base_share_percentage,
            performance_multiplier,
            min_share_percentage,
            max_share_percentage,
        )
    }

    /// Create revenue distribution
    ///
    /// Creates a revenue distribution record
    ///
    /// # Compute Units
    /// Recommended: 45,000 CU
    pub fn create_revenue_distribution(
        ctx: Context<CreateRevenueDistribution>,
        distribution_id: u64,
        partnership_id: u64,
        total_revenue: u64,
        share_percentage: u64,
        period_start: i64,
        period_end: i64,
        distribution_hash: [u8; 32],
        partner_recipient: Pubkey,
    ) -> Result<()> {
        instructions::create_revenue_distribution_handler(
            ctx,
            distribution_id,
            partnership_id,
            total_revenue,
            share_percentage,
            period_start,
            period_end,
            distribution_hash,
            partner_recipient,
        )
    }

    /// Deposit partnership revenue
    ///
    /// Deposits revenue into treasury and calculates sharing
    ///
    /// # Compute Units
    /// Recommended: 35,000 CU
    pub fn deposit_partnership_revenue(
        ctx: Context<DepositPartnershipRevenue>,
        total_revenue: u64,
        partnership_id: u64,
    ) -> Result<()> {
        instructions::deposit_partnership_revenue_handler(ctx, total_revenue, partnership_id)
    }

    // ===== METRICS TRACKING INSTRUCTIONS =====

    /// Track partnership metrics
    ///
    /// Creates a metrics record for a partnership
    ///
    /// # Compute Units
    /// Recommended: 45,000 CU
    pub fn track_partnership_metrics(
        ctx: Context<TrackPartnershipMetrics>,
        metrics_id: u64,
        partnership_id: u64,
        period: partnerships::metrics::MetricsPeriod,
        period_start: i64,
        period_end: i64,
        performance_score: u64,
        risk_score: u64,
        revenue_generated: u64,
        revenue_target: u64,
        milestones_completed: u64,
        total_milestones: u64,
        engagement_score: u64,
        satisfaction_score: u64,
        metrics_hash: [u8; 32],
    ) -> Result<()> {
        instructions::track_partnership_metrics_handler(
            ctx,
            metrics_id,
            partnership_id,
            period,
            period_start,
            period_end,
            performance_score,
            risk_score,
            revenue_generated,
            revenue_target,
            milestones_completed,
            total_milestones,
            engagement_score,
            satisfaction_score,
            metrics_hash,
        )
    }
}
