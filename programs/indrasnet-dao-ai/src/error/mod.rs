//! Error types for the IndrasNet DAO AI program

use anchor_lang::prelude::*;

#[error_code]
pub enum IndrasError {
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Invalid input")]
    InvalidInput,
    
    #[msg("AI operation failed")]
    AIOperationFailed,
    
    #[msg("Model not found")]
    ModelNotFound,
    
    #[msg("String too long")]
    StringTooLong,
    
    #[msg("Data too large")]
    DataTooLarge,
    
    #[msg("Invalid timestamp")]
    InvalidTimestamp,
    
    #[msg("Invalid operation")]
    InvalidOperation,
    
    #[msg("Invalid state")]
    InvalidState,
    
    #[msg("Invalid score")]
    InvalidScore,
    
    #[msg("Invalid program - account owner does not match expected program")]
    InvalidProgram,
}
