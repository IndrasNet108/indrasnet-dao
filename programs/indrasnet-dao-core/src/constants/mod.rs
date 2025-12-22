//! Constants for the IndrasNet DAO Core program

use anchor_lang::prelude::*;
use std::str::FromStr;

/// ED25519 signatures are 64 bytes long.
pub const MAX_MESH_SIGNATURE_LEN: usize = 64;

/// Seed for the program's primary treasury PDA.
pub const SEED_TREASURY: &[u8] = b"treasury";

/// Maximum length for DAO names.
pub const MAX_DAO_NAME_LEN: usize = 32;

/// Maximum length for governance configuration URIs.
pub const MAX_GOVERNANCE_CONFIG_URI_LEN: usize = 200;

/// Maximum length for DAO descriptions.
pub const MAX_DAO_DESCRIPTION_LEN: usize = 500;

/// Maximum length for phenomenon names.
pub const MAX_PHENOMENON_NAME_LEN: usize = 64;

/// Maximum length for phenomenon URIs.
pub const MAX_PHENOMENON_URI_LEN: usize = 128;

/// Maximum length for idea titles.
pub const MAX_IDEA_TITLE_LEN: usize = 8;

/// Maximum length for idea content URIs.
pub const MAX_IDEA_CONTENT_URI_LEN: usize = 16;

/// Maximum number of tags an idea can have.
pub const MAX_IDEA_TAGS: usize = 10;

/// Maximum length for an individual idea tag.
pub const MAX_IDEA_TAG_LENGTH: usize = 30;

/// Maximum length for idea group names.
pub const MAX_IDEAGROUP_NAME_LEN: usize = 64;

/// Maximum length for idea group descriptions.
pub const MAX_IDEAGROUP_DESCRIPTION_LEN: usize = 256;

/// Maximum number of members in a MeshGroup.
pub const MAX_MESHGROUP_MEMBERS: usize = 20;

/// Maximum number of ideas in a MeshGroup.
pub const MAX_MESHGROUP_IDEAS: usize = 20;

/// Maximum number of phenomena in a MeshGroup.
pub const MAX_MESHGROUP_PHENOMENONS: usize = 20;

/// Maximum number of related ideas a phenomenon can have.
pub const MAX_RELATED_IDEAS: usize = 10;

/// Maximum length for comments.
pub const MAX_COMMENT_LEN: usize = 256;

// Cockpit related constants
/// Maximum number of reflexions a Cockpit can store.
pub const MAX_REFLEXIONS: usize = 1;

/// Maximum length for reflexion content.
pub const MAX_REFLEXION_CONTENT_LENGTH: usize = 256;

/// Maximum number of capabilities a Cockpit can have.
pub const MAX_CAPABILITIES: usize = 1;

/// Maximum length for a capability name.
pub const MAX_CAPABILITY_NAME_LENGTH: usize = 50;

/// Maximum number of active ideas a Cockpit can track.
pub const MAX_ACTIVE_IDEAS: usize = 1;

/// Maximum number of observed phenomena a Cockpit can track.
pub const MAX_OBSERVED_PHENOMENONS: usize = 1;

// CRDT Document related constants
/// Maximum number of nodes in a document graph.
pub const MAX_GRAPH_NODES: usize = 1000;

/// Maximum number of edges in a document graph.
pub const MAX_GRAPH_EDGES: usize = 2000;

/// Maximum length for edge type strings.
pub const MAX_EDGE_TYPE_LEN: usize = 50;

/// Maximum length for node type strings.
pub const MAX_NODE_TYPE_LEN: usize = 50;

/// Maximum length for document IDs.
pub const MAX_DOCUMENT_ID_LEN: usize = 32;

/// Maximum number of snapshots per document.
pub const MAX_DOCUMENT_SNAPSHOTS: usize = 100;

// IPFS Publication related constants
/// Maximum length for pinning service names.
pub const MAX_PINNING_SERVICE_LEN: usize = 50;

/// Maximum number of IPFS replicas for MVP.
pub const MIN_IPFS_REPLICAS: usize = 2;

/// Maximum number of IPFS replicas for MVP.
pub const MAX_IPFS_REPLICAS: usize = 2;

