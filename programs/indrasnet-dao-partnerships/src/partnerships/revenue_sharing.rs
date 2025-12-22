//! Partnership Revenue Sharing Module
//! 
//! Comprehensive revenue sharing and distribution system for partnerships.
//! Handles revenue calculation, distribution, and tracking.
//! 
//! Hybrid model: Off-chain complex financial calculations, on-chain distribution records
//! 
//! Principle: "Blockchain = Proof, not Storage"
//! - ON-CHAIN: Revenue distribution records, share calculations, payment tracking
//! - OFF-CHAIN: Complex financial analysis, tax calculations, detailed reporting
//!
//! Improved version with enhanced revenue sharing logic and comprehensive testing

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::constants::PRECISION_FACTOR;
use super::types::*;

// ============================================================================
// ON-CHAIN STRUCTURES (Anchor Account Types)
// ============================================================================

/// Revenue share type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum RevenueShareType {
    /// Fixed percentage share
    FixedPercentage,
    /// Tiered percentage share
    TieredPercentage,
    /// Performance-based share
    PerformanceBased,
    /// Milestone-based share
    MilestoneBased,
}

impl Space for RevenueShareType {
    const INIT_SPACE: usize = 1;
}

/// Revenue distribution status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum DistributionStatus {
    /// Distribution pending
    Pending,
    /// Distribution in progress
    InProgress,
    /// Distribution completed
    Completed,
    /// Distribution failed
    Failed,
    /// Distribution cancelled
    Cancelled,
}

impl Space for DistributionStatus {
    const INIT_SPACE: usize = 1;
}

/// Revenue distribution record
#[account]
#[derive(InitSpace)]
pub struct RevenueDistribution {
    /// Distribution ID
    pub distribution_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Total revenue amount (lamports)
    pub total_revenue: u64,
    /// Partner share amount (lamports)
    pub partner_share: u64,
    /// DAO share amount (lamports)
    pub dao_share: u64,
    /// Revenue share percentage (in fixed-point, 0 to PRECISION_FACTOR)
    pub share_percentage: u64,
    /// Distribution status
    pub status: DistributionStatus,
    /// Distribution period start
    pub period_start: i64,
    /// Distribution period end
    pub period_end: i64,
    /// Distribution timestamp
    pub distributed_at: Option<i64>,
    /// Distribution hash (for verification)
    pub distribution_hash: [u8; 32],
    /// Distributor (system or authority)
    pub distributor: Pubkey,
    /// Partner recipient address
    pub partner_recipient: Pubkey,
    /// Bump seed for PDA
    pub bump: u8,
}

/// Revenue share configuration
#[account]
#[derive(InitSpace)]
pub struct RevenueShareConfig {
    /// Config ID
    pub config_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Share type
    pub share_type: RevenueShareType,
    /// Base share percentage (in fixed-point)
    pub base_share_percentage: u64,
    /// Performance multiplier (in fixed-point, 1.0 = no change)
    pub performance_multiplier: u64,
    /// Minimum share percentage (in fixed-point)
    pub min_share_percentage: u64,
    /// Maximum share percentage (in fixed-point)
    pub max_share_percentage: u64,
    /// Config creator
    pub creator: Pubkey,
    /// Config creation timestamp
    pub created_at: i64,
    /// Config last update timestamp
    pub updated_at: i64,
    /// Whether config is active
    pub is_active: bool,
    /// Bump seed for PDA
    pub bump: u8,
}

// ============================================================================
// ON-CHAIN FUNCTIONS (Anchor Handlers)
// ============================================================================

/// Create revenue share configuration
pub fn create_revenue_share_config(
    config: &mut RevenueShareConfig,
    config_id: u64,
    partnership_id: u64,
    share_type: RevenueShareType,
    base_share_percentage: u64,
    performance_multiplier: u64,
    min_share_percentage: u64,
    max_share_percentage: u64,
    creator: Pubkey,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    create_revenue_share_config_with_time(
        config,
        config_id,
        partnership_id,
        share_type,
        base_share_percentage,
        performance_multiplier,
        min_share_percentage,
        max_share_percentage,
        creator,
        current_time,
    )
}

