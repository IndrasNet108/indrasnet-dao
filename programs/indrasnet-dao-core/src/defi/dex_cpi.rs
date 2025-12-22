//! DEX CPI (Cross-Program Invocation) module
//!
//! DeFi DEX operations via CPI
//!
//! On-chain: Metadata for DEX operations, state management
//! Off-chain: Actual DEX interactions, trade execution (in separate service)

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// DEX operation type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DEXOperationType {
    /// Swap operation
    Swap,
    /// Add liquidity
    AddLiquidity,
    /// Remove liquidity
    RemoveLiquidity,
    /// Stake
    Stake,
    /// Unstake
    Unstake,
}

/// DEX operation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DEXOperationStatus {
    /// Operation pending
    Pending,
    /// Operation in progress
    InProgress,
    /// Operation completed
    Completed,
    /// Operation failed
    Failed,
}

/// DEX operation metadata (on-chain)
///
/// Stores metadata for DEX operations via CPI
#[account]
#[derive(InitSpace)]
pub struct DEXOperationMetadata {
    /// Operation ID
    pub operation_id: u64,
    /// Operation type
    pub operation_type: DEXOperationType,
    /// Status
    pub status: DEXOperationStatus,
    /// DEX program ID
    pub dex_program_id: Pubkey,
    /// Created at
    pub created_at: i64,
    /// Completed at
    pub completed_at: Option<i64>,
    /// Operation data hash
    pub operation_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

impl DEXOperationMetadata {
    /// Update operation status
    pub fn update_status(&mut self, new_status: DEXOperationStatus, current_time: i64) {
        self.status = new_status;
        
        if new_status == DEXOperationStatus::Completed || new_status == DEXOperationStatus::Failed {
            self.completed_at = Some(current_time);
        }
    }
}

/// On-chain functions for DEX CPI
pub mod onchain {
    use super::*;

    /// Initialize DEX operation
    pub fn initialize_dex_operation(
        operation: &mut DEXOperationMetadata,
        operation_id: u64,
        operation_type: DEXOperationType,
        dex_program_id: Pubkey,
        operation_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(operation_id > 0, IndrasError::InvalidInput);
        
        operation.operation_id = operation_id;
        operation.operation_type = operation_type;
        operation.status = DEXOperationStatus::Pending;
        operation.dex_program_id = dex_program_id;
        operation.created_at = current_time;
        operation.completed_at = None;
        operation.operation_data_hash = operation_data_hash;
        operation.bump = bump;
        
        Ok(())
    }

    /// Update DEX operation status
    pub fn update_dex_operation_status(
        operation: &mut DEXOperationMetadata,
        new_status: DEXOperationStatus,
        current_time: i64,
    ) -> Result<()> {
        operation.update_status(new_status, current_time);
        Ok(())
    }
}

/// Off-chain functions for DEX CPI
///
/// These functions should be implemented in off-chain service
/// for actual DEX interactions.
pub mod offchain {
    // Off-chain functions will be implemented in separate service
    
    /// Execute DEX swap
    pub fn execute_swap(_operation_id: u64, _amount_in: u64, _min_amount_out: u64) -> bool {
        // Implementation in off-chain service
        // Executes swap via DEX CPI
        false
    }

    /// Execute add liquidity
    pub fn execute_add_liquidity(_operation_id: u64, _amount_a: u64, _amount_b: u64) -> bool {
        // Implementation in off-chain service
        // Executes add liquidity via DEX CPI
        false
    }
}
