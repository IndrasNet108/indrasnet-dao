// Accounts structures for semantic distance instructions (Track B: B2)
// NOTE: Types like DaoConfig and anchor_lang types are already imported in lib.rs before include!()
// AccountInfo, Account, Signer, etc. are available via lib.rs imports

/// Accounts for verifying semantic distance bundle
#[derive(Accounts)]
pub struct VerifySemanticDistance<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    /// SECURITY: Verifier must be DAO authority or authorized provider
    /// Note: Full provider verification is done in handler (requires deserialization)
    /// CHECK: In production, should verify provider is in AIServiceRegistry
    pub verifier: Signer<'info>,
    
    /// AI Service Registry (optional - for provider verification)
    /// CHECK: If provided, provider must be registered and active
    /// NOTE: Using UncheckedAccount to avoid lifetime issues in Anchor 0.32.1
    pub ai_service_registry: Option<UncheckedAccount<'info>>,
    
    /// Source Idea account (optional - for full binding verification)
    /// CHECK: If provided, must have embedding_hash set
    /// NOTE: Using AccountInfo to reduce stack size (BPF limit)
    pub source_idea: Option<AccountInfo<'info>>,
    
    /// Target Idea account (optional - for full binding verification)
    /// CHECK: If provided, must have embedding_hash set
    /// NOTE: Using AccountInfo to reduce stack size (BPF limit)
    pub target_idea: Option<AccountInfo<'info>>,
    
    /// Source MeshGroup account (optional - for full binding verification)
    /// CHECK: If provided, must have embedding_hash set
    /// NOTE: Using AccountInfo to reduce stack size (BPF limit)
    pub source_mesh_group: Option<AccountInfo<'info>>,
    
    /// Target MeshGroup account (optional - for full binding verification)
    /// CHECK: If provided, must have embedding_hash set
    /// NOTE: Using AccountInfo to reduce stack size (BPF limit)
    pub target_mesh_group: Option<AccountInfo<'info>>,
}
