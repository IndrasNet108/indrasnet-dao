//! Governance Analytics instruction handlers
//!
//! Handlers for governance analytics operations:
//! - initialize_governance_analytics - Initialize analytics metadata
//! - initialize_governance_participation - Initialize participation tracking
//! - initialize_governance_voting - Initialize voting metadata

use anchor_lang::prelude::*;
use crate::state::governance::{
    analytics::GovernanceAnalyticsType,
    participation::GovernanceParticipationType,
    voting::GovernanceVotingType,
};

/// Initialize governance analytics metadata
///
/// This handler initializes analytics metadata for tracking governance metrics.
pub fn initialize_governance_analytics_handler(
    ctx: Context<crate::InitializeGovernanceAnalytics>,
    analytics_id: u64,
    governance_id: u64,
    analytics_type: GovernanceAnalyticsType,
    analytics_config_hash: [u8; 32],
) -> Result<()> {
    let analytics = &mut ctx.accounts.analytics;
    let current_time = Clock::get()?.unix_timestamp;
    
    crate::state::governance::analytics::onchain::initialize_governance_analytics(
        analytics,
        analytics_id,
        governance_id,
        analytics_type,
        analytics_config_hash,
        current_time,
        ctx.bumps.analytics,
    )?;
    
    msg!("Governance analytics {} initialized for governance {}", analytics_id, governance_id);
    Ok(())
}

/// Initialize governance participation metadata
///
/// This handler initializes participation tracking metadata for a member.
pub fn initialize_governance_participation_handler(
    ctx: Context<crate::InitializeGovernanceParticipation>,
    participation_id: u64,
    member_id: u64,
    participation_type: GovernanceParticipationType,
    participation_config_hash: [u8; 32],
) -> Result<()> {
    let participation = &mut ctx.accounts.participation;
    let current_time = Clock::get()?.unix_timestamp;
    
    crate::state::governance::participation::onchain::initialize_governance_participation(
        participation,
        participation_id,
        member_id,
        participation_type,
        participation_config_hash,
        current_time,
        ctx.bumps.participation,
    )?;
    
    msg!("Governance participation {} initialized for member {}", participation_id, member_id);
    Ok(())
}

/// Initialize governance voting metadata
///
/// This handler initializes voting metadata for a proposal.
pub fn initialize_governance_voting_handler(
    ctx: Context<crate::InitializeGovernanceVoting>,
    voting_id: u64,
    proposal_id: u64,
    voting_type: GovernanceVotingType,
    voting_data_hash: [u8; 32],
) -> Result<()> {
    let voting = &mut ctx.accounts.voting;
    let current_time = Clock::get()?.unix_timestamp;
    
    crate::state::governance::voting::onchain::initialize_governance_voting(
        voting,
        voting_id,
        proposal_id,
        voting_type,
        voting_data_hash,
        current_time,
        ctx.bumps.voting,
    )?;
    
    msg!("Governance voting {} initialized for proposal {}", voting_id, proposal_id);
    Ok(())
}
