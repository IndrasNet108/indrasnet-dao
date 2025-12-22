//! Notifications module
//!
//! Partnership notifications management
//!
//! On-chain: Metadata for notifications
//! Off-chain: Actual notification delivery

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Notification status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum NotificationStatus {
    /// Notification pending
    Pending,
    /// Notification sent
    Sent,
    /// Notification failed
    Failed,
}

/// Notification type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum NotificationType {
    /// Email notification
    Email,
    /// SMS notification
    SMS,
    /// Push notification
    Push,
    /// In-app notification
    InApp,
}

/// Partnership notification metadata (on-chain)
///
/// Stores metadata for partnership notifications
#[account]
#[derive(InitSpace)]
pub struct PartnershipNotificationMetadata {
    /// Notification ID
    pub notification_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Notification type
    pub notification_type: NotificationType,
    /// Status
    pub status: NotificationStatus,
    /// Created at
    pub created_at: i64,
    /// Sent at
    pub sent_at: Option<i64>,
    /// Notification data hash
    pub notification_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for notifications
pub mod onchain {
    use super::*;

    /// Initialize partnership notification
    pub fn initialize_partnership_notification(
        notification: &mut PartnershipNotificationMetadata,
        notification_id: u64,
        partnership_id: u64,
        notification_type: NotificationType,
        notification_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(notification_id > 0, IndrasError::InvalidInput);
        
        notification.notification_id = notification_id;
        notification.partnership_id = partnership_id;
        notification.notification_type = notification_type;
        notification.status = NotificationStatus::Pending;
        notification.created_at = current_time;
        notification.sent_at = None;
        notification.notification_data_hash = notification_data_hash;
        notification.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for notifications
pub mod offchain {
    /// Send notification
    pub fn send_notification(_notification_id: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}
