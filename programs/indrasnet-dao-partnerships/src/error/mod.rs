//! Error types for the IndrasNet DAO Partnerships program

use anchor_lang::prelude::*;

#[error_code]
pub enum IndrasError {
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Invalid input")]
    InvalidInput,
    
    #[msg("Partnership not found")]
    PartnershipNotFound,
    
    #[msg("Partnership operation failed")]
    PartnershipOperationFailed,
    
    #[msg("String too long")]
    StringTooLong,
    
    #[msg("Amount too small")]
    AmountTooSmall,
    
    #[msg("Amount too large")]
    AmountTooLarge,
    
    #[msg("Overflow")]
    Overflow,
    
    #[msg("Division by zero")]
    DivisionByZero,
    
    #[msg("Invalid state")]
    InvalidState,
    
    #[msg("Invalid score")]
    InvalidScore,
    
    #[msg("Arithmetic underflow")]
    Underflow,
    
    #[msg("Not implemented")]
    NotImplemented,
}
