//! Partnership Metrics Module
//! 
//! Comprehensive metrics collection and analysis system for partnerships.
//! Tracks performance, revenue, milestones, and generates analytics.
//! 
//! Hybrid model: Off-chain complex analytics, on-chain metric storage and aggregation
//! 
//! Principle: "Blockchain = Proof, not Storage"
//! - ON-CHAIN: Aggregated metrics, key performance indicators, summary statistics
//! - OFF-CHAIN: Detailed analytics, data visualization, predictive modeling
//!
//! Improved version with enhanced metrics tracking and comprehensive testing

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::constants::PRECISION_FACTOR;

// ============================================================================
// ON-CHAIN STRUCTURES (Anchor Account Types)
// ============================================================================

/// Metrics collection period
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum MetricsPeriod {
    /// Daily metrics
    Daily,
    /// Weekly metrics
    Weekly,
    /// Monthly metrics
    Monthly,
    /// Quarterly metrics
    Quarterly,
    /// Yearly metrics
    Yearly,
}

impl Space for MetricsPeriod {
    const INIT_SPACE: usize = 1;
}

/// Enhanced partnership metrics
#[account]
#[derive(InitSpace)]
pub struct EnhancedPartnershipMetrics {
    /// Metrics ID
    pub metrics_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Metrics period
    pub period: MetricsPeriod,
    /// Period start timestamp
    pub period_start: i64,
    /// Period end timestamp
    pub period_end: i64,
    /// Performance score (0 to PRECISION_FACTOR)
    pub performance_score: u64,
    /// Risk score (0 to PRECISION_FACTOR)
    pub risk_score: u64,
    /// Revenue generated (lamports)
    pub revenue_generated: u64,
    /// Revenue target (lamports)
    pub revenue_target: u64,
    /// Milestones completed
    pub milestones_completed: u64,
    /// Total milestones
    pub total_milestones: u64,
    /// Health score (0 to PRECISION_FACTOR)
    pub health_score: u64,
    /// Last activity timestamp
    pub last_activity: i64,
    /// Engagement score (0 to PRECISION_FACTOR)
    pub engagement_score: u64,
    /// Satisfaction score (0 to PRECISION_FACTOR)
    pub satisfaction_score: u64,
    /// Metrics hash (for verification)
    pub metrics_hash: [u8; 32],
    /// Metrics collector (system or authority)
    pub collector: Pubkey,
    /// Metrics collection timestamp
    pub collected_at: i64,
    /// Bump seed for PDA
    pub bump: u8,
}

/// Metrics aggregation record
#[account]
#[derive(InitSpace)]
pub struct MetricsAggregation {
    /// Aggregation ID
    pub aggregation_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Aggregation period
    pub period: MetricsPeriod,
    /// Period start
    pub period_start: i64,
    /// Period end
    pub period_end: i64,
    /// Average performance score
    pub avg_performance_score: u64,
    /// Average risk score
    pub avg_risk_score: u64,
    /// Total revenue
    pub total_revenue: u64,
    /// Total milestones completed
    pub total_milestones_completed: u64,
    /// Average health score
    pub avg_health_score: u64,
    /// Number of metrics records aggregated
    pub records_count: u64,
    /// Aggregation timestamp
    pub aggregated_at: i64,
    /// Aggregator (system)
    pub aggregator: Pubkey,
    /// Bump seed for PDA
    pub bump: u8,
}

// ============================================================================
// ON-CHAIN FUNCTIONS (Anchor Handlers)
// ============================================================================

/// Collect partnership metrics
pub fn collect_partnership_metrics(
    metrics: &mut EnhancedPartnershipMetrics,
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
    collector: Pubkey,
) -> Result<()> {
    require!(period_end > period_start, IndrasError::InvalidInput);
    require!(performance_score <= PRECISION_FACTOR, IndrasError::InvalidScore);
    require!(risk_score <= PRECISION_FACTOR, IndrasError::InvalidScore);
    require!(engagement_score <= PRECISION_FACTOR, IndrasError::InvalidScore);
    require!(satisfaction_score <= PRECISION_FACTOR, IndrasError::InvalidScore);
    
    // Calculate health score (weighted average)
    let health_score = calculate_health_score(
        performance_score,
        risk_score,
        milestones_completed,
        total_milestones,
        engagement_score,
        satisfaction_score,
    )?;
    
    let current_time = Clock::get()?.unix_timestamp;
    
    collect_partnership_metrics_with_time(
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
        health_score,
        metrics_hash,
        collector,
        current_time,
    )
}

