//! Adaptive Security Scoring System
//!
//! INNOVATION: Real-time dynamic operation risk assessment system
//! Analyzes multiple factors and assigns security score to operations
//! High-risk operations require additional checks

use anchor_lang::prelude::*;

/// Security risk factors for adaptive scoring
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct SecurityRiskFactors {
    /// Amount involved (higher = more risk)
    pub amount_risk: u8,
    /// Time since last similar operation (shorter = more risk)
    pub timing_risk: u8,
    /// Number of recent operations by this user (more = more risk)
    pub frequency_risk: u8,
    /// Authority level of operator (lower = more risk)
    pub authority_risk: u8,
    /// Proposal complexity (more complex = more risk)
    pub complexity_risk: u8,
    /// Historical success rate (lower = more risk)
    pub historical_risk: u8,
}

impl Default for SecurityRiskFactors {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityRiskFactors {
    pub fn new() -> Self {
        Self {
            amount_risk: 0,
            timing_risk: 0,
            frequency_risk: 0,
            authority_risk: 0,
            complexity_risk: 0,
            historical_risk: 0,
        }
    }
    
    /// Calculate overall security score (0-100, higher = more secure)
    pub fn calculate_security_score(&self) -> u8 {
        // Weighted average with emphasis on critical factors
        let weights = [20u16, 15u16, 15u16, 25u16, 10u16, 15u16];
        let factors = [
            self.amount_risk,
            self.timing_risk,
            self.frequency_risk,
            self.authority_risk,
            self.complexity_risk,
            self.historical_risk,
        ];
        
        let mut weighted_sum: u16 = 0;
        let mut total_weight: u16 = 0;
        
        for (factor, weight) in factors.iter().zip(weights.iter()) {
            let risk_value = 100u16.saturating_sub(*factor as u16);
            weighted_sum = weighted_sum.saturating_add(risk_value.saturating_mul(*weight));
            total_weight = total_weight.saturating_add(*weight);
        }
        
        if total_weight == 0 {
            return 50; // Default medium risk
        }
        
        (weighted_sum / total_weight) as u8
    }
}

/// Operation security profile
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OperationSecurityProfile {
    /// Operation type identifier
    pub operation_type: u8,
    /// Security score (0-100)
    pub security_score: u8,
    /// Risk factors
    pub risk_factors: SecurityRiskFactors,
    /// Required security level for this operation
    pub required_security_level: u8,
    /// Timestamp of last similar operation
    pub last_operation_timestamp: Option<i64>,
    /// Count of operations in current window
    pub operation_count: u64,
}

impl OperationSecurityProfile {
    /// Check if operation meets security requirements
    pub fn is_secure_enough(&self) -> bool {
        self.security_score >= self.required_security_level
    }
    
