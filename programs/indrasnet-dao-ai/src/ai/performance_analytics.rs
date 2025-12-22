//! Performance Analytics Module
//! 
//! Comprehensive performance analytics and metrics system for ideas, mesh groups, and operations.
//! Integrates with AI analysis and other modules for performance evaluation.
//! 
//! Hybrid model: Off-chain performance analytics and analysis, on-chain metadata storage
//! 
//! Principle: "Blockchain = Proof, not Storage"
//! - ON-CHAIN: Only performance metrics metadata, scores, and summary records
//! - OFF-CHAIN: All actual performance analytics, data aggregation, and detailed analysis
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-ai/src/ai/performance_analytics.rs

use anchor_lang::prelude::*;
use crate::error::IndrasError;

// ============================================================================
// ON-CHAIN STRUCTURES (Anchor Account Types)
// ============================================================================

/// Performance metric type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum PerformanceMetricType {
    /// Execution speed
    ExecutionSpeed,
    /// Resource utilization
    ResourceUtilization,
    /// Success rate
    SuccessRate,
    /// User satisfaction
    UserSatisfaction,
    /// Cost efficiency
    CostEfficiency,
    /// Innovation impact
    InnovationImpact,
}

impl Space for PerformanceMetricType {
    const INIT_SPACE: usize = 1;
}

/// Performance analytics record
#[account]
#[derive(InitSpace)]
pub struct PerformanceAnalytics {
    /// Analytics ID
    pub analytics_id: u64,
    /// Entity ID (idea, mesh group, etc.)
    pub entity_id: u64,
    /// Entity type
    #[max_len(50)]
    pub entity_type: String,
    /// Performance scores by metric type (0-100)
    pub performance_scores: PerformanceScores,
    /// Overall performance score (0-100)
    pub overall_score: u8,
    /// Analytics timestamp
    pub analyzed_at: i64,
    /// Analyzer (AI system or authority)
    pub analyzer: Pubkey,
    /// Analytics metadata URI (IPFS or similar)
    #[max_len(500)]
    pub metadata_uri: String,
    /// Analytics hash (for verification)
    pub analytics_hash: [u8; 32],
    /// Bump seed for PDA
    pub bump: u8,
}

/// Performance scores by metric type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct PerformanceScores {
    /// Execution speed score (0-100)
    pub execution_speed: u8,
    /// Resource utilization score (0-100)
    pub resource_utilization: u8,
    /// Success rate score (0-100)
    pub success_rate: u8,
    /// User satisfaction score (0-100)
    pub user_satisfaction: u8,
    /// Cost efficiency score (0-100)
    pub cost_efficiency: u8,
    /// Innovation impact score (0-100)
    pub innovation_impact: u8,
}

impl Space for PerformanceScores {
    const INIT_SPACE: usize = 6; // 6 * u8
}

// ============================================================================
// ON-CHAIN FUNCTIONS (Anchor Handlers)
// ============================================================================

/// Create performance analytics
pub fn create_performance_analytics(
    analytics: &mut PerformanceAnalytics,
    analytics_id: u64,
    entity_id: u64,
    entity_type: String,
    performance_scores: PerformanceScores,
    metadata_uri: String,
    analytics_hash: [u8; 32],
    analyzer: Pubkey,
) -> Result<()> {
    require!(entity_type.len() <= 50, IndrasError::StringTooLong);
    require!(metadata_uri.len() <= 500, IndrasError::StringTooLong);
    
    // Validate performance scores (0-100)
    let scores = [
        performance_scores.execution_speed,
        performance_scores.resource_utilization,
        performance_scores.success_rate,
        performance_scores.user_satisfaction,
        performance_scores.cost_efficiency,
        performance_scores.innovation_impact,
    ];
    
    for score in scores.iter() {
        require!(*score <= 100, IndrasError::InvalidScore);
    }
    
    // Calculate overall performance score (weighted average)
    let weights = [15u32, 15u32, 20u32, 20u32, 15u32, 15u32]; // Total = 100
    let weighted_sum: u32 = scores.iter()
        .zip(weights.iter())
        .map(|(score, weight)| (*score as u32) * (*weight as u32))
        .sum();
    
    let overall_score = (weighted_sum / 100) as u8;
    
    analytics.analytics_id = analytics_id;
    analytics.entity_id = entity_id;
    analytics.entity_type = entity_type;
    analytics.performance_scores = performance_scores;
    analytics.overall_score = overall_score;
    analytics.analyzed_at = Clock::get()?.unix_timestamp;
    analytics.analyzer = analyzer;
    analytics.metadata_uri = metadata_uri;
    analytics.analytics_hash = analytics_hash;
    
    msg!("Performance analytics {} created for entity {} (overall score: {})", 
         analytics_id, entity_id, overall_score);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_performance_analytics() {
        let mut analytics = PerformanceAnalytics {
            analytics_id: 0,
            entity_id: 0,
            entity_type: String::new(),
            performance_scores: PerformanceScores {
                execution_speed: 0,
                resource_utilization: 0,
                success_rate: 0,
                user_satisfaction: 0,
                cost_efficiency: 0,
                innovation_impact: 0,
            },
            overall_score: 0,
            analyzed_at: 0,
            analyzer: Pubkey::default(),
            metadata_uri: String::new(),
            analytics_hash: [0u8; 32],
            bump: 0,
        };

        let analyzer = Pubkey::from([1u8; 32]);
        let analytics_hash = [1u8; 32];
        let performance_scores = PerformanceScores {
            execution_speed: 80,
            resource_utilization: 75,
            success_rate: 90,
            user_satisfaction: 85,
            cost_efficiency: 70,
            innovation_impact: 88,
        };

        let result = create_performance_analytics(
            &mut analytics,
            1,
            100,
            "mesh_group".to_string(),
            performance_scores.clone(),
            "ipfs://test".to_string(),
            analytics_hash,
            analyzer,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, 1);
        assert_eq!(analytics.entity_id, 100);
        assert!(analytics.overall_score > 0);
    }
}