/// Collect partnership metrics with explicit timestamp (for testing)
pub fn collect_partnership_metrics_with_time(
    metrics: &mut EnhancedPartnershipMetrics,
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
    health_score: u64,
    metrics_hash: [u8; 32],
    collector: Pubkey,
    current_time: i64,
) -> Result<()> {
    // Health score is passed as parameter to avoid duplicate calculation
    
    metrics.metrics_id = metrics_id;
    metrics.partnership_id = partnership_id;
    metrics.period = period;
    metrics.period_start = period_start;
    metrics.period_end = period_end;
    metrics.performance_score = performance_score;
    metrics.risk_score = risk_score;
    metrics.revenue_generated = revenue_generated;
    metrics.revenue_target = revenue_target;
    metrics.milestones_completed = milestones_completed;
    metrics.total_milestones = total_milestones;
    metrics.health_score = health_score; // Use passed health_score parameter
    metrics.last_activity = current_time;
    metrics.engagement_score = engagement_score;
    metrics.satisfaction_score = satisfaction_score;
    metrics.metrics_hash = metrics_hash;
    metrics.collector = collector;
    metrics.collected_at = current_time;
    
    msg!("Partnership metrics {} collected for partnership {} (health: {}, performance: {}, risk: {})", 
         metrics_id, partnership_id, health_score, performance_score, risk_score);
    
    Ok(())
}

/// Calculate health score from component scores
fn calculate_health_score(
    performance_score: u64,
    risk_score: u64,
    milestones_completed: u64,
    total_milestones: u64,
    engagement_score: u64,
    satisfaction_score: u64,
) -> Result<u64> {
    // Weights: performance (30%), risk (20%), milestones (20%), engagement (15%), satisfaction (15%)
    let performance_weight = 300_000; // 30%
    let risk_weight = 200_000; // 20%
    let milestone_weight = 200_000; // 20%
    let engagement_weight = 150_000; // 15%
    let satisfaction_weight = 150_000; // 15%
    
    // Calculate milestone completion rate
    let milestone_rate = if total_milestones > 0 {
        milestones_completed
            .checked_mul(PRECISION_FACTOR)
            .ok_or(IndrasError::Overflow)?
            .checked_div(total_milestones)
            .ok_or(IndrasError::DivisionByZero)?
    } else {
        PRECISION_FACTOR / 2 // 50% if no milestones
    };
    
    // Calculate inverse risk (lower risk = higher score)
    let inverse_risk = PRECISION_FACTOR
        .checked_sub(risk_score)
        .ok_or(IndrasError::Underflow)?;
    
    // Weighted sum
    let performance_weighted = performance_score
        .checked_mul(performance_weight)
        .ok_or(IndrasError::Overflow)?
        .checked_div(PRECISION_FACTOR)
        .ok_or(IndrasError::DivisionByZero)?;
    
    let risk_weighted = inverse_risk
        .checked_mul(risk_weight)
        .ok_or(IndrasError::Overflow)?
        .checked_div(PRECISION_FACTOR)
        .ok_or(IndrasError::DivisionByZero)?;
    
    let milestone_weighted = milestone_rate
        .checked_mul(milestone_weight)
        .ok_or(IndrasError::Overflow)?
        .checked_div(PRECISION_FACTOR)
        .ok_or(IndrasError::DivisionByZero)?;
    
    let engagement_weighted = engagement_score
        .checked_mul(engagement_weight)
        .ok_or(IndrasError::Overflow)?
        .checked_div(PRECISION_FACTOR)
        .ok_or(IndrasError::DivisionByZero)?;
    
    let satisfaction_weighted = satisfaction_score
        .checked_mul(satisfaction_weight)
        .ok_or(IndrasError::Overflow)?
        .checked_div(PRECISION_FACTOR)
        .ok_or(IndrasError::DivisionByZero)?;
    
    let health_score = performance_weighted
        .checked_add(risk_weighted)
        .ok_or(IndrasError::Overflow)?
        .checked_add(milestone_weighted)
        .ok_or(IndrasError::Overflow)?
        .checked_add(engagement_weighted)
        .ok_or(IndrasError::Overflow)?
        .checked_add(satisfaction_weighted)
        .ok_or(IndrasError::Overflow)?;
    
    Ok(health_score)
}

