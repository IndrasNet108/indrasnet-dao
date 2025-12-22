//! Partnership instruction handlers
//!
//! Handlers for Partnership instructions
//!
//! On-chain: Validation and state management
//! Off-chain: Complex analytics and reporting (in separate service)

use anchor_lang::prelude::*;
use crate::partnerships::types::*;
use crate::partnerships::manager::PartnershipManagerImpl;
use crate::constants::PRECISION_FACTOR;

/// Create partnership
///
/// Creates a new partnership record
///
/// # Compute Units
/// Recommended: 50,000 CU
/// - Validation: ~15,000 CU
/// - Account initialization: ~35,000 CU
pub fn create_partnership_handler(
    ctx: Context<crate::CreatePartnership>,
    partnership_id: u64,
    partner_name: String,
    partnership_type: PartnershipType,
    tier: PartnershipTier,
    description: String,
    terms: PartnershipTerms,
    metadata_uri: String,
) -> Result<()> {
    let partnership = &mut ctx.accounts.partnership;
    let creator = ctx.accounts.authority.key();
    let partner_address = ctx.accounts.partner.key();
    let current_time = Clock::get()?.unix_timestamp;

    require!(
        ctx.accounts.role_registry.is_partnership_admin(&creator),
        crate::error::IndrasError::Unauthorized
    );
    
    // Validate terms using PartnershipManagerImpl
    PartnershipManagerImpl::validate_terms(&terms, &ctx.accounts.config.config)?;
    
    partnership.partnership_id = partnership_id;
    partnership.partner_name = partner_name;
    partnership.partnership_type = partnership_type;
    partnership.tier = tier;
    partnership.description = description;
    partnership.terms = terms;
    partnership.status = PartnershipStatus::Draft;
    partnership.partner_address = partner_address;
    partnership.creator = creator;
    partnership.created_at = current_time;
    partnership.updated_at = current_time;
    partnership.version = 1;
    partnership.metadata_uri = metadata_uri;
    partnership.bump = ctx.bumps.partnership;
    
    msg!("Partnership {} created (type: {:?}, tier: {:?})", partnership_id, partnership_type, tier);
    
    Ok(())
}

/// Initialize partnership configuration
pub fn initialize_partnership_config_handler(
    ctx: Context<crate::InitializePartnershipConfig>,
    max_partnerships: u64,
    min_partnership_value: u64,
    max_partnership_value: u64,
    default_revenue_share: u64,
    governance_threshold: u64,
    auto_renewal_enabled: bool,
    risk_threshold: u64,
) -> Result<()> {
    let config_account = &mut ctx.accounts.partnership_config;
    let authority = ctx.accounts.authority.key();

    PartnershipManagerImpl::initialize_system(
        &mut config_account.config,
        max_partnerships,
        min_partnership_value,
        max_partnership_value,
    )?;

    require!(
        default_revenue_share <= PRECISION_FACTOR,
        crate::error::IndrasError::InvalidScore
    );
    require!(
        governance_threshold <= PRECISION_FACTOR,
        crate::error::IndrasError::InvalidScore
    );

    config_account.config.default_revenue_share = default_revenue_share;
    config_account.config.governance_threshold = governance_threshold;
    config_account.config.auto_renewal_enabled = auto_renewal_enabled;
    config_account.config.risk_threshold = risk_threshold;
    config_account.authority = authority;
    config_account.bump = ctx.bumps.partnership_config;

    msg!("Partnership config initialized by {}", authority);

    Ok(())
}

/// Update partnership
///
/// Updates an existing partnership
///
/// # Compute Units
/// Recommended: 30,000 CU
/// - Validation: ~10,000 CU
/// - State update: ~20,000 CU
pub fn update_partnership_handler(
    ctx: Context<crate::UpdatePartnership>,
    partner_name: Option<String>,
    description: Option<String>,
    terms: Option<PartnershipTerms>,
    status: Option<PartnershipStatus>,
) -> Result<()> {
    let partnership = &mut ctx.accounts.partnership;
    let current_time = Clock::get()?.unix_timestamp;

    let updater = ctx.accounts.authority.key();
    require!(
        ctx.accounts.role_registry.is_partnership_admin(&updater),
        crate::error::IndrasError::Unauthorized
    );
    
    if let Some(name) = partner_name {
        require!(name.len() <= 200, crate::error::IndrasError::StringTooLong);
        partnership.partner_name = name;
    }
    
    if let Some(desc) = description {
        require!(desc.len() <= 1000, crate::error::IndrasError::StringTooLong);
        partnership.description = desc;
    }
    
    if let Some(new_terms) = terms {
        PartnershipManagerImpl::validate_terms(&new_terms, &ctx.accounts.config.config)?;
        partnership.update_terms(new_terms, current_time);
    }
    
    if let Some(new_status) = status {
        partnership.update_status(new_status, current_time);
    }
    
    msg!("Partnership {} updated (version: {})", partnership.partnership_id, partnership.version);
    
    Ok(())
}
