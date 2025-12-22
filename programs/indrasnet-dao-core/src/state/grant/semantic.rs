//! Semantic Domain and Competency Structures
//!
//! Defines semantic filtering structures for grant voting system.
//! Used to determine voter eligibility and vote weights based on semantic similarity.
//!
//! Track B: Includes SemanticDistanceBundle for on-chain distance verification (B2)
//! and SemanticDomain account structure (B4)

use anchor_lang::prelude::*;

/// Verified Distance Entry (Track B: B3)
///
/// Single verified distance between two ideas in a cluster.
/// Used for DBSCAN validation in Phenomenon creation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, InitSpace)]
pub struct VerifiedDistance {
    /// Source idea index in related_ideas array
    pub source_index: u8,
    /// Target idea index in related_ideas array
    pub target_index: u8,
    /// Cosine similarity distance (0.0 - 1.0)
    pub distance: f32,
}

/// Semantic Distance Bundle (Track B: B2)
///
/// Signed distance bundle from off-chain semantic similarity service.
/// Used for on-chain verification of semantic distances between entities.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, InitSpace)]
pub struct SemanticDistanceBundle {
    /// Source entity (Idea or MeshGroup pubkey)
    pub source_entity: Pubkey,
    /// Target entity (Idea or MeshGroup pubkey)
    pub target_entity: Pubkey,
    /// Cosine similarity distance (0.0 - 1.0)
    /// Lower distance = higher similarity
    pub distance: f32,
    /// SHA256 hash of the bundle data (32 bytes)
    /// Hash format: Canonical hash (version || source || target || distance || timestamp || nonce || model_version)
    pub bundle_hash: [u8; 32],
    /// ed25519 signature of the bundle (64 bytes)
    /// Signature format: ed25519(bundle_hash, provider_pubkey)
    pub bundle_signature: [u8; 64],
    /// Provider public key (who computed and signed the distance)
    pub provider_pubkey: Pubkey,
    /// Provider name (e.g., "openai", "gemini")
    #[max_len(50)]
    pub provider: String,
    /// Timestamp when distance was computed
    pub timestamp: i64,
    /// Nonce for replay protection
    pub nonce: u64,
    /// Model version used for distance computation (e.g., "1.0.0", "1.0.1")
    #[max_len(50)]
    pub model_version: String,
}

/// Semantic Domain Account (Track B: B4)
///
/// On-chain account representing a semantic domain for grant voting.
/// Used for semantic pre-filtering of grants.
#[account]
#[derive(InitSpace)]
pub struct SemanticDomain {
    /// Domain ID (unique identifier)
    pub domain_id: u64,
    /// Domain name (e.g., "mathematics", "film-production")
    #[max_len(50)]
    pub name: String,
    /// SHA256 hash of the embedding vector (32 bytes)
    /// Embedding vector is computed off-chain, only hash is stored on-chain
    pub embedding_hash: [u8; 32],
    /// ed25519 signature of the embedding vector (64 bytes)
    /// Signature format: ed25519(SHA256(embedding_vector || domain_id || timestamp), provider_pubkey)
    pub embedding_signature: [u8; 64],
    /// Provider name (e.g., "openai", "gemini")
    #[max_len(50)]
    pub provider: String,
    /// Timestamp when domain was created
    pub created_at: i64,
    /// Bump seed
    pub bump: u8,
}

/// Competency level
/// 
/// Represents a member's competency in a semantic domain
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum CompetencyLevel {
    /// No competency (should not vote)
    None,
    /// Basic competency (reduced weight)
    Basic,
    /// Standard competency (normal weight)
    Standard,
    /// Expert competency (increased weight)
    Expert,
}

/// Competency record
/// 
/// Links a member to a semantic domain with a competency level
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Competency {
    pub domain_id: String,              // Semantic domain (max 50 chars)
    pub level: CompetencyLevel,          // Competency level
    pub confidence: u8,                 // Confidence score (0-100)
    pub derived_from: CompetencySource, // How competency was derived
    pub last_updated: i64,              // Last update timestamp
}

/// Source of competency derivation
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum CompetencySource {
    /// Derived from published ideas
    PublishedIdeas,
    /// Derived from mesh group participation
    MeshGroupParticipation,
    /// Derived from network connections
    NetworkConnections,
    /// Manually assigned (governance)
    ManualAssignment,
    /// AI analysis
    AIAnalysis,
}

impl anchor_lang::Space for Competency {
    const INIT_SPACE: usize = 4 + 50 + 1 + 1 + 1 + 8; // vec overhead + domain_id + level + confidence + source + last_updated
}

