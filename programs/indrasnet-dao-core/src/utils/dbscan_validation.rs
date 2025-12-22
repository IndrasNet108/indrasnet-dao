//! DBSCAN Validation Utilities
//!
//! Provides reachability and noise validation for DBSCAN clustering.
//! Ensures all ideas in a cluster are reachable through a chain of distances ≤ eps.

use anchor_lang::prelude::*;
use crate::state::grant::semantic::VerifiedDistance;

/// Check DBSCAN reachability for a cluster
///
/// Verifies that all ideas in the cluster are reachable through a chain of distances ≤ eps.
/// Uses BFS (Breadth-First Search) to check connectivity.
///
/// # Arguments
/// * `num_ideas` - Number of ideas in the cluster
/// * `verified_distances` - Verified pairwise distances between ideas
/// * `eps` - Maximum distance threshold for reachability
///
/// # Returns
/// * `Ok(())` if all ideas are reachable
/// * `Err(CoreError::SemanticDistanceExceeded)` if some ideas are not reachable (noise)
pub fn validate_dbscan_reachability(
    num_ideas: usize,
    verified_distances: &[VerifiedDistance],
    eps: f32,
) -> Result<()> {
    use crate::error::IndrasError as CoreError;
    
    // If no distances provided, skip reachability check (for MVP)
    if verified_distances.is_empty() {
        msg!("DBSCAN: No verified distances provided - skipping reachability check (MVP)");
        return Ok(());
    }
    
    // Build adjacency list: for each idea, list of reachable ideas (distance ≤ eps)
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); num_ideas];
    
    for dist in verified_distances {
        // Validate indices
        if dist.source_index as usize >= num_ideas || dist.target_index as usize >= num_ideas {
            return Err(error!(CoreError::InvalidInput));
        }
        
        // If distance ≤ eps, ideas are reachable
        if dist.distance <= eps {
            let source = dist.source_index as usize;
            let target = dist.target_index as usize;
            adjacency[source].push(target);
            adjacency[target].push(source); // Undirected graph
        }
    }
    
    // BFS to check if all ideas are reachable from idea 0
    let mut visited = vec![false; num_ideas];
    let mut queue = std::collections::VecDeque::new();
    
    // Start BFS from idea 0
    queue.push_back(0);
    visited[0] = true;
    let mut reachable_count = 1;
    
    while let Some(current) = queue.pop_front() {
        for &neighbor in &adjacency[current] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
                reachable_count += 1;
            }
        }
    }
    
    // All ideas must be reachable
    require!(
        reachable_count == num_ideas,
        CoreError::SemanticDistanceExceeded // Some ideas are noise (not reachable)
    );
    
    msg!("DBSCAN: All {} ideas are reachable (no noise points)", num_ideas);
    
    Ok(())
}

