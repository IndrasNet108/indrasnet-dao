//! Permissions module
//!
//! Partnership permissions management
//!
//! On-chain: Metadata for permissions, access control
//! Off-chain: Actual permission checking, enforcement

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Permission type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PermissionType {
    /// Read permission
    Read,
    /// Write permission
    Write,
    /// Admin permission
    Admin,
    /// Custom permission
    Custom,
}

/// Partnership permission metadata (on-chain)
///
/// Stores metadata for partnership permissions
#[account]
#[derive(InitSpace)]
pub struct PartnershipPermissionMetadata {
    /// Permission ID
    pub permission_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// User pubkey
    pub user_pubkey: Pubkey,
    /// Permission type
    pub permission_type: PermissionType,
    /// Created at
    pub created_at: i64,
    /// Permission data hash
    pub permission_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for permissions
pub mod onchain {
    use super::*;

    /// Initialize partnership permission
    pub fn initialize_partnership_permission(
        permission: &mut PartnershipPermissionMetadata,
        permission_id: u64,
        partnership_id: u64,
        user_pubkey: Pubkey,
        permission_type: PermissionType,
        permission_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(permission_id > 0, IndrasError::InvalidInput);
        
        permission.permission_id = permission_id;
        permission.partnership_id = partnership_id;
        permission.user_pubkey = user_pubkey;
        permission.permission_type = permission_type;
        permission.created_at = current_time;
        permission.permission_data_hash = permission_data_hash;
        permission.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for permissions
pub mod offchain {
    /// Check permission
    pub fn check_permission(_permission_id: u64, _action: &str) -> bool {
        // Implementation in off-chain service
        false
    }
}
