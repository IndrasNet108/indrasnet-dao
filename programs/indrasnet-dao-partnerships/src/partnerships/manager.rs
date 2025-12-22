//! Partnership Manager - Core partnership management logic
//!
//! Hybrid model: Off-chain analytics and reporting, on-chain validation and state management
//! 
//! Principle: "Blockchain = Proof, not Storage"
//! - ON-CHAIN: Validation, state management, basic calculations
//! - OFF-CHAIN: Complex analytics, reporting, recommendations generation
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-partnerships/src/partnerships/manager.rs

use anchor_lang::prelude::*;
use crate::constants::PRECISION_FACTOR;
use crate::error::IndrasError;
use super::types::*;
use indrasnet_dao_core::types::RiskLevel;

// ============================================================================
// ON-CHAIN MANAGER (Validation and State Management)
// ============================================================================

/// Partnership Manager implementation
/// 
/// ON-CHAIN: Validates inputs, manages state, performs basic calculations
/// OFF-CHAIN: Complex analytics, reporting, recommendations (see OFF-CHAIN section below)
pub struct PartnershipManagerImpl;

impl PartnershipManagerImpl {
    /// Initialize partnership system
    pub fn initialize_system(
        config: &mut PartnershipConfig,
        max_partnerships: u64,
        min_value: u64,
        max_value: u64,
    ) -> Result<()> {
        require!(max_partnerships > 0, IndrasError::InvalidInput);
        require!(min_value > 0, IndrasError::InvalidInput);
        require!(max_value > min_value, IndrasError::InvalidInput);

        config.max_partnerships = max_partnerships;
        config.min_partnership_value = min_value;
        config.max_partnership_value = max_value;

        Ok(())
    }

    /// Validate partnership terms
    /// 
    /// ON-CHAIN: Validates terms against configuration
    /// OFF-CHAIN: Legal review, compliance checking happens off-chain
    pub fn validate_terms(
        terms: &PartnershipTerms,
        config: &PartnershipConfig,
    ) -> Result<()> {
        // Validate value range
        require!(
            terms.estimated_value >= config.min_partnership_value,
            IndrasError::AmountTooSmall
        );
        require!(
            terms.estimated_value <= config.max_partnership_value,
            IndrasError::AmountTooLarge
        );

        // Validate revenue share (0-100%)
        require!(
            terms.revenue_share <= PRECISION_FACTOR,
            IndrasError::InvalidInput
        );

        // Validate governance power (0-100%)
        require!(
            terms.governance_power <= PRECISION_FACTOR,
            IndrasError::InvalidInput
        );

        // Validate dates
        if let Some(end_date) = terms.end_date {
            require!(
                end_date > terms.start_date,
                IndrasError::InvalidInput
            );
        }

        // Validate technical requirements
        for req in &terms.technical_requirements {
            require!(req.len() <= 200, IndrasError::StringTooLong);
        }

        // Validate legal requirements
        for req in &terms.legal_requirements {
            require!(req.len() <= 200, IndrasError::StringTooLong);
        }

        // Validate success metrics
        for metric in &terms.success_metrics {
            require!(metric.len() <= 200, IndrasError::StringTooLong);
        }

        Ok(())
    }

    /// Calculate partnership ROI
    /// 
    /// ON-CHAIN: Basic ROI calculation
    /// OFF-CHAIN: Complex financial analysis, forecasting happens off-chain
    pub fn calculate_roi(
        partnership: &Partnership,
        metrics: &PartnershipMetrics,
        time_period: i64,
    ) -> Result<u64> {
        let investment = partnership.terms.estimated_value;
        let revenue = metrics.revenue_generated;
        let time_factor = if time_period > 0 {
            PRECISION_FACTOR
                .checked_mul(365 * 24 * 60 * 60) // 1 year in seconds
                .ok_or(IndrasError::Overflow)?
                .checked_div(time_period as u64)
                .ok_or(IndrasError::DivisionByZero)?
        } else {
            PRECISION_FACTOR
        };

        if investment == 0 {
            return Ok(0);
        }

        let annualized_revenue = revenue
            .checked_mul(time_factor)
            .ok_or(IndrasError::Overflow)?
            .checked_div(PRECISION_FACTOR)
            .ok_or(IndrasError::DivisionByZero)?;

        let roi = annualized_revenue
            .checked_mul(PRECISION_FACTOR)
            .ok_or(IndrasError::Overflow)?
            .checked_div(investment)
            .ok_or(IndrasError::DivisionByZero)?;

        Ok(roi)
    }

