//! Audit and Compliance System for IndrasNet DAO
//! 
//! This module provides comprehensive audit logging, compliance monitoring,
//! and security event tracking functionality.
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-security/src/security/audit/
//!
//! ## Module Structure
//!
//! - `types.rs` - All type definitions (enums, structs, constants)
//! - `operations.rs` - Core audit operations (event creation, config, trails)
//! - `compliance.rs` - Compliance rule management and checking
//! - `reporting.rs` - Audit statistics and reporting

pub mod types;
pub mod operations;
pub mod compliance;
pub mod reporting;

// Re-export types for external use
pub use types::*;
pub use operations::AuditManager;
// Note: compliance and reporting modules extend AuditManager via impl blocks
// They don't export separate types, so no re-exports needed

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;
    use anchor_lang::Space;

    fn create_test_audit_event_direct(
        event_id: u64,
        event_type: AuditEventType,
        severity: AuditSeverity,
        status: AuditEventStatus,
        user: Pubkey,
        description: String,
        details: String,
        resource: String,
        action: String,
        timestamp: i64,
        client_ip: Option<String>,
        user_agent: Option<String>,
        metadata: Vec<String>,
        event_hash: Vec<u8>,
    ) -> AuditEvent {
        AuditEvent {
            event_id,
            event_type,
            severity,
            status,
            user,
            description,
            details,
            resource,
            action,
            timestamp,
            client_ip,
            user_agent,
            metadata,
            event_hash,
        }
    }

    #[test]
    fn test_audit_event_type_enum() {
        assert_eq!(AuditEventType::Authentication, AuditEventType::Authentication);
        assert_ne!(AuditEventType::Authentication, AuditEventType::Authorization);
        assert_eq!(AuditEventType::INIT_SPACE, 1);
    }

    #[test]
    fn test_audit_severity_enum() {
        assert_eq!(AuditSeverity::Low, AuditSeverity::Low);
        assert_ne!(AuditSeverity::Low, AuditSeverity::Critical);
        assert_eq!(AuditSeverity::INIT_SPACE, 1);
    }

    #[test]
    fn test_audit_event_status_enum() {
        assert_eq!(AuditEventStatus::Pending, AuditEventStatus::Pending);
        assert_ne!(AuditEventStatus::Pending, AuditEventStatus::Resolved);
    }

    #[test]
    fn test_audit_event_creation() {
        let user = Pubkey::from([1u8; 32]);
        let event = create_test_audit_event_direct(
            1,
            AuditEventType::Security,
            AuditSeverity::High,
            AuditEventStatus::Pending,
            user,
            "Test event".to_string(),
            "Test details".to_string(),
            "Test resource".to_string(),
            "Test action".to_string(),
            1234567890,
            Some("127.0.0.1".to_string()),
            Some("Test Agent".to_string()),
            vec![],
            vec![0u8; 32],
        );

        assert_eq!(event.event_id, 1);
        assert_eq!(event.event_type, AuditEventType::Security);
        assert_eq!(event.severity, AuditSeverity::High);
        assert_eq!(event.user, user);
    }

    #[test]
    fn test_audit_event_options_default() {
        let opts = AuditEventOptions::default();
        assert!(opts.client_ip.is_none());
        assert!(opts.user_agent.is_none());
        assert!(opts.metadata.is_empty());
    }

    #[test]
    fn test_audit_log_config_creation() {
        let config = AuditLogConfig {
            config_id: 1,
            max_events: 1000,
            retention_days: 365,
            event_types: vec![AuditEventType::Security, AuditEventType::Authentication],
            min_severity: AuditSeverity::Medium,
            is_enabled: true,
            created_at: 1234567890,
            updated_at: 1234567890,
        };

        assert_eq!(config.config_id, 1);
        assert_eq!(config.max_events, 1000);
        assert_eq!(config.event_types.len(), 2);
        assert!(config.is_enabled);
    }

    #[test]
    fn test_audit_trail_creation() {
        let user = Pubkey::from([2u8; 32]);
        let trail = AuditTrail {
            trail_id: 1,
            user,
            events: Vec::new(),
            start_time: 1234567890,
            end_time: None,
            is_active: true,
            created_at: 1234567890,
            updated_at: 1234567890,
        };

        assert_eq!(trail.trail_id, 1);
        assert_eq!(trail.user, user);
        assert!(trail.is_active);
        assert!(trail.end_time.is_none());
    }

    #[test]
    fn test_compliance_rule_creation() {
        let rule = ComplianceRule {
            rule_id: 1,
            name: "Test Rule".to_string(),
            description: "Test Description".to_string(),
            conditions: vec!["high_severity".to_string()],
            actions: vec!["alert".to_string()],
            severity: AuditSeverity::High,
            is_active: true,
            created_at: 1234567890,
            updated_at: 1234567890,
        };

        assert_eq!(rule.rule_id, 1);
        assert_eq!(rule.name, "Test Rule");
        assert!(rule.is_active);
    }

    #[test]
    fn test_update_event_status() {
        let user = Pubkey::from([3u8; 32]);
        let mut event = create_test_audit_event_direct(
            1, AuditEventType::Security, AuditSeverity::High,
            AuditEventStatus::Pending, user,
            "Test".to_string(), "Details".to_string(),
            "Resource".to_string(), "Action".to_string(),
            1234567890, None, None, vec![], vec![0u8; 32],
        );

        AuditManager::update_event_status(&mut event, AuditEventStatus::Resolved).unwrap();
        assert_eq!(event.status, AuditEventStatus::Resolved);
    }

    #[test]
    fn test_evaluate_condition_high_severity() {
        let user = Pubkey::from([4u8; 32]);
        let event = create_test_audit_event_direct(
            1, AuditEventType::Security, AuditSeverity::High,
            AuditEventStatus::Pending, user,
            "Test".to_string(), "Details".to_string(),
            "Resource".to_string(), "Action".to_string(),
            1234567890, None, None, vec![], vec![0u8; 32],
        );

        let result = AuditManager::evaluate_condition(&event, "high_severity").unwrap();
        assert!(result);
    }

    #[test]
    fn test_evaluate_condition_critical_severity() {
        let user = Pubkey::from([5u8; 32]);
        let event = create_test_audit_event_direct(
            1, AuditEventType::Security, AuditSeverity::Critical,
            AuditEventStatus::Pending, user,
            "Test".to_string(), "Details".to_string(),
            "Resource".to_string(), "Action".to_string(),
            1234567890, None, None, vec![], vec![0u8; 32],
        );

        let result = AuditManager::evaluate_condition(&event, "critical_severity").unwrap();
        assert!(result);
    }

    #[test]
    fn test_evaluate_condition_authentication_failure() {
        let user = Pubkey::from([6u8; 32]);
        let event = create_test_audit_event_direct(
            1, AuditEventType::Authentication, AuditSeverity::Medium,
            AuditEventStatus::Pending, user,
            "Test".to_string(), "Details".to_string(),
            "Resource".to_string(), "login_failed".to_string(),
            1234567890, None, None, vec![], vec![0u8; 32],
        );

        let result = AuditManager::evaluate_condition(&event, "authentication_failure").unwrap();
        assert!(result);
    }

    #[test]
    fn test_check_compliance() {
        let user = Pubkey::from([7u8; 32]);
        let event = create_test_audit_event_direct(
            1, AuditEventType::Security, AuditSeverity::Critical,
            AuditEventStatus::Pending, user,
            "Test".to_string(), "Details".to_string(),
            "Resource".to_string(), "Action".to_string(),
            1234567890, None, None, vec![], vec![0u8; 32],
        );

        let rule = ComplianceRule {
            rule_id: 1,
            name: "Critical Rule".to_string(),
            description: "Test".to_string(),
            conditions: vec!["critical_severity".to_string()],
            actions: vec!["alert".to_string()],
            severity: AuditSeverity::Critical,
            is_active: true,
            created_at: 1234567890,
            updated_at: 1234567890,
        };

        let violated = AuditManager::check_compliance(&event, &[rule]).unwrap();
        assert_eq!(violated.len(), 1);
        assert_eq!(violated[0], 1);
    }

    #[test]
    fn test_check_compliance_no_violation() {
        let user = Pubkey::from([8u8; 32]);
        let event = create_test_audit_event_direct(
            1, AuditEventType::Security, AuditSeverity::Low,
            AuditEventStatus::Pending, user,
            "Test".to_string(), "Details".to_string(),
            "Resource".to_string(), "Action".to_string(),
            1234567890, None, None, vec![], vec![0u8; 32],
        );

        let rule = ComplianceRule {
            rule_id: 1,
            name: "Critical Rule".to_string(),
            description: "Test".to_string(),
            conditions: vec!["critical_severity".to_string()],
            actions: vec!["alert".to_string()],
            severity: AuditSeverity::Critical,
            is_active: true,
            created_at: 1234567890,
            updated_at: 1234567890,
        };

        let violated = AuditManager::check_compliance(&event, &[rule]).unwrap();
        assert_eq!(violated.len(), 0);
    }

    #[test]
    fn test_get_audit_statistics() {
        let user = Pubkey::from([9u8; 32]);
        let events = vec![
            create_test_audit_event_direct(
                1, AuditEventType::Security, AuditSeverity::Critical,
                AuditEventStatus::Pending, user,
                "Test1".to_string(), "Details1".to_string(),
                "Resource1".to_string(), "Action1".to_string(),
                1234567890, None, None, vec![], vec![0u8; 32],
            ),
            create_test_audit_event_direct(
                2, AuditEventType::Security, AuditSeverity::High,
                AuditEventStatus::Resolved, user,
                "Test2".to_string(), "Details2".to_string(),
                "Resource2".to_string(), "Action2".to_string(),
                1234567900, None, None, vec![], vec![0u8; 32],
            ),
            create_test_audit_event_direct(
                3, AuditEventType::Security, AuditSeverity::Low,
                AuditEventStatus::Pending, user,
                "Test3".to_string(), "Details3".to_string(),
                "Resource3".to_string(), "Action3".to_string(),
                1234568000, None, None, vec![], vec![0u8; 32],
            ),
        ];

        let stats = AuditManager::get_audit_statistics(&events, (1234567890, 1234568000)).unwrap();
        assert_eq!(stats.total_events, 3);
        assert_eq!(stats.critical_events, 1);
        assert_eq!(stats.high_events, 1);
        assert_eq!(stats.low_events, 1);
        assert_eq!(stats.pending_events, 2);
        assert_eq!(stats.resolved_events, 1);
    }

    #[test]
    fn test_audit_statistics_time_range() {
        let user = Pubkey::from([10u8; 32]);
        let events = vec![
            create_test_audit_event_direct(
                1, AuditEventType::Security, AuditSeverity::High,
                AuditEventStatus::Pending, user,
                "Test1".to_string(), "Details1".to_string(),
                "Resource1".to_string(), "Action1".to_string(),
                1234567890, None, None, vec![], vec![0u8; 32],
            ),
            create_test_audit_event_direct(
                2, AuditEventType::Security, AuditSeverity::High,
                AuditEventStatus::Pending, user,
                "Test2".to_string(), "Details2".to_string(),
                "Resource2".to_string(), "Action2".to_string(),
                1234569000, None, None, vec![], vec![0u8; 32],
            ),
        ];

        let stats = AuditManager::get_audit_statistics(&events, (1234568000, 1234570000)).unwrap();
        assert_eq!(stats.total_events, 1); // Only second event in range
    }

    #[test]
    fn test_audit_constants() {
        assert_eq!(constants::MAX_EVENT_DESCRIPTION_LENGTH, 500);
        assert_eq!(constants::MAX_EVENT_DETAILS_LENGTH, 1000);
        assert_eq!(constants::MAX_RESOURCE_LENGTH, 200);
        assert_eq!(constants::MAX_ACTION_LENGTH, 100);
        assert_eq!(constants::MAX_METADATA_ITEMS, 20);
    }

    #[test]
    fn test_create_audit_event_with_time() {
        let user = Pubkey::from([11u8; 32]);
        let timestamp = 1234567890;
        let params = AuditEventParams {
            event_type: AuditEventType::Security,
            severity: AuditSeverity::High,
            user,
            description: "Test event".to_string(),
            details: "Test details".to_string(),
            resource: "Test resource".to_string(),
            action: "Test action".to_string(),
            opts: AuditEventOptions::default(),
        };

        let event = AuditManager::create_audit_event_with_time(params, timestamp).unwrap();
        assert_eq!(event.event_type, AuditEventType::Security);
        assert_eq!(event.severity, AuditSeverity::High);
        assert_eq!(event.user, user);
        assert_eq!(event.status, AuditEventStatus::Pending);
        assert_eq!(event.timestamp, timestamp);
    }

    #[test]
    fn test_create_audit_event_with_time_empty_description() {
        let user = Pubkey::from([12u8; 32]);
        let params = AuditEventParams {
            event_type: AuditEventType::Security,
            severity: AuditSeverity::High,
            user,
            description: "".to_string(), // Empty - should fail
            details: "Details".to_string(),
            resource: "Resource".to_string(),
            action: "Action".to_string(),
            opts: AuditEventOptions::default(),
        };

        let result = AuditManager::create_audit_event_with_time(params, 1234567890);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_audit_config_with_time() {
        let current_time = 1234567890;
        let event_types = vec![AuditEventType::Security, AuditEventType::Authentication];
        
        let config = AuditManager::create_audit_config_with_time(
            1,
            1000,
            365,
            event_types.clone(),
            AuditSeverity::Medium,
            current_time,
        ).unwrap();

        assert_eq!(config.config_id, 1);
        assert_eq!(config.max_events, 1000);
        assert_eq!(config.retention_days, 365);
        assert_eq!(config.event_types, event_types);
        assert_eq!(config.min_severity, AuditSeverity::Medium);
        assert!(config.is_enabled);
        assert_eq!(config.created_at, current_time);
        assert_eq!(config.updated_at, current_time);
    }

    #[test]
    fn test_create_audit_trail_with_time() {
        let user = Pubkey::from([13u8; 32]);
        let current_time = 1234567890;

        let trail = AuditManager::create_audit_trail_with_time(1, user, 100, current_time).unwrap();
        assert_eq!(trail.trail_id, 1);
        assert_eq!(trail.user, user);
        assert_eq!(trail.events.capacity(), 100);
        assert!(trail.is_active);
        assert_eq!(trail.start_time, current_time);
        assert_eq!(trail.created_at, current_time);
        assert_eq!(trail.updated_at, current_time);
    }

    #[test]
    fn test_add_event_to_trail_with_time() {
        let user = Pubkey::from([14u8; 32]);
        let mut trail = AuditManager::create_audit_trail_with_time(1, user, 100, 1234567890).unwrap();
        
        let event = create_test_audit_event_direct(
            1, AuditEventType::Security, AuditSeverity::High, AuditEventStatus::Pending,
            user, "Test".to_string(), "Details".to_string(), "Resource".to_string(),
            "Action".to_string(), 1234567890, None, None, vec![], vec![0u8; 32],
        );

        let update_time = 1234569000;
        let result = AuditManager::add_event_to_trail_with_time(&mut trail, event, update_time);
        assert!(result.is_ok());
        assert_eq!(trail.events.len(), 1);
        assert_eq!(trail.updated_at, update_time);
    }

    #[test]
    fn test_close_audit_trail_with_time() {
        let user = Pubkey::from([15u8; 32]);
        let mut trail = AuditManager::create_audit_trail_with_time(1, user, 100, 1234567890).unwrap();

        let close_time = 1234569000;
        let result = AuditManager::close_audit_trail_with_time(&mut trail, close_time);
        assert!(result.is_ok());
        assert!(!trail.is_active);
        assert_eq!(trail.end_time, Some(close_time));
        assert_eq!(trail.updated_at, close_time);
    }

    #[test]
    fn test_create_compliance_rule_with_time() {
        let current_time = 1234567890;
        let conditions = vec!["condition1".to_string(), "condition2".to_string()];
        let actions = vec!["action1".to_string(), "action2".to_string()];

        let rule = AuditManager::create_compliance_rule_with_time(
            1,
            "Test Rule".to_string(),
            "Test Description".to_string(),
            conditions.clone(),
            actions.clone(),
            AuditSeverity::High,
            current_time,
        ).unwrap();

        assert_eq!(rule.rule_id, 1);
        assert_eq!(rule.name, "Test Rule");
        assert_eq!(rule.description, "Test Description");
        assert_eq!(rule.conditions, conditions);
        assert_eq!(rule.actions, actions);
        assert_eq!(rule.severity, AuditSeverity::High);
        assert!(rule.is_active);
        assert_eq!(rule.created_at, current_time);
        assert_eq!(rule.updated_at, current_time);
    }
}
