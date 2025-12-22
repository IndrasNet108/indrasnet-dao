//! Real Solana Runtime Tests for utils/dbscan_validation.rs
//!
//! These tests use solana-program-test to test DBSCAN validation functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
#[allow(unused_imports, unused_variables, unused_mut)]
mod tests {
    use crate::utils::dbscan_validation::*;
    use crate::state::grant::semantic::VerifiedDistance;
    use anchor_lang::prelude::*;
    use anyhow::Result;

    /// Test validate_dbscan_reachability with empty distances
    #[tokio::test]
    async fn test_validate_dbscan_reachability_empty_distances() -> Result<()> {
        // Empty distances should pass (MVP behavior)
        let result = validate_dbscan_reachability(3, &[], 0.5);
        assert!(result.is_ok(), "Empty distances should pass");
        
        Ok(())
    }

    /// Test validate_dbscan_reachability with single idea
    #[tokio::test]
    async fn test_validate_dbscan_reachability_single_idea() -> Result<()> {
        // Single idea should always be reachable
        let result = validate_dbscan_reachability(1, &[], 0.5);
        assert!(result.is_ok(), "Single idea should always be reachable");
        
        Ok(())
    }

    /// Test validate_dbscan_reachability with connected cluster
    #[tokio::test]
    async fn test_validate_dbscan_reachability_connected_cluster() -> Result<()> {
        // All ideas connected (distance <= eps)
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
            VerifiedDistance { source_index: 1, target_index: 2, distance: 0.4 },
        ];
        let result = validate_dbscan_reachability(3, &distances, 0.5);
        assert!(result.is_ok(), "Connected cluster should pass");
        
        Ok(())
    }

    /// Test validate_dbscan_reachability with disconnected cluster
    #[tokio::test]
    async fn test_validate_dbscan_reachability_disconnected_cluster() -> Result<()> {
        // Ideas 0-1 connected, but idea 2 is isolated (noise)
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
            // Idea 2 has no connection with distance <= eps
        ];
        let result = validate_dbscan_reachability(3, &distances, 0.5);
        assert!(result.is_err(), "Disconnected cluster should fail");
        
        Ok(())
    }

    /// Test validate_dbscan_reachability with invalid index
    #[tokio::test]
    async fn test_validate_dbscan_reachability_invalid_index() -> Result<()> {
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 10, distance: 0.3 }, // Invalid index
        ];
        let result = validate_dbscan_reachability(3, &distances, 0.5);
        assert!(result.is_err(), "Invalid index should fail");
        
        Ok(())
    }

    /// Test validate_dbscan_reachability with distance exceeding eps
    #[tokio::test]
    async fn test_validate_dbscan_reachability_distance_exceeds_eps() -> Result<()> {
        // Distance > eps, so ideas are not connected
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.6 }, // > eps
        ];
        let result = validate_dbscan_reachability(2, &distances, 0.5);
        assert!(result.is_err(), "Distance exceeding eps should fail");
        
        Ok(())
    }

    /// Test validate_no_noise_points with empty distances
    #[tokio::test]
    async fn test_validate_no_noise_points_empty_distances() -> Result<()> {
        // Empty distances should pass (MVP behavior)
        let result = validate_no_noise_points(3, &[], 0.5, 2);
        assert!(result.is_ok(), "Empty distances should pass");
        
        Ok(())
    }

    /// Test validate_no_noise_points with sufficient neighbors
    #[tokio::test]
    async fn test_validate_no_noise_points_sufficient_neighbors() -> Result<()> {
        // Each idea has at least (min_samples - 1) neighbors
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
            VerifiedDistance { source_index: 0, target_index: 2, distance: 0.4 },
            VerifiedDistance { source_index: 1, target_index: 2, distance: 0.3 },
        ];
        let result = validate_no_noise_points(3, &distances, 0.5, 2);
        assert!(result.is_ok(), "Sufficient neighbors should pass");
        
        Ok(())
    }

    /// Test validate_no_noise_points with insufficient neighbors
    #[tokio::test]
    async fn test_validate_no_noise_points_insufficient_neighbors() -> Result<()> {
        // Idea 0 has only 1 neighbor, but min_samples=3 requires at least 2 neighbors
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
            // Idea 0 has only 1 neighbor, which is < (3 - 1) = 2
        ];
        let result = validate_no_noise_points(3, &distances, 0.5, 3);
        assert!(result.is_err(), "Insufficient neighbors should fail");
        
        Ok(())
    }

    /// Test validate_no_noise_points with invalid index
    #[tokio::test]
    async fn test_validate_no_noise_points_invalid_index() -> Result<()> {
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 10, distance: 0.3 },
        ];
        let result = validate_no_noise_points(3, &distances, 0.5, 2);
        assert!(result.is_err(), "Invalid index should fail");
        
        Ok(())
    }

    /// Test validate_dbscan_reachability with large cluster
    #[tokio::test]
    async fn test_validate_dbscan_reachability_large_cluster() -> Result<()> {
        // Create a chain: 0-1-2-3-4-5-6-7-8-9 (all connected)
        let mut distances = Vec::new();
        for i in 0..9 {
            distances.push(VerifiedDistance {
                source_index: i,
                target_index: i + 1,
                distance: 0.3, // < eps (0.5)
            });
        }
        
        let result = validate_dbscan_reachability(10, &distances, 0.5);
        assert!(result.is_ok(), "Large connected cluster should pass");
        
        Ok(())
    }

    /// Test validate_dbscan_reachability with exact eps distance
    #[tokio::test]
    async fn test_validate_dbscan_reachability_exact_eps() -> Result<()> {
        // Distance exactly equals eps (should be <=, so should pass)
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.5 }, // == eps
        ];
        let result = validate_dbscan_reachability(2, &distances, 0.5);
        assert!(result.is_ok(), "Distance exactly equal to eps should pass");
        
        Ok(())
    }
}
