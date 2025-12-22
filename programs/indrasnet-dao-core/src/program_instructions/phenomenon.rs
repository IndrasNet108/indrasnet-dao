    // ===== PHENOMENON INSTRUCTIONS (Track B) =====
    
    /// Create a new phenomenon
    ///
    /// SEC-INV-10: All phenomena MUST have cryptographically signed embeddings.
    /// This handler verifies the embedding signature via CPI to ed25519_program.
    pub fn create_phenomenon(
        ctx: Context<CreatePhenomenon>,
        phenomenon_id: u64,
        name: String,
        metadata_uri: String,
        related_ideas: Vec<Pubkey>,
        // Track B: Clustering metadata
        similarity_score: f32,
        clustering_proof: [u8; 32],
        discovery_method: crate::state::DiscoveryMethod,
        // Track B: Embedding signature (SEC-INV-10) - REQUIRED
        embedding_hash: [u8; 32],
        embedding_signature: [u8; 64],
        embedding_provider: String,
        embedding_provider_pubkey: Pubkey,
        // Track B: Governance
        proposal_id: Option<u64>,
        // Track B: Network effects
        associated_mesh_groups: Vec<Pubkey>,
        grant_priority: u8,
        // Track B: DBSCAN validation parameters (B3)
        cluster_hash: [u8; 32],
        cluster_signature: [u8; 64],
        temporal_window_days: u8,
        author_overlap_count: u8,
        // Track B: DBSCAN parameters
        eps: f32,  // Similarity threshold (default: 0.7)
        min_samples: u8,  // Minimum ideas per cluster (default: 3)
        // Track B: Verified semantic distances (from B2)
        verified_distances: Option<Vec<crate::state::grant::semantic::VerifiedDistance>>,
    ) -> Result<()> {
        instructions::create_phenomenon_handler(
            ctx,
            phenomenon_id,
            name,
            metadata_uri,
            related_ideas,
            similarity_score,
            clustering_proof,
            discovery_method,
            embedding_hash,
            embedding_signature,
            embedding_provider,
            embedding_provider_pubkey,
            proposal_id,
            associated_mesh_groups,
            grant_priority,
            cluster_hash,
            cluster_signature,
            temporal_window_days,
            author_overlap_count,
            eps,
            min_samples,
            verified_distances,
        )
    }