/// Create revenue share configuration with explicit timestamp (for testing)
pub fn create_revenue_share_config_with_time(
    config: &mut RevenueShareConfig,
    config_id: u64,
    partnership_id: u64,
    share_type: RevenueShareType,
    base_share_percentage: u64,
    performance_multiplier: u64,
    min_share_percentage: u64,
    max_share_percentage: u64,
    creator: Pubkey,
    current_time: i64,
) -> Result<()> {
    require!(base_share_percentage <= PRECISION_FACTOR, IndrasError::InvalidScore);
    require!(min_share_percentage <= PRECISION_FACTOR, IndrasError::InvalidScore);
    require!(max_share_percentage <= PRECISION_FACTOR, IndrasError::InvalidScore);
    require!(min_share_percentage <= max_share_percentage, IndrasError::InvalidInput);
    require!(base_share_percentage >= min_share_percentage, IndrasError::InvalidInput);
    require!(base_share_percentage <= max_share_percentage, IndrasError::InvalidInput);
    require!(performance_multiplier > 0, IndrasError::InvalidInput);
    
    config.config_id = config_id;
    config.partnership_id = partnership_id;
    config.share_type = share_type;
    config.base_share_percentage = base_share_percentage;
    config.performance_multiplier = performance_multiplier;
    config.min_share_percentage = min_share_percentage;
    config.max_share_percentage = max_share_percentage;
    config.creator = creator;
    config.created_at = current_time;
    config.updated_at = current_time;
    config.is_active = true;
    
    msg!("Revenue share config {} created for partnership {} (type: {:?}, base: {}%)", 
         config_id, partnership_id, share_type, base_share_percentage * 100 / PRECISION_FACTOR);
    
    Ok(())
}

/// Calculate revenue share
pub fn calculate_revenue_share(
    config: &RevenueShareConfig,
    total_revenue: u64,
    performance_score: Option<u64>,
) -> Result<u64> {
    require!(config.is_active, IndrasError::InvalidState);
    require!(total_revenue > 0, IndrasError::InvalidInput);
    
    let share_percentage = match config.share_type {
        RevenueShareType::FixedPercentage => config.base_share_percentage,
        RevenueShareType::PerformanceBased => {
            if let Some(perf_score) = performance_score {
                require!(perf_score <= PRECISION_FACTOR, IndrasError::InvalidScore);
                
                // Apply performance multiplier
                let adjusted_share = config.base_share_percentage
                    .checked_mul(config.performance_multiplier)
                    .ok_or(IndrasError::Overflow)?
                    .checked_mul(perf_score)
                    .ok_or(IndrasError::Overflow)?
                    .checked_div(PRECISION_FACTOR)
                    .ok_or(IndrasError::DivisionByZero)?
                    .checked_div(PRECISION_FACTOR)
                    .ok_or(IndrasError::DivisionByZero)?;
                
                // Clamp to min/max range
                adjusted_share.max(config.min_share_percentage).min(config.max_share_percentage)
            } else {
                return Err(IndrasError::InvalidInput.into());
            }
        }
        RevenueShareType::TieredPercentage => {
            // Simplified tiered calculation - full implementation off-chain
            config.base_share_percentage
        }
        RevenueShareType::MilestoneBased => {
            // Simplified milestone-based calculation - full implementation off-chain
            config.base_share_percentage
        }
    };
    
    // Calculate partner share
    let partner_share = total_revenue
        .checked_mul(share_percentage)
        .ok_or(IndrasError::Overflow)?
        .checked_div(PRECISION_FACTOR)
        .ok_or(IndrasError::DivisionByZero)?;
    
    Ok(partner_share)
}

/// Create revenue distribution
pub fn create_revenue_distribution(
    distribution: &mut RevenueDistribution,
    distribution_id: u64,
    partnership_id: u64,
    total_revenue: u64,
    share_percentage: u64,
    period_start: i64,
    period_end: i64,
    distribution_hash: [u8; 32],
    distributor: Pubkey,
    partner_recipient: Pubkey,
) -> Result<()> {
    require!(total_revenue > 0, IndrasError::InvalidInput);
    require!(period_end > period_start, IndrasError::InvalidInput);
    require!(share_percentage <= PRECISION_FACTOR, IndrasError::InvalidScore);
    
    // Calculate shares
    let partner_share = total_revenue
        .checked_mul(share_percentage)
        .ok_or(IndrasError::Overflow)?
        .checked_div(PRECISION_FACTOR)
        .ok_or(IndrasError::DivisionByZero)?;
    
    let dao_share = total_revenue
        .checked_sub(partner_share)
        .ok_or(IndrasError::Underflow)?;
    
    distribution.distribution_id = distribution_id;
    distribution.partnership_id = partnership_id;
    distribution.total_revenue = total_revenue;
    distribution.partner_share = partner_share;
    distribution.dao_share = dao_share;
    distribution.share_percentage = share_percentage;
    distribution.status = DistributionStatus::Pending;
    distribution.period_start = period_start;
    distribution.period_end = period_end;
    distribution.distributed_at = None;
    distribution.distribution_hash = distribution_hash;
    distribution.distributor = distributor;
    distribution.partner_recipient = partner_recipient;
    
    msg!("Revenue distribution {} created for partnership {} (total: {} lamports, partner share: {}%, partner: {} lamports, DAO: {} lamports)", 
         distribution_id, partnership_id, total_revenue, 
         share_percentage * 100 / PRECISION_FACTOR, partner_share, dao_share);
    
    Ok(())
}