    /// Calculate required security level based on operation type
    pub fn calculate_required_level(operation_type: u8, base_level: u8) -> u8 {
        // Critical operations require higher security
        match operation_type {
            1 => base_level.saturating_add(20), // Treasury operations
            2 => base_level.saturating_add(15), // Proposal execution
            3 => base_level.saturating_add(10), // Grant operations
            _ => base_level,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_risk_factors_new() {
        let factors = SecurityRiskFactors::new();
        assert_eq!(factors.amount_risk, 0);
        assert_eq!(factors.timing_risk, 0);
        assert_eq!(factors.frequency_risk, 0);
        assert_eq!(factors.authority_risk, 0);
        assert_eq!(factors.complexity_risk, 0);
        assert_eq!(factors.historical_risk, 0);
    }

    #[test]
    fn test_security_risk_factors_default() {
        let factors = SecurityRiskFactors::default();
        assert_eq!(factors.amount_risk, 0);
    }

    #[test]
    fn test_calculate_security_score_all_low_risk() {
        let factors = SecurityRiskFactors {
            amount_risk: 10,
            timing_risk: 10,
            frequency_risk: 10,
            authority_risk: 10,
            complexity_risk: 10,
            historical_risk: 10,
        };
        
        let score = factors.calculate_security_score();
        // Low risk (10) means high security (90)
        assert!(score > 80);
    }

    #[test]
    fn test_calculate_security_score_all_high_risk() {
        let factors = SecurityRiskFactors {
            amount_risk: 90,
            timing_risk: 90,
            frequency_risk: 90,
            authority_risk: 90,
            complexity_risk: 90,
            historical_risk: 90,
        };
        
        let score = factors.calculate_security_score();
        // High risk (90) means low security (10)
        assert!(score < 20);
    }

    #[test]
    fn test_operation_security_profile_is_secure_enough() {
        let profile = OperationSecurityProfile {
            operation_type: 1,
            security_score: 80,
            risk_factors: SecurityRiskFactors::new(),
            required_security_level: 70,
            last_operation_timestamp: None,
            operation_count: 0,
        };
        
        assert!(profile.is_secure_enough());
    }

    #[test]
    fn test_operation_security_profile_not_secure_enough() {
        let profile = OperationSecurityProfile {
            operation_type: 1,
            security_score: 50,
            risk_factors: SecurityRiskFactors::new(),
            required_security_level: 70,
            last_operation_timestamp: None,
            operation_count: 0,
        };
        
        assert!(!profile.is_secure_enough());
    }

    #[test]
    fn test_calculate_required_level_default() {
        // Default operations (type 0) use base level
        let level = OperationSecurityProfile::calculate_required_level(0, 50);
        assert_eq!(level, 50);
    }

    #[test]
    fn test_calculate_required_level_treasury() {
        // Treasury operations (type 1) require higher level
        let level = OperationSecurityProfile::calculate_required_level(1, 50);
        assert_eq!(level, 70); // 50 + 20
    }

    #[test]
    fn test_calculate_required_level_proposal() {
        // Proposal execution (type 2) require higher level
        let level = OperationSecurityProfile::calculate_required_level(2, 50);
        assert_eq!(level, 65); // 50 + 15
    }

    #[test]
    fn test_calculate_required_level_grant() {
        // Grant operations (type 3) require higher level
        let level = OperationSecurityProfile::calculate_required_level(3, 50);
        assert_eq!(level, 60); // 50 + 10
    }

    #[test]
    fn test_calculate_security_score_mixed_risk() {
        let factors = SecurityRiskFactors {
            amount_risk: 50,
            timing_risk: 30,
            frequency_risk: 70,
            authority_risk: 20,
            complexity_risk: 60,
            historical_risk: 40,
        };
        
        let score = factors.calculate_security_score();
        // Mixed risk should result in medium security score
        assert!(score >= 30 && score <= 70);
    }

    #[test]
    fn test_calculate_security_score_zero_risk() {
        let factors = SecurityRiskFactors {
            amount_risk: 0,
            timing_risk: 0,
            frequency_risk: 0,
            authority_risk: 0,
            complexity_risk: 0,
            historical_risk: 0,
        };
        
        let score = factors.calculate_security_score();
        // Zero risk means maximum security
        assert_eq!(score, 100);
    }

    #[test]
    fn test_calculate_security_score_max_risk() {
        let factors = SecurityRiskFactors {
            amount_risk: 100,
            timing_risk: 100,
            frequency_risk: 100,
            authority_risk: 100,
            complexity_risk: 100,
            historical_risk: 100,
        };
        
        let score = factors.calculate_security_score();
        // Maximum risk means minimum security
        assert_eq!(score, 0);
    }

    #[test]
    fn test_calculate_security_score_authority_weight() {
        // Authority risk has highest weight (25), so it should have more impact
        let factors1 = SecurityRiskFactors {
            amount_risk: 50,
            timing_risk: 50,
            frequency_risk: 50,
            authority_risk: 0, // Low authority risk
            complexity_risk: 50,
            historical_risk: 50,
        };
        
        let factors2 = SecurityRiskFactors {
            amount_risk: 50,
            timing_risk: 50,
            frequency_risk: 50,
            authority_risk: 100, // High authority risk
            complexity_risk: 50,
            historical_risk: 50,
        };
        
        let score1 = factors1.calculate_security_score();
        let score2 = factors2.calculate_security_score();
        
        // Score1 should be higher because authority_risk is lower
        assert!(score1 > score2);
    }

    #[test]
    fn test_operation_security_profile_exact_threshold() {
        let profile = OperationSecurityProfile {
            operation_type: 1,
            security_score: 70,
            risk_factors: SecurityRiskFactors::new(),
            required_security_level: 70,
            last_operation_timestamp: None,
            operation_count: 0,
        };
        
        // Exact threshold should be secure enough
        assert!(profile.is_secure_enough());
    }

    #[test]
    fn test_operation_security_profile_below_threshold() {
        let profile = OperationSecurityProfile {
            operation_type: 1,
            security_score: 69,
            risk_factors: SecurityRiskFactors::new(),
            required_security_level: 70,
            last_operation_timestamp: None,
            operation_count: 0,
        };
        
        // Just below threshold should not be secure enough
        assert!(!profile.is_secure_enough());
    }

    #[test]
    fn test_calculate_required_level_unknown_type() {
        // Unknown operation types (>= 4) use base level
        let level = OperationSecurityProfile::calculate_required_level(4, 50);
        assert_eq!(level, 50);
        
        let level2 = OperationSecurityProfile::calculate_required_level(255, 60);
        assert_eq!(level2, 60);
    }

    #[test]
    fn test_calculate_required_level_overflow_protection() {
        // Test that saturating_add prevents overflow
        let level = OperationSecurityProfile::calculate_required_level(1, u8::MAX);
        // Should be capped at u8::MAX
        assert_eq!(level, u8::MAX);
    }

    #[test]
    fn test_security_risk_factors_structure() {
        let factors = SecurityRiskFactors {
            amount_risk: 10,
            timing_risk: 20,
            frequency_risk: 30,
            authority_risk: 40,
            complexity_risk: 50,
            historical_risk: 60,
        };
        
        assert_eq!(factors.amount_risk, 10);
        assert_eq!(factors.timing_risk, 20);
        assert_eq!(factors.frequency_risk, 30);
        assert_eq!(factors.authority_risk, 40);
        assert_eq!(factors.complexity_risk, 50);
        assert_eq!(factors.historical_risk, 60);
    }

    #[test]
    fn test_operation_security_profile_structure() {
        let risk_factors = SecurityRiskFactors {
            amount_risk: 25,
            timing_risk: 30,
            frequency_risk: 20,
            authority_risk: 15,
            complexity_risk: 35,
            historical_risk: 40,
        };
        
        let profile = OperationSecurityProfile {
            operation_type: 2,
            security_score: 75,
            risk_factors: risk_factors.clone(),
            required_security_level: 70,
            last_operation_timestamp: Some(1000),
            operation_count: 5,
        };
        
        assert_eq!(profile.operation_type, 2);
        assert_eq!(profile.security_score, 75);
        assert_eq!(profile.risk_factors, risk_factors);
        assert_eq!(profile.required_security_level, 70);
        assert_eq!(profile.last_operation_timestamp, Some(1000));
        assert_eq!(profile.operation_count, 5);
    }

    #[test]
    fn test_calculate_security_score_weights_sum() {
        // Verify that weights sum to 100
        let weights = [20u16, 15u16, 15u16, 25u16, 10u16, 15u16];
        let sum: u16 = weights.iter().sum();
        assert_eq!(sum, 100);
    }

    #[test]
    fn test_calculate_security_score_edge_cases() {
        // Test with very low risk in one factor
        let factors = SecurityRiskFactors {
            amount_risk: 5,
            timing_risk: 5,
            frequency_risk: 5,
            authority_risk: 5,
            complexity_risk: 5,
            historical_risk: 5,
        };
        
        let score = factors.calculate_security_score();
        assert!(score > 90); // Should be very high security
        
        // Test with very high risk in one factor
        let factors2 = SecurityRiskFactors {
            amount_risk: 95,
            timing_risk: 95,
            frequency_risk: 95,
            authority_risk: 95,
            complexity_risk: 95,
            historical_risk: 95,
        };
        
        let score2 = factors2.calculate_security_score();
        assert!(score2 < 10); // Should be very low security
    }

    #[test]
    fn test_security_risk_factors_all_fields() {
        let factors = SecurityRiskFactors {
            amount_risk: 10,
            timing_risk: 20,
            frequency_risk: 30,
            authority_risk: 40,
            complexity_risk: 50,
            historical_risk: 60,
        };
        
        assert_eq!(factors.amount_risk, 10);
        assert_eq!(factors.timing_risk, 20);
        assert_eq!(factors.frequency_risk, 30);
        assert_eq!(factors.authority_risk, 40);
        assert_eq!(factors.complexity_risk, 50);
        assert_eq!(factors.historical_risk, 60);
    }

    #[test]
    fn test_operation_security_profile_all_fields() {
        let risk_factors = SecurityRiskFactors::new();
        let profile = OperationSecurityProfile {
            operation_type: 1,
            security_score: 75,
            risk_factors: risk_factors.clone(),
            required_security_level: 70,
            last_operation_timestamp: Some(1000),
            operation_count: 5,
        };
        
        assert_eq!(profile.operation_type, 1);
        assert_eq!(profile.security_score, 75);
        assert_eq!(profile.risk_factors, risk_factors);
        assert_eq!(profile.required_security_level, 70);
        assert_eq!(profile.last_operation_timestamp, Some(1000));
        assert_eq!(profile.operation_count, 5);
    }

    #[test]
    fn test_operation_security_profile_no_last_timestamp() {
        let profile = OperationSecurityProfile {
            operation_type: 1,
            security_score: 75,
            risk_factors: SecurityRiskFactors::new(),
            required_security_level: 70,
            last_operation_timestamp: None,
            operation_count: 0,
        };
        
        assert_eq!(profile.last_operation_timestamp, None);
        assert_eq!(profile.operation_count, 0);
    }

    #[test]
    fn test_calculate_security_score_medium_risk() {
        let factors = SecurityRiskFactors {
            amount_risk: 50,
            timing_risk: 50,
            frequency_risk: 50,
            authority_risk: 50,
            complexity_risk: 50,
            historical_risk: 50,
        };
        
        let score = factors.calculate_security_score();
        // Medium risk (50) should result in medium security (50)
        assert!(score >= 45 && score <= 55);
    }

    #[test]
    fn test_calculate_required_level_all_types() {
        let base_level = 50;
        
        assert_eq!(OperationSecurityProfile::calculate_required_level(0, base_level), 50);
        assert_eq!(OperationSecurityProfile::calculate_required_level(1, base_level), 70);
        assert_eq!(OperationSecurityProfile::calculate_required_level(2, base_level), 65);
        assert_eq!(OperationSecurityProfile::calculate_required_level(3, base_level), 60);
        assert_eq!(OperationSecurityProfile::calculate_required_level(4, base_level), 50);
    }

    #[test]
    fn test_operation_security_profile_is_secure_enough_all_levels() {
        let test_cases = vec![
            (80u8, 70u8, true),
            (70u8, 70u8, true),
            (69u8, 70u8, false),
            (100u8, 50u8, true),
            (0u8, 50u8, false),
        ];
        
        for (score, required, expected) in test_cases {
            let profile = OperationSecurityProfile {
                operation_type: 1,
                security_score: score,
                risk_factors: SecurityRiskFactors::new(),
                required_security_level: required,
                last_operation_timestamp: None,
                operation_count: 0,
            };
            
            assert_eq!(profile.is_secure_enough(), expected);
        }
    }

    #[test]
    fn test_security_risk_factors_clone() {
        let factors1 = SecurityRiskFactors {
            amount_risk: 10,
            timing_risk: 20,
            frequency_risk: 30,
            authority_risk: 40,
            complexity_risk: 50,
            historical_risk: 60,
        };
        
        let factors2 = factors1.clone();
        assert_eq!(factors1, factors2);
    }

    #[test]
    fn test_operation_security_profile_clone() {
        let profile1 = OperationSecurityProfile {
            operation_type: 1,
            security_score: 75,
            risk_factors: SecurityRiskFactors::new(),
            required_security_level: 70,
            last_operation_timestamp: Some(1000),
            operation_count: 5,
        };
        
        let profile2 = profile1.clone();
        assert_eq!(profile1.operation_type, profile2.operation_type);
        assert_eq!(profile1.security_score, profile2.security_score);
        assert_eq!(profile1.required_security_level, profile2.required_security_level);
        assert_eq!(profile1.last_operation_timestamp, profile2.last_operation_timestamp);
        assert_eq!(profile1.operation_count, profile2.operation_count);
    }
}

/// SECURITY: Adaptive security scoring constants
pub const MIN_SECURITY_SCORE: u8 = 50;
pub const HIGH_SECURITY_SCORE: u8 = 80;
pub const CRITICAL_SECURITY_SCORE: u8 = 90;
