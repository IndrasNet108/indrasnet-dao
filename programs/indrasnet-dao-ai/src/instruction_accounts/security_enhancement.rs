// Account structures for Security Enhancement instructions


/// Account structure for enhancing security analysis via CPI
///
/// Called by Security module to get AI-enhanced threat interpretation
#[derive(Accounts)]
pub struct EnhanceSecurityAnalysis<'info> {
    /// CHECK: Caller (Security program or authorized signer)
    /// For CPI calls, this will be the Security program
    pub caller: Signer<'info>,
    
    /// CHECK: System program (not used, but required for CPI)
    pub system_program: Program<'info, System>,
}