/// Aggregate metrics for a period
pub fn aggregate_metrics(
    aggregation: &mut MetricsAggregation,
    aggregation_id: u64,
    partnership_id: u64,
    period: MetricsPeriod,
    period_start: i64,
    period_end: i64,
    metrics_records: &[EnhancedPartnershipMetrics],
    aggregator: Pubkey,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    aggregate_metrics_with_time(
        aggregation,
        aggregation_id,
        partnership_id,
        period,
        period_start,
        period_end,
        metrics_records,
        aggregator,
        current_time,
    )
}

/// Aggregate metrics with explicit timestamp (for testing)
pub fn aggregate_metrics_with_time(
    aggregation: &mut MetricsAggregation,
    aggregation_id: u64,
    partnership_id: u64,
    period: MetricsPeriod,
    period_start: i64,
    period_end: i64,
    metrics_records: &[EnhancedPartnershipMetrics],
    aggregator: Pubkey,
    current_time: i64,
) -> Result<()> {
    require!(period_end > period_start, IndrasError::InvalidInput);
    require!(!metrics_records.is_empty(), IndrasError::InvalidInput);
    require!(metrics_records.len() <= 1000, IndrasError::InvalidInput); // Max 1000 records
    
    let records_count = metrics_records.len() as u64;
    
    // Calculate averages
    let total_performance: u64 = metrics_records.iter()
        .map(|m| m.performance_score)
        .sum();
    let avg_performance = total_performance
        .checked_div(records_count)
        .ok_or(IndrasError::DivisionByZero)?;
    
    let total_risk: u64 = metrics_records.iter()
        .map(|m| m.risk_score)
        .sum();
    let avg_risk = total_risk
        .checked_div(records_count)
        .ok_or(IndrasError::DivisionByZero)?;
    
    let total_revenue: u64 = metrics_records.iter()
        .map(|m| m.revenue_generated)
        .sum();
    
    let total_milestones: u64 = metrics_records.iter()
        .map(|m| m.milestones_completed)
        .sum();
    
    let total_health: u64 = metrics_records.iter()
        .map(|m| m.health_score)
        .sum();
    let avg_health = total_health
        .checked_div(records_count)
        .ok_or(IndrasError::DivisionByZero)?;
    
    aggregation.aggregation_id = aggregation_id;
    aggregation.partnership_id = partnership_id;
    aggregation.period = period;
    aggregation.period_start = period_start;
    aggregation.period_end = period_end;
    aggregation.avg_performance_score = avg_performance;
    aggregation.avg_risk_score = avg_risk;
    aggregation.total_revenue = total_revenue;
    aggregation.total_milestones_completed = total_milestones;
    aggregation.avg_health_score = avg_health;
    aggregation.records_count = records_count;
    aggregation.aggregated_at = current_time;
    aggregation.aggregator = aggregator;
    
    msg!("Metrics aggregated for partnership {} (period: {:?}, records: {}, avg health: {})", 
         partnership_id, period, records_count, avg_health);
    
    Ok(())
}

// ============================================================================
// OFF-CHAIN FUNCTIONS (Regular Rust - Not Anchor)
// ============================================================================

