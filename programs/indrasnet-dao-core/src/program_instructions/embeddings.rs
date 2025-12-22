    // ===== EMBEDDING UPDATE INSTRUCTIONS =====
    
    /// Update idea embedding
    ///
    /// Updates the embedding for an idea with signature verification.
    pub fn update_idea_embedding(
        ctx: Context<UpdateIdeaEmbedding>,
        idea_id: u64,
        embedding_hash: [u8; 32],
        embedding_signature: [u8; 64],
        embedding_provider: String,
        embedding_model: Option<String>,
        embedding_model_version: Option<String>,
        embedding_provider_pubkey: Pubkey,
    ) -> Result<()> {
        instructions::update_idea_embedding_handler(
            ctx,
            idea_id,
            embedding_hash,
            embedding_signature,
            embedding_provider,
            embedding_model,
            embedding_model_version,
            embedding_provider_pubkey,
        )
    }
    
    /// Update mesh group embedding
    ///
    /// Updates the embedding for a mesh group with signature verification.
    pub fn update_mesh_group_embedding(
        ctx: Context<UpdateMeshGroupEmbedding>,
        mesh_group_id: u64,
        embedding_hash: [u8; 32],
        embedding_signature: [u8; 64],
        embedding_provider: String,
        embedding_model: Option<String>,
        embedding_model_version: Option<String>,
        embedding_provider_pubkey: Pubkey,
    ) -> Result<()> {
        instructions::update_mesh_group_embedding_handler(
            ctx,
            mesh_group_id,
            embedding_hash,
            embedding_signature,
            embedding_provider,
            embedding_model,
            embedding_model_version,
            embedding_provider_pubkey,
        )
    }
