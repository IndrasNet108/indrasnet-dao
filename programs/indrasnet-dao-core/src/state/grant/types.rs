//! Grant types and enums

use anchor_lang::prelude::*;

/// Grant status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GrantStatus {
    Pending,        // Awaiting approval
    Approved,       // Approved
    Active,         // Active (disbursing)
    Suspended,      // Suspended
    Completed,      // Completed
    Cancelled,      // Cancelled
    Rejected,       // Rejected (voting failed)
    Expired,        // Expired
    Archived,       // Archived
}

impl Space for GrantStatus {
    const INIT_SPACE: usize = 1;
}

/// Grant category enum
/// 
/// Defines the type of grant that DAO can issue to participants and projects.
/// This determines the strategy, trust, and purpose of the grant.
/// 
/// **MVP Categories** (v1.0.0):
/// - Research: Early-stage innovation and prototypes
/// - Development: MVP and product development
/// - Community: Community strengthening and engagement
/// 
/// **Future Categories** (see Roadmap):
/// - Impact: Social, cultural, or environmental impact projects
/// - Infrastructure: Network and ecosystem development
/// - Creative: Cultural and media projects
/// - Acceleration: Scaling and market expansion
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GrantCategory {
    /// Research Grants - For research ideas, prototypes, academic collaborations
    /// Small amounts (€5k–€20k) for hypothesis testing
    /// Goal: Stimulate innovation at early stage
    Research,
    
    /// Development Grants - For MVP, pilots, and technical implementation
    /// Medium amounts (€20k–€100k)
    /// Goal: Bring idea to working product
    Development,
    
    /// Community Grants - For DAO participants: education, workshops, local initiatives
    /// Small amounts (€1k–€10k)
    /// Goal: Strengthen community and engagement
    Community,
}

impl Space for GrantCategory {
    const INIT_SPACE: usize = 1;
}

/// Grant type by development stage
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GrantType {
    Initial,    // Initial development grant
    Core,       // Core development grant
    Final,      // Finalization grant
}

impl Space for GrantType {
    const INIT_SPACE: usize = 1;
}

/// Grant disbursement type - defines how funds are disbursed
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GrantDisbursementType {
    /// Urgent grant - funds disbursed immediately (for raw materials, supplies, urgent expenses)
    /// No report required, funds immediately available to recipient
    Urgent,
    
    /// Escrow grant - funds held in escrow until final report (for code, documents, long-term projects)
    /// Funds locked in escrow account until final report is submitted and approved
    Escrow,
    
    /// Standard grant - milestone-based disbursement (default)
    /// Funds disbursed as milestones are completed with intermediate reports
    Standard,
}

impl Space for GrantDisbursementType {
    const INIT_SPACE: usize = 1;
}

impl GrantDisbursementType {
    /// Determines if report is required for account closure
    pub fn requires_report(&self) -> bool {
        matches!(self, GrantDisbursementType::Escrow | GrantDisbursementType::Standard)
    }
    
    /// Determines if escrow account is required
    pub fn requires_escrow(&self) -> bool {
        matches!(self, GrantDisbursementType::Escrow)
    }
    
    /// Determines if account can be closed without report
    pub fn can_close_without_report(&self) -> bool {
        matches!(self, GrantDisbursementType::Urgent)
    }
}

/// Verification status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum VerificationStatus {
    Pending,    // Awaiting verification
    Verified,   // Verified
    Rejected,   // Rejected
}

