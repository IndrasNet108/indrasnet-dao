//! Compliance Checking Module
//! 
//! Comprehensive compliance checking and validation system.
//! Integrates with audit and other security modules for compliance verification.
//! 
//! Hybrid model: Off-chain compliance checking and analysis, on-chain metadata storage
//! 
//! Principle: "Blockchain = Proof, not Storage"
//! - ON-CHAIN: Only compliance check metadata, results, and validation records
//! - OFF-CHAIN: All actual compliance checking, rule validation, and detailed analysis
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-security/src/security/compliance_checking/
//! Combined from: part_1.rs, part_2.rs, part_3.rs

use anchor_lang::prelude::*;
use crate::error::IndrasError;

// ============================================================================
// ON-CHAIN STRUCTURES (Anchor Account Types)
// ============================================================================

/// Compliance framework type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum ComplianceFramework {
    /// GDPR compliance
    Gdpr,
    /// ISO 27001 compliance
    Iso27001,
    /// SOC 2 compliance
    Soc2,
    /// HIPAA compliance
    Hipaa,
    /// PCI DSS compliance
    PciDss,
    /// NIS2 Directive compliance
    Nis2,
    /// Custom framework
    Custom,
}

impl Space for ComplianceFramework {
    const INIT_SPACE: usize = 1;
}

/// Compliance check status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum ComplianceCheckStatus {
    /// Check passed
    Passed,
    /// Check failed
    Failed,
    /// Check in progress
    InProgress,
    /// Check requires review
    RequiresReview,
}

impl Space for ComplianceCheckStatus {
    const INIT_SPACE: usize = 1;
}

/// Compliance check record
#[account]
#[derive(InitSpace)]
pub struct ComplianceCheck {
    /// Check ID
    pub check_id: u64,
    /// Subject of compliance check
    pub subject: Pubkey,
    /// Compliance framework
    pub framework: ComplianceFramework,
    /// Check name
    #[max_len(200)]
    pub name: String,
    /// Check description
    #[max_len(1000)]
    pub description: String,
    /// Check status
    pub status: ComplianceCheckStatus,
    /// Check result hash (for verification)
    pub result_hash: [u8; 32],
    /// Check metadata URI (IPFS or similar)
    #[max_len(500)]
    pub metadata_uri: String,
    /// Check timestamp
    pub checked_at: i64,
    /// Checker (compliance system)
    pub checker: Pubkey,
    /// Bump seed for PDA
    pub bump: u8,
}

// ============================================================================
// ON-CHAIN FUNCTIONS (Anchor Handlers)
// ============================================================================

/// Create compliance check
pub fn create_compliance_check(
    check: &mut ComplianceCheck,
    check_id: u64,
    subject: Pubkey,
    framework: ComplianceFramework,
    name: String,
    description: String,
    status: ComplianceCheckStatus,
    result_hash: [u8; 32],
    metadata_uri: String,
    checker: Pubkey,
    bump: u8,
) -> Result<()> {
    require!(!name.is_empty(), IndrasError::InvalidInput);
    require!(name.len() <= 200, IndrasError::StringTooLong);
    require!(description.len() <= 1000, IndrasError::StringTooLong);
    require!(metadata_uri.len() <= 500, IndrasError::StringTooLong);
    
    check.check_id = check_id;
    check.subject = subject;
    check.framework = framework;
    check.name = name;
    check.description = description;
    check.status = status;
    check.result_hash = result_hash;
    check.metadata_uri = metadata_uri;
    check.checked_at = Clock::get()?.unix_timestamp;
    check.checker = checker;
    check.bump = bump;
    
    msg!("Compliance check {} created (framework: {:?}, status: {:?})", 
         check_id, framework, status);
    
    Ok(())
}

/// Update compliance check status
pub fn update_compliance_check_status(
    check: &mut ComplianceCheck,
    status: ComplianceCheckStatus,
    result_hash: Option<[u8; 32]>,
) -> Result<()> {
    check.status = status;
    
    if let Some(hash) = result_hash {
        check.result_hash = hash;
    }
    
    check.checked_at = Clock::get()?.unix_timestamp;
    
    msg!("Compliance check {} status updated to {:?}", check.check_id, status);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_compliance_check() {
        let mut check = ComplianceCheck {
            check_id: 0,
            subject: Pubkey::default(),
            framework: ComplianceFramework::Gdpr,
            name: String::new(),
            description: String::new(),
            status: ComplianceCheckStatus::InProgress,
            result_hash: [0u8; 32],
            metadata_uri: String::new(),
            checked_at: 0,
            checker: Pubkey::default(),
            bump: 0,
        };

        let checker = Pubkey::from([1u8; 32]);
        let subject = Pubkey::from([2u8; 32]);
        let result_hash = [1u8; 32];

        let result = create_compliance_check(
            &mut check,
            1,
            subject,
            ComplianceFramework::Gdpr,
            "Test Check".to_string(),
            "Test Description".to_string(),
            ComplianceCheckStatus::Passed,
            result_hash,
            "ipfs://test".to_string(),
            checker,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(check.check_id, 1);
        assert_eq!(check.subject, subject);
        assert_eq!(check.framework, ComplianceFramework::Gdpr);
        assert_eq!(check.status, ComplianceCheckStatus::Passed);
    }
}