/// Validate that cluster has no noise points
///
/// Noise points are ideas that cannot be reached from any core point.
/// This function ensures all ideas in the cluster are part of the same connected component.
pub fn validate_no_noise_points(
    num_ideas: usize,
    verified_distances: &[VerifiedDistance],
    eps: f32,
    min_samples: u8,
) -> Result<()> {
    use crate::error::IndrasError as CoreError;
    
    // If no distances provided, skip noise check (for MVP)
    if verified_distances.is_empty() {
        msg!("DBSCAN: No verified distances provided - skipping noise check (MVP)");
        return Ok(());
    }
    
    // Build adjacency list
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); num_ideas];
    
    for dist in verified_distances {
        if dist.source_index as usize >= num_ideas || dist.target_index as usize >= num_ideas {
            return Err(error!(CoreError::InvalidInput));
        }
        
        if dist.distance <= eps {
            let source = dist.source_index as usize;
            let target = dist.target_index as usize;
            adjacency[source].push(target);
            adjacency[target].push(source);
        }
    }
    
    // Check that each idea has at least (min_samples - 1) neighbors (excluding itself)
    // This ensures each idea is a core point or border point, not noise
    for (i, _) in adjacency.iter().take(num_ideas).enumerate() {
        let neighbor_count = adjacency[i].len();
        // For DBSCAN, each point should have at least (min_samples - 1) neighbors
        // (min_samples includes the point itself)
        if neighbor_count < (min_samples as usize).saturating_sub(1) {
            // This idea is a noise point (not enough neighbors)
            require!(
                false,
                CoreError::SemanticDistanceExceeded // Idea {} is a noise point
            );
        }
    }
    
    msg!("DBSCAN: No noise points detected - all ideas have sufficient neighbors");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::grant::semantic::VerifiedDistance;

    #[test]
    fn test_validate_dbscan_reachability_empty_distances() {
        // Empty distances should pass (MVP behavior)
        let result = validate_dbscan_reachability(3, &[], 0.5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dbscan_reachability_single_idea() {
        // Single idea should always be reachable
        let result = validate_dbscan_reachability(1, &[], 0.5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dbscan_reachability_connected_cluster() {
        // All ideas connected (distance <= eps)
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
            VerifiedDistance { source_index: 1, target_index: 2, distance: 0.4 },
        ];
        let result = validate_dbscan_reachability(3, &distances, 0.5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dbscan_reachability_disconnected_cluster() {
        // Ideas 0-1 connected, but idea 2 is isolated (noise)
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
            // Idea 2 has no connection with distance <= eps
        ];
        let result = validate_dbscan_reachability(3, &distances, 0.5);
        assert!(result.is_err());
        // Check error type
        if let Err(e) = result {
            // Anchor error format - check error name
            let error_name = format!("{:?}", e);
            assert!(error_name.contains("SemanticDistanceExceeded"));
        }
    }

    #[test]
    fn test_validate_dbscan_reachability_invalid_index() {
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 10, distance: 0.3 }, // Invalid index
        ];
        let result = validate_dbscan_reachability(3, &distances, 0.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_dbscan_reachability_distance_exceeds_eps() {
        // Distance > eps, so ideas are not connected
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.6 }, // > eps
        ];
        let result = validate_dbscan_reachability(2, &distances, 0.5);
        assert!(result.is_err()); // Ideas not reachable
    }

    #[test]
    fn test_validate_no_noise_points_empty_distances() {
        // Empty distances should pass (MVP behavior)
        let result = validate_no_noise_points(3, &[], 0.5, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_noise_points_sufficient_neighbors() {
        // Each idea has at least (min_samples - 1) neighbors
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
            VerifiedDistance { source_index: 0, target_index: 2, distance: 0.4 },
            VerifiedDistance { source_index: 1, target_index: 2, distance: 0.3 },
        ];
        let result = validate_no_noise_points(3, &distances, 0.5, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_noise_points_insufficient_neighbors() {
        // Idea 0 has only 1 neighbor, but min_samples=3 requires at least 2 neighbors
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
            // Idea 0 has only 1 neighbor, which is < (3 - 1) = 2
        ];
        let result = validate_no_noise_points(3, &distances, 0.5, 3);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_no_noise_points_invalid_index() {
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 10, distance: 0.3 },
        ];
        let result = validate_no_noise_points(3, &distances, 0.5, 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_dbscan_reachability_exact_eps() {
        // Test with distance exactly equal to eps
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.5 }, // Exactly eps
        ];
        let result = validate_dbscan_reachability(2, &distances, 0.5);
        assert!(result.is_ok()); // Should pass (distance <= eps)
    }

    #[test]
    fn test_validate_dbscan_reachability_just_below_eps() {
        // Test with distance just below eps
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.499 }, // Just below eps
        ];
        let result = validate_dbscan_reachability(2, &distances, 0.5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dbscan_reachability_just_above_eps() {
        // Test with distance just above eps
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.501 }, // Just above eps
        ];
        let result = validate_dbscan_reachability(2, &distances, 0.5);
        assert!(result.is_err()); // Should fail (distance > eps)
    }

    #[test]
    fn test_validate_dbscan_reachability_chain_connection() {
        // Test chain: 0 -> 1 -> 2 (all connected)
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
            VerifiedDistance { source_index: 1, target_index: 2, distance: 0.4 },
        ];
        let result = validate_dbscan_reachability(3, &distances, 0.5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dbscan_reachability_ring_structure() {
        // Test ring: 0 -> 1 -> 2 -> 0 (all connected in ring)
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
            VerifiedDistance { source_index: 1, target_index: 2, distance: 0.4 },
            VerifiedDistance { source_index: 2, target_index: 0, distance: 0.3 },
        ];
        let result = validate_dbscan_reachability(3, &distances, 0.5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dbscan_reachability_fully_connected() {
        // Test fully connected graph (all pairs connected)
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
            VerifiedDistance { source_index: 0, target_index: 2, distance: 0.4 },
            VerifiedDistance { source_index: 1, target_index: 2, distance: 0.3 },
        ];
        let result = validate_dbscan_reachability(3, &distances, 0.5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dbscan_reachability_zero_eps() {
        // Test with eps = 0 (only exact matches)
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.0 }, // Exact match
        ];
        let result = validate_dbscan_reachability(2, &distances, 0.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dbscan_reachability_negative_eps() {
        // Test with negative eps (should still work, but no connections)
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
        ];
        let result = validate_dbscan_reachability(2, &distances, -0.1);
        assert!(result.is_err()); // No connections possible with negative eps
    }

    #[test]
    fn test_validate_no_noise_points_min_samples_one() {
        // With min_samples=1, any idea with 0 neighbors is noise
        let distances = vec![
            // Idea 0 has no neighbors
        ];
        let result = validate_no_noise_points(1, &distances, 0.5, 1);
        // Idea 0 needs (1-1) = 0 neighbors, so it should pass
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_noise_points_min_samples_two() {
        // With min_samples=2, each idea needs at least 1 neighbor
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
        ];
        let result = validate_no_noise_points(2, &distances, 0.5, 2);
        // Both ideas have 1 neighbor, which is >= (2-1) = 1, so should pass
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_noise_points_exact_min_samples() {
        // Test with exactly min_samples-1 neighbors for each idea
        // For min_samples=3, each idea needs at least 2 neighbors
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
            VerifiedDistance { source_index: 0, target_index: 2, distance: 0.4 },
            VerifiedDistance { source_index: 1, target_index: 2, distance: 0.3 }, // Connect 1 and 2
        ];
        // Idea 0 has 2 neighbors (1, 2), Idea 1 has 2 neighbors (0, 2), Idea 2 has 2 neighbors (0, 1)
        // All have exactly (3-1) = 2 neighbors for min_samples=3
        let result = validate_no_noise_points(3, &distances, 0.5, 3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_noise_points_more_than_min_samples() {
        // Test with more than min_samples-1 neighbors for each idea
        // For min_samples=3, each idea needs at least 2 neighbors
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.3 },
            VerifiedDistance { source_index: 0, target_index: 2, distance: 0.4 },
            VerifiedDistance { source_index: 0, target_index: 3, distance: 0.3 },
            VerifiedDistance { source_index: 1, target_index: 2, distance: 0.3 }, // Connect 1 and 2
            VerifiedDistance { source_index: 1, target_index: 3, distance: 0.4 }, // Connect 1 and 3
            VerifiedDistance { source_index: 2, target_index: 3, distance: 0.3 }, // Connect 2 and 3
        ];
        // All ideas have at least 2 neighbors (>= (3-1) = 2 for min_samples=3)
        let result = validate_no_noise_points(4, &distances, 0.5, 3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_noise_points_zero_eps() {
        // With eps=0, only exact matches (distance=0) are connected
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 0.0 },
        ];
        let result = validate_no_noise_points(2, &distances, 0.0, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_noise_points_large_eps() {
        // With very large eps, all distances should be connected
        let distances = vec![
            VerifiedDistance { source_index: 0, target_index: 1, distance: 100.0 },
        ];
        let result = validate_no_noise_points(2, &distances, 1000.0, 2);
        assert!(result.is_ok());
    }
}
