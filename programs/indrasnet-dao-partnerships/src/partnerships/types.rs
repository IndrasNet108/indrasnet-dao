//! Partnership types and structures
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-partnerships/src/partnerships/types.rs

use anchor_lang::prelude::*;

// ============================================================================
// ON-CHAIN STRUCTURES (Anchor Types)
// ============================================================================

/// Partnership types
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum PartnershipType {
    Strategic,      // Long-term strategic partnership
    Technical,      // Technical integration partnership
    Marketing,      // Marketing and promotion partnership
    Financial,      // Financial investment partnership
    Governance,     // Governance and voting partnership
    DeFi,          // DeFi protocol integration
    Oracle,        // Oracle data partnership
    Infrastructure, // Infrastructure and tooling partnership
}

impl Space for PartnershipType {
    const INIT_SPACE: usize = 1;
}

/// Partnership tiers
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum PartnershipTier {
    Basic,      // Basic partnership level
    Standard,   // Standard partnership level
    Premium,    // Premium partnership level
    Strategic,  // Strategic partnership level
    Enterprise, // Enterprise partnership level
}

impl Space for PartnershipTier {
    const INIT_SPACE: usize = 1;
}

/// Partnership status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum PartnershipStatus {
    Draft,          // Partnership in draft state
    Pending,        // Awaiting approval
    Active,         // Active partnership
    Paused,         // Temporarily paused
    Completed,      // Successfully completed
    Terminated,     // Terminated by either party
    Expired,        // Expired due to time
}

impl Space for PartnershipStatus {
    const INIT_SPACE: usize = 1;
}

/// Milestone status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum MilestoneStatus {
    NotStarted,     // Milestone not yet started
    InProgress,     // Milestone in progress
    Completed,      // Milestone completed
    Delayed,        // Milestone delayed
    Cancelled,      // Milestone cancelled
}

impl Space for MilestoneStatus {
    const INIT_SPACE: usize = 1;
}

/// Partnership terms
/// 
/// ON-CHAIN: Stores partnership terms metadata
/// OFF-CHAIN: Detailed terms and legal documents stored off-chain (IPFS)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct PartnershipTerms {
    pub start_date: i64,                    // Partnership start date
    pub end_date: Option<i64>,              // Partnership end date (None for indefinite)
    pub estimated_value: u64,               // Estimated partnership value in lamports
    pub revenue_share: u64,                 // Revenue share percentage (in fixed-point)
    pub governance_power: u64,              // Governance voting power (in fixed-point)
    pub technical_requirements: Vec<String>, // Technical requirements
    pub legal_requirements: Vec<String>,    // Legal requirements
    pub success_metrics: Vec<String>,       // Success measurement criteria
}

impl Space for PartnershipTerms {
    const INIT_SPACE: usize = 8 + 8 + 8 + 8 + 8 + 4 + 4 + 4; // Basic fields + Vec overhead
}

/// Partnership milestone
/// 
/// ON-CHAIN: Stores milestone metadata
/// OFF-CHAIN: Detailed milestone documentation stored off-chain
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct PartnershipMilestone {
    pub id: u64,                            // Milestone ID
    pub title: String,                      // Milestone title
    pub description: String,                // Milestone description
    pub due_date: i64,                      // Due date
    pub status: MilestoneStatus,            // Current status
    pub value: u64,                         // Milestone value in lamports
    pub completion_criteria: Vec<String>,   // Completion criteria
}

impl Space for PartnershipMilestone {
    const INIT_SPACE: usize = 8 + 4 + 4 + 8 + 1 + 8 + 4; // Basic fields + Vec overhead
}

/// Partnership metrics
/// 
/// ON-CHAIN: Stores aggregated metrics
/// OFF-CHAIN: Detailed analytics and reporting happen off-chain
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct PartnershipMetrics {
    pub performance_score: u64,             // Performance score (0 to PRECISION_FACTOR)
    pub risk_score: u64,                    // Risk score (0 to PRECISION_FACTOR)
    pub revenue_generated: u64,             // Total revenue generated
    pub milestones_completed: u64,          // Number of completed milestones
    pub total_milestones: u64,              // Total number of milestones
    pub last_activity: i64,                 // Last activity timestamp
    pub health_score: u64,                  // Overall health score
}

