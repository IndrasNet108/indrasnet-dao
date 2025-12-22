//! Quorum module
//!
//! Quorum management for governance voting
//!
//! On-chain: Quorum requirements, calculations
//! Off-chain: Quorum optimization, analytics

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Quorum calculation method
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum QuorumCalculationMethod {
    /// Fixed percentage
    FixedPercentage,
    /// Dynamic based on activity
    Dynamic,
    /// Weighted by reputation
    Weighted,
}

/// Quorum metadata (on-chain)
///
/// Stores quorum requirements and calculations
#[account]
#[derive(InitSpace)]
pub struct QuorumMetadata {
    /// Quorum ID
    pub quorum_id: u64,
    /// Proposal ID (if specific)
    pub proposal_id: Option<u64>,
    /// Required quorum percentage
    pub required_percentage: u8,
    /// Calculation method
    pub calculation_method: QuorumCalculationMethod,
    /// Current quorum percentage
    pub current_percentage: u8,
    /// Quorum reached
    pub quorum_reached: bool,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for quorum
pub mod onchain {
    use super::*;

    /// Initialize quorum metadata
    pub fn initialize_quorum_metadata(
        quorum: &mut QuorumMetadata,
        quorum_id: u64,
        proposal_id: Option<u64>,
        required_percentage: u8,
        calculation_method: QuorumCalculationMethod,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(quorum_id > 0, IndrasError::InvalidInput);
        require!(required_percentage <= 100, IndrasError::InvalidInput);
        
        quorum.quorum_id = quorum_id;
        quorum.proposal_id = proposal_id;
        quorum.required_percentage = required_percentage;
        quorum.calculation_method = calculation_method;
        quorum.current_percentage = 0;
        quorum.quorum_reached = false;
        quorum.created_at = current_time;
        quorum.updated_at = current_time;
        quorum.bump = bump;
        
        Ok(())
    }

    /// Update quorum percentage
    pub fn update_quorum_percentage(
        quorum: &mut QuorumMetadata,
        new_percentage: u8,
        current_time: i64,
    ) -> Result<()> {
        require!(new_percentage <= 100, IndrasError::InvalidInput);
        
        quorum.current_percentage = new_percentage;
        quorum.quorum_reached = new_percentage >= quorum.required_percentage;
        quorum.updated_at = current_time;
        
        Ok(())
    }
}

/// Off-chain functions for quorum
///
/// These functions should be implemented in off-chain service
/// for quorum optimization and analytics.
pub mod offchain {
    // Off-chain functions will be implemented in separate service
    
    /// Calculate optimal quorum
    pub fn calculate_optimal_quorum(_proposal_id: u64) -> u8 {
        // Implementation in off-chain service
        // Calculates optimal quorum based on activity and context
        50
    }

