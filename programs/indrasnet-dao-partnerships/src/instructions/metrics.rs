//! Metrics Tracking instruction handlers
//!
//! Handlers for Metrics Tracking instructions
//!
//! On-chain: Validation and state management
//! Off-chain: Complex analytics (in separate service)

use anchor_lang::prelude::*;
use crate::partnerships::metrics::*;

/// Track partnership metrics
///
/// Creates a metrics record for a partnership
///
/// # Compute Units
/// Recommended: 45,000 CU
/// - Validation: ~10,000 CU
/// - Account initialization: ~35,000 CU
pub fn track_partnership_metrics_handler(
    ctx: Context<crate::TrackPartnershipMetrics>,
    metrics_id: u64,
    partnership_id: u64,
    period: MetricsPeriod,
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
    let metrics = &mut ctx.accounts.metrics;
    let collector = ctx.accounts.authority.key();

    require!(
        ctx.accounts.role_registry.is_metrics_admin(&collector),
        crate::error::IndrasError::Unauthorized
    );

    require!(
        ctx.accounts.partnership.partnership_id == partnership_id,
        crate::error::IndrasError::InvalidInput
    );
    
    collect_partnership_metrics(
        metrics,
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
        collector,
    )
}
