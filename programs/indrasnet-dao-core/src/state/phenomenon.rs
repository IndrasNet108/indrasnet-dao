//! Phenomenon account structure
//!
//! Phenomenon - AI analytical solution for grouping similar ideas
//! Phenomena are created by AI AFTER grant issuance for analytics
//! Megaphenomena - AI analytics for grouping similar phenomena
//!
//! Includes analytics, tracking, and evolution functionality

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Discovery method for phenomena
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DiscoveryMethod {
    /// AI clustering
    AIClustering,
    /// Manual creation
    Manual,
    /// Semantic similarity
    SemanticSimilarity,
}

/// Phenomenon status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PhenomenonStatus {
    /// Proposed - awaiting approval
    Proposed,
    /// Active - approved and active
    Active,
    /// Archived - no longer active
    Archived,
}

/// Phenomenon
/// 
/// Phenomenon is created by AI AFTER grant issuance for analytics
/// Phenomenon groups similar ideas for analysis
/// Megaphenomena are created from phenomena for strategy
#[account]
#[derive(InitSpace)]
pub struct Phenomenon {
    pub observer: Pubkey,              // AI or user who created phenomenon
    pub created_at: i64,
    /// Related ideas (maximum 10)
    /// Use Vec for flexibility (InitSpace will handle automatically)
    #[max_len(10)]
    pub related_ideas: Vec<Pubkey>,    // Ideas included in phenomenon
    #[max_len(100)]
    pub name: String,                  // Phenomenon name
    #[max_len(500)]
    pub metadata_uri: String,          // Metadata URI (IPFS, Arweave, etc.)
    pub ethics_score: u8,              // Ethics score (0-255)
    pub similarity_score: f32,         // Similarity score (0.0-1.0)
    pub clustering_proof: [u8; 32],    // Clustering proof hash
    pub discovered_by: Pubkey,        // Who discovered this phenomenon
    pub discovery_method: DiscoveryMethod, // How it was discovered
    pub embedding_hash: Option<[u8; 32]>, // Embedding hash (optional)
    pub embedding_signature: Option<[u8; 64]>, // Embedding signature (optional)
    #[max_len(100)]
    pub embedding_provider: Option<String>, // Embedding provider (optional)
    pub status: PhenomenonStatus,      // Status (Proposed, Active, Archived)
    pub proposal_id: Option<u64>,     // Proposal ID if created via governance
    pub approved_at: Option<i64>,      // When approved
    #[max_len(10)]
    pub associated_mesh_groups: Vec<Pubkey>, // Associated mesh groups
    pub grant_priority: u8,           // Grant priority (0-255)
    pub cluster_hash: Option<[u8; 32]>, // Cluster hash (optional)
    pub cluster_signature: Option<[u8; 64]>, // Cluster signature (optional)
    pub temporal_window_days: Option<u16>, // Temporal window in days (optional)
    pub author_overlap_count: Option<u8>, // Author overlap count (optional)
    pub bump: u8,
}

impl Phenomenon {
    /// Maximum number of related ideas
    pub const MAX_RELATED_IDEAS: usize = 10;
    
    /// Add idea to phenomenon
    pub fn add_idea(&mut self, idea: Pubkey) -> Result<()> {
        require!(
            self.related_ideas.len() < Self::MAX_RELATED_IDEAS,
            crate::error::IndrasError::InvalidInput
        );
        require!(
            !self.related_ideas.contains(&idea),
            crate::error::IndrasError::AlreadyExists
        );
        
        self.related_ideas.push(idea);
        Ok(())
    }
    
    /// Remove idea from phenomenon
    pub fn remove_idea(&mut self, idea: Pubkey) -> Result<()> {
        if let Some(index) = self.related_ideas.iter().position(|&id| id == idea) {
            self.related_ideas.remove(index);
        }
        Ok(())
    }
    
    /// Check if phenomenon contains idea
    pub fn contains_idea(&self, idea: Pubkey) -> bool {
        self.related_ideas.contains(&idea)
    }
    
