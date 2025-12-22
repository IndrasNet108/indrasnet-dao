//! Revenue Sharing instruction handlers
//!
//! Handlers for Revenue Sharing instructions
//!
//! On-chain: Validation and state management
//! Off-chain: Complex financial calculations (in separate service)

use anchor_lang::prelude::*;
use crate::partnerships::revenue_sharing::*;

/// Create revenue share configuration
///
/// Creates a revenue share configuration for a partnership
///
/// # Compute Units
/// Recommended: 40,000 CU
/// - Validation: ~10,000 CU
/// - Account initialization: ~30,000 CU
pub fn create_revenue_share_config_handler(
    ctx: Context<crate::CreateRevenueShareConfig>,
    config_id: u64,
    partnership_id: u64,
    share_type: RevenueShareType,
    base_share_percentage: u64,
    performance_multiplier: u64,
    min_share_percentage: u64,
    max_share_percentage: u64,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let creator = ctx.accounts.authority.key();

    require!(
        ctx.accounts.role_registry.is_revenue_admin(&creator),
        crate::error::IndrasError::Unauthorized
    );
    
    create_revenue_share_config(
        config,
        config_id,
        partnership_id,
        share_type,
        base_share_percentage,
        performance_multiplier,
        min_share_percentage,
        max_share_percentage,
        creator,
    )
}

/// Create revenue distribution
///
/// Creates a revenue distribution record
///
/// # Compute Units
/// Recommended: 45,000 CU
/// - Validation: ~10,000 CU
/// - Account initialization: ~35,000 CU
pub fn create_revenue_distribution_handler(
    ctx: Context<crate::CreateRevenueDistribution>,
    distribution_id: u64,
    partnership_id: u64,
    total_revenue: u64,
    share_percentage: u64,
    period_start: i64,
    period_end: i64,
    distribution_hash: [u8; 32],
    partner_recipient: Pubkey,
) -> Result<()> {
    let distribution = &mut ctx.accounts.distribution;
    let distributor = ctx.accounts.authority.key();

    require!(
        ctx.accounts.role_registry.is_revenue_admin(&distributor),
        crate::error::IndrasError::Unauthorized
    );
    
    create_revenue_distribution(
        distribution,
        distribution_id,
        partnership_id,
        total_revenue,
        share_percentage,
        period_start,
        period_end,
        distribution_hash,
        distributor,
        partner_recipient,
    )
}

/// Deposit partnership revenue
///
/// Deposits revenue into treasury and calculates sharing
///
/// # Compute Units
/// Recommended: 35,000 CU
/// - Validation: ~5,000 CU
/// - Calculations: ~15,000 CU
/// - State update: ~15,000 CU
pub fn deposit_partnership_revenue_handler(
    ctx: Context<crate::DepositPartnershipRevenue>,
    total_revenue: u64,
    partnership_id: u64,
) -> Result<()> {
    let distribution = &mut ctx.accounts.distribution;
    let depositor = ctx.accounts.authority.key();

    require!(
        ctx.accounts.role_registry.is_revenue_admin(&depositor),
        crate::error::IndrasError::Unauthorized
    );

    require!(
        distribution.partnership_id == partnership_id,
        crate::error::IndrasError::InvalidInput
    );

    require!(
        distribution.partnership_id == ctx.accounts.partnership.partnership_id,
        crate::error::IndrasError::InvalidInput
    );
    
    // Use existing share_percentage from distribution
    let share_percentage = distribution.share_percentage;
    
    // Calculate revenue share
    let partner_share = total_revenue
        .checked_mul(share_percentage)
        .ok_or(crate::error::IndrasError::Overflow)?
        .checked_div(crate::constants::PRECISION_FACTOR)
        .ok_or(crate::error::IndrasError::DivisionByZero)?;
    
    let dao_share = total_revenue
        .checked_sub(partner_share)
        .ok_or(crate::error::IndrasError::Underflow)?;
    
    distribution.partner_share = partner_share;
    distribution.dao_share = dao_share;
    distribution.total_revenue = total_revenue;
    
    msg!("Revenue deposited: partner={}, dao={}, total={}", 
         partner_share, dao_share, total_revenue);
    
    Ok(())
}
