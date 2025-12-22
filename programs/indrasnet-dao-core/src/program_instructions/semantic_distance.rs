    // ===== SEMANTIC DISTANCE INSTRUCTIONS (Track B) =====
    
    /// Verify semantic distance bundle (Track B: B2)
    ///
    /// Verifies a signed distance bundle from off-chain semantic similarity service.
    /// This handler verifies the bundle signature and hash before accepting the distance.
    pub fn verify_semantic_distance(
        ctx: Context<VerifySemanticDistance>,
        bundle: crate::state::grant::semantic::SemanticDistanceBundle,
    ) -> Result<()> {
        instructions::verify_semantic_distance_handler(ctx, bundle)
    }