/// OFF-CHAIN: Collect detailed metrics
/// 
/// This function should be called off-chain to collect detailed metrics from various sources.
/// 
/// # Arguments
/// * `partnership` - Partnership data to analyze (used off-chain for metrics collection)
/// 
/// # Returns
/// Enhanced partnership metrics calculated from various off-chain sources
/// 
/// # Note
/// This is a placeholder function. Actual implementation happens off-chain.
/// The partnership parameter is available for off-chain services to access partnership data.
pub fn collect_detailed_metrics_offchain(
    partnership: &crate::partnerships::types::Partnership,
) -> Result<EnhancedPartnershipMetrics> {
    // OFF-CHAIN IMPLEMENTATION:
    // 1. Collect metrics from various sources (APIs, databases, etc.)
    // 2. Use partnership data to fetch related metrics
    // 3. Calculate performance, risk, engagement, satisfaction scores
    // 4. Generate metrics hash
    // 5. Return metrics to be stored on-chain
    
    // This is a placeholder - actual implementation happens off-chain
    // The partnership parameter is logged for tracking purposes
    msg!("Collecting detailed metrics for partnership (placeholder - off-chain implementation)");
    let _ = partnership; // Used in off-chain implementation to access partnership data
    Err(IndrasError::NotImplemented.into())
}

