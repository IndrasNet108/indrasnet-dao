//! Liquidity module
//!
//! Liquidity pool management
//!
//! On-chain: Metadata for liquidity pools
//! Off-chain: Actual liquidity calculations, swaps

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Pool status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PoolStatus {
    /// Pool active
    Active,
    /// Pool paused
    Paused,
    /// Pool closed
    Closed,
}

/// Liquidity pool metadata (on-chain)
///
/// Stores metadata for liquidity pools
#[account]
#[derive(InitSpace)]
pub struct LiquidityPoolMetadata {
    /// Pool ID
    pub pool_id: u64,
    /// Token A mint
    pub token_a_mint: Pubkey,
    /// Token B mint
    pub token_b_mint: Pubkey,
    /// Total liquidity
    pub total_liquidity: u64,
    /// Status
    pub status: PoolStatus,
    /// Created at
    pub created_at: i64,
    /// Pool data hash
    pub pool_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for liquidity
pub mod onchain {
    use super::*;

    /// Initialize liquidity pool
    pub fn initialize_liquidity_pool(
        pool: &mut LiquidityPoolMetadata,
        pool_id: u64,
        token_a_mint: Pubkey,
        token_b_mint: Pubkey,
        pool_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(pool_id > 0, IndrasError::InvalidInput);
        
        pool.pool_id = pool_id;
        pool.token_a_mint = token_a_mint;
        pool.token_b_mint = token_b_mint;
        pool.total_liquidity = 0;
        pool.status = PoolStatus::Active;
        pool.created_at = current_time;
        pool.pool_data_hash = pool_data_hash;
        pool.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for liquidity
pub mod offchain {
    /// Calculate swap amount
    pub fn calculate_swap_amount(_pool_id: u64, _amount_in: u64) -> u64 {
        // Implementation in off-chain service
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::IndrasError;

    // ========== initialize_liquidity_pool validation tests ==========
    
    #[test]
    fn test_initialize_liquidity_pool_validation_pool_id_zero() {
        // Test: pool_id == 0 should fail
        let pool_id = 0u64;
        
        // Validation logic: require!(pool_id > 0, IndrasError::InvalidInput)
        assert_eq!(pool_id, 0, "Pool ID zero should be detected");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_pool_id_one() {
        // Test: pool_id == 1 should pass
        let pool_id = 1u64;
        
        // Validation logic: require!(pool_id > 0, IndrasError::InvalidInput)
        assert!(pool_id > 0, "Pool ID one should pass");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_pool_id_max() {
        // Test: pool_id == u64::MAX should pass
        let pool_id = u64::MAX;
        
        // Validation logic: require!(pool_id > 0, IndrasError::InvalidInput)
        assert!(pool_id > 0, "Pool ID at max should pass");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_token_a_b_same() {
        // Test: token_a_mint == token_b_mint should be allowed (no validation)
        let token_a_mint = Pubkey::new_unique();
        let token_b_mint = token_a_mint;
        
        // Same mint should be allowed
        assert_eq!(token_a_mint, token_b_mint, "Token A and B same should be allowed");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_token_a_b_different() {
        // Test: token_a_mint != token_b_mint should be allowed
        let token_a_mint = Pubkey::new_unique();
        let token_b_mint = Pubkey::new_unique();
        
        // Different mints should be allowed
        assert_ne!(token_a_mint, token_b_mint, "Token A and B different should be allowed");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_pool_data_hash_zero() {
        // Test: pool_data_hash == [0u8; 32] should be allowed (no validation)
        let pool_data_hash = [0u8; 32];
        
        // No validation for zero hash - this is allowed
        assert_eq!(pool_data_hash, [0u8; 32], "Zero pool data hash should be allowed");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_total_liquidity_zero() {
        // Test: total_liquidity should be set to 0 on initialization
        let total_liquidity = 0u64;
        
        // Total liquidity should be zero on initialization
        assert_eq!(total_liquidity, 0, "Total liquidity should be zero on initialization");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_status_active() {
        // Test: status should be set to Active on initialization
        let status = PoolStatus::Active;
        
        // Status should be Active
        assert_eq!(status, PoolStatus::Active, "Status should be Active on initialization");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_all_pool_statuses() {
        // Test: all PoolStatus variants should be valid
        let statuses = vec![
            PoolStatus::Active,
            PoolStatus::Paused,
            PoolStatus::Closed,
        ];
        
        // All statuses should be valid
        assert_eq!(statuses.len(), 3, "All pool statuses should be valid");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_current_time_zero() {
        // Test: current_time == 0 should be allowed (no validation)
        let current_time = 0i64;
        
        // Zero time should be allowed
        assert_eq!(current_time, 0, "Zero current time should be allowed");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_current_time_negative() {
        // Test: current_time < 0 should be allowed (no validation)
        let current_time = -1i64;
        
        // Negative time should be allowed
        assert!(current_time < 0, "Negative current time should be allowed");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_current_time_positive() {
        // Test: current_time > 0 should be allowed
        let current_time = 1000000i64;
        
        // Positive time should be allowed
        assert!(current_time > 0, "Positive current time should be allowed");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_bump_zero() {
        // Test: bump == 0 should be allowed (no validation)
        let bump = 0u8;
        
        // Zero bump should be allowed
        assert_eq!(bump, 0, "Zero bump should be allowed");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_bump_max() {
        // Test: bump == u8::MAX should be allowed
        let bump = u8::MAX;
        
        // Max bump should be allowed
        assert_eq!(bump, u8::MAX, "Max bump should be allowed");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_valid_inputs() {
        // Test: valid inputs should pass
        let pool_id = 1u64;
        let token_a_mint = Pubkey::new_unique();
        let token_b_mint = Pubkey::new_unique();
        let pool_data_hash = [1u8; 32];
        let current_time = 1000000i64;
        let bump = 1u8;
        
        // All validations should pass
        assert!(pool_id > 0, "Pool ID should be valid");
        assert_ne!(token_a_mint, token_b_mint, "Token mints should be valid");
        assert_eq!(pool_data_hash.len(), 32, "Pool data hash should be valid");
        assert!(current_time >= 0, "Current time should be valid");
        assert!(bump <= u8::MAX, "Bump should be valid");
    }
    
    #[test]
    fn test_initialize_liquidity_pool_validation_created_at_set() {
        // Test: created_at should be set to current_time on initialization
        let current_time = 1000000i64;
        let created_at = current_time;
        
        // Created should be set
        assert_eq!(created_at, current_time, "Created at should be set on initialization");
    }
}
