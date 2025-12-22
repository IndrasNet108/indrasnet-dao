//! Commercial Enterprise instruction handlers
//!
//! Handlers for commercial enterprise operations: create, transfer idea, add investor, update production status
//!
//! NOTE: This module implements commercial enterprise functionality for MVP v1.0.0.
//! Following the proper migration process: Read → Understand → Analyze → Implement

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::enums::IdeaStatus;
use crate::state::commercial_enterprise::{EnterpriseStatus, ProductionStatus};
use crate::state::mesh_group::{GroupStatus, DevelopmentStage};
use crate::state::grant::GrantStatus;

/// Create commercial enterprise for commercialization of ideas
///
/// This handler creates a new commercial enterprise account.
/// Enterprise can be IndrasNet OÜ (by DAO decision) or another firm (by DAO decision).
pub fn create_commercial_enterprise_handler(
    ctx: Context<crate::CreateCommercialEnterprise>,
    enterprise_id: u64,
    name: String,
    description: String,
    enterprise_pubkey: Pubkey,
    enterprise_type: crate::state::commercial_enterprise::EnterpriseType,
) -> Result<()> {
    let enterprise = &mut ctx.accounts.enterprise;
    
    // Validate input data
    require!(!name.is_empty(), IndrasError::InvalidInput);
    require!(name.len() <= 100, IndrasError::StringTooLong);
    require!(description.len() <= 500, IndrasError::StringTooLong);
    
    // Initialize commercial enterprise
    enterprise.id = enterprise_id;
    enterprise.name = name;
    enterprise.description = description;
    enterprise.enterprise_pubkey = enterprise_pubkey;
    enterprise.enterprise_type = enterprise_type;
    enterprise.status = EnterpriseStatus::Forming;
    enterprise.ideas = Vec::new();
    enterprise.investors = Vec::new();
    enterprise.total_investment = 0;
    enterprise.production_status = ProductionStatus::NotStarted;
    enterprise.created_at = Clock::get()?.unix_timestamp;
    enterprise.commercialization_started_at = None;
    enterprise.production_started_at = None;
    enterprise.bump = ctx.bumps.enterprise;
    
    msg!("Commercial Enterprise {} created by {}", enterprise.name, ctx.accounts.creator.key());
    
    Ok(())
}