impl Space for PartnershipMetrics {
    const INIT_SPACE: usize = 8 + 8 + 8 + 8 + 8 + 8 + 8;
}

/// Main partnership structure
/// 
/// ON-CHAIN: Stores partnership metadata and state
/// OFF-CHAIN: Detailed partnership documents, analytics, reporting happen off-chain
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct Partnership {
    pub id: u64,                            // Unique partnership ID
    pub partner_name: String,               // Partner organization name
    pub partner_type: PartnershipType,      // Type of partnership
    pub description: String,                // Partnership description
    pub terms: PartnershipTerms,            // Partnership terms
    pub status: PartnershipStatus,          // Current status
    pub milestones: Vec<PartnershipMilestone>, // Partnership milestones
    pub created_at: i64,                    // Creation timestamp
    pub updated_at: i64,                    // Last update timestamp
    pub version: u64,                       // Version number for updates
}

impl Space for Partnership {
    const INIT_SPACE: usize = 8 + 4 + 1 + 4 + 8 + 1 + 4 + 8 + 8 + 8; // Basic fields + Vec overhead
}

/// Partnership event types
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum PartnershipEventType {
    Created,        // Partnership created
    StatusChanged,  // Status changed
    MilestoneAdded, // Milestone added
    MilestoneCompleted, // Milestone completed
    TermsUpdated,   // Terms updated
    ValueCalculated, // Value calculated
    HealthUpdated,  // Health score updated
    Expired,        // Partnership expired
    Terminated,     // Partnership terminated
}

impl Space for PartnershipEventType {
    const INIT_SPACE: usize = 1;
}

/// Partnership event
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct PartnershipEvent {
    pub partnership_id: u64,                // Partnership ID
    pub event_type: PartnershipEventType,   // Event type
    pub timestamp: i64,                     // Event timestamp
    pub data: String,                       // Event data
    pub version: u64,                       // Event version
}

impl Space for PartnershipEvent {
    const INIT_SPACE: usize = 8 + 1 + 8 + 4 + 8;
}

/// Partnership configuration
/// 
/// ON-CHAIN: Stores system configuration
/// OFF-CHAIN: Configuration management and updates happen off-chain
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct PartnershipConfig {
    pub max_partnerships: u64,              // Maximum number of partnerships
    pub min_partnership_value: u64,         // Minimum partnership value
    pub max_partnership_value: u64,         // Maximum partnership value
    pub default_revenue_share: u64,         // Default revenue share percentage
    pub governance_threshold: u64,          // Governance power threshold
    pub auto_renewal_enabled: bool,         // Auto-renewal enabled
    pub risk_threshold: u64,                // Risk threshold for warnings
}

impl Space for PartnershipConfig {
    const INIT_SPACE: usize = 8 + 8 + 8 + 8 + 8 + 1 + 8;
}

impl Default for PartnershipConfig {
    fn default() -> Self {
        Self {
            max_partnerships: 100,
            min_partnership_value: 1_000_000_000, // 1 SOL
            max_partnership_value: 1_000_000_000_000, // 1000 SOL
            default_revenue_share: 100_000, // 10% (using PRECISION_FACTOR = 1_000_000)
            governance_threshold: 1_000_000, // 100%
            auto_renewal_enabled: true,
            risk_threshold: 700_000, // 70%
        }
    }
}

use anchor_lang::prelude::event;

/// Partnership proposal created event
#[event]
#[derive(Clone, Debug)]
pub struct PartnershipProposalCreated {
    pub proposal_id: u64,
    pub partnership_type: PartnershipType,
    pub partnership_tier: PartnershipTier,
    pub proposer: Pubkey,
    pub partner_organization: String,
    pub created_at: i64,
    pub description: String,
}