    /// Assess partnership risk
    /// 
    /// ON-CHAIN: Basic risk assessment based on metrics
    /// OFF-CHAIN: Advanced risk modeling, ML-based risk analysis happens off-chain
    pub fn assess_risk(
        partnership: &Partnership,
        metrics: &PartnershipMetrics,
    ) -> Result<RiskLevel> {
        let mut risk_score = 0u64;

        // Time-based risk
        let current_time = Clock::get()?.unix_timestamp;
        let partnership_age = current_time - partnership.created_at;
        let age_risk = if partnership_age > 365 * 24 * 60 * 60 { // 1 year
            200_000 // 20% risk
        } else if partnership_age > 180 * 24 * 60 * 60 { // 6 months
            100_000 // 10% risk
        } else {
            50_000 // 5% risk
        };
        risk_score = risk_score.checked_add(age_risk).ok_or(IndrasError::Overflow)?;

        // Performance risk
        let performance_risk = if metrics.performance_score < 300_000 { // < 30%
            300_000 // 30% risk
        } else if metrics.performance_score < 500_000 { // < 50%
            200_000 // 20% risk
        } else {
            100_000 // 10% risk
        };
        risk_score = risk_score.checked_add(performance_risk).ok_or(IndrasError::Overflow)?;

        // Milestone completion risk
        let milestone_completion_rate = if metrics.total_milestones > 0 {
            metrics.milestones_completed
                .checked_mul(PRECISION_FACTOR)
                .ok_or(IndrasError::Overflow)?
                .checked_div(metrics.total_milestones)
                .ok_or(IndrasError::DivisionByZero)?
        } else {
            PRECISION_FACTOR / 2 // 50% if no milestones
        };

        let milestone_risk = if milestone_completion_rate < 300_000 { // < 30%
            200_000 // 20% risk
        } else if milestone_completion_rate < 500_000 { // < 50%
            100_000 // 10% risk
        } else {
            50_000 // 5% risk
        };
        risk_score = risk_score.checked_add(milestone_risk).ok_or(IndrasError::Overflow)?;

        // Activity risk
        let days_since_activity = (current_time - metrics.last_activity) / (24 * 60 * 60);
        let activity_risk = if days_since_activity > 90 { // 3 months
            300_000 // 30% risk
        } else if days_since_activity > 30 { // 1 month
            150_000 // 15% risk
        } else {
            50_000 // 5% risk
        };
        risk_score = risk_score.checked_add(activity_risk).ok_or(IndrasError::Overflow)?;

        // Cap risk score at 100%
        risk_score = risk_score.min(PRECISION_FACTOR);

        // Determine risk level
        let risk_level = if risk_score > 800_000 {
            RiskLevel::Critical
        } else if risk_score > 600_000 {
            RiskLevel::High
        } else if risk_score > 400_000 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        Ok(risk_level)
    }

    /// Generate partnership report
    /// 
    /// NOTE: Complex report generation happens OFF-CHAIN.
    /// This function creates a basic report structure.
    /// 
    /// ON-CHAIN: Basic report with calculated metrics
    /// OFF-CHAIN: Detailed analytics, visualizations, recommendations generation happens off-chain
    pub fn generate_report(
        partnership: &Partnership,
        metrics: &PartnershipMetrics,
    ) -> Result<PartnershipReport> {
        let health_score = Self::calculate_health_score(partnership, metrics)?;
        let roi = Self::calculate_roi(partnership, metrics, 365 * 24 * 60 * 60)?; // 1 year
        let risk_level = Self::assess_risk(partnership, metrics)?;

        Ok(PartnershipReport {
            partnership_id: partnership.id,
            generated_at: Clock::get()?.unix_timestamp,
            health_score,
            roi,
            risk_level,
            performance_score: metrics.performance_score,
            revenue_generated: metrics.revenue_generated,
            milestones_completed: metrics.milestones_completed,
            total_milestones: metrics.total_milestones,
            recommendations: Self::generate_recommendations(partnership, metrics)?,
        })
    }

