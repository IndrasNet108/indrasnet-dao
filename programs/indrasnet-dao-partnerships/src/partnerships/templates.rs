//! Templates module
//!
//! Partnership templates management
//!
//! On-chain: Metadata for templates
//! Off-chain: Actual template processing, generation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Template type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TemplateType {
    /// Partnership agreement template
    Agreement,
    /// Contract template
    Contract,
    /// Proposal template
    Proposal,
    /// Custom template
    Custom,
}

/// Partnership template metadata (on-chain)
///
/// Stores metadata for partnership templates
#[account]
#[derive(InitSpace)]
pub struct PartnershipTemplateMetadata {
    /// Template ID
    pub template_id: u64,
    /// Template type
    pub template_type: TemplateType,
    /// Created at
    pub created_at: i64,
    /// Template data hash
    pub template_data_hash: [u8; 32],
    /// Template URI
    #[max_len(200)]
    pub template_uri: String,
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for templates
pub mod onchain {
    use super::*;

    /// Initialize partnership template
    pub fn initialize_partnership_template(
        template: &mut PartnershipTemplateMetadata,
        template_id: u64,
        template_type: TemplateType,
        template_data_hash: [u8; 32],
        template_uri: String,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(template_id > 0, IndrasError::InvalidInput);
        require!(template_uri.len() <= 200, IndrasError::InvalidInput);
        
        template.template_id = template_id;
        template.template_type = template_type;
        template.created_at = current_time;
        template.template_data_hash = template_data_hash;
        template.template_uri = template_uri;
        template.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for templates
pub mod offchain {
    /// Generate template
    pub fn generate_template(_template_id: u64) -> Vec<u8> {
        // Implementation in off-chain service
        vec![]
    }
}
