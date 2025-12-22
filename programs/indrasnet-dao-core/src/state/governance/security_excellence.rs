//! Security Excellence module
//!
//! Security excellence metrics and tracking
//!
//! On-chain: Metadata for security excellence metrics
//! Off-chain: Actual metrics calculation, analytics

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Security excellence score metadata (on-chain)
///
/// Stores security excellence metrics
#[account]
#[derive(InitSpace)]
pub struct SecurityExcellenceMetadata {
    /// Score ID
    pub score_id: u64,
    /// Overall score (0-100)
    pub overall_score: u8,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Score data hash
    pub score_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for security excellence
pub mod onchain {
    use super::*;

    /// Initialize security excellence score
    pub fn initialize_score(
        score: &mut SecurityExcellenceMetadata,
        score_id: u64,
        overall_score: u8,
        score_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(score_id > 0, IndrasError::InvalidInput);
        require!(overall_score <= 100, IndrasError::InvalidInput);
        
        score.score_id = score_id;
        score.overall_score = overall_score;
        score.created_at = current_time;
        score.updated_at = current_time;
        score.score_data_hash = score_data_hash;
        score.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for security excellence
pub mod offchain {
    /// Calculate security excellence score
    pub fn calculate_score() -> u8 {
        // Implementation in off-chain service
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_score() -> SecurityExcellenceMetadata {
        SecurityExcellenceMetadata {
            score_id: 1,
            overall_score: 75,
            created_at: 1000,
            updated_at: 1000,
            score_data_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_security_excellence_metadata_structure() {
        let score = create_test_score();
        assert_eq!(score.score_id, 1);
        assert_eq!(score.overall_score, 75);
        assert_eq!(score.created_at, 1000);
        assert_eq!(score.updated_at, 1000);
        assert_eq!(score.bump, 255);
    }

    #[test]
    fn test_initialize_score() {
        let mut score = SecurityExcellenceMetadata {
            score_id: 0,
            overall_score: 0,
            created_at: 0,
            updated_at: 0,
            score_data_hash: [0u8; 32],
            bump: 0,
        };

        let data_hash = [5u8; 32];
        let result = onchain::initialize_score(
            &mut score,
            600,
            85,
            data_hash,
            11000,
            96,
        );

        assert!(result.is_ok());
        assert_eq!(score.score_id, 600);
        assert_eq!(score.overall_score, 85);
        assert_eq!(score.created_at, 11000);
        assert_eq!(score.updated_at, 11000);
        assert_eq!(score.score_data_hash, data_hash);
        assert_eq!(score.bump, 96);
    }

    #[test]
    fn test_initialize_score_invalid_id() {
        let mut score = create_test_score();

        let result = onchain::initialize_score(
            &mut score,
            0, // Invalid: score_id must be > 0
            85,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_score_invalid_percentage() {
        let mut score = create_test_score();

        let result = onchain::initialize_score(
            &mut score,
            1,
            101, // Invalid: overall_score must be <= 100
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_score_boundaries() {
        let mut score = create_test_score();

        // Test 0
        assert!(onchain::initialize_score(&mut score, 1, 0, [0u8; 32], 1000, 255).is_ok());
        assert_eq!(score.overall_score, 0);

        // Test 100
        assert!(onchain::initialize_score(&mut score, 1, 100, [0u8; 32], 2000, 255).is_ok());
        assert_eq!(score.overall_score, 100);
    }

    #[test]
    fn test_initialize_score_data_hash() {
        let mut score = create_test_score();
        let custom_hash = [99u8; 32];

        let result = onchain::initialize_score(
            &mut score,
            1,
            90,
            custom_hash,
            3000,
            150,
        );

        assert!(result.is_ok());
        assert_eq!(score.score_data_hash, custom_hash);
    }

    #[test]
    fn test_initialize_score_timestamps() {
        let mut score = create_test_score();

        let result = onchain::initialize_score(
            &mut score,
            1,
            50,
            [0u8; 32],
            54321,
            180,
        );

        assert!(result.is_ok());
        // Both created_at and updated_at should be set to current_time
        assert_eq!(score.created_at, 54321);
        assert_eq!(score.updated_at, 54321);
    }

    #[test]
    fn test_initialize_score_various_scores() {
        let scores = vec![0u8, 25u8, 50u8, 75u8, 100u8];

        for overall_score in scores {
            let mut score = create_test_score();
            let result = onchain::initialize_score(
                &mut score,
                1,
                overall_score,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(score.overall_score, overall_score);
        }
    }

    #[test]
    fn test_initialize_score_bump_seed() {
        let mut score = create_test_score();

        for bump in [0u8, 128u8, 255u8] {
            let result = onchain::initialize_score(
                &mut score,
                1,
                80,
                [0u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(score.bump, bump);
        }
    }

    #[test]
    fn test_initialize_score_all_percentages() {
        let percentages = vec![0u8, 1u8, 50u8, 99u8, 100u8];
        
        for percentage in percentages {
            let mut score = create_test_score();
            let result = onchain::initialize_score(
                &mut score,
                1,
                percentage,
                [0u8; 32],
                1000,
                255,
            );
            
            assert!(result.is_ok());
            assert_eq!(score.overall_score, percentage);
        }
    }

    #[test]
    fn test_initialize_score_large_score_id() {
        let mut score = create_test_score();
        
        let result = onchain::initialize_score(
            &mut score,
            u64::MAX,
            75,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(score.score_id, u64::MAX);
    }

    #[test]
    fn test_initialize_score_custom_hash() {
        let mut score = create_test_score();
        let custom_hash = [255u8; 32];
        
        let result = onchain::initialize_score(
            &mut score,
            1,
            80,
            custom_hash,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(score.score_data_hash, custom_hash);
    }

    #[test]
    fn test_initialize_score_different_timestamps() {
        let mut score = create_test_score();
        
        let result = onchain::initialize_score(
            &mut score,
            1,
            75,
            [0u8; 32],
            7777,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(score.created_at, 7777);
        assert_eq!(score.updated_at, 7777);
    }

    #[test]
    fn test_initialize_score_different_bump_seeds() {
        let mut score = create_test_score();
        
        for bump in [0u8, 128u8, 255u8] {
            let result = onchain::initialize_score(
                &mut score,
                1,
                75,
                [0u8; 32],
                1000,
                bump,
            );
            
            assert!(result.is_ok());
            assert_eq!(score.bump, bump);
        }
    }

    #[test]
    fn test_initialize_score_preserves_other_fields() {
        let mut score = SecurityExcellenceMetadata {
            score_id: 999,
            overall_score: 50,
            created_at: 1000,
            updated_at: 2000,
            score_data_hash: [1u8; 32],
            bump: 50,
        };
        
        let new_hash = [2u8; 32];
        let result = onchain::initialize_score(
            &mut score,
            1,
            90,
            new_hash,
            3000,
            100,
        );
        
        assert!(result.is_ok());
        // All fields should be updated
        assert_eq!(score.score_id, 1);
        assert_eq!(score.overall_score, 90);
        assert_eq!(score.created_at, 3000);
        assert_eq!(score.updated_at, 3000);
        assert_eq!(score.score_data_hash, new_hash);
        assert_eq!(score.bump, 100);
    }

    #[test]
    fn test_security_excellence_metadata_all_fields() {
        let score = SecurityExcellenceMetadata {
            score_id: 123,
            overall_score: 85,
            created_at: 5000,
            updated_at: 6000,
            score_data_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(score.score_id, 123);
        assert_eq!(score.overall_score, 85);
        assert_eq!(score.created_at, 5000);
        assert_eq!(score.updated_at, 6000);
        assert_eq!(score.score_data_hash, [42u8; 32]);
        assert_eq!(score.bump, 128);
    }

    #[test]
    fn test_initialize_score_timestamps_same() {
        let mut score = create_test_score();
        
        let result = onchain::initialize_score(
            &mut score,
            1,
            75,
            [0u8; 32],
            8888,
            255,
        );
        
        assert!(result.is_ok());
        // Both timestamps should be the same
        assert_eq!(score.created_at, score.updated_at);
        assert_eq!(score.created_at, 8888);
    }
}