    /// Calculate health score
    /// 
    /// ON-CHAIN: Basic health score calculation
    /// OFF-CHAIN: Advanced health modeling happens off-chain
    fn calculate_health_score(
        _partnership: &Partnership,
        metrics: &PartnershipMetrics,
    ) -> Result<u64> {
        let performance_weight = 400_000; // 40%
        let milestone_weight = 300_000; // 30%
        let activity_weight = 300_000; // 30%

        let performance_score = metrics.performance_score
            .checked_mul(performance_weight)
            .ok_or(IndrasError::Overflow)?
            .checked_div(PRECISION_FACTOR)
            .ok_or(IndrasError::DivisionByZero)?;

        let milestone_score = if metrics.total_milestones > 0 {
            metrics.milestones_completed
                .checked_mul(PRECISION_FACTOR)
                .ok_or(IndrasError::Overflow)?
                .checked_div(metrics.total_milestones)
                .ok_or(IndrasError::DivisionByZero)?
        } else {
            PRECISION_FACTOR / 2 // 50% if no milestones
        };

        let milestone_weighted = milestone_score
            .checked_mul(milestone_weight)
            .ok_or(IndrasError::Overflow)?
            .checked_div(PRECISION_FACTOR)
            .ok_or(IndrasError::DivisionByZero)?;

        let current_time = Clock::get()?.unix_timestamp;
        let days_since_activity = (current_time - metrics.last_activity) / (24 * 60 * 60);
        let activity_score = if days_since_activity <= 7 {
            PRECISION_FACTOR // 100%
        } else if days_since_activity <= 30 {
            PRECISION_FACTOR * 3 / 4 // 75%
        } else if days_since_activity <= 90 {
            PRECISION_FACTOR / 2 // 50%
        } else {
            PRECISION_FACTOR / 4 // 25%
        };

        let activity_weighted = activity_score
            .checked_mul(activity_weight)
            .ok_or(IndrasError::Overflow)?
            .checked_div(PRECISION_FACTOR)
            .ok_or(IndrasError::DivisionByZero)?;

        let total_score = performance_score
            .checked_add(milestone_weighted)
            .ok_or(IndrasError::Overflow)?
            .checked_add(activity_weighted)
            .ok_or(IndrasError::Overflow)?;

        Ok(total_score)
    }

    /// Generate recommendations
    /// 
    /// ON-CHAIN: Basic recommendations based on metrics
    /// OFF-CHAIN: AI-powered recommendations, detailed analysis happens off-chain
    fn generate_recommendations(
        partnership: &Partnership,
        metrics: &PartnershipMetrics,
    ) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        // Performance recommendations
        if metrics.performance_score < 500_000 {
            recommendations.push("Consider reviewing partnership terms and objectives".to_string());
        }

        // Milestone recommendations
        if metrics.total_milestones > 0 {
            let completion_rate = metrics.milestones_completed
                .checked_mul(100)
                .ok_or(IndrasError::Overflow)?
                .checked_div(metrics.total_milestones)
                .ok_or(IndrasError::DivisionByZero)?;

            if completion_rate < 50 {
                recommendations.push("Focus on completing pending milestones".to_string());
            }
        }

        // Activity recommendations
        let current_time = Clock::get()?.unix_timestamp;
        let days_since_activity = (current_time - metrics.last_activity) / (24 * 60 * 60);
        if days_since_activity > 30 {
            recommendations.push("Increase partnership activity and communication".to_string());
        }

        // Risk recommendations
        if metrics.risk_score > 700_000 {
            recommendations.push("Address high-risk factors immediately".to_string());
        }

        // Revenue recommendations
        if metrics.revenue_generated == 0 && partnership.terms.estimated_value > 0 {
            recommendations.push("Focus on revenue generation strategies".to_string());
        }

        Ok(recommendations)
    }
}

// ============================================================================
// ON-CHAIN STRUCTURES
// ============================================================================

/// Partnership report structure
/// 
/// ON-CHAIN: Stores basic report data
/// OFF-CHAIN: Detailed reports with visualizations stored off-chain
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct PartnershipReport {
    pub partnership_id: u64,
    pub generated_at: i64,
    pub health_score: u64,
    pub roi: u64,
    pub risk_level: RiskLevel,
    pub performance_score: u64,
    pub revenue_generated: u64,
    pub milestones_completed: u64,
    pub total_milestones: u64,
    pub recommendations: Vec<String>,
}

impl Space for PartnershipReport {
    const INIT_SPACE: usize = 8 + 8 + 8 + 8 + 1 + 8 + 8 + 8 + 8 + 4; // Basic fields + Vec overhead
}

// ============================================================================
// OFF-CHAIN IMPLEMENTATION NOTES
// ============================================================================

/*

OFF-CHAIN PARTNERSHIP ANALYTICS (Standard Rust, not Anchor)

The following operations MUST be performed off-chain:

1. **Advanced Analytics:**
   ```rust
   // offchain_services/src/partnerships/analytics.rs
   use ml_models::partnership_analysis;
   
   pub async fn generate_detailed_report_offchain(
       partnership: &Partnership,
       metrics: &PartnershipMetrics,
   ) -> Result<DetailedReport> {
       // 1. Advanced financial analysis
       // 2. ML-based risk modeling
       // 3. Predictive analytics
       // 4. Generate visualizations
       // 5. Create detailed recommendations
   }
   ```

2. **Recommendations Generation:**
   ```rust
   pub async fn generate_ai_recommendations(
       partnership: &Partnership,
       metrics: &PartnershipMetrics,
   ) -> Result<Vec<Recommendation>> {
       // 1. Analyze partnership performance
       // 2. Use ML models for recommendations
       // 3. Consider market conditions
       // 4. Generate actionable recommendations
   }
   ```

Required off-chain dependencies:
- ML models for partnership analysis
- Financial analysis libraries
- Data visualization tools
- Statistical analysis libraries

*/