impl Space for VerificationStatus {
    const INIT_SPACE: usize = 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grant_status_variants() {
        let variants = vec![
            GrantStatus::Pending,
            GrantStatus::Approved,
            GrantStatus::Active,
            GrantStatus::Suspended,
            GrantStatus::Completed,
            GrantStatus::Cancelled,
            GrantStatus::Rejected,
            GrantStatus::Expired,
            GrantStatus::Archived,
        ];
        
        // Check all variants are unique
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_grant_category_variants() {
        assert_eq!(GrantCategory::Research, GrantCategory::Research);
        assert_eq!(GrantCategory::Development, GrantCategory::Development);
        assert_eq!(GrantCategory::Community, GrantCategory::Community);
    }

    #[test]
    fn test_grant_type_variants() {
        assert_eq!(GrantType::Initial, GrantType::Initial);
        assert_eq!(GrantType::Core, GrantType::Core);
        assert_eq!(GrantType::Final, GrantType::Final);
    }

    #[test]
    fn test_verification_status_variants() {
        assert_eq!(VerificationStatus::Pending, VerificationStatus::Pending);
        assert_eq!(VerificationStatus::Verified, VerificationStatus::Verified);
        assert_eq!(VerificationStatus::Rejected, VerificationStatus::Rejected);
    }

    #[test]
    fn test_grant_status_equality() {
        assert_eq!(GrantStatus::Pending, GrantStatus::Pending);
        assert_ne!(GrantStatus::Pending, GrantStatus::Approved);
        assert_eq!(GrantStatus::Active, GrantStatus::Active);
        assert_ne!(GrantStatus::Active, GrantStatus::Completed);
    }

    #[test]
    fn test_grant_category_equality() {
        assert_eq!(GrantCategory::Research, GrantCategory::Research);
        assert_ne!(GrantCategory::Research, GrantCategory::Development);
        assert_eq!(GrantCategory::Community, GrantCategory::Community);
        assert_ne!(GrantCategory::Development, GrantCategory::Community);
    }

    #[test]
    fn test_grant_type_equality() {
        assert_eq!(GrantType::Initial, GrantType::Initial);
        assert_ne!(GrantType::Initial, GrantType::Core);
        assert_eq!(GrantType::Core, GrantType::Core);
        assert_ne!(GrantType::Core, GrantType::Final);
        assert_eq!(GrantType::Final, GrantType::Final);
    }

    #[test]
    fn test_verification_status_equality() {
        assert_eq!(VerificationStatus::Pending, VerificationStatus::Pending);
        assert_ne!(VerificationStatus::Pending, VerificationStatus::Verified);
        assert_eq!(VerificationStatus::Verified, VerificationStatus::Verified);
        assert_ne!(VerificationStatus::Verified, VerificationStatus::Rejected);
        assert_eq!(VerificationStatus::Rejected, VerificationStatus::Rejected);
    }

    #[test]
    fn test_grant_status_all_variants_unique() {
        let statuses = vec![
            GrantStatus::Pending,
            GrantStatus::Approved,
            GrantStatus::Active,
            GrantStatus::Suspended,
            GrantStatus::Completed,
            GrantStatus::Cancelled,
            GrantStatus::Rejected,
            GrantStatus::Expired,
            GrantStatus::Archived,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j]);
            }
        }
    }

    #[test]
    fn test_grant_category_all_variants() {
        let categories = vec![
            GrantCategory::Research,
            GrantCategory::Development,
            GrantCategory::Community,
        ];
        
        for i in 0..categories.len() {
            for j in (i + 1)..categories.len() {
                assert_ne!(categories[i], categories[j]);
            }
        }
    }

    #[test]
    fn test_grant_type_all_variants() {
        let types = vec![
            GrantType::Initial,
            GrantType::Core,
            GrantType::Final,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j]);
            }
        }
    }

    #[test]
    fn test_verification_status_all_variants() {
        let statuses = vec![
            VerificationStatus::Pending,
            VerificationStatus::Verified,
            VerificationStatus::Rejected,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j]);
            }
        }
    }

    #[test]
    fn test_grant_status_space() {
        // Test that Space trait is implemented
        assert_eq!(<GrantStatus as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_grant_category_space() {
        // Test that Space trait is implemented
        assert_eq!(<GrantCategory as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_grant_type_space() {
        // Test that Space trait is implemented
        assert_eq!(<GrantType as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_verification_status_space() {
        // Test that Space trait is implemented
        assert_eq!(<VerificationStatus as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_grant_status_copy() {
        let status1 = GrantStatus::Pending;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_grant_category_copy() {
        let cat1 = GrantCategory::Research;
        let cat2 = cat1; // Copy trait
        assert_eq!(cat1, cat2);
    }

    #[test]
    fn test_grant_type_copy() {
        let type1 = GrantType::Initial;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_verification_status_copy() {
        let status1 = VerificationStatus::Pending;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_grant_category_all_variants_unique() {
        let categories = vec![
            GrantCategory::Research,
            GrantCategory::Development,
            GrantCategory::Community,
        ];
        
        for i in 0..categories.len() {
            for j in (i + 1)..categories.len() {
                assert_ne!(categories[i], categories[j], "Duplicate category found");
            }
        }
    }

    #[test]
    fn test_grant_type_all_variants_unique() {
        let types = vec![
            GrantType::Initial,
            GrantType::Core,
            GrantType::Final,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_verification_status_all_variants_unique() {
        let statuses = vec![
            VerificationStatus::Pending,
            VerificationStatus::Verified,
            VerificationStatus::Rejected,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }
}