/// Mark distribution as completed
pub fn complete_revenue_distribution(
    distribution: &mut RevenueDistribution,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    complete_revenue_distribution_with_time(distribution, current_time)
}

/// Mark distribution as completed with explicit timestamp (for testing)
pub fn complete_revenue_distribution_with_time(
    distribution: &mut RevenueDistribution,
    current_time: i64,
) -> Result<()> {
    require!(
        distribution.status == DistributionStatus::Pending || 
        distribution.status == DistributionStatus::InProgress,
        IndrasError::InvalidState
    );
    
    distribution.status = DistributionStatus::Completed;
    distribution.distributed_at = Some(current_time);
    
    msg!("Revenue distribution {} completed (partner: {} lamports, DAO: {} lamports)", 
         distribution.distribution_id, distribution.partner_share, distribution.dao_share);
    
    Ok(())
}

// ============================================================================
// OFF-CHAIN FUNCTIONS (Regular Rust - Not Anchor)
// ============================================================================

/// OFF-CHAIN: Calculate complex revenue share
/// 
/// This function should be called off-chain for complex revenue share calculations.
pub fn calculate_complex_revenue_share_offchain(
    _config: &RevenueShareConfig,
    _total_revenue: u64,
    _partnership: &Partnership,
    _metrics: &PartnershipMetrics,
) -> Result<u64> {
    // OFF-CHAIN IMPLEMENTATION:
    // 1. Analyze partnership performance
    // 2. Apply complex tiered or milestone-based calculations
    // 3. Consider market conditions
    // 4. Return calculated share to be stored on-chain
    
    // This is a placeholder - actual implementation happens off-chain
    Err(IndrasError::NotImplemented.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_revenue_share_config() {
        let mut config = RevenueShareConfig {
            config_id: 0,
            partnership_id: 0,
            share_type: RevenueShareType::FixedPercentage,
            base_share_percentage: 0,
            performance_multiplier: 0,
            min_share_percentage: 0,
            max_share_percentage: 0,
            creator: Pubkey::default(),
            created_at: 0,
            updated_at: 0,
            is_active: false,
            bump: 0,
        };

        let creator = Pubkey::from([1u8; 32]);

        let result = create_revenue_share_config_with_time(
            &mut config,
            1,
            100,
            RevenueShareType::FixedPercentage,
            100_000, // 10%
            1_000_000, // 1.0x multiplier
            50_000, // 5% min
            200_000, // 20% max
            creator,
            1000, // Mock timestamp
        );

        assert!(result.is_ok());
        assert_eq!(config.config_id, 1);
        assert_eq!(config.partnership_id, 100);
        assert_eq!(config.base_share_percentage, 100_000);
        assert!(config.is_active);
    }

    #[test]
    fn test_calculate_revenue_share_fixed() {
        let config = RevenueShareConfig {
            config_id: 1,
            partnership_id: 100,
            share_type: RevenueShareType::FixedPercentage,
            base_share_percentage: 100_000, // 10%
            performance_multiplier: 1_000_000,
            min_share_percentage: 50_000,
            max_share_percentage: 200_000,
            creator: Pubkey::default(),
            created_at: 0,
            updated_at: 0,
            is_active: true,
            bump: 0,
        };

        let total_revenue = 10_000_000_000; // 10 SOL
        let partner_share = calculate_revenue_share(&config, total_revenue, None).unwrap();

        // 10% of 10 SOL = 1 SOL
        assert_eq!(partner_share, 1_000_000_000);
    }

    #[test]
    fn test_calculate_revenue_share_performance_based() {
        let config = RevenueShareConfig {
            config_id: 1,
            partnership_id: 100,
            share_type: RevenueShareType::PerformanceBased,
            base_share_percentage: 100_000, // 10%
            performance_multiplier: 1_200_000, // 1.2x
            min_share_percentage: 50_000, // 5%
            max_share_percentage: 200_000, // 20%
            creator: Pubkey::default(),
            created_at: 0,
            updated_at: 0,
            is_active: true,
            bump: 0,
        };

        let total_revenue = 10_000_000_000; // 10 SOL
        let performance_score = 800_000; // 80% performance

        let partner_share = calculate_revenue_share(&config, total_revenue, Some(performance_score)).unwrap();

        // Adjusted share = 10% * 1.2 * 0.8 = 9.6%, clamped to min/max
        // Should be between 5% and 20%
        assert!(partner_share >= 500_000_000); // At least 5%
        assert!(partner_share <= 2_000_000_000); // At most 20%
    }

    #[test]
    fn test_create_revenue_distribution() {
        let mut distribution = RevenueDistribution {
            distribution_id: 0,
            partnership_id: 0,
            total_revenue: 0,
            partner_share: 0,
            dao_share: 0,
            share_percentage: 0,
            status: DistributionStatus::Pending,
            period_start: 0,
            period_end: 0,
            distributed_at: None,
            distribution_hash: [0u8; 32],
            distributor: Pubkey::default(),
            partner_recipient: Pubkey::default(),
            bump: 0,
        };

        let distributor = Pubkey::from([1u8; 32]);
        let partner_recipient = Pubkey::from([2u8; 32]);
        let distribution_hash = [1u8; 32];

        let total_revenue = 10_000_000_000; // 10 SOL
        let share_percentage = 100_000; // 10%

        let result = create_revenue_distribution(
            &mut distribution,
            1,
            100,
            total_revenue,
            share_percentage,
            1000,
            2000,
            distribution_hash,
            distributor,
            partner_recipient,
        );

        assert!(result.is_ok());
        assert_eq!(distribution.distribution_id, 1);
        assert_eq!(distribution.total_revenue, total_revenue);
        assert_eq!(distribution.partner_share, 1_000_000_000); // 10% of 10 SOL
        assert_eq!(distribution.dao_share, 9_000_000_000); // 90% of 10 SOL
        assert_eq!(distribution.status, DistributionStatus::Pending);
    }

    #[test]
    fn test_complete_revenue_distribution() {
        let mut distribution = RevenueDistribution {
            distribution_id: 1,
            partnership_id: 100,
            total_revenue: 10_000_000_000,
            partner_share: 1_000_000_000,
            dao_share: 9_000_000_000,
            share_percentage: 100_000,
            status: DistributionStatus::Pending,
            period_start: 1000,
            period_end: 2000,
            distributed_at: None,
            distribution_hash: [0u8; 32],
            distributor: Pubkey::default(),
            partner_recipient: Pubkey::default(),
            bump: 0,
        };

        let result = complete_revenue_distribution_with_time(&mut distribution, 1000);

        assert!(result.is_ok());
        assert_eq!(distribution.status, DistributionStatus::Completed);
        assert!(distribution.distributed_at.is_some());
    }

    #[test]
    fn test_revenue_distribution_calculates_shares_correctly() {
        let mut distribution = RevenueDistribution {
            distribution_id: 0,
            partnership_id: 0,
            total_revenue: 0,
            partner_share: 0,
            dao_share: 0,
            share_percentage: 0,
            status: DistributionStatus::Pending,
            period_start: 0,
            period_end: 0,
            distributed_at: None,
            distribution_hash: [0u8; 32],
            distributor: Pubkey::default(),
            partner_recipient: Pubkey::default(),
            bump: 0,
        };

        let total_revenue = 100_000_000_000; // 100 SOL
        let share_percentage = 150_000; // 15%

        create_revenue_distribution(
            &mut distribution,
            1,
            100,
            total_revenue,
            share_percentage,
            1000,
            2000,
            [0u8; 32],
            Pubkey::default(),
            Pubkey::default(),
        ).unwrap();

        assert_eq!(distribution.partner_share, 15_000_000_000); // 15 SOL
        assert_eq!(distribution.dao_share, 85_000_000_000); // 85 SOL
        assert_eq!(distribution.partner_share + distribution.dao_share, total_revenue);
    }
}