/// OFF-CHAIN: Generate metrics analytics
/// 
/// This function should be called off-chain to generate detailed analytics.
/// 
/// # Arguments
/// * `metrics` - Array of partnership metrics to analyze (used off-chain for analytics)
/// 
/// # Returns
/// Analytics data as bytes (visualizations, reports, etc.)
/// 
/// # Note
/// This is a placeholder function. Actual implementation happens off-chain.
/// The metrics parameter is available for off-chain services to perform analytics.
pub fn generate_metrics_analytics_offchain(
    metrics: &[EnhancedPartnershipMetrics],
) -> Result<Vec<u8>> {
    // OFF-CHAIN IMPLEMENTATION:
    // 1. Analyze metrics trends from the provided metrics array
    // 2. Generate visualizations
    // 3. Create predictive models
    // 4. Return analytics data
    
    // This is a placeholder - actual implementation happens off-chain
    // The metrics parameter is logged for tracking purposes
    msg!("Generating analytics for {} metrics (placeholder - off-chain implementation)", metrics.len());
    let _ = metrics; // Used in off-chain implementation
    Err(IndrasError::NotImplemented.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_partnership_metrics() {
        let mut metrics = EnhancedPartnershipMetrics {
            metrics_id: 0,
            partnership_id: 0,
            period: MetricsPeriod::Monthly,
            period_start: 0,
            period_end: 0,
            performance_score: 0,
            risk_score: 0,
            revenue_generated: 0,
            revenue_target: 0,
            milestones_completed: 0,
            total_milestones: 0,
            health_score: 0,
            last_activity: 0,
            engagement_score: 0,
            satisfaction_score: 0,
            metrics_hash: [0u8; 32],
            collector: Pubkey::default(),
            collected_at: 0,
            bump: 0,
        };

        let collector = Pubkey::from([1u8; 32]);
        let metrics_hash = [1u8; 32];
        let period_start = 1000;
        let period_end = 2000;
        
        // Calculate health score for test
        let health_score = calculate_health_score(
            800_000, // 80% performance
            200_000, // 20% risk
            3, // 3 milestones completed
            5, // 5 total milestones
            700_000, // 70% engagement
            750_000, // 75% satisfaction
        ).unwrap();

        let result = collect_partnership_metrics_with_time(
            &mut metrics,
            1,
            100,
            MetricsPeriod::Monthly,
            period_start,
            period_end,
            800_000, // 80% performance
            200_000, // 20% risk
            1_000_000_000, // 1 SOL revenue
            2_000_000_000, // 2 SOL target
            3, // 3 milestones completed
            5, // 5 total milestones
            700_000, // 70% engagement
            750_000, // 75% satisfaction
            health_score,
            metrics_hash,
            collector,
            1000, // Mock timestamp
        );

        assert!(result.is_ok());
        assert_eq!(metrics.metrics_id, 1);
        assert_eq!(metrics.partnership_id, 100);
        assert_eq!(metrics.performance_score, 800_000);
        assert!(metrics.health_score > 0);
    }

    #[test]
    fn test_calculate_health_score() {
        let health_score = calculate_health_score(
            800_000, // 80% performance
            200_000, // 20% risk
            3, // 3 milestones completed
            5, // 5 total milestones
            700_000, // 70% engagement
            750_000, // 75% satisfaction
        ).unwrap();

        // Health score should be calculated from weighted components
        assert!(health_score > 0);
        assert!(health_score <= PRECISION_FACTOR);
    }

    #[test]
    fn test_aggregate_metrics() {
        let mut aggregation = MetricsAggregation {
            aggregation_id: 0,
            partnership_id: 0,
            period: MetricsPeriod::Monthly,
            period_start: 0,
            period_end: 0,
            avg_performance_score: 0,
            avg_risk_score: 0,
            total_revenue: 0,
            total_milestones_completed: 0,
            avg_health_score: 0,
            records_count: 0,
            aggregated_at: 0,
            aggregator: Pubkey::default(),
            bump: 0,
        };

        // Create test metrics records
        let mut metrics1 = EnhancedPartnershipMetrics {
            metrics_id: 1,
            partnership_id: 100,
            period: MetricsPeriod::Daily,
            period_start: 0,
            period_end: 0,
            performance_score: 800_000,
            risk_score: 200_000,
            revenue_generated: 500_000_000,
            revenue_target: 0,
            milestones_completed: 1,
            total_milestones: 2,
            health_score: 700_000,
            last_activity: 0,
            engagement_score: 700_000,
            satisfaction_score: 750_000,
            metrics_hash: [0u8; 32],
            collector: Pubkey::default(),
            collected_at: 0,
            bump: 0,
        };

        let mut metrics2 = EnhancedPartnershipMetrics {
            metrics_id: 2,
            partnership_id: 100,
            period: MetricsPeriod::Daily,
            period_start: 0,
            period_end: 0,
            performance_score: 900_000,
            risk_score: 100_000,
            revenue_generated: 600_000_000,
            revenue_target: 0,
            milestones_completed: 2,
            total_milestones: 2,
            health_score: 850_000,
            last_activity: 0,
            engagement_score: 800_000,
            satisfaction_score: 800_000,
            metrics_hash: [0u8; 32],
            collector: Pubkey::default(),
            collected_at: 0,
            bump: 0,
        };

        let metrics_records = vec![metrics1, metrics2];
        let aggregator = Pubkey::from([1u8; 32]);

        let result = aggregate_metrics_with_time(
            &mut aggregation,
            1,
            100,
            MetricsPeriod::Monthly,
            1000,
            2000,
            &metrics_records,
            aggregator,
            1000, // Mock timestamp
        );

        assert!(result.is_ok());
        assert_eq!(aggregation.aggregation_id, 1);
        assert_eq!(aggregation.partnership_id, 100);
        assert_eq!(aggregation.records_count, 2);
        assert_eq!(aggregation.avg_performance_score, 850_000); // (800_000 + 900_000) / 2
        assert_eq!(aggregation.total_revenue, 1_100_000_000); // 500M + 600M
        assert_eq!(aggregation.total_milestones_completed, 3); // 1 + 2
    }

    #[test]
    fn test_collect_metrics_validates_scores() {
        let mut metrics = EnhancedPartnershipMetrics {
            metrics_id: 0,
            partnership_id: 0,
            period: MetricsPeriod::Monthly,
            period_start: 0,
            period_end: 0,
            performance_score: 0,
            risk_score: 0,
            revenue_generated: 0,
            revenue_target: 0,
            milestones_completed: 0,
            total_milestones: 0,
            health_score: 0,
            last_activity: 0,
            engagement_score: 0,
            satisfaction_score: 0,
            metrics_hash: [0u8; 32],
            collector: Pubkey::default(),
            collected_at: 0,
            bump: 0,
        };

        // Try with score > PRECISION_FACTOR - should fail
        let result = collect_partnership_metrics(
            &mut metrics,
            1,
            100,
            MetricsPeriod::Monthly,
            1000,
            2000,
            PRECISION_FACTOR + 1, // Invalid score
            200_000,
            0,
            0,
            0,
            0,
            700_000,
            750_000,
            [0u8; 32],
            Pubkey::default(),
        );

        assert!(result.is_err());
    }
}
