//! Audit operations
//!
//! This module provides core audit operations including event creation,
//! configuration management, and trail management.
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-security/src/security/audit/operations.rs

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use super::types::*;

/// Audit manager for handling audit operations
pub struct AuditManager;

impl AuditManager {
    /// Create new audit event
    pub fn create_audit_event(params: AuditEventParams) -> Result<AuditEvent> {
        Self::create_audit_event_with_time(params, Clock::get()?.unix_timestamp)
    }

    /// Create new audit event with explicit time
    pub fn create_audit_event_with_time(params: AuditEventParams, timestamp: i64) -> Result<AuditEvent> {
        require!(!params.description.is_empty(), IndrasError::InvalidInput);
        require!(params.description.len() <= constants::MAX_EVENT_DESCRIPTION_LENGTH, IndrasError::InvalidInput);
        require!(!params.details.is_empty(), IndrasError::InvalidInput);
        require!(params.details.len() <= constants::MAX_EVENT_DETAILS_LENGTH, IndrasError::InvalidInput);
        require!(!params.resource.is_empty(), IndrasError::InvalidInput);
        require!(params.resource.len() <= constants::MAX_RESOURCE_LENGTH, IndrasError::InvalidInput);
        require!(!params.action.is_empty(), IndrasError::InvalidInput);
        require!(params.action.len() <= constants::MAX_ACTION_LENGTH, IndrasError::InvalidInput);
        require!(params.opts.metadata.len() <= constants::MAX_METADATA_ITEMS, IndrasError::InvalidInput);
        
        let event_id = Self::generate_event_id(params.user, timestamp);
        
        // Create event hash for integrity
        let event_data = format!("{}{}{}{}{}{}", 
            event_id, params.user, params.description, params.resource, params.action, timestamp);
        let event_hash = Self::hash_event_data(&event_data)?;
        
        Ok(AuditEvent {
            event_id,
            event_type: params.event_type,
            severity: params.severity,
            status: AuditEventStatus::Pending,
            user: params.user,
            description: params.description,
            details: params.details,
            resource: params.resource,
            action: params.action,
            timestamp,
            client_ip: params.opts.client_ip,
            user_agent: params.opts.user_agent,
            metadata: params.opts.metadata,
            event_hash,
        })
    }
    
    /// Update audit event status
    pub fn update_event_status(
        event: &mut AuditEvent,
        new_status: AuditEventStatus,
    ) -> Result<()> {
        event.status = new_status;
        Ok(())
    }
    
    /// Create audit log configuration
    pub fn create_audit_config(
        config_id: u64,
        max_events: u32,
        retention_days: u32,
        event_types: Vec<AuditEventType>,
        min_severity: AuditSeverity,
    ) -> Result<AuditLogConfig> {
        Self::create_audit_config_with_time(config_id, max_events, retention_days, event_types, min_severity, Clock::get()?.unix_timestamp)
    }

    /// Create audit log configuration with explicit time
    pub fn create_audit_config_with_time(
        config_id: u64,
        max_events: u32,
        retention_days: u32,
        event_types: Vec<AuditEventType>,
        min_severity: AuditSeverity,
        current_time: i64,
    ) -> Result<AuditLogConfig> {
        require!(max_events > 0, IndrasError::InvalidInput);
        require!(max_events <= constants::MAX_AUDIT_EVENTS as u32, IndrasError::InvalidInput);
        require!(retention_days > 0, IndrasError::InvalidInput);
        require!(retention_days <= constants::MAX_RETENTION_DAYS, IndrasError::InvalidInput);
        require!(!event_types.is_empty(), IndrasError::InvalidInput);
        require!(event_types.len() <= constants::MAX_EVENT_TYPES, IndrasError::InvalidInput);
        
        Ok(AuditLogConfig {
            config_id,
            max_events,
            retention_days,
            event_types,
            min_severity,
            is_enabled: true,
            created_at: current_time,
            updated_at: current_time,
        })
    }
    
    /// Update audit log configuration
    pub fn update_audit_config(
        config: &mut AuditLogConfig,
        max_events: Option<u32>,
        retention_days: Option<u32>,
        event_types: Option<Vec<AuditEventType>>,
        min_severity: Option<AuditSeverity>,
        is_enabled: Option<bool>,
    ) -> Result<()> {
        Self::update_audit_config_with_time(config, max_events, retention_days, event_types, min_severity, is_enabled, Clock::get()?.unix_timestamp)
    }

