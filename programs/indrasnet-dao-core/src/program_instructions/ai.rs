    // ===== AI ANALYSIS INSTRUCTIONS (from indrasnet-dao-ai) =====
    
    /// Analyze idea for compliance with DAO norms
    ///
    /// Checks idea for compliance with DAO norms:
    /// - Ethical requirements (ethics_compliance_score >= 50)
    /// - Legal requirements (legal_compliance_score >= 50)
    /// - Innovation (impact_score >= 70, feasibility_score >= 70)
    /// - Uniqueness (uniqueness_score >= 70) - CRITICAL!
    /// - Feasibility (feasibility_score >= 70 with artifacts)
    ///
    /// NOTE: Real AI analysis happens off-chain (Gemini API).
    /// This handler records analysis results on-chain.
    pub fn analyze_idea(
        ctx: Context<AnalyzeIdea>,
        idea_id: u64,
        charter_compliance_score: u8,
        governance_compliance_score: u8,
        ethics_compliance_score: u8,
        legal_compliance_score: u8,
        technical_feasibility_score: u8,
        uniqueness_score: u8,
        impact_score: u8,
        feasibility_score: u8,
        artifacts_hash: Option<[u8; 32]>,
        artifacts_verified: bool,
        detailed_feedback: String,
        checked_governance_docs: Vec<u64>,
        checked_legal_frameworks: Vec<u64>,
        checked_ethics_frameworks: Vec<u64>,
        // REQUIRED fields (CRITICAL for verification and audit)
        input_hash: [u8; 32],
        output_hash: [u8; 32],
        model_id: String,
        model_version: String,
        // SEC-INV-7: Model hash (optional for MVP, required if ModelRegistry provided)
        model_hash: Option<[u8; 32]>,
        analyzer_pubkey: Pubkey,
        analyzer_signature: [u8; 64],
        // Replay attack protection (REQUIRED)
        nonce: u64,
        expires_at: Option<i64>,
        // OPTIONAL fields (for backward compatibility)
        thresholds_profile: Option<u8>,
    ) -> Result<()> {
        instructions::analyze_idea_handler(
            ctx,
            idea_id,
            charter_compliance_score,
            governance_compliance_score,
            ethics_compliance_score,
            legal_compliance_score,
            technical_feasibility_score,
            uniqueness_score,
            impact_score,
            feasibility_score,
            artifacts_hash,
            artifacts_verified,
            detailed_feedback,
            checked_governance_docs,
            checked_legal_frameworks,
            checked_ethics_frameworks,
            input_hash,
            output_hash,
            model_id,
            model_version,
            model_hash,  // SEC-INV-7
            analyzer_pubkey,
            analyzer_signature,
            nonce,
            expires_at,
            thresholds_profile,
        )
    }
    
    /// Update idea status from AI analysis
    ///
    /// Updates idea status based on AI analysis results.
    pub fn update_idea_status_from_analysis(
        ctx: Context<UpdateIdeaStatusFromAnalysis>,
        idea_id: u64,
    ) -> Result<()> {
        instructions::update_idea_status_from_analysis_handler(ctx, idea_id)
    }
