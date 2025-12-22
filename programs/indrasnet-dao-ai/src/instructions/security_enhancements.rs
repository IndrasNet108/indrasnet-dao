//! Security Enhancements handlers
//!
//! Handlers for Security Enhancements instructions
//!
//! On-chain: Validation and state management
//! Off-chain: Actual security analysis and recommendations (in separate service)

use anchor_lang::prelude::*;
use crate::ai::security_enhancements::*;

/// Create security enhancement recommendation
///
/// Creates a security enhancement recommendation for an entity
///
/// # Compute Units
/// Recommended: 40,000 CU
/// - Validation: ~10,000 CU
/// - Account initialization: ~30,000 CU
pub fn create_security_enhancement_handler(
    ctx: Context<crate::CreateSecurityEnhancement>,
    enhancement_id: u64,
    entity_id: u64,
    entity_type: String,
    title: String,
    description: String,
    priority: EnhancementPriority,
    score_improvement: u8,
    metadata_uri: String,
    enhancement_hash: [u8; 32],
) -> Result<()> {
    let enhancement = &mut ctx.accounts.enhancement;
    let recommender = ctx.accounts.authority.key();
    
    create_security_enhancement(
        enhancement,
        enhancement_id,
        entity_id,
        entity_type,
        title,
        description,
        priority,
        score_improvement,
        metadata_uri,
        enhancement_hash,
        recommender,
    )
}

/// Update security enhancement status
///
/// Updates the status of a security enhancement
///
/// # Compute Units
/// Recommended: 20,000 CU
/// - Validation: ~5,000 CU
/// - State update: ~15,000 CU
pub fn update_security_enhancement_status_handler(
    ctx: Context<crate::UpdateSecurityEnhancementStatus>,
    status: EnhancementStatus,
) -> Result<()> {
    let enhancement = &mut ctx.accounts.enhancement;
    
    update_security_enhancement_status(enhancement, status)
}
