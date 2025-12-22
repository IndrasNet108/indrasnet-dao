//! Performance Analytics handlers
//!
//! Handlers for Performance Analytics instructions
//!
//! On-chain: Validation and state management
//! Off-chain: Actual performance analytics and analysis (in separate service)

use anchor_lang::prelude::*;
use crate::ai::performance_analytics::*;

/// Create performance analytics
///
/// Creates a performance analytics record for an entity (idea, mesh group, etc.)
///
/// # Compute Units
/// Recommended: 40,000 CU
/// - Validation: ~10,000 CU
/// - Account initialization: ~30,000 CU
pub fn create_performance_analytics_handler(
    ctx: Context<crate::CreatePerformanceAnalytics>,
    analytics_id: u64,
    entity_id: u64,
    entity_type: String,
    performance_scores: PerformanceScores,
    metadata_uri: String,
    analytics_hash: [u8; 32],
) -> Result<()> {
    let analytics = &mut ctx.accounts.analytics;
    let analyzer = ctx.accounts.authority.key();
    
    create_performance_analytics(
        analytics,
        analytics_id,
        entity_id,
        entity_type,
        performance_scores,
        metadata_uri,
        analytics_hash,
        analyzer,
    )
}