    /// Get number of related ideas
    pub fn idea_count(&self) -> usize {
        self.related_ideas.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    fn create_test_phenomenon(observer: Pubkey) -> Phenomenon {
        Phenomenon {
            observer,
            created_at: 1000,
            related_ideas: Vec::new(),
            name: "Test Phenomenon".to_string(),
            metadata_uri: "https://example.com/metadata".to_string(),
            ethics_score: 100,
            similarity_score: 0.8,
            clustering_proof: [0u8; 32],
            discovered_by: observer,
            discovery_method: DiscoveryMethod::AIClustering,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            status: PhenomenonStatus::Active,
            proposal_id: None,
            approved_at: None,
            associated_mesh_groups: Vec::new(),
            grant_priority: 50,
            cluster_hash: None,
            cluster_signature: None,
            temporal_window_days: None,
            author_overlap_count: None,
            bump: 255,
        }
    }

    #[test]
    fn test_phenomenon_add_idea() {
        let observer = create_test_pubkey(1);
        let mut phenomenon = create_test_phenomenon(observer);
        let idea = create_test_pubkey(2);
        
        assert!(phenomenon.add_idea(idea).is_ok());
        assert_eq!(phenomenon.idea_count(), 1);
        assert!(phenomenon.contains_idea(idea));
    }

    #[test]
    fn test_phenomenon_add_idea_duplicate() {
        let observer = create_test_pubkey(1);
        let mut phenomenon = create_test_phenomenon(observer);
        let idea = create_test_pubkey(2);
        
        assert!(phenomenon.add_idea(idea).is_ok());
        // Try to add duplicate - should fail
        assert!(phenomenon.add_idea(idea).is_err());
    }

    #[test]
    fn test_phenomenon_add_idea_max_limit() {
        let observer = create_test_pubkey(1);
        let mut phenomenon = create_test_phenomenon(observer);
        
        // Add 10 ideas (max)
        for i in 0..10 {
            let idea = create_test_pubkey(i + 2);
            assert!(phenomenon.add_idea(idea).is_ok());
        }
        
        // Try to add 11th idea - should fail
        let idea = create_test_pubkey(12);
        assert!(phenomenon.add_idea(idea).is_err());
    }

    #[test]
    fn test_phenomenon_remove_idea() {
        let observer = create_test_pubkey(1);
        let mut phenomenon = create_test_phenomenon(observer);
        let idea = create_test_pubkey(2);
        
        phenomenon.add_idea(idea).unwrap();
        assert_eq!(phenomenon.idea_count(), 1);
        
        phenomenon.remove_idea(idea).unwrap();
        assert_eq!(phenomenon.idea_count(), 0);
        assert!(!phenomenon.contains_idea(idea));
    }

    #[test]
    fn test_phenomenon_remove_nonexistent_idea() {
        let observer = create_test_pubkey(1);
        let mut phenomenon = create_test_phenomenon(observer);
        let idea = create_test_pubkey(2);
        
        // Remove non-existent idea - should not error (graceful)
        assert!(phenomenon.remove_idea(idea).is_ok());
    }

    #[test]
    fn test_phenomenon_contains_idea() {
        let observer = create_test_pubkey(1);
        let mut phenomenon = create_test_phenomenon(observer);
        let idea1 = create_test_pubkey(2);
        let idea2 = create_test_pubkey(3);
        
        phenomenon.add_idea(idea1).unwrap();
        
        assert!(phenomenon.contains_idea(idea1));
        assert!(!phenomenon.contains_idea(idea2));
    }

    #[test]
    fn test_phenomenon_idea_count() {
        let observer = create_test_pubkey(1);
        let mut phenomenon = create_test_phenomenon(observer);
        
        assert_eq!(phenomenon.idea_count(), 0);
        
        for i in 0..5 {
            let idea = create_test_pubkey(i as u8 + 2);
            phenomenon.add_idea(idea).unwrap();
        }
        
        assert_eq!(phenomenon.idea_count(), 5);
    }

    #[test]
    fn test_phenomenon_initialize_analytics() {
        let mut analytics = PhenomenonAnalyticsMetadata {
            analytics_id: 0,
            phenomenon_id: 0,
            analytics_type: PhenomenonAnalyticsType::Growth,
            status: PhenomenonAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_phenomenon_analytics(
            &mut analytics,
            1,
            10,
            PhenomenonAnalyticsType::Impact,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, 1);
        assert_eq!(analytics.phenomenon_id, 10);
        assert_eq!(analytics.analytics_type, PhenomenonAnalyticsType::Impact);
        assert_eq!(analytics.status, PhenomenonAnalyticsStatus::Active);
    }

    #[test]
    fn test_phenomenon_initialize_tracking() {
        let mut tracking = PhenomenonTrackingMetadata {
            tracking_id: 0,
            phenomenon_id: 0,
            tracking_type: PhenomenonTrackingType::Growth,
            status: PhenomenonTrackingStatus::Stopped,
            created_at: 0,
            tracking_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_phenomenon_tracking(
            &mut tracking,
            1,
            10,
            PhenomenonTrackingType::Impact,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(tracking.tracking_id, 1);
        assert_eq!(tracking.phenomenon_id, 10);
        assert_eq!(tracking.tracking_type, PhenomenonTrackingType::Impact);
        assert_eq!(tracking.status, PhenomenonTrackingStatus::Active);
    }

    #[test]
    fn test_phenomenon_initialize_evolution() {
        let mut evolution = PhenomenonEvolutionMetadata {
            evolution_id: 0,
            phenomenon_id: 0,
            evolution_stage: PhenomenonEvolutionStage::Emergence,
            status: PhenomenonEvolutionStatus::Completed,
            created_at: 0,
            evolution_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_phenomenon_evolution(
            &mut evolution,
            1,
            10,
            PhenomenonEvolutionStage::Growth,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(evolution.evolution_id, 1);
        assert_eq!(evolution.phenomenon_id, 10);
        assert_eq!(evolution.evolution_stage, PhenomenonEvolutionStage::Growth);
        assert_eq!(evolution.status, PhenomenonEvolutionStatus::Active);
    }

    #[test]
    fn test_phenomenon_status_variants() {
        assert_eq!(PhenomenonStatus::Proposed, PhenomenonStatus::Proposed);
        assert_eq!(PhenomenonStatus::Active, PhenomenonStatus::Active);
        assert_eq!(PhenomenonStatus::Archived, PhenomenonStatus::Archived);
    }

    #[test]
    fn test_discovery_method_variants() {
        assert_eq!(DiscoveryMethod::AIClustering, DiscoveryMethod::AIClustering);
        assert_eq!(DiscoveryMethod::Manual, DiscoveryMethod::Manual);
        assert_eq!(DiscoveryMethod::SemanticSimilarity, DiscoveryMethod::SemanticSimilarity);
    }

    #[test]
    fn test_phenomenon_structure_all_fields() {
        let observer = create_test_pubkey(1);
        let discovered_by = create_test_pubkey(2);
        let idea1 = create_test_pubkey(3);
        let idea2 = create_test_pubkey(4);
        let mesh_group = create_test_pubkey(5);
        
        let phenomenon = Phenomenon {
            observer,
            created_at: 5000,
            related_ideas: vec![idea1, idea2],
            name: "Test Phenomenon Name".to_string(),
            metadata_uri: "ipfs://test".to_string(),
            ethics_score: 200,
            similarity_score: 0.95,
            clustering_proof: [99u8; 32],
            discovered_by,
            discovery_method: DiscoveryMethod::SemanticSimilarity,
            embedding_hash: Some([88u8; 32]),
            embedding_signature: Some([77u8; 64]),
            embedding_provider: Some("openai".to_string()),
            status: PhenomenonStatus::Proposed,
            proposal_id: Some(123),
            approved_at: Some(6000),
            associated_mesh_groups: vec![mesh_group],
            grant_priority: 150,
            cluster_hash: Some([66u8; 32]),
            cluster_signature: Some([55u8; 64]),
            temporal_window_days: Some(30),
            author_overlap_count: Some(5),
            bump: 128,
        };
        
        assert_eq!(phenomenon.observer, observer);
        assert_eq!(phenomenon.created_at, 5000);
        assert_eq!(phenomenon.related_ideas.len(), 2);
        assert_eq!(phenomenon.name, "Test Phenomenon Name");
        assert_eq!(phenomenon.ethics_score, 200);
        assert_eq!(phenomenon.similarity_score, 0.95);
        assert_eq!(phenomenon.discovered_by, discovered_by);
        assert_eq!(phenomenon.discovery_method, DiscoveryMethod::SemanticSimilarity);
        assert_eq!(phenomenon.status, PhenomenonStatus::Proposed);
        assert_eq!(phenomenon.proposal_id, Some(123));
        assert_eq!(phenomenon.grant_priority, 150);
    }

    #[test]
    fn test_phenomenon_add_and_remove_multiple_ideas() {
        let observer = create_test_pubkey(1);
        let mut phenomenon = create_test_phenomenon(observer);
        
        let ideas: Vec<Pubkey> = (2..=6).map(|i| create_test_pubkey(i)).collect();
        
        // Add multiple ideas
        for idea in &ideas {
            assert!(phenomenon.add_idea(*idea).is_ok());
        }
        assert_eq!(phenomenon.idea_count(), 5);
        
        // Remove middle idea
        assert!(phenomenon.remove_idea(ideas[2]).is_ok());
        assert_eq!(phenomenon.idea_count(), 4);
        assert!(!phenomenon.contains_idea(ideas[2]));
        
        // Remove first idea
        assert!(phenomenon.remove_idea(ideas[0]).is_ok());
        assert_eq!(phenomenon.idea_count(), 3);
    }

    #[test]
    fn test_phenomenon_analytics_all_types() {
        let types = vec![
            PhenomenonAnalyticsType::Growth,
            PhenomenonAnalyticsType::Impact,
            PhenomenonAnalyticsType::Network,
            PhenomenonAnalyticsType::Custom,
        ];

        for analytics_type in types {
            let mut analytics = PhenomenonAnalyticsMetadata {
                analytics_id: 0,
                phenomenon_id: 0,
                analytics_type: PhenomenonAnalyticsType::Growth,
                status: PhenomenonAnalyticsStatus::Disabled,
                created_at: 0,
                analytics_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_phenomenon_analytics(
                &mut analytics,
                1,
                1,
                analytics_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(analytics.analytics_type, analytics_type);
        }
    }

    #[test]
    fn test_phenomenon_tracking_all_types() {
        let types = vec![
            PhenomenonTrackingType::Growth,
            PhenomenonTrackingType::Impact,
            PhenomenonTrackingType::Network,
            PhenomenonTrackingType::Custom,
        ];

        for tracking_type in types {
            let mut tracking = PhenomenonTrackingMetadata {
                tracking_id: 0,
                phenomenon_id: 0,
                tracking_type: PhenomenonTrackingType::Growth,
                status: PhenomenonTrackingStatus::Stopped,
                created_at: 0,
                tracking_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_phenomenon_tracking(
                &mut tracking,
                1,
                1,
                tracking_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(tracking.tracking_type, tracking_type);
        }
    }

    #[test]
    fn test_phenomenon_evolution_all_stages() {
        let stages = vec![
            PhenomenonEvolutionStage::Emergence,
            PhenomenonEvolutionStage::Growth,
            PhenomenonEvolutionStage::Maturity,
            PhenomenonEvolutionStage::Custom,
        ];

        for evolution_stage in stages {
            let mut evolution = PhenomenonEvolutionMetadata {
                evolution_id: 0,
                phenomenon_id: 0,
                evolution_stage: PhenomenonEvolutionStage::Emergence,
                status: PhenomenonEvolutionStatus::Completed,
                created_at: 0,
                evolution_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_phenomenon_evolution(
                &mut evolution,
                1,
                1,
                evolution_stage,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(evolution.evolution_stage, evolution_stage);
        }
    }

    #[test]
    fn test_phenomenon_analytics_status_always_active_on_init() {
        let mut analytics = PhenomenonAnalyticsMetadata {
            analytics_id: 0,
            phenomenon_id: 0,
            analytics_type: PhenomenonAnalyticsType::Growth,
            status: PhenomenonAnalyticsStatus::Paused,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };

        let result = onchain::initialize_phenomenon_analytics(
            &mut analytics,
            1,
            1,
            PhenomenonAnalyticsType::Impact,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(analytics.status, PhenomenonAnalyticsStatus::Active);
    }

    #[test]
    fn test_phenomenon_tracking_status_always_active_on_init() {
        let mut tracking = PhenomenonTrackingMetadata {
            tracking_id: 0,
            phenomenon_id: 0,
            tracking_type: PhenomenonTrackingType::Growth,
            status: PhenomenonTrackingStatus::Stopped,
            created_at: 0,
            tracking_config_hash: [0u8; 32],
            bump: 0,
        };

        let result = onchain::initialize_phenomenon_tracking(
            &mut tracking,
            1,
            1,
            PhenomenonTrackingType::Impact,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(tracking.status, PhenomenonTrackingStatus::Active);
    }

    #[test]
    fn test_phenomenon_evolution_status_always_active_on_init() {
        let mut evolution = PhenomenonEvolutionMetadata {
            evolution_id: 0,
            phenomenon_id: 0,
            evolution_stage: PhenomenonEvolutionStage::Emergence,
            status: PhenomenonEvolutionStatus::Completed,
            created_at: 0,
            evolution_config_hash: [0u8; 32],
            bump: 0,
        };

        let result = onchain::initialize_phenomenon_evolution(
            &mut evolution,
            1,
            1,
            PhenomenonEvolutionStage::Growth,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(evolution.status, PhenomenonEvolutionStatus::Active);
    }
}

/// Phenomenon analytics type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PhenomenonAnalyticsType {
    /// Growth analytics
    Growth,
    /// Impact analytics
    Impact,
    /// Network analytics
    Network,
    /// Custom analytics
    Custom,
}

/// Phenomenon analytics status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PhenomenonAnalyticsStatus {
    /// Analytics active
    Active,
    /// Analytics paused
    Paused,
    /// Analytics disabled
    Disabled,
}

/// Phenomenon tracking type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PhenomenonTrackingType {
    /// Growth tracking
    Growth,
    /// Impact tracking
    Impact,
    /// Network tracking
    Network,
    /// Custom tracking
    Custom,
}

/// Phenomenon tracking status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PhenomenonTrackingStatus {
    /// Tracking active
    Active,
    /// Tracking paused
    Paused,
    /// Tracking stopped
    Stopped,
}

/// Phenomenon evolution stage
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PhenomenonEvolutionStage {
    /// Emergence
    Emergence,
    /// Growth
    Growth,
    /// Maturity
    Maturity,
    /// Custom stage
    Custom,
}

/// Phenomenon evolution status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PhenomenonEvolutionStatus {
    /// Evolution active
    Active,
    /// Evolution paused
    Paused,
    /// Evolution completed
    Completed,
}

/// Phenomenon analytics metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PhenomenonAnalyticsMetadata {
    /// Analytics ID
    pub analytics_id: u64,
    /// Phenomenon ID
    pub phenomenon_id: u64,
    /// Analytics type
    pub analytics_type: PhenomenonAnalyticsType,
    /// Status
    pub status: PhenomenonAnalyticsStatus,
    /// Created at
    pub created_at: i64,
    /// Analytics config hash
    pub analytics_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// Phenomenon tracking metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PhenomenonTrackingMetadata {
    /// Tracking ID
    pub tracking_id: u64,
    /// Phenomenon ID
    pub phenomenon_id: u64,
    /// Tracking type
    pub tracking_type: PhenomenonTrackingType,
    /// Status
    pub status: PhenomenonTrackingStatus,
    /// Created at
    pub created_at: i64,
    /// Tracking config hash
    pub tracking_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// Phenomenon evolution metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PhenomenonEvolutionMetadata {
    /// Evolution ID
    pub evolution_id: u64,
    /// Phenomenon ID
    pub phenomenon_id: u64,
    /// Evolution stage
    pub evolution_stage: PhenomenonEvolutionStage,
    /// Status
    pub status: PhenomenonEvolutionStatus,
    /// Created at
    pub created_at: i64,
    /// Evolution config hash
    pub evolution_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for phenomenon analytics, tracking, and evolution
pub mod onchain {
    use super::*;

    pub fn initialize_phenomenon_analytics(
        analytics: &mut PhenomenonAnalyticsMetadata,
        analytics_id: u64,
        phenomenon_id: u64,
        analytics_type: PhenomenonAnalyticsType,
        analytics_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analytics_id > 0, IndrasError::InvalidInput);
        analytics.analytics_id = analytics_id;
        analytics.phenomenon_id = phenomenon_id;
        analytics.analytics_type = analytics_type;
        analytics.status = PhenomenonAnalyticsStatus::Active;
        analytics.created_at = current_time;
        analytics.analytics_config_hash = analytics_config_hash;
        analytics.bump = bump;
        Ok(())
    }

    pub fn initialize_phenomenon_tracking(
        tracking: &mut PhenomenonTrackingMetadata,
        tracking_id: u64,
        phenomenon_id: u64,
        tracking_type: PhenomenonTrackingType,
        tracking_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(tracking_id > 0, IndrasError::InvalidInput);
        tracking.tracking_id = tracking_id;
        tracking.phenomenon_id = phenomenon_id;
        tracking.tracking_type = tracking_type;
        tracking.status = PhenomenonTrackingStatus::Active;
        tracking.created_at = current_time;
        tracking.tracking_config_hash = tracking_config_hash;
        tracking.bump = bump;
        Ok(())
    }

    pub fn initialize_phenomenon_evolution(
        evolution: &mut PhenomenonEvolutionMetadata,
        evolution_id: u64,
        phenomenon_id: u64,
        evolution_stage: PhenomenonEvolutionStage,
        evolution_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(evolution_id > 0, IndrasError::InvalidInput);
        evolution.evolution_id = evolution_id;
        evolution.phenomenon_id = phenomenon_id;
        evolution.evolution_stage = evolution_stage;
        evolution.status = PhenomenonEvolutionStatus::Active;
        evolution.created_at = current_time;
        evolution.evolution_config_hash = evolution_config_hash;
        evolution.bump = bump;
        Ok(())
    }
}

/// Off-chain functions for phenomenon analytics, tracking, and evolution
pub mod offchain {
    pub fn generate_phenomenon_analytics(_analytics_id: u64) -> Vec<u8> {
        vec![]
    }

    pub fn track_phenomenon(_tracking_id: u64) -> Vec<u8> {
        vec![]
    }

    pub fn track_evolution(_evolution_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod offchain_tests {
    use super::offchain;

    #[test]
    fn test_offchain_generate_phenomenon_analytics() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::generate_phenomenon_analytics(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_track_phenomenon() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::track_phenomenon(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_track_evolution() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::track_evolution(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_phenomenon_functions_different_ids() {
        // Test with different IDs
        let result1 = offchain::generate_phenomenon_analytics(1);
        let result2 = offchain::generate_phenomenon_analytics(999);
        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
        
        let result3 = offchain::track_phenomenon(1);
        let result4 = offchain::track_phenomenon(999);
        assert_eq!(result3, Vec::<u8>::new());
        assert_eq!(result4, Vec::<u8>::new());
        
        let result5 = offchain::track_evolution(1);
        let result6 = offchain::track_evolution(999);
        assert_eq!(result5, Vec::<u8>::new());
        assert_eq!(result6, Vec::<u8>::new());
    }
}

#[cfg(test)]
mod enum_tests {
    use super::*;

    #[test]
    fn test_discovery_method_all_variants_unique() {
        let methods = vec![
            DiscoveryMethod::AIClustering,
            DiscoveryMethod::Manual,
            DiscoveryMethod::SemanticSimilarity,
        ];
        
        for i in 0..methods.len() {
            for j in (i + 1)..methods.len() {
                assert_ne!(methods[i], methods[j], "Duplicate method found");
            }
        }
    }

    #[test]
    fn test_phenomenon_status_all_variants_unique() {
        let statuses = vec![
            PhenomenonStatus::Proposed,
            PhenomenonStatus::Active,
            PhenomenonStatus::Archived,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_discovery_method_copy() {
        let m1 = DiscoveryMethod::AIClustering;
        let m2 = m1; // Copy trait
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_phenomenon_status_copy() {
        let s1 = PhenomenonStatus::Active;
        let s2 = s1; // Copy trait
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_discovery_method_space() {
        assert_eq!(<DiscoveryMethod as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_phenomenon_status_space() {
        assert_eq!(<PhenomenonStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }
}
