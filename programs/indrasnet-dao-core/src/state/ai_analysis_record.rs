//! AI analysis registration record
//!
//! Core-owned record proving AI analysis was produced by the AI program via CPI.

use anchor_lang::prelude::*;

/// Core-owned record linking an AI program analysis to an idea.
#[account]
#[derive(InitSpace)]
pub struct AIAnalysisRecord {
    pub idea_id: u64,
    pub analysis: Pubkey,
    pub ai_program: Pubkey,
    pub registered_at: i64,
    pub bump: u8,
}
