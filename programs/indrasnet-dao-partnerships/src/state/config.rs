//! Partnership configuration state structure

use anchor_lang::prelude::*;
use crate::partnerships::types::PartnershipConfig;

/// Partnership configuration account
#[account]
#[derive(InitSpace)]
pub struct PartnershipConfigAccount {
    /// Configuration data
    pub config: PartnershipConfig,
    /// Authority that can update config
    pub authority: Pubkey,
    /// Bump seed for PDA
    pub bump: u8,
}