/// AI Program ID (indrasnet_dao_ai)
/// Program: GwDihBiWfAmej9enpYRrie4D2XoPsfD44ML3sqDQvcXe
/// NOTE: Function to get AI program ID (cannot use const due to Pubkey::from limitations)
pub fn ai_program_id() -> Pubkey {
    // Base58: GwDihBiWfAmej9enpYRrie4D2XoPsfD44ML3sqDQvcXe
    // Decoded bytes (approximate - need to verify)
    Pubkey::from_str("GwDihBiWfAmej9enpYRrie4D2XoPsfD44ML3sqDQvcXe").unwrap_or_else(|_| {
        // Fallback if from_str fails
        Pubkey::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_mesh_signature_len() {
        assert_eq!(MAX_MESH_SIGNATURE_LEN, 64);
    }

    #[test]
    fn test_seed_treasury() {
        assert_eq!(SEED_TREASURY, b"treasury");
    }

    #[test]
    fn test_max_dao_name_len() {
        assert_eq!(MAX_DAO_NAME_LEN, 32);
    }

    #[test]
    fn test_max_governance_config_uri_len() {
        assert_eq!(MAX_GOVERNANCE_CONFIG_URI_LEN, 200);
    }

    #[test]
    fn test_max_dao_description_len() {
        assert_eq!(MAX_DAO_DESCRIPTION_LEN, 500);
    }

    #[test]
    fn test_max_phenomenon_name_len() {
        assert_eq!(MAX_PHENOMENON_NAME_LEN, 64);
    }

    #[test]
    fn test_max_phenomenon_uri_len() {
        assert_eq!(MAX_PHENOMENON_URI_LEN, 128);
    }

    #[test]
    fn test_max_idea_title_len() {
        assert_eq!(MAX_IDEA_TITLE_LEN, 8);
    }

    #[test]
    fn test_max_idea_content_uri_len() {
        assert_eq!(MAX_IDEA_CONTENT_URI_LEN, 16);
    }

    #[test]
    fn test_max_idea_tags() {
        assert_eq!(MAX_IDEA_TAGS, 10);
    }

    #[test]
    fn test_max_idea_tag_length() {
        assert_eq!(MAX_IDEA_TAG_LENGTH, 30);
    }

    #[test]
    fn test_max_ideagroup_name_len() {
        assert_eq!(MAX_IDEAGROUP_NAME_LEN, 64);
    }

    #[test]
    fn test_max_ideagroup_description_len() {
        assert_eq!(MAX_IDEAGROUP_DESCRIPTION_LEN, 256);
    }

    #[test]
    fn test_max_meshgroup_members() {
        assert_eq!(MAX_MESHGROUP_MEMBERS, 20);
    }

    #[test]
    fn test_max_meshgroup_ideas() {
        assert_eq!(MAX_MESHGROUP_IDEAS, 20);
    }

    #[test]
    fn test_max_meshgroup_phenomenons() {
        assert_eq!(MAX_MESHGROUP_PHENOMENONS, 20);
    }

    #[test]
    fn test_max_related_ideas() {
        assert_eq!(MAX_RELATED_IDEAS, 10);
    }

    #[test]
    fn test_max_comment_len() {
        assert_eq!(MAX_COMMENT_LEN, 256);
    }

    #[test]
    fn test_max_reflexions() {
        assert_eq!(MAX_REFLEXIONS, 1);
    }

    #[test]
    fn test_max_reflexion_content_length() {
        assert_eq!(MAX_REFLEXION_CONTENT_LENGTH, 256);
    }

    #[test]
    fn test_max_capabilities() {
        assert_eq!(MAX_CAPABILITIES, 1);
    }

    #[test]
    fn test_max_capability_name_length() {
        assert_eq!(MAX_CAPABILITY_NAME_LENGTH, 50);
    }

    #[test]
    fn test_max_active_ideas() {
        assert_eq!(MAX_ACTIVE_IDEAS, 1);
    }

    #[test]
    fn test_max_observed_phenomenons() {
        assert_eq!(MAX_OBSERVED_PHENOMENONS, 1);
    }

    #[test]
    fn test_max_graph_nodes() {
        assert_eq!(MAX_GRAPH_NODES, 1000);
    }

    #[test]
    fn test_max_graph_edges() {
        assert_eq!(MAX_GRAPH_EDGES, 2000);
    }

    #[test]
    fn test_max_edge_type_len() {
        assert_eq!(MAX_EDGE_TYPE_LEN, 50);
    }

    #[test]
    fn test_max_node_type_len() {
        assert_eq!(MAX_NODE_TYPE_LEN, 50);
    }

    #[test]
    fn test_max_document_id_len() {
        assert_eq!(MAX_DOCUMENT_ID_LEN, 32);
    }

    #[test]
    fn test_max_document_snapshots() {
        assert_eq!(MAX_DOCUMENT_SNAPSHOTS, 100);
    }

    #[test]
    fn test_max_pinning_service_len() {
        assert_eq!(MAX_PINNING_SERVICE_LEN, 50);
    }

    #[test]
    fn test_min_ipfs_replicas() {
        assert_eq!(MIN_IPFS_REPLICAS, 2);
    }

    #[test]
    fn test_max_ipfs_replicas() {
        assert_eq!(MAX_IPFS_REPLICAS, 2);
    }

    #[test]
    fn test_ipfs_replicas_range() {
        // Test that min <= max
        assert!(MIN_IPFS_REPLICAS <= MAX_IPFS_REPLICAS);
    }

    #[test]
    fn test_ai_program_id() {
        let program_id = ai_program_id();
        // Test that function returns a valid Pubkey (not default)
        assert_ne!(program_id, Pubkey::default());
    }

    #[test]
    fn test_ai_program_id_consistency() {
        // Test that function returns same value on multiple calls
        let id1 = ai_program_id();
        let id2 = ai_program_id();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_constants_positive_values() {
        // Test that all numeric constants are positive
        assert!(MAX_MESH_SIGNATURE_LEN > 0);
        assert!(MAX_DAO_NAME_LEN > 0);
        assert!(MAX_GOVERNANCE_CONFIG_URI_LEN > 0);
        assert!(MAX_DAO_DESCRIPTION_LEN > 0);
        assert!(MAX_PHENOMENON_NAME_LEN > 0);
        assert!(MAX_PHENOMENON_URI_LEN > 0);
        assert!(MAX_IDEA_TITLE_LEN > 0);
        assert!(MAX_IDEA_CONTENT_URI_LEN > 0);
        assert!(MAX_IDEA_TAGS > 0);
        assert!(MAX_IDEA_TAG_LENGTH > 0);
        assert!(MAX_IDEAGROUP_NAME_LEN > 0);
        assert!(MAX_IDEAGROUP_DESCRIPTION_LEN > 0);
        assert!(MAX_MESHGROUP_MEMBERS > 0);
        assert!(MAX_MESHGROUP_IDEAS > 0);
        assert!(MAX_MESHGROUP_PHENOMENONS > 0);
        assert!(MAX_RELATED_IDEAS > 0);
        assert!(MAX_COMMENT_LEN > 0);
        assert!(MAX_REFLEXIONS > 0);
        assert!(MAX_REFLEXION_CONTENT_LENGTH > 0);
        assert!(MAX_CAPABILITIES > 0);
        assert!(MAX_CAPABILITY_NAME_LENGTH > 0);
        assert!(MAX_ACTIVE_IDEAS > 0);
        assert!(MAX_OBSERVED_PHENOMENONS > 0);
        assert!(MAX_GRAPH_NODES > 0);
        assert!(MAX_GRAPH_EDGES > 0);
        assert!(MAX_EDGE_TYPE_LEN > 0);
        assert!(MAX_NODE_TYPE_LEN > 0);
        assert!(MAX_DOCUMENT_ID_LEN > 0);
        assert!(MAX_DOCUMENT_SNAPSHOTS > 0);
        assert!(MAX_PINNING_SERVICE_LEN > 0);
        assert!(MIN_IPFS_REPLICAS > 0);
        assert!(MAX_IPFS_REPLICAS > 0);
    }

    #[test]
    fn test_seed_treasury_not_empty() {
        assert!(!SEED_TREASURY.is_empty());
    }
}