    /// Analyze quorum trends
    pub fn analyze_quorum_trends() -> Vec<String> {
        // Implementation in off-chain service
        // Analyzes quorum trends and returns insights
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_quorum() -> QuorumMetadata {
        QuorumMetadata {
            quorum_id: 1,
            proposal_id: Some(100),
            required_percentage: 50,
            calculation_method: QuorumCalculationMethod::FixedPercentage,
            current_percentage: 0,
            quorum_reached: false,
            created_at: 1000,
            updated_at: 1000,
            bump: 255,
        }
    }

    #[test]
    fn test_quorum_calculation_method_variants() {
        assert_eq!(QuorumCalculationMethod::FixedPercentage, QuorumCalculationMethod::FixedPercentage);
        assert_eq!(QuorumCalculationMethod::Dynamic, QuorumCalculationMethod::Dynamic);
        assert_eq!(QuorumCalculationMethod::Weighted, QuorumCalculationMethod::Weighted);
    }

    #[test]
    fn test_quorum_metadata_structure() {
        let quorum = create_test_quorum();
        assert_eq!(quorum.quorum_id, 1);
        assert_eq!(quorum.proposal_id, Some(100));
        assert_eq!(quorum.required_percentage, 50);
        assert_eq!(quorum.calculation_method, QuorumCalculationMethod::FixedPercentage);
        assert_eq!(quorum.current_percentage, 0);
        assert_eq!(quorum.quorum_reached, false);
        assert_eq!(quorum.created_at, 1000);
        assert_eq!(quorum.updated_at, 1000);
        assert_eq!(quorum.bump, 255);
    }

    #[test]
    fn test_initialize_quorum_metadata() {
        let mut quorum = QuorumMetadata {
            quorum_id: 0,
            proposal_id: None,
            required_percentage: 0,
            calculation_method: QuorumCalculationMethod::FixedPercentage,
            current_percentage: 0,
            quorum_reached: false,
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };

        let result = onchain::initialize_quorum_metadata(
            &mut quorum,
            200,
            Some(300),
            75,
            QuorumCalculationMethod::Dynamic,
            7000,
            64,
        );

        assert!(result.is_ok());
        assert_eq!(quorum.quorum_id, 200);
        assert_eq!(quorum.proposal_id, Some(300));
        assert_eq!(quorum.required_percentage, 75);
        assert_eq!(quorum.calculation_method, QuorumCalculationMethod::Dynamic);
        assert_eq!(quorum.current_percentage, 0);
        assert_eq!(quorum.quorum_reached, false);
        assert_eq!(quorum.created_at, 7000);
        assert_eq!(quorum.updated_at, 7000);
        assert_eq!(quorum.bump, 64);
    }

    #[test]
    fn test_initialize_quorum_metadata_invalid_id() {
        let mut quorum = create_test_quorum();

        let result = onchain::initialize_quorum_metadata(
            &mut quorum,
            0, // Invalid: quorum_id must be > 0
            Some(300),
            75,
            QuorumCalculationMethod::FixedPercentage,
            7000,
            64,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_quorum_metadata_invalid_percentage() {
        let mut quorum = create_test_quorum();

        let result = onchain::initialize_quorum_metadata(
            &mut quorum,
            1,
            Some(300),
            101, // Invalid: required_percentage must be <= 100
            QuorumCalculationMethod::FixedPercentage,
            7000,
            64,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_quorum_metadata_without_proposal_id() {
        let mut quorum = create_test_quorum();

        let result = onchain::initialize_quorum_metadata(
            &mut quorum,
            1,
            None, // No specific proposal
            60,
            QuorumCalculationMethod::Weighted,
            5000,
            128,
        );

        assert!(result.is_ok());
        assert_eq!(quorum.proposal_id, None);
    }

    #[test]
    fn test_update_quorum_percentage() {
        let mut quorum = create_test_quorum();
        quorum.required_percentage = 50;

        let result = onchain::update_quorum_percentage(
            &mut quorum,
            60,
            8000,
        );

        assert!(result.is_ok());
        assert_eq!(quorum.current_percentage, 60);
        assert_eq!(quorum.quorum_reached, true); // 60 >= 50
        assert_eq!(quorum.updated_at, 8000);
    }

    #[test]
    fn test_update_quorum_percentage_not_reached() {
        let mut quorum = create_test_quorum();
        quorum.required_percentage = 50;

        let result = onchain::update_quorum_percentage(
            &mut quorum,
            30,
            9000,
        );

        assert!(result.is_ok());
        assert_eq!(quorum.current_percentage, 30);
        assert_eq!(quorum.quorum_reached, false); // 30 < 50
        assert_eq!(quorum.updated_at, 9000);
    }

    #[test]
    fn test_update_quorum_percentage_exact_threshold() {
        let mut quorum = create_test_quorum();
        quorum.required_percentage = 50;

        let result = onchain::update_quorum_percentage(
            &mut quorum,
            50,
            10000,
        );

        assert!(result.is_ok());
        assert_eq!(quorum.current_percentage, 50);
        assert_eq!(quorum.quorum_reached, true); // 50 >= 50
    }

    #[test]
    fn test_update_quorum_percentage_invalid() {
        let mut quorum = create_test_quorum();

        let result = onchain::update_quorum_percentage(
            &mut quorum,
            101, // Invalid: new_percentage must be <= 100
            10000,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_quorum_all_calculation_methods() {
        let methods = vec![
            QuorumCalculationMethod::FixedPercentage,
            QuorumCalculationMethod::Dynamic,
            QuorumCalculationMethod::Weighted,
        ];

        for method in methods {
            let mut quorum = create_test_quorum();
            let result = onchain::initialize_quorum_metadata(
                &mut quorum,
                1,
                None,
                50,
                method,
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(quorum.calculation_method, method);
        }
    }

    #[test]
    fn test_quorum_percentage_boundaries() {
        let mut quorum = create_test_quorum();
        quorum.required_percentage = 50;

        // Test 0%
        assert!(onchain::update_quorum_percentage(&mut quorum, 0, 1000).is_ok());
        assert_eq!(quorum.current_percentage, 0);
        assert_eq!(quorum.quorum_reached, false);

        // Test 100%
        assert!(onchain::update_quorum_percentage(&mut quorum, 100, 2000).is_ok());
        assert_eq!(quorum.current_percentage, 100);
        assert_eq!(quorum.quorum_reached, true);
    }

    #[test]
    fn test_quorum_calculation_method_all_variants_unique() {
        let methods = vec![
            QuorumCalculationMethod::FixedPercentage,
            QuorumCalculationMethod::Dynamic,
            QuorumCalculationMethod::Weighted,
        ];
        
        for i in 0..methods.len() {
            for j in (i + 1)..methods.len() {
                assert_ne!(methods[i], methods[j], "Duplicate method found");
            }
        }
    }

    #[test]
    fn test_quorum_calculation_method_equality() {
        assert_eq!(QuorumCalculationMethod::FixedPercentage, QuorumCalculationMethod::FixedPercentage);
        assert_ne!(QuorumCalculationMethod::FixedPercentage, QuorumCalculationMethod::Dynamic);
        assert_eq!(QuorumCalculationMethod::Dynamic, QuorumCalculationMethod::Dynamic);
        assert_ne!(QuorumCalculationMethod::Dynamic, QuorumCalculationMethod::Weighted);
        assert_eq!(QuorumCalculationMethod::Weighted, QuorumCalculationMethod::Weighted);
    }

    #[test]
    fn test_quorum_calculation_method_copy() {
        let method1 = QuorumCalculationMethod::FixedPercentage;
        let method2 = method1; // Copy trait
        assert_eq!(method1, method2);
    }

    #[test]
    fn test_quorum_calculation_method_space() {
        assert_eq!(<QuorumCalculationMethod as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_quorum_metadata_all_percentages() {
        let percentages = vec![0u8, 1u8, 50u8, 99u8, 100u8];
        
        for percentage in percentages {
            let mut quorum = create_test_quorum();
            let result = onchain::initialize_quorum_metadata(
                &mut quorum,
                1,
                None,
                percentage,
                QuorumCalculationMethod::FixedPercentage,
                1000,
                255,
            );
            
            assert!(result.is_ok());
            assert_eq!(quorum.required_percentage, percentage);
        }
    }

    #[test]
    fn test_initialize_quorum_metadata_large_quorum_id() {
        let mut quorum = create_test_quorum();
        
        let result = onchain::initialize_quorum_metadata(
            &mut quorum,
            u64::MAX,
            None,
            50,
            QuorumCalculationMethod::FixedPercentage,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(quorum.quorum_id, u64::MAX);
    }

    #[test]
    fn test_update_quorum_percentage_preserves_other_fields() {
        let mut quorum = create_test_quorum();
        quorum.quorum_id = 999;
        quorum.proposal_id = Some(888);
        quorum.required_percentage = 60;
        quorum.calculation_method = QuorumCalculationMethod::Dynamic;
        quorum.created_at = 5000;
        
        let result = onchain::update_quorum_percentage(&mut quorum, 70, 6000);
        
        assert!(result.is_ok());
        // Other fields should be preserved
        assert_eq!(quorum.quorum_id, 999);
        assert_eq!(quorum.proposal_id, Some(888));
        assert_eq!(quorum.required_percentage, 60);
        assert_eq!(quorum.calculation_method, QuorumCalculationMethod::Dynamic);
        assert_eq!(quorum.created_at, 5000);
        // Updated fields
        assert_eq!(quorum.current_percentage, 70);
        assert_eq!(quorum.quorum_reached, true);
        assert_eq!(quorum.updated_at, 6000);
    }

    #[test]
    fn test_update_quorum_percentage_from_below_to_above() {
        let mut quorum = create_test_quorum();
        quorum.required_percentage = 50;
        
        // Start below threshold
        onchain::update_quorum_percentage(&mut quorum, 30, 1000).unwrap();
        assert_eq!(quorum.quorum_reached, false);
        
        // Update to above threshold
        onchain::update_quorum_percentage(&mut quorum, 60, 2000).unwrap();
        assert_eq!(quorum.quorum_reached, true);
    }

    #[test]
    fn test_update_quorum_percentage_from_above_to_below() {
        let mut quorum = create_test_quorum();
        quorum.required_percentage = 50;
        
        // Start above threshold
        onchain::update_quorum_percentage(&mut quorum, 60, 1000).unwrap();
        assert_eq!(quorum.quorum_reached, true);
        
        // Update to below threshold (shouldn't happen in practice, but test logic)
        onchain::update_quorum_percentage(&mut quorum, 30, 2000).unwrap();
        assert_eq!(quorum.quorum_reached, false);
    }

    #[test]
    fn test_quorum_metadata_all_fields() {
        let quorum = QuorumMetadata {
            quorum_id: 123,
            proposal_id: Some(456),
            required_percentage: 75,
            calculation_method: QuorumCalculationMethod::Weighted,
            current_percentage: 80,
            quorum_reached: true,
            created_at: 1000,
            updated_at: 2000,
            bump: 128,
        };
        
        assert_eq!(quorum.quorum_id, 123);
        assert_eq!(quorum.proposal_id, Some(456));
        assert_eq!(quorum.required_percentage, 75);
        assert_eq!(quorum.calculation_method, QuorumCalculationMethod::Weighted);
        assert_eq!(quorum.current_percentage, 80);
        assert_eq!(quorum.quorum_reached, true);
        assert_eq!(quorum.created_at, 1000);
        assert_eq!(quorum.updated_at, 2000);
        assert_eq!(quorum.bump, 128);
    }

    #[test]
    fn test_initialize_quorum_metadata_always_starts_at_zero() {
        let mut quorum = QuorumMetadata {
            quorum_id: 0,
            proposal_id: None,
            required_percentage: 0,
            calculation_method: QuorumCalculationMethod::FixedPercentage,
            current_percentage: 100, // Will be reset
            quorum_reached: true,    // Will be reset
            created_at: 0,
            updated_at: 0,
            bump: 0,
        };
        
        let result = onchain::initialize_quorum_metadata(
            &mut quorum,
            1,
            None,
            50,
            QuorumCalculationMethod::FixedPercentage,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(quorum.current_percentage, 0);
        assert_eq!(quorum.quorum_reached, false);
    }

    #[test]
    fn test_update_quorum_percentage_timestamp_update() {
        let mut quorum = create_test_quorum();
        quorum.updated_at = 1000;
        
        let result = onchain::update_quorum_percentage(&mut quorum, 50, 5000);
        
        assert!(result.is_ok());
        assert_eq!(quorum.updated_at, 5000);
    }

    #[test]
    fn test_offchain_calculate_optimal_quorum() {
        // Test that offchain function exists and returns default value
        let result = offchain::calculate_optimal_quorum(1);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_offchain_calculate_optimal_quorum_different_ids() {
        // Test with different IDs
        let result1 = offchain::calculate_optimal_quorum(1);
        let result2 = offchain::calculate_optimal_quorum(999);
        assert_eq!(result1, 50);
        assert_eq!(result2, 50);
    }

    #[test]
    fn test_offchain_analyze_quorum_trends() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::analyze_quorum_trends();
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_update_quorum_percentage_below_threshold_comprehensive() {
        let mut quorum = QuorumMetadata {
            quorum_id: 1,
            proposal_id: None,
            required_percentage: 50,
            calculation_method: QuorumCalculationMethod::FixedPercentage,
            current_percentage: 0,
            quorum_reached: false,
            created_at: 1000,
            updated_at: 1000,
            bump: 255,
        };
        
        // 49% - should not reach quorum
        assert!(onchain::update_quorum_percentage(&mut quorum, 49, 2000).is_ok());
        assert_eq!(quorum.current_percentage, 49);
        assert_eq!(quorum.quorum_reached, false);
    }

    #[test]
    fn test_update_quorum_percentage_invalid_over_100_comprehensive() {
        let mut quorum = QuorumMetadata {
            quorum_id: 1,
            proposal_id: None,
            required_percentage: 50,
            calculation_method: QuorumCalculationMethod::FixedPercentage,
            current_percentage: 0,
            quorum_reached: false,
            created_at: 1000,
            updated_at: 1000,
            bump: 255,
        };
        
        // 101% - should fail
        assert!(onchain::update_quorum_percentage(&mut quorum, 101, 2000).is_err());
    }
}
