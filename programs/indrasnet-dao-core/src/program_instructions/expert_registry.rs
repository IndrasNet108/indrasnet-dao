    // ===== EXPERT REGISTRY INSTRUCTIONS =====
    
    /// Initialize expert registry
    pub fn initialize_expert_registry(
    ctx: Context<InitializeExpertRegistry>,
    ) -> Result<()> {
    instructions::initialize_expert_registry_handler(ctx)
    }
    
    /// Add expert to registry for a semantic domain
    pub fn add_expert(
    ctx: Context<AddExpert>,
    domain_id: String,
    competency_level: crate::state::grant::semantic::CompetencyLevel,
    confidence: u8,
    source: crate::state::grant::semantic::CompetencySource,
    ) -> Result<()> {
    instructions::add_expert_handler(ctx, domain_id, competency_level, confidence, source)
    }
    
    /// Remove expert from registry
    pub fn remove_expert(
    ctx: Context<RemoveExpert>,
    domain_id: String,
    ) -> Result<()> {
    instructions::remove_expert_handler(ctx, domain_id)
    }
    
    /// Update expert entry (e.g., update confidence or competency level)
    pub fn update_expert(
    ctx: Context<UpdateExpert>,
    new_confidence: Option<u8>,
    new_competency_level: Option<crate::state::grant::semantic::CompetencyLevel>,
    ) -> Result<()> {
    instructions::update_expert_handler(ctx, new_confidence, new_competency_level)
    }
