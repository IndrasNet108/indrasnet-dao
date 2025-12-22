//! Portals module
//!
//! Partnership portal management
//!
//! On-chain: Metadata for portals
//! Off-chain: Actual portal rendering, access control

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Portal status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PortalStatus {
    /// Portal active
    Active,
    /// Portal inactive
    Inactive,
    /// Portal maintenance
    Maintenance,
}

/// Partnership portal metadata (on-chain)
///
/// Stores metadata for partnership portals
#[account]
#[derive(InitSpace)]
pub struct PartnershipPortalMetadata {
    /// Portal ID
    pub portal_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Portal name
    #[max_len(100)]
    pub name: String,
    /// Status
    pub status: PortalStatus,
    /// Created at
    pub created_at: i64,
    /// Portal config hash
    pub portal_config_hash: [u8; 32],
    /// Portal URI
    #[max_len(200)]
    pub portal_uri: String,
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for portals
pub mod onchain {
    use super::*;

    /// Initialize partnership portal
    pub fn initialize_partnership_portal(
        portal: &mut PartnershipPortalMetadata,
        portal_id: u64,
        partnership_id: u64,
        name: String,
        portal_config_hash: [u8; 32],
        portal_uri: String,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(portal_id > 0, IndrasError::InvalidInput);
        require!(!name.is_empty(), IndrasError::InvalidInput);
        require!(name.len() <= 100, IndrasError::InvalidInput);
        require!(portal_uri.len() <= 200, IndrasError::InvalidInput);
        
        portal.portal_id = portal_id;
        portal.partnership_id = partnership_id;
        portal.name = name;
        portal.status = PortalStatus::Active;
        portal.created_at = current_time;
        portal.portal_config_hash = portal_config_hash;
        portal.portal_uri = portal_uri;
        portal.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for portals
pub mod offchain {
    /// Render portal
    pub fn render_portal(_portal_id: u64) -> Vec<u8> {
        // Implementation in off-chain service
        vec![]
    }
}
