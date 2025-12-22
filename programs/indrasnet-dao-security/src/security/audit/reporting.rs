//! Audit reporting and statistics
//!
//! This module provides audit statistics and reporting functionality.
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-security/src/security/audit/reporting.rs

use anchor_lang::prelude::*;
use super::types::*;
use super::operations::AuditManager;

/// Audit reporting operations
impl AuditManager {
    /// Get audit statistics
    pub fn get_audit_statistics(
        events: &[AuditEvent],
        time_range: (i64, i64),
    ) -> Result<AuditStatistics> {
        let filtered_events: Vec<&AuditEvent> = events.iter()
            .filter(|e| e.timestamp >= time_range.0 && e.timestamp <= time_range.1)
            .collect();
        
        let total_events = filtered_events.len();
        let critical_events = filtered_events.iter()
            .filter(|e| e.severity == AuditSeverity::Critical)
            .count();
        let high_events = filtered_events.iter()
            .filter(|e| e.severity == AuditSeverity::High)
            .count();
        let medium_events = filtered_events.iter()
            .filter(|e| e.severity == AuditSeverity::Medium)
            .count();
        let low_events = filtered_events.iter()
            .filter(|e| e.severity == AuditSeverity::Low)
            .count();
        
        let pending_events = filtered_events.iter()
            .filter(|e| e.status == AuditEventStatus::Pending)
            .count();
        let resolved_events = filtered_events.iter()
            .filter(|e| e.status == AuditEventStatus::Resolved)
            .count();
        
        Ok(AuditStatistics {
            total_events,
            critical_events,
            high_events,
            medium_events,
            low_events,
            pending_events,
            resolved_events,
            time_range,
        })
    }
}
