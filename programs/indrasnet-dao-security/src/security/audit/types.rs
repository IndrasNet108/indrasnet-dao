//! Audit types and data structures
//!
//! This module contains all type definitions for the audit system.
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-security/src/security/audit/types.rs

use anchor_lang::prelude::*;

/// Options for creating audit events (optional fields for additional context)
#[derive(Default)]
pub struct AuditEventOptions {
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Vec<String>,
}

/// Audit event types
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum AuditEventType {
    /// User authentication events
    Authentication,
    /// Authorization and access control events
    Authorization,
    /// Data access events
    DataAccess,
    /// Data modification events
    DataModification,
    /// System configuration changes
    Configuration,
    /// Security events
    Security,
    /// Governance events
    Governance,
    /// Treasury events
    Treasury,
    /// AI operations
    AI,
    /// Partnership events
    Partnership,
    /// Error and exception events
    Error,
    /// System events
    System,
}

impl anchor_lang::Space for AuditEventType {
    const INIT_SPACE: usize = 1;
}

/// Audit event severity levels
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum AuditSeverity {
    /// Low severity - informational
    Low,
    /// Medium severity - warning
    Medium,
    /// High severity - error
    High,
    /// Critical severity - security breach
    Critical,
}

impl anchor_lang::Space for AuditSeverity {
    const INIT_SPACE: usize = 1;
}

/// Audit event status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum AuditEventStatus {
    /// Event is pending review
    Pending,
    /// Event is under investigation
    Investigating,
    /// Event has been resolved
    Resolved,
    /// Event has been dismissed
    Dismissed,
    /// Event requires action
    ActionRequired,
}

/// Audit event structure
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct AuditEvent {
    /// Event ID
    pub event_id: u64,
    /// Event type
    pub event_type: AuditEventType,
    /// Event severity
    pub severity: AuditSeverity,
    /// Event status
    pub status: AuditEventStatus,
    /// User who triggered the event
    pub user: Pubkey,
    /// Event description
    pub description: String,
    /// Event details
    pub details: String,
    /// Resource affected
    pub resource: String,
    /// Action performed
    pub action: String,
    /// Event timestamp
    pub timestamp: i64,
    /// Client IP (if available)
    pub client_ip: Option<String>,
    /// User agent (if available)
    pub user_agent: Option<String>,
    /// Additional metadata
    pub metadata: Vec<String>,
    /// Event hash for integrity
    pub event_hash: Vec<u8>,
}

/// Audit log configuration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct AuditLogConfig {
    /// Configuration ID
    pub config_id: u64,
    /// Maximum number of events to store
    pub max_events: u32,
    /// Retention period in days
    pub retention_days: u32,
    /// Events to log
    pub event_types: Vec<AuditEventType>,
    /// Minimum severity to log
    pub min_severity: AuditSeverity,
    /// Is logging enabled
    pub is_enabled: bool,
    /// Created timestamp
    pub created_at: i64,
    /// Updated timestamp
    pub updated_at: i64,
}

/// Audit trail structure
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct AuditTrail {
    /// Trail ID
    pub trail_id: u64,
    /// User being audited
    pub user: Pubkey,
    /// Events in the trail
    pub events: Vec<AuditEvent>,
    /// Trail start time
    pub start_time: i64,
    /// Trail end time
    pub end_time: Option<i64>,
    /// Is trail active
    pub is_active: bool,
    /// Created timestamp
    pub created_at: i64,
    /// Updated timestamp
    pub updated_at: i64,
}

/// Compliance rule structure
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct ComplianceRule {
    /// Rule ID
    pub rule_id: u64,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule conditions
    pub conditions: Vec<String>,
    /// Rule actions
    pub actions: Vec<String>,
    /// Rule severity
    pub severity: AuditSeverity,
    /// Is rule active
    pub is_active: bool,
    /// Created timestamp
    pub created_at: i64,
    /// Updated timestamp
    pub updated_at: i64,
}

/// Parameters for `create_audit_event`
pub struct AuditEventParams {
    pub event_type: AuditEventType,
    pub severity: AuditSeverity,
    pub user: Pubkey,
    pub description: String,
    pub details: String,
    pub resource: String,
    pub action: String,
    pub opts: AuditEventOptions,
}

/// Audit statistics structure
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct AuditStatistics {
    /// Total number of events
    pub total_events: usize,
    /// Number of critical events
    pub critical_events: usize,
    /// Number of high severity events
    pub high_events: usize,
    /// Number of medium severity events
    pub medium_events: usize,
    /// Number of low severity events
    pub low_events: usize,
    /// Number of pending events
    pub pending_events: usize,
    /// Number of resolved events
    pub resolved_events: usize,
    /// Time range for statistics
    pub time_range: (i64, i64),
}

/// Security constants for audit system
pub mod constants {
    pub const MAX_EVENT_DESCRIPTION_LENGTH: usize = 500;
    pub const MAX_EVENT_DETAILS_LENGTH: usize = 1000;
    pub const MAX_RESOURCE_LENGTH: usize = 200;
    pub const MAX_ACTION_LENGTH: usize = 100;
    pub const MAX_METADATA_ITEMS: usize = 20;
    pub const MAX_METADATA_ITEM_LENGTH: usize = 100;
    pub const MAX_RULE_NAME_LENGTH: usize = 100;
    pub const MAX_RULE_DESCRIPTION_LENGTH: usize = 500;
    pub const MAX_CONDITIONS: usize = 10;
    pub const MAX_ACTIONS: usize = 10;
    pub const MAX_EVENTS_PER_TRAIL: usize = 10000;
    pub const MAX_AUDIT_EVENTS: usize = 100000;
    pub const MAX_RETENTION_DAYS: u32 = 3650; // 10 years
    pub const MAX_EVENT_TYPES: usize = 20;
}
