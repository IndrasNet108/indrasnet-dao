//! Communication module
//!
//! Partnership communication management
//!
//! On-chain: Metadata for communications
//! Off-chain: Actual communication delivery, messaging

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Communication type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum CommunicationType {
    /// Email
    Email,
    /// Message
    Message,
    /// Meeting
    Meeting,
    /// Call
    Call,
}

/// Communication status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum CommunicationStatus {
    /// Communication pending
    Pending,
    /// Communication sent
    Sent,
    /// Communication delivered
    Delivered,
    /// Communication failed
    Failed,
}

/// Partnership communication metadata (on-chain)
///
/// Stores metadata for partnership communications
#[account]
#[derive(InitSpace)]
pub struct PartnershipCommunicationMetadata {
    /// Communication ID
    pub communication_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Communication type
    pub communication_type: CommunicationType,
    /// Status
    pub status: CommunicationStatus,
    /// Created at
    pub created_at: i64,
    /// Delivered at
    pub delivered_at: Option<i64>,
    /// Communication data hash
    pub communication_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for communication
pub mod onchain {
    use super::*;

    /// Initialize partnership communication
    pub fn initialize_partnership_communication(
        communication: &mut PartnershipCommunicationMetadata,
        communication_id: u64,
        partnership_id: u64,
        communication_type: CommunicationType,
        communication_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(communication_id > 0, IndrasError::InvalidInput);
        
        communication.communication_id = communication_id;
        communication.partnership_id = partnership_id;
        communication.communication_type = communication_type;
        communication.status = CommunicationStatus::Pending;
        communication.created_at = current_time;
        communication.delivered_at = None;
        communication.communication_data_hash = communication_data_hash;
        communication.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for communication
pub mod offchain {
    /// Send communication
    pub fn send_communication(_communication_id: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}
