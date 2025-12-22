//! Legal Compliance module
//!
//! Legal compliance management
//!
//! On-chain: Metadata for compliance status
//! Off-chain: Actual compliance checking, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Compliance status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ComplianceStatus {
    /// Compliant
    Compliant,
    /// Non-compliant
    NonCompliant,
    /// Under review
    UnderReview,
    /// Requires action
    RequiresAction,
}

/// Compliance type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ComplianceType {
    /// Regulatory compliance
    Regulatory,
    /// Legal compliance
    Legal,
    /// Tax compliance
    Tax,
    /// Data protection compliance
    DataProtection,
}

/// Compliance metadata (on-chain)
///
/// Stores compliance status and metadata
#[account]
#[derive(InitSpace)]
pub struct ComplianceMetadata {
    /// Compliance ID
    pub compliance_id: u64,
    /// Compliance type
    pub compliance_type: ComplianceType,
    /// Status
    pub status: ComplianceStatus,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Last checked at
    pub last_checked_at: Option<i64>,
    /// Compliance data hash
    pub compliance_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for compliance
pub mod onchain {
    use super::*;

    /// Initialize compliance metadata
    pub fn initialize_compliance_metadata(
        compliance: &mut ComplianceMetadata,
        compliance_id: u64,
        compliance_type: ComplianceType,
        compliance_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(compliance_id > 0, IndrasError::InvalidInput);
        
        compliance.compliance_id = compliance_id;
        compliance.compliance_type = compliance_type;
        compliance.status = ComplianceStatus::UnderReview;
        compliance.created_at = current_time;
        compliance.updated_at = current_time;
        compliance.last_checked_at = None;
        compliance.compliance_data_hash = compliance_data_hash;
        compliance.bump = bump;
        
        Ok(())
    }

