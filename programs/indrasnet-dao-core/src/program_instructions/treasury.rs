    // ===== TREASURY INSTRUCTIONS =====
    
    /// Initialize treasury
    pub fn initialize_treasury(
        ctx: Context<InitializeTreasury>,
        treasury_name: String,
    ) -> Result<()> {
        instructions::initialize_treasury_handler(ctx, treasury_name)
    }
    
    /// Deposit to treasury
    pub fn deposit_to_treasury(
        ctx: Context<DepositToTreasury>,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit_to_treasury_handler(ctx, amount)
    }
    
    /// Withdraw from treasury with capability
    pub fn withdraw_treasury_with_capability(
        ctx: Context<WithdrawTreasuryWithCapability>,
        amount: u64,
    ) -> Result<()> {
        instructions::withdraw_treasury_with_capability_handler(ctx, amount)
    }
    
    /// Grant capability for treasury operations
    pub fn grant_capability(
        ctx: Context<GrantCapability>,
        grantee: Pubkey,
        capability_type: String,
        expires_at: i64,
    ) -> Result<()> {
        instructions::grant_capability_handler(ctx, grantee, capability_type, expires_at)
    }
    
    /// Revoke capability
    pub fn revoke_capability(
        ctx: Context<RevokeCapability>,
    ) -> Result<()> {
        instructions::revoke_capability_handler(ctx)
    }
