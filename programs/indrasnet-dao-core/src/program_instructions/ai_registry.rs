    // ===== AI REGISTRY INSTRUCTIONS =====
    
    /// Initialize AI Service Registry
    pub fn initialize_ai_service_registry(
        ctx: Context<InitializeAiServiceRegistry>,
    ) -> Result<()> {
        instructions::initialize_ai_service_registry_handler(ctx)
    }
    
    /// Add AI service to registry
    pub fn add_ai_service(
        ctx: Context<AddAiService>,
        service_pubkey: Pubkey,
        model_ids: Vec<String>,
    ) -> Result<()> {
        instructions::add_ai_service_handler(ctx, service_pubkey, model_ids)
    }