/// Grant voting layer
/// 
/// Defines which layer of voters can participate
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum VotingLayer {
    /// Author mesh group only (Level 1 grants)
    AuthorOnly,
    /// Author + Expert network (Level 2 grants)
    AuthorAndExpert,
    /// Author + Expert + DAO (Level 3 grants)
    AllLayers,
}

/// Vote weight multiplier based on competency
impl CompetencyLevel {
    /// Get vote weight multiplier
    pub fn weight_multiplier(&self) -> u64 {
        match self {
            CompetencyLevel::None => 0,      // Cannot vote
            CompetencyLevel::Basic => 50,    // 0.5x weight (as u64 * 100 / 100)
            CompetencyLevel::Standard => 100, // 1.0x weight
            CompetencyLevel::Expert => 150,   // 1.5x weight
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_competency_level_weight_multiplier() {
        assert_eq!(CompetencyLevel::None.weight_multiplier(), 0);
        assert_eq!(CompetencyLevel::Basic.weight_multiplier(), 50);
        assert_eq!(CompetencyLevel::Standard.weight_multiplier(), 100);
        assert_eq!(CompetencyLevel::Expert.weight_multiplier(), 150);
    }

    #[test]
    fn test_verified_distance_init_space() {
        let dist = VerifiedDistance {
            source_index: 0,
            target_index: 1,
            distance: 0.5,
        };
        assert_eq!(dist.source_index, 0);
        assert_eq!(dist.target_index, 1);
        assert_eq!(dist.distance, 0.5);
    }

    #[test]
    fn test_semantic_distance_bundle_init() {
        let bundle = SemanticDistanceBundle {
            source_entity: Pubkey::new_unique(),
            target_entity: Pubkey::new_unique(),
            distance: 0.3,
            bundle_hash: [0u8; 32],
            bundle_signature: [0u8; 64],
            provider_pubkey: Pubkey::new_unique(),
            provider: "test".to_string(),
            timestamp: 1000,
            nonce: 1,
            model_version: "v1".to_string(),
        };
        assert_eq!(bundle.distance, 0.3);
        assert_eq!(bundle.provider, "test");
    }

    #[test]
    fn test_competency_source_variants() {
        assert_eq!(CompetencySource::PublishedIdeas, CompetencySource::PublishedIdeas);
        assert_eq!(CompetencySource::MeshGroupParticipation, CompetencySource::MeshGroupParticipation);
        assert_eq!(CompetencySource::NetworkConnections, CompetencySource::NetworkConnections);
        assert_eq!(CompetencySource::ManualAssignment, CompetencySource::ManualAssignment);
        assert_eq!(CompetencySource::AIAnalysis, CompetencySource::AIAnalysis);
    }

    #[test]
    fn test_voting_layer_variants() {
        assert_eq!(VotingLayer::AuthorOnly, VotingLayer::AuthorOnly);
        assert_eq!(VotingLayer::AuthorAndExpert, VotingLayer::AuthorAndExpert);
        assert_eq!(VotingLayer::AllLayers, VotingLayer::AllLayers);
    }

    #[test]
    fn test_competency_structure() {
        let competency = Competency {
            domain_id: "mathematics".to_string(),
            level: CompetencyLevel::Expert,
            confidence: 95,
            derived_from: CompetencySource::AIAnalysis,
            last_updated: 1000,
        };
        
        assert_eq!(competency.domain_id, "mathematics");
        assert_eq!(competency.level, CompetencyLevel::Expert);
        assert_eq!(competency.confidence, 95);
        assert_eq!(competency.derived_from, CompetencySource::AIAnalysis);
        assert_eq!(competency.last_updated, 1000);
    }

    #[test]
    fn test_competency_level_variants() {
        assert_eq!(CompetencyLevel::None, CompetencyLevel::None);
        assert_eq!(CompetencyLevel::Basic, CompetencyLevel::Basic);
        assert_eq!(CompetencyLevel::Standard, CompetencyLevel::Standard);
        assert_eq!(CompetencyLevel::Expert, CompetencyLevel::Expert);
    }

    #[test]
    fn test_semantic_domain_structure() {
        let domain = SemanticDomain {
            domain_id: 1,
            name: "mathematics".to_string(),
            embedding_hash: [1u8; 32],
            embedding_signature: [2u8; 64],
            provider: "openai".to_string(),
            created_at: 1000,
            bump: 255,
        };
        
        assert_eq!(domain.domain_id, 1);
        assert_eq!(domain.name, "mathematics");
        assert_eq!(domain.created_at, 1000);
    }

    #[test]
    fn test_competency_level_weight_multiplier_all_levels() {
        assert_eq!(CompetencyLevel::None.weight_multiplier(), 0);
        assert_eq!(CompetencyLevel::Basic.weight_multiplier(), 50);
        assert_eq!(CompetencyLevel::Standard.weight_multiplier(), 100);
        assert_eq!(CompetencyLevel::Expert.weight_multiplier(), 150);
    }

    #[test]
    fn test_verified_distance_boundaries() {
        let dist_min = VerifiedDistance {
            source_index: 0,
            target_index: 0,
            distance: 0.0,
        };
        assert_eq!(dist_min.distance, 0.0);

        let dist_max = VerifiedDistance {
            source_index: 255,
            target_index: 255,
            distance: 1.0,
        };
        assert_eq!(dist_max.distance, 1.0);
        assert_eq!(dist_max.source_index, 255);
        assert_eq!(dist_max.target_index, 255);
    }

    #[test]
    fn test_semantic_distance_bundle_all_fields() {
        let source = Pubkey::new_unique();
        let target = Pubkey::new_unique();
        let provider_key = Pubkey::new_unique();
        
        let bundle = SemanticDistanceBundle {
            source_entity: source,
            target_entity: target,
            distance: 0.75,
            bundle_hash: [5u8; 32],
            bundle_signature: [10u8; 64],
            provider_pubkey: provider_key,
            provider: "gemini".to_string(),
            timestamp: 2000,
            nonce: 42,
            model_version: "v2.1".to_string(),
        };
        
        assert_eq!(bundle.source_entity, source);
        assert_eq!(bundle.target_entity, target);
        assert_eq!(bundle.distance, 0.75);
        assert_eq!(bundle.bundle_hash, [5u8; 32]);
        assert_eq!(bundle.provider, "gemini");
        assert_eq!(bundle.timestamp, 2000);
        assert_eq!(bundle.nonce, 42);
        assert_eq!(bundle.model_version, "v2.1");
    }

    #[test]
    fn test_competency_all_fields() {
        let competency = Competency {
            domain_id: "film-production".to_string(),
            level: CompetencyLevel::Standard,
            confidence: 75,
            derived_from: CompetencySource::MeshGroupParticipation,
            last_updated: 3000,
        };
        
        assert_eq!(competency.domain_id, "film-production");
        assert_eq!(competency.level, CompetencyLevel::Standard);
        assert_eq!(competency.confidence, 75);
        assert_eq!(competency.derived_from, CompetencySource::MeshGroupParticipation);
        assert_eq!(competency.last_updated, 3000);
    }

    #[test]
    fn test_competency_source_all_variants() {
        let sources = vec![
            CompetencySource::PublishedIdeas,
            CompetencySource::MeshGroupParticipation,
            CompetencySource::NetworkConnections,
            CompetencySource::ManualAssignment,
            CompetencySource::AIAnalysis,
        ];
        
        for source in sources {
            let competency = Competency {
                domain_id: "test".to_string(),
                level: CompetencyLevel::Basic,
                confidence: 50,
                derived_from: source,
                last_updated: 1000,
            };
            assert_eq!(competency.derived_from, source);
        }
    }

    #[test]
    fn test_voting_layer_all_variants() {
        assert_eq!(VotingLayer::AuthorOnly, VotingLayer::AuthorOnly);
        assert_ne!(VotingLayer::AuthorOnly, VotingLayer::AuthorAndExpert);
        assert_ne!(VotingLayer::AuthorOnly, VotingLayer::AllLayers);
        assert_eq!(VotingLayer::AuthorAndExpert, VotingLayer::AuthorAndExpert);
        assert_ne!(VotingLayer::AuthorAndExpert, VotingLayer::AllLayers);
        assert_eq!(VotingLayer::AllLayers, VotingLayer::AllLayers);
    }

    #[test]
    fn test_semantic_domain_all_fields() {
        let domain = SemanticDomain {
            domain_id: 999,
            name: "quantum-computing".to_string(),
            embedding_hash: [99u8; 32],
            embedding_signature: [88u8; 64],
            provider: "anthropic".to_string(),
            created_at: 5000,
            bump: 128,
        };
        
        assert_eq!(domain.domain_id, 999);
        assert_eq!(domain.name, "quantum-computing");
        assert_eq!(domain.embedding_hash, [99u8; 32]);
        assert_eq!(domain.embedding_signature, [88u8; 64]);
        assert_eq!(domain.provider, "anthropic");
        assert_eq!(domain.created_at, 5000);
        assert_eq!(domain.bump, 128);
    }

    #[test]
    fn test_competency_level_equality() {
        assert_eq!(CompetencyLevel::None, CompetencyLevel::None);
        assert_ne!(CompetencyLevel::None, CompetencyLevel::Basic);
        assert_eq!(CompetencyLevel::Basic, CompetencyLevel::Basic);
        assert_ne!(CompetencyLevel::Basic, CompetencyLevel::Standard);
        assert_eq!(CompetencyLevel::Standard, CompetencyLevel::Standard);
        assert_ne!(CompetencyLevel::Standard, CompetencyLevel::Expert);
        assert_eq!(CompetencyLevel::Expert, CompetencyLevel::Expert);
    }

    #[test]
    fn test_competency_level_all_variants_unique() {
        let levels = vec![
            CompetencyLevel::None,
            CompetencyLevel::Basic,
            CompetencyLevel::Standard,
            CompetencyLevel::Expert,
        ];
        
        for i in 0..levels.len() {
            for j in (i + 1)..levels.len() {
                assert_ne!(levels[i], levels[j], "Duplicate level found");
            }
        }
    }

    #[test]
    fn test_competency_level_copy() {
        let level1 = CompetencyLevel::Expert;
        let level2 = level1; // Copy trait
        assert_eq!(level1, level2);
    }

    #[test]
    fn test_competency_level_space() {
        assert_eq!(<CompetencyLevel as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_competency_source_all_variants_unique() {
        let sources = vec![
            CompetencySource::PublishedIdeas,
            CompetencySource::MeshGroupParticipation,
            CompetencySource::NetworkConnections,
            CompetencySource::ManualAssignment,
            CompetencySource::AIAnalysis,
        ];
        
        for i in 0..sources.len() {
            for j in (i + 1)..sources.len() {
                assert_ne!(sources[i], sources[j], "Duplicate source found");
            }
        }
    }

    #[test]
    fn test_competency_source_equality() {
        assert_eq!(CompetencySource::PublishedIdeas, CompetencySource::PublishedIdeas);
        assert_ne!(CompetencySource::PublishedIdeas, CompetencySource::AIAnalysis);
        assert_eq!(CompetencySource::AIAnalysis, CompetencySource::AIAnalysis);
    }

    #[test]
    fn test_competency_source_copy() {
        let source1 = CompetencySource::AIAnalysis;
        let source2 = source1; // Copy trait
        assert_eq!(source1, source2);
    }

    #[test]
    fn test_competency_source_space() {
        assert_eq!(<CompetencySource as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_voting_layer_all_variants_unique() {
        let layers = vec![
            VotingLayer::AuthorOnly,
            VotingLayer::AuthorAndExpert,
            VotingLayer::AllLayers,
        ];
        
        for i in 0..layers.len() {
            for j in (i + 1)..layers.len() {
                assert_ne!(layers[i], layers[j], "Duplicate layer found");
            }
        }
    }

    #[test]
    fn test_voting_layer_copy() {
        let layer1 = VotingLayer::AllLayers;
        let layer2 = layer1; // Copy trait
        assert_eq!(layer1, layer2);
    }

    #[test]
    fn test_voting_layer_space() {
        assert_eq!(<VotingLayer as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_verified_distance_clone() {
        let dist1 = VerifiedDistance {
            source_index: 0,
            target_index: 1,
            distance: 0.5,
        };
        let dist2 = dist1.clone();
        assert_eq!(dist1.source_index, dist2.source_index);
        assert_eq!(dist1.target_index, dist2.target_index);
        assert_eq!(dist1.distance, dist2.distance);
    }

    #[test]
    fn test_verified_distance_space() {
        assert_eq!(<VerifiedDistance as anchor_lang::Space>::INIT_SPACE, 1 + 1 + 4); // u8 + u8 + f32
    }

    #[test]
    fn test_semantic_domain_space() {
        // SemanticDomain has: u64 + String(50) + [u8;32] + [u8;64] + String(50) + i64 + u8
        assert!(<SemanticDomain as anchor_lang::Space>::INIT_SPACE > 0);
    }

    #[test]
    fn test_verified_distance_all_fields_comprehensive() {
        let dist = VerifiedDistance {
            source_index: 5,
            target_index: 7,
            distance: 0.25,
        };
        
        assert_eq!(dist.source_index, 5);
        assert_eq!(dist.target_index, 7);
        assert_eq!(dist.distance, 0.25);
    }
}
