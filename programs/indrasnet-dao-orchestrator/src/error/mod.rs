//! Error types for the IndrasNet DAO Orchestrator program

use anchor_lang::prelude::*;

#[error_code]
pub enum IndrasError {
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Invalid input")]
    InvalidInput,
    
    #[msg("Orchestration failed")]
    OrchestrationFailed,
    
    #[msg("CPI call failed")]
    CpiCallFailed,
    
    #[msg("String too long")]
    StringTooLong,
    
    #[msg("Amount too large")]
    AmountTooLarge,
}