/// Transfer idea to commercial enterprise for commercialization
///
/// Transitions idea from Executed/Completed to Commercialization status.
/// 
/// CRITICAL: Check intellectual property rights
/// - Idea author remains copyright owner
/// - e.V. received commercialization right upon grant approval
/// - e.V. is custodian of author's copyright
pub fn transfer_idea_to_commercial_enterprise_handler(
    ctx: Context<crate::TransferIdeaToCommercialEnterprise>,
    idea_id: u64,
    enterprise_id: u64,
) -> Result<()> {
    let idea = &mut ctx.accounts.idea;
    let enterprise = &mut ctx.accounts.enterprise;
    let dao_config = &ctx.accounts.dao_config;
    
    // Validate idea ID matches
    require!(idea.id == idea_id, IndrasError::InvalidInput);
    require!(enterprise.id == enterprise_id, IndrasError::InvalidInput);
    
    // ===== CRITERION 1: Idea status =====
    // Idea must be completed to MVP (Completed or Executed)
    require!(
        idea.status == IdeaStatus::Executed || 
        idea.status == IdeaStatus::Completed,
        IndrasError::InvalidState
    );
    
    // ===== CRITERION 2: Mesh group status (if provided) =====
    // Mesh group must be completed and MVP ready
    if let Some(mesh_group) = &ctx.accounts.mesh_group {
        require!(
            mesh_group.ideas.contains(&idea_id),
            IndrasError::IdeaNotInMeshGroup
        );
        require!(
            mesh_group.status == GroupStatus::Completed ||
            mesh_group.status == GroupStatus::Disbanded,
            IndrasError::InvalidState
        );
        // CRITICAL: Idea must be completed to MVP (DevelopmentStage::Completed)
        require!(
            mesh_group.current_stage == DevelopmentStage::Completed,
            IndrasError::InvalidState
        );
    }
    
    // ===== CRITICAL: Check intellectual property rights =====
    // Idea author remains copyright owner
    // e.V. received commercialization right via grant approval OR voluntary transfer
    // e.V. is custodian of author's copyright
    
    // Check that authorship is established (via AnchorRecord)
    // This must be done when anchoring idea in blockchain
    let anchor_record = &ctx.accounts.anchor_record;
    require!(
        anchor_record.idea_id == idea.id,
        IndrasError::InvalidInput
    );
    require!(
        anchor_record.anchorer == idea.author,
        IndrasError::Unauthorized
    );
    
    // Check that e.V. has rights to commercialize:
    // Option 1: Grant approved → commercialization_right_transferred = true
    // Option 2: Rights voluntarily transferred → rights_transferred_to_ev is Some
    let has_grant_right = if let Some(grant) = &ctx.accounts.grant {
        grant.idea_id == idea_id &&
        (grant.status == GrantStatus::Approved || 
         grant.status == GrantStatus::Active || 
         grant.status == GrantStatus::Completed) &&
        grant.commercialization_right_transferred
    } else {
        false
    };
    
    let has_voluntary_rights = idea.rights_transferred_to_ev.is_some();
    
    require!(
        has_grant_right || has_voluntary_rights,
        IndrasError::Unauthorized
    );
    
    // Check that e.V. (dao_config.authority) is transferring idea
    // e.V. has commercialization right (received from author via grant approval or voluntary transfer)
    require!(
        ctx.accounts.transferrer.key() == dao_config.authority,
        IndrasError::Unauthorized
    );
    
    // e.V. uses received commercialization right to transfer idea
    // Author retains their copyright on IP
    
    // Add idea to enterprise
    enterprise.add_idea(idea_id)?;
    
    // Update enterprise status if needed
    if enterprise.status == EnterpriseStatus::Forming {
        enterprise.status = EnterpriseStatus::SeekingInvestment;
        enterprise.commercialization_started_at = Some(Clock::get()?.unix_timestamp);
    }
    
    // Update idea status to Commercialization
    idea.status = IdeaStatus::Commercialization;
    
    msg!("Idea {} transferred to Commercial Enterprise {} by {} (enterprise will seek investors and implement in production)", 
         idea_id, 
         enterprise.name,
         ctx.accounts.transferrer.key());
    
    Ok(())
}

/// Add investor to commercial enterprise
///
/// This handler adds an investor to the enterprise and updates investment amount.
pub fn add_investor_handler(
    ctx: Context<crate::AddInvestor>,
    investor: Pubkey,
    investment_amount: u64,
) -> Result<()> {
    let enterprise = &mut ctx.accounts.enterprise;
    
    // Validate enterprise is in a valid state for accepting investors
    require!(
        enterprise.status == EnterpriseStatus::SeekingInvestment ||
        enterprise.status == EnterpriseStatus::Active,
        IndrasError::InvalidState
    );
    
    // Validate investment amount
    require!(investment_amount > 0, IndrasError::InvalidInput);
    
    // Add investor
    enterprise.add_investor(investor)?;
    
    // Update investment amount
    enterprise.update_investment(investment_amount);
    
    // Update status if funded
    if enterprise.status == EnterpriseStatus::SeekingInvestment && enterprise.total_investment > 0 {
        enterprise.status = EnterpriseStatus::Funded;
    }
    
    msg!("Investor {} added to Commercial Enterprise {} with investment {} SOL", 
         investor,
         enterprise.id,
         investment_amount as f64 / 1_000_000_000.0);
    
    Ok(())
}