    /// Update audit log configuration with explicit time
    pub fn update_audit_config_with_time(
        config: &mut AuditLogConfig,
        max_events: Option<u32>,
        retention_days: Option<u32>,
        event_types: Option<Vec<AuditEventType>>,
        min_severity: Option<AuditSeverity>,
        is_enabled: Option<bool>,
        current_time: i64,
    ) -> Result<()> {
        if let Some(max_events) = max_events {
            require!(max_events > 0, IndrasError::InvalidInput);
            require!(max_events <= constants::MAX_AUDIT_EVENTS as u32, IndrasError::InvalidInput);
            config.max_events = max_events;
        }
        
        if let Some(retention_days) = retention_days {
            require!(retention_days > 0, IndrasError::InvalidInput);
            require!(retention_days <= constants::MAX_RETENTION_DAYS, IndrasError::InvalidInput);
            config.retention_days = retention_days;
        }
        
        if let Some(event_types) = event_types {
            require!(!event_types.is_empty(), IndrasError::InvalidInput);
            require!(event_types.len() <= constants::MAX_EVENT_TYPES, IndrasError::InvalidInput);
            config.event_types = event_types;
        }
        
        if let Some(min_severity) = min_severity {
            config.min_severity = min_severity;
        }
        
        if let Some(is_enabled) = is_enabled {
            config.is_enabled = is_enabled;
        }
        
        config.updated_at = current_time;
        
        Ok(())
    }
    
    /// Create audit trail
    pub fn create_audit_trail(
        trail_id: u64,
        user: Pubkey,
        max_events: u32,
    ) -> Result<AuditTrail> {
        Self::create_audit_trail_with_time(trail_id, user, max_events, Clock::get()?.unix_timestamp)
    }

    /// Create audit trail with explicit time
    pub fn create_audit_trail_with_time(
        trail_id: u64,
        user: Pubkey,
        max_events: u32,
        current_time: i64,
    ) -> Result<AuditTrail> {
        require!(max_events > 0, IndrasError::InvalidInput);
        require!(max_events <= constants::MAX_EVENTS_PER_TRAIL as u32, IndrasError::InvalidInput);
        
        Ok(AuditTrail {
            trail_id,
            user,
            events: Vec::with_capacity(max_events as usize),
            start_time: current_time,
            end_time: None,
            is_active: true,
            created_at: current_time,
            updated_at: current_time,
        })
    }
    
    /// Add event to audit trail
    pub fn add_event_to_trail(
        trail: &mut AuditTrail,
        event: AuditEvent,
    ) -> Result<()> {
        Self::add_event_to_trail_with_time(trail, event, Clock::get()?.unix_timestamp)
    }

    /// Add event to audit trail with explicit time
    pub fn add_event_to_trail_with_time(
        trail: &mut AuditTrail,
        event: AuditEvent,
        current_time: i64,
    ) -> Result<()> {
        require!(trail.is_active, IndrasError::InvalidState);
        require!(trail.user == event.user, IndrasError::InvalidInput);
        
        trail.events.push(event);
        trail.updated_at = current_time;
        
        Ok(())
    }
    
    /// Close audit trail
    pub fn close_audit_trail(trail: &mut AuditTrail) -> Result<()> {
        Self::close_audit_trail_with_time(trail, Clock::get()?.unix_timestamp)
    }

    /// Close audit trail with explicit time
    pub fn close_audit_trail_with_time(trail: &mut AuditTrail, current_time: i64) -> Result<()> {
        require!(trail.is_active, IndrasError::InvalidState);
        
        trail.is_active = false;
        trail.end_time = Some(current_time);
        trail.updated_at = current_time;
        
        Ok(())
    }
    
    /// Generate unique event ID
    fn generate_event_id(user: Pubkey, timestamp: i64) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        user.hash(&mut hasher);
        timestamp.hash(&mut hasher);
        hasher.finish()
    }

    /// Hash event data for integrity
    fn hash_event_data(data: &str) -> Result<Vec<u8>> {
        // Simple hash function (in real implementation, use proper cryptographic hash)
        let mut hash = Vec::new();
        for chunk in data.as_bytes().chunks(32) {
            let mut sum = 0u8;
            for &byte in chunk {
                sum = sum.wrapping_add(byte);
            }
            hash.push(sum);
        }
        hash.resize(32, 0);
        Ok(hash)
    }
}
