//! Compliance rules and checking
//!
//! This module provides compliance rule management and checking functionality.
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-security/src/security/audit/compliance.rs

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use super::types::*;
use super::operations::AuditManager;

/// Compliance rule operations
impl AuditManager {
    /// Create compliance rule
    pub fn create_compliance_rule(
        rule_id: u64,
        name: String,
        description: String,
        conditions: Vec<String>,
        actions: Vec<String>,
        severity: AuditSeverity,
    ) -> Result<ComplianceRule> {
        Self::create_compliance_rule_with_time(rule_id, name, description, conditions, actions, severity, Clock::get()?.unix_timestamp)
    }

    /// Create compliance rule with explicit time
    pub fn create_compliance_rule_with_time(
        rule_id: u64,
        name: String,
        description: String,
        conditions: Vec<String>,
        actions: Vec<String>,
        severity: AuditSeverity,
        current_time: i64,
    ) -> Result<ComplianceRule> {
        require!(!name.is_empty(), IndrasError::InvalidInput);
        require!(name.len() <= constants::MAX_RULE_NAME_LENGTH, IndrasError::InvalidInput);
        require!(!description.is_empty(), IndrasError::InvalidInput);
        require!(description.len() <= constants::MAX_RULE_DESCRIPTION_LENGTH, IndrasError::InvalidInput);
        require!(!conditions.is_empty(), IndrasError::InvalidInput);
        require!(conditions.len() <= constants::MAX_CONDITIONS, IndrasError::InvalidInput);
        require!(!actions.is_empty(), IndrasError::InvalidInput);
        require!(actions.len() <= constants::MAX_ACTIONS, IndrasError::InvalidInput);
        
        Ok(ComplianceRule {
            rule_id,
            name,
            description,
            conditions,
            actions,
            severity,
            is_active: true,
            created_at: current_time,
            updated_at: current_time,
        })
    }
    
    /// Update compliance rule
    pub fn update_compliance_rule(
        rule: &mut ComplianceRule,
        name: Option<String>,
        description: Option<String>,
        conditions: Option<Vec<String>>,
        actions: Option<Vec<String>>,
        severity: Option<AuditSeverity>,
        is_active: Option<bool>,
    ) -> Result<()> {
        Self::update_compliance_rule_with_time(rule, name, description, conditions, actions, severity, is_active, Clock::get()?.unix_timestamp)
    }

    /// Update compliance rule with explicit time
    pub fn update_compliance_rule_with_time(
        rule: &mut ComplianceRule,
        name: Option<String>,
        description: Option<String>,
        conditions: Option<Vec<String>>,
        actions: Option<Vec<String>>,
        severity: Option<AuditSeverity>,
        is_active: Option<bool>,
        current_time: i64,
    ) -> Result<()> {
        if let Some(name) = name {
            require!(!name.is_empty(), IndrasError::InvalidInput);
            require!(name.len() <= constants::MAX_RULE_NAME_LENGTH, IndrasError::InvalidInput);
            rule.name = name;
        }
        
        if let Some(description) = description {
            require!(!description.is_empty(), IndrasError::InvalidInput);
            require!(description.len() <= constants::MAX_RULE_DESCRIPTION_LENGTH, IndrasError::InvalidInput);
            rule.description = description;
        }
        
        if let Some(conditions) = conditions {
            require!(!conditions.is_empty(), IndrasError::InvalidInput);
            require!(conditions.len() <= constants::MAX_CONDITIONS, IndrasError::InvalidInput);
            rule.conditions = conditions;
        }
        
        if let Some(actions) = actions {
            require!(!actions.is_empty(), IndrasError::InvalidInput);
            require!(actions.len() <= constants::MAX_ACTIONS, IndrasError::InvalidInput);
            rule.actions = actions;
        }
        
        if let Some(severity) = severity {
            rule.severity = severity;
        }
        
        if let Some(is_active) = is_active {
            rule.is_active = is_active;
        }
        
        rule.updated_at = current_time;
        
        Ok(())
    }
    
    /// Check compliance against rules
    pub fn check_compliance(
        event: &AuditEvent,
        rules: &[ComplianceRule],
    ) -> Result<Vec<u64>> {
        let mut violated_rules = Vec::new();
        
        for rule in rules {
            if !rule.is_active {
                continue;
            }
            
            if Self::evaluate_rule_conditions(event, rule)? {
                violated_rules.push(rule.rule_id);
            }
        }
        
        Ok(violated_rules)
    }
    
    /// Evaluate rule conditions
    fn evaluate_rule_conditions(
        event: &AuditEvent,
        rule: &ComplianceRule,
    ) -> Result<bool> {
        for condition in &rule.conditions {
            if Self::evaluate_condition(event, condition)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
    
    /// Evaluate single condition
    pub fn evaluate_condition(
        event: &AuditEvent,
        condition: &str,
    ) -> Result<bool> {
        // Simple condition evaluation (in real implementation, use proper expression parser)
        match condition {
            "high_severity" => Ok(event.severity == AuditSeverity::High || event.severity == AuditSeverity::Critical),
            "critical_severity" => Ok(event.severity == AuditSeverity::Critical),
            "authentication_failure" => Ok(event.event_type == AuditEventType::Authentication && event.action == "login_failed"),
            "unauthorized_access" => Ok(event.event_type == AuditEventType::Authorization && event.action == "access_denied"),
            "data_breach" => Ok(event.event_type == AuditEventType::Security && event.action == "data_access"),
            _ => Ok(false),
        }
    }
}
