    // ===== GOVERNANCE INSTRUCTIONS =====
    
    /// Initialize DAO
    pub fn initialize_dao(
        ctx: Context<InitializeDao>,
        name: String,
        description: String,
    ) -> Result<()> {
        instructions::initialize_dao_handler(ctx, name, description)
    }
    
    /// Manage quorum for voting
    ///
    /// NOTE: It is recommended to use governance_params for adaptive governance,
    /// but this instruction is available for direct quorum management.
    pub fn manage_quorum(
        ctx: Context<ManageQuorum>,
        quorum_id: u64,
        required_votes: u64,
        quorum_threshold: u64,
    ) -> Result<()> {
        instructions::manage_quorum_handler(ctx, quorum_id, required_votes, quorum_threshold)
    }
    
    /// Initialize governance parameters
    ///
    /// Initializes adaptive governance parameters.
    pub fn initialize_governance_params(
        ctx: Context<InitializeGovernanceParams>,
        quorum_percentage: u8,
        vote_duration_hours: u64,
        delegate_weight_percentage: u8,
        early_quorum_enabled: bool,
    ) -> Result<()> {
        instructions::initialize_governance_params_handler(ctx, quorum_percentage, vote_duration_hours, delegate_weight_percentage, early_quorum_enabled)
    }
    
    /// Update governance parameters (adaptive governance)
    ///
    /// Updates adaptive governance parameters:
    /// - quorum_percentage - quorum percentage (0-100)
    /// - vote_duration_hours - voting duration in hours
    /// - delegate_weight_percentage - delegate weight (0-100)
    /// - early_quorum_enabled - early quorum enabled
    ///
    /// Constraints:
    /// - Maximum quorum_percentage change per update: ±10%
    /// - vote_duration_hours: 24-720 hours (1 day - 30 days)
    /// - delegate_weight_percentage: 0-100%
    pub fn update_governance_params(
        ctx: Context<UpdateGovernanceParams>,
        quorum_percentage: Option<u8>,
        vote_duration_hours: Option<u64>,
        delegate_weight_percentage: Option<u8>,
        early_quorum_enabled: Option<bool>,
    ) -> Result<()> {
        instructions::update_governance_params_handler(
            ctx,
            quorum_percentage,
            vote_duration_hours,
            delegate_weight_percentage,
            early_quorum_enabled,
        )
    }
