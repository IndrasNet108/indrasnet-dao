//! Error types for the IndrasNet DAO Security program

use anchor_lang::prelude::*;

#[error_code]
pub enum IndrasError {
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Invalid input")]
    InvalidInput,
    
    #[msg("Security violation")]
    SecurityViolation,
    
    #[msg("Authentication failed")]
    AuthenticationFailed,
    
    #[msg("Access denied")]
    AccessDenied,
    
    #[msg("Invalid state")]
    InvalidState,
    
    #[msg("Amount too small")]
    AmountTooSmall,
    
    #[msg("Amount too large")]
    AmountTooLarge,
    
    #[msg("String too long")]
    StringTooLong,
    
    #[msg("Not implemented")]
    NotImplemented,
    
    #[msg("Arithmetic overflow")]
    Overflow,
    
    #[msg("Invalid score")]
    InvalidScore,
}