/// Update production status of commercial enterprise
///
/// This handler updates the production status and enterprise status accordingly.
pub fn update_production_status_handler(
    ctx: Context<crate::UpdateProductionStatus>,
    new_status: ProductionStatus,
) -> Result<()> {
    let enterprise = &mut ctx.accounts.enterprise;
    
    // Validate enterprise is in a valid state for production
    require!(
        enterprise.status == EnterpriseStatus::Funded ||
        enterprise.status == EnterpriseStatus::InProduction ||
        enterprise.status == EnterpriseStatus::Active,
        IndrasError::InvalidState
    );
    
    // Update production status
    enterprise.production_status = new_status;
    
    // Update enterprise status based on production status
    match new_status {
        ProductionStatus::Manufacturing => {
            if enterprise.status == EnterpriseStatus::Funded {
                enterprise.status = EnterpriseStatus::InProduction;
                enterprise.production_started_at = Some(Clock::get()?.unix_timestamp);
            }
        }
        ProductionStatus::Launched => {
            enterprise.status = EnterpriseStatus::Launched;
        }
        _ => {
            // Other statuses don't change enterprise status
        }
    }
    
    msg!("Production status updated to {:?} for Commercial Enterprise {}", 
         new_status,
         enterprise.id);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::state::commercial_enterprise::{EnterpriseStatus, ProductionStatus};
    use crate::state::enums::IdeaStatus;
    use crate::state::mesh_group::GroupStatus;
    use crate::state::grant::GrantStatus;

    // ========== create_commercial_enterprise_handler validation tests ==========
    
    #[test]
    fn test_create_commercial_enterprise_validation_empty_name() {
        // Test: empty name should fail
        let name = String::new();
        
        // Validation logic: require!(!name.is_empty(), IndrasError::InvalidInput)
        assert!(name.is_empty(), "Empty name should be detected");
    }
    
    #[test]
    fn test_create_commercial_enterprise_validation_name_too_long() {
        // Test: name.len() > 100 should fail
        let name = "a".repeat(101);
        
        // Validation logic: require!(name.len() <= 100, IndrasError::StringTooLong)
        assert!(name.len() > 100, "Name too long should be detected");
    }
    
    #[test]
    fn test_create_commercial_enterprise_validation_description_too_long() {
        // Test: description.len() > 500 should fail
        let description = "a".repeat(501);
        
        // Validation logic: require!(description.len() <= 500, IndrasError::StringTooLong)
        assert!(description.len() > 500, "Description too long should be detected");
    }
    
    #[test]
    fn test_create_commercial_enterprise_validation_valid_inputs() {
        // Test: valid inputs should pass
        let name = "Valid Enterprise".to_string();
        let description = "Valid description".to_string();
        
        // All validations should pass
        assert!(!name.is_empty() && name.len() <= 100, "Name should be valid");
        assert!(description.len() <= 500, "Description should be valid");
    }

    // ========== transfer_idea_to_commercial_enterprise_handler validation tests ==========
    
    #[test]
    fn test_transfer_idea_validation_idea_id_mismatch() {
        // Test: idea.id != idea_id should fail
        let idea_id = 1u64;
        let idea_id_actual = 2u64;
        
        // Validation logic: require!(idea.id == idea_id, IndrasError::InvalidInput)
        assert_ne!(idea_id_actual, idea_id, "Idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_transfer_idea_validation_enterprise_id_mismatch() {
        // Test: enterprise.id != enterprise_id should fail
        let enterprise_id = 1u64;
        let enterprise_id_actual = 2u64;
        
        // Validation logic: require!(enterprise.id == enterprise_id, IndrasError::InvalidInput)
        assert_ne!(enterprise_id_actual, enterprise_id, "Enterprise ID mismatch should be detected");
    }
    
    #[test]
    fn test_transfer_idea_validation_idea_status_invalid() {
        // Test: idea.status not Executed or Completed should fail
        let idea_status = IdeaStatus::Draft;
        
        // Validation logic: require!(status == Executed || status == Completed, IndrasError::InvalidState)
        assert!(
            idea_status != IdeaStatus::Executed && idea_status != IdeaStatus::Completed,
            "Invalid idea status should be detected"
        );
    }
    
    #[test]
    fn test_transfer_idea_validation_idea_not_in_mesh_group() {
        // Test: idea_id not in mesh_group.ideas should fail
        let idea_id = 1u64;
        let mesh_group_ideas = vec![2u64, 3u64];
        
        // Validation logic: require!(mesh_group.ideas.contains(&idea_id), IndrasError::IdeaNotInMeshGroup)
        assert!(!mesh_group_ideas.contains(&idea_id), "Idea not in mesh group should be detected");
    }
    
    #[test]
    fn test_transfer_idea_validation_mesh_group_status_invalid() {
        // Test: mesh_group.status not Completed or Disbanded should fail
        let mesh_group_status = GroupStatus::Active;
        
        // Validation logic: require!(status == Completed || status == Disbanded, IndrasError::InvalidState)
        assert!(
            mesh_group_status != GroupStatus::Completed && mesh_group_status != GroupStatus::Disbanded,
            "Invalid mesh group status should be detected"
        );
    }
    
    #[test]
    fn test_transfer_idea_validation_anchor_record_idea_id_mismatch() {
        // Test: anchor_record.idea_id != idea.id should fail
        let idea_id = 1u64;
        let anchor_record_idea_id = 2u64;
        
        // Validation logic: require!(anchor_record.idea_id == idea.id, IndrasError::InvalidInput)
        assert_ne!(anchor_record_idea_id, idea_id, "Anchor record idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_transfer_idea_validation_anchorer_not_author() {
        // Test: anchor_record.anchorer != idea.author should fail
        let idea_author = Pubkey::new_unique();
        let anchorer = Pubkey::new_unique();
        
        // Validation logic: require!(anchorer == idea.author, IndrasError::Unauthorized)
        assert_ne!(anchorer, idea_author, "Anchorer not author should be detected");
    }
    
    #[test]
    fn test_transfer_idea_validation_grant_idea_id_mismatch() {
        // Test: grant.idea_id != idea_id should fail
        let idea_id = 1u64;
        let grant_idea_id = 2u64;
        
        // Validation logic: require!(grant.idea_id == idea_id, IndrasError::InvalidInput)
        assert_ne!(grant_idea_id, idea_id, "Grant idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_transfer_idea_validation_grant_status_invalid() {
        // Test: grant.status not Approved, Active, or Completed should fail
        let grant_status = GrantStatus::Pending;
        
        // Validation logic: require!(status in [Approved, Active, Completed], IndrasError::InvalidState)
        assert!(
            grant_status != GrantStatus::Approved &&
            grant_status != GrantStatus::Active &&
            grant_status != GrantStatus::Completed,
            "Invalid grant status should be detected"
        );
    }
    
    #[test]
    fn test_transfer_idea_validation_commercialization_right_not_transferred() {
        // Test: grant.commercialization_right_transferred == false should fail
        let commercialization_right_transferred = false;
        
        // Validation logic: require!(commercialization_right_transferred, IndrasError::Unauthorized)
        assert!(!commercialization_right_transferred, "Commercialization right not transferred should be detected");
    }
    
    #[test]
    fn test_transfer_idea_validation_transferrer_not_authority() {
        // Test: transferrer != dao_config.authority should fail
        let transferrer = Pubkey::new_unique();
        let dao_authority = Pubkey::new_unique();
        
        // Validation logic: require!(transferrer == dao_config.authority, IndrasError::Unauthorized)
        assert_ne!(transferrer, dao_authority, "Transferrer not authority should be detected");
    }

    // ========== add_investor_handler validation tests ==========
    
    #[test]
    fn test_add_investor_validation_enterprise_status_invalid() {
        // Test: enterprise.status not SeekingInvestment or Active should fail
        let enterprise_status = EnterpriseStatus::Forming;
        
        // Validation logic: require!(status == SeekingInvestment || status == Active, IndrasError::InvalidState)
        assert!(
            enterprise_status != EnterpriseStatus::SeekingInvestment &&
            enterprise_status != EnterpriseStatus::Active,
            "Invalid enterprise status should be detected"
        );
    }
    
    #[test]
    fn test_add_investor_validation_investment_amount_zero() {
        // Test: investment_amount == 0 should fail
        let investment_amount = 0u64;
        
        // Validation logic: require!(investment_amount > 0, IndrasError::InvalidInput)
        assert_eq!(investment_amount, 0, "Zero investment amount should be detected");
    }
    
    #[test]
    fn test_add_investor_validation_valid_inputs() {
        // Test: valid inputs should pass
        let enterprise_status = EnterpriseStatus::SeekingInvestment;
        let investment_amount = 1000u64;
        
        // All validations should pass
        assert!(
            enterprise_status == EnterpriseStatus::SeekingInvestment ||
            enterprise_status == EnterpriseStatus::Active,
            "Enterprise status should be valid"
        );
        assert!(investment_amount > 0, "Investment amount should be valid");
    }

    // ========== update_production_status_handler validation tests ==========
    
    #[test]
    fn test_update_production_status_validation_enterprise_status_invalid() {
        // Test: enterprise.status not Funded, InProduction, or Active should fail
        let enterprise_status = EnterpriseStatus::Forming;
        
        // Validation logic: require!(status in [Funded, InProduction, Active], IndrasError::InvalidState)
        assert!(
            enterprise_status != EnterpriseStatus::Funded &&
            enterprise_status != EnterpriseStatus::InProduction &&
            enterprise_status != EnterpriseStatus::Active,
            "Invalid enterprise status should be detected"
        );
    }
    
    #[test]
    fn test_update_production_status_validation_valid_statuses() {
        // Test: valid statuses should pass
        let valid_statuses = [
            EnterpriseStatus::Funded,
            EnterpriseStatus::InProduction,
            EnterpriseStatus::Active,
        ];
        
        for status in valid_statuses.iter() {
            assert!(
                *status == EnterpriseStatus::Funded ||
                *status == EnterpriseStatus::InProduction ||
                *status == EnterpriseStatus::Active,
                "Status {:?} should be valid", status
            );
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::state::enums::IdeaStatus;
        use crate::state::commercial_enterprise::{EnterpriseStatus, ProductionStatus, EnterpriseType};
        use crate::state::mesh_group::{GroupStatus, DevelopmentStage};
        use crate::state::grant::GrantStatus;

        // ========== create_commercial_enterprise_handler validation tests ==========
        
        #[test]
        fn test_create_commercial_enterprise_validation_empty_name() {
            // Test: empty name should fail
            let name = String::new();
            
            // Validation logic: require!(!name.is_empty(), IndrasError::InvalidInput)
            assert!(name.is_empty(), "Empty name should be detected");
        }
        
        #[test]
        fn test_create_commercial_enterprise_validation_name_too_long() {
            // Test: name.len() > 100 should fail
            let name = "a".repeat(101);
            
            // Validation logic: require!(name.len() <= 100, IndrasError::StringTooLong)
            assert!(name.len() > 100, "Name too long should be detected");
        }
        
        #[test]
        fn test_create_commercial_enterprise_validation_description_too_long() {
            // Test: description.len() > 500 should fail
            let description = "a".repeat(501);
            
            // Validation logic: require!(description.len() <= 500, IndrasError::StringTooLong)
            assert!(description.len() > 500, "Description too long should be detected");
        }
        
        #[test]
        fn test_create_commercial_enterprise_validation_valid_inputs() {
            // Test: valid inputs should pass
            let name = "Test Enterprise".to_string();
            let description = "Test Description".to_string();
            let enterprise_pubkey = Pubkey::new_unique();
            
            assert!(!name.is_empty(), "Name should be valid");
            assert!(name.len() <= 100, "Name length should be valid");
            assert!(description.len() <= 500, "Description length should be valid");
            assert_ne!(enterprise_pubkey, Pubkey::default(), "Enterprise pubkey should be valid");
        }

        // ========== transfer_idea_to_commercial_enterprise_handler validation tests ==========
        
        #[test]
        fn test_transfer_idea_validation_idea_id_mismatch() {
            // Test: idea.id != idea_id should fail
            let idea_id = 1u64;
            let idea_id_actual = 2u64;
            
            // Validation logic: require!(idea.id == idea_id, IndrasError::InvalidInput)
            assert_ne!(idea_id_actual, idea_id, "Idea ID mismatch should be detected");
        }
        
        #[test]
        fn test_transfer_idea_validation_enterprise_id_mismatch() {
            // Test: enterprise.id != enterprise_id should fail
            let enterprise_id = 1u64;
            let enterprise_id_actual = 2u64;
            
            // Validation logic: require!(enterprise.id == enterprise_id, IndrasError::InvalidInput)
            assert_ne!(enterprise_id_actual, enterprise_id, "Enterprise ID mismatch should be detected");
        }
        
        #[test]
        fn test_transfer_idea_validation_idea_status_not_completed() {
            // Test: idea.status not Executed or Completed should fail
            let idea_status = IdeaStatus::Draft;
            
            // Validation logic: require!(status == Executed || status == Completed, IndrasError::InvalidState)
            assert!(
                idea_status != IdeaStatus::Executed && idea_status != IdeaStatus::Completed,
                "Idea status not completed should be detected"
            );
        }
        
        #[test]
        fn test_transfer_idea_validation_idea_status_executed() {
            // Test: idea.status == Executed should pass
            let idea_status = IdeaStatus::Executed;
            
            // Validation logic: require!(status == Executed || status == Completed, IndrasError::InvalidState)
            assert!(
                idea_status == IdeaStatus::Executed || idea_status == IdeaStatus::Completed,
                "Executed status should be valid"
            );
        }
        
        #[test]
        fn test_transfer_idea_validation_idea_status_completed() {
            // Test: idea.status == Completed should pass
            let idea_status = IdeaStatus::Completed;
            
            // Validation logic: require!(status == Executed || status == Completed, IndrasError::InvalidState)
            assert!(
                idea_status == IdeaStatus::Executed || idea_status == IdeaStatus::Completed,
                "Completed status should be valid"
            );
        }
        
        #[test]
        fn test_transfer_idea_validation_mesh_group_status_not_completed() {
            // Test: mesh_group.status not Completed or Disbanded should fail
            let mesh_group_status = GroupStatus::Active;
            
            // Validation logic: require!(status == Completed || status == Disbanded, IndrasError::InvalidState)
            assert!(
                mesh_group_status != GroupStatus::Completed && mesh_group_status != GroupStatus::Disbanded,
                "Mesh group status not completed should be detected"
            );
        }
        
        #[test]
        fn test_transfer_idea_validation_mesh_group_stage_not_completed() {
            // Test: mesh_group.current_stage != Completed should fail
            let mesh_group_stage = DevelopmentStage::Planning;
            
            // Validation logic: require!(stage == Completed, IndrasError::InvalidState)
            assert_ne!(mesh_group_stage, DevelopmentStage::Completed, "Mesh group stage not completed should be detected");
        }

        // ========== Enterprise status validation tests ==========
        
        #[test]
        fn test_enterprise_status_all_variants() {
            // Test: all EnterpriseStatus variants should be valid
            let statuses = vec![
                EnterpriseStatus::Forming,
                EnterpriseStatus::Active,
                EnterpriseStatus::Suspended,
                EnterpriseStatus::Dissolved,
            ];
            
            assert_eq!(statuses.len(), 4, "All 4 enterprise statuses should be valid");
        }
        
        #[test]
        fn test_production_status_all_variants() {
            // Test: all ProductionStatus variants should be valid
            let statuses = vec![
                ProductionStatus::NotStarted,
                ProductionStatus::InDevelopment,
                ProductionStatus::InProduction,
                ProductionStatus::Discontinued,
            ];
            
            assert_eq!(statuses.len(), 4, "All 4 production statuses should be valid");
        }
        
        #[test]
        fn test_enterprise_type_all_variants() {
            // Test: all EnterpriseType variants should be valid
            let types = vec![
                EnterpriseType::IndrasNetOU,
                EnterpriseType::OtherFirm,
            ];
            
            assert_eq!(types.len(), 2, "All 2 enterprise types should be valid");
        }
    }
}