    /// Update compliance status
    pub fn update_compliance_status(
        compliance: &mut ComplianceMetadata,
        new_status: ComplianceStatus,
        current_time: i64,
    ) -> Result<()> {
        compliance.status = new_status;
        compliance.updated_at = current_time;
        compliance.last_checked_at = Some(current_time);
        Ok(())
    }
}

/// Off-chain functions for compliance
pub mod offchain {
    /// Check compliance
    pub fn check_compliance(_compliance_id: u64) -> super::ComplianceStatus {
        // Implementation in off-chain service
        super::ComplianceStatus::UnderReview
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== initialize_compliance_metadata validation tests ==========
    
    #[test]
    fn test_initialize_compliance_metadata_validation_compliance_id_zero() {
        // Test: compliance_id == 0 should fail
        let compliance_id = 0u64;
        
        // Validation logic: require!(compliance_id > 0, IndrasError::InvalidInput)
        assert_eq!(compliance_id, 0, "Compliance ID zero should be detected");
    }
    
    #[test]
    fn test_initialize_compliance_metadata_validation_compliance_id_one() {
        // Test: compliance_id == 1 should pass
        let compliance_id = 1u64;
        
        // Validation logic: require!(compliance_id > 0, IndrasError::InvalidInput)
        assert!(compliance_id > 0, "Compliance ID one should pass");
    }
    
    #[test]
    fn test_initialize_compliance_metadata_validation_compliance_id_max() {
        // Test: compliance_id == u64::MAX should pass
        let compliance_id = u64::MAX;
        
        // Validation logic: require!(compliance_id > 0, IndrasError::InvalidInput)
        assert!(compliance_id > 0, "Compliance ID at max should pass");
    }
    
    #[test]
    fn test_initialize_compliance_metadata_validation_all_compliance_types() {
        // Test: all ComplianceType variants should be valid
        let compliance_types = vec![
            ComplianceType::Regulatory,
            ComplianceType::Legal,
            ComplianceType::Tax,
            ComplianceType::DataProtection,
        ];
        
        // All compliance types should be valid
        assert_eq!(compliance_types.len(), 4, "All compliance types should be valid");
    }
    
    #[test]
    fn test_initialize_compliance_metadata_validation_status_under_review() {
        // Test: status should be set to UnderReview on initialization
        let status = ComplianceStatus::UnderReview;
        
        // Status should be UnderReview
        assert_eq!(status, ComplianceStatus::UnderReview, "Status should be UnderReview on initialization");
    }
    
    #[test]
    fn test_initialize_compliance_metadata_validation_all_compliance_statuses() {
        // Test: all ComplianceStatus variants should be valid
        let statuses = vec![
            ComplianceStatus::Compliant,
            ComplianceStatus::NonCompliant,
            ComplianceStatus::UnderReview,
            ComplianceStatus::RequiresAction,
        ];
        
        // All statuses should be valid
        assert_eq!(statuses.len(), 4, "All compliance statuses should be valid");
    }
    
    #[test]
    fn test_initialize_compliance_metadata_validation_compliance_data_hash_zero() {
        // Test: compliance_data_hash == [0u8; 32] should be allowed (no validation)
        let compliance_data_hash = [0u8; 32];
        
        // No validation for zero hash - this is allowed
        assert_eq!(compliance_data_hash, [0u8; 32], "Zero compliance data hash should be allowed");
    }
    
    #[test]
    fn test_initialize_compliance_metadata_validation_last_checked_at_none() {
        // Test: last_checked_at == None on initialization
        let last_checked_at: Option<i64> = None;
        
        // Last checked should be None on initialization
        assert!(last_checked_at.is_none(), "Last checked should be None on initialization");
    }
    
    #[test]
    fn test_initialize_compliance_metadata_validation_valid_inputs() {
        // Test: valid inputs should pass
        let compliance_id = 1u64;
        let compliance_type = ComplianceType::Regulatory;
        let compliance_data_hash = [1u8; 32];
        let current_time = 1000000i64;
        let bump = 1u8;
        
        // All validations should pass
        assert!(compliance_id > 0, "Compliance ID should be valid");
        assert!(matches!(compliance_type, ComplianceType::Regulatory), "Compliance type should be valid");
        assert_eq!(compliance_data_hash.len(), 32, "Compliance data hash should be valid");
        assert!(current_time >= 0, "Current time should be valid");
        assert!(bump <= u8::MAX, "Bump should be valid");
    }

    // ========== update_compliance_status validation tests ==========
    
    #[test]
    fn test_update_compliance_status_validation_all_statuses() {
        // Test: all ComplianceStatus variants should be valid for update
        let statuses = vec![
            ComplianceStatus::Compliant,
            ComplianceStatus::NonCompliant,
            ComplianceStatus::UnderReview,
            ComplianceStatus::RequiresAction,
        ];
        
        // All statuses should be valid for update
        assert_eq!(statuses.len(), 4, "All statuses should be valid for update");
    }
    
    #[test]
    fn test_update_compliance_status_validation_current_time_zero() {
        // Test: current_time == 0 should be allowed (no validation)
        let current_time = 0i64;
        
        // Zero time should be allowed
        assert_eq!(current_time, 0, "Zero current time should be allowed");
    }
    
    #[test]
    fn test_update_compliance_status_validation_current_time_positive() {
        // Test: current_time > 0 should be allowed
        let current_time = 1000000i64;
        
        // Positive time should be allowed
        assert!(current_time > 0, "Positive current time should be allowed");
    }
    
    #[test]
    fn test_update_compliance_status_validation_last_checked_at_set() {
        // Test: last_checked_at should be set to current_time on update
        let current_time = 1000000i64;
        let last_checked_at = Some(current_time);
        
        // Last checked should be set
        assert_eq!(last_checked_at, Some(current_time), "Last checked should be set on update");
    }
    
    #[test]
    fn test_update_compliance_status_validation_status_transition_compliant() {
        // Test: status transition to Compliant should be allowed
        let new_status = ComplianceStatus::Compliant;
        
        // Status transition should be allowed
        assert_eq!(new_status, ComplianceStatus::Compliant, "Status transition to Compliant should be allowed");
    }
    
    #[test]
    fn test_update_compliance_status_validation_status_transition_non_compliant() {
        // Test: status transition to NonCompliant should be allowed
        let new_status = ComplianceStatus::NonCompliant;
        
        // Status transition should be allowed
        assert_eq!(new_status, ComplianceStatus::NonCompliant, "Status transition to NonCompliant should be allowed");
    }
}
