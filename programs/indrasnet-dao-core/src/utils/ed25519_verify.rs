//! ed25519 Signature Verification Utilities
//!
//! Helper functions for verifying ed25519 signatures via CPI to ed25519_program.
//! Used for embedding signatures, distance bundle signatures, and cluster signatures.

use anchor_lang::prelude::*;
#[cfg(not(feature = "skip-ed25519-verify"))]
use anchor_lang::solana_program::{
    instruction::Instruction,
    program::invoke,
};
#[cfg(not(feature = "skip-ed25519-verify"))]
use std::str::FromStr;

#[cfg(not(feature = "skip-ed25519-verify"))]
const PUBKEY_SERIALIZED_SIZE: usize = 32;
#[cfg(not(feature = "skip-ed25519-verify"))]
const SIGNATURE_SERIALIZED_SIZE: usize = 64;
#[cfg(not(feature = "skip-ed25519-verify"))]
const SIGNATURE_OFFSETS_SERIALIZED_SIZE: usize = 14;
#[cfg(not(feature = "skip-ed25519-verify"))]
const SIGNATURE_OFFSETS_START: usize = 2;
#[cfg(not(feature = "skip-ed25519-verify"))]
const DATA_START: usize = SIGNATURE_OFFSETS_SERIALIZED_SIZE + SIGNATURE_OFFSETS_START;

#[cfg(not(feature = "skip-ed25519-verify"))]
fn build_ed25519_instruction(
    message: &[u8],
    signature: &[u8; 64],
    public_key: &Pubkey,
) -> Result<Instruction> {
    let mut data = Vec::with_capacity(
        DATA_START
            .saturating_add(PUBKEY_SERIALIZED_SIZE)
            .saturating_add(SIGNATURE_SERIALIZED_SIZE)
            .saturating_add(message.len()),
    );

    let num_signatures: u8 = 1;
    let public_key_offset = DATA_START;
    let signature_offset = public_key_offset.saturating_add(PUBKEY_SERIALIZED_SIZE);
    let message_data_offset = signature_offset.saturating_add(SIGNATURE_SERIALIZED_SIZE);
    let instruction_index = u16::MAX;

    data.push(num_signatures);
    data.push(0);
    data.extend_from_slice(&(signature_offset as u16).to_le_bytes());
    data.extend_from_slice(&instruction_index.to_le_bytes());
    data.extend_from_slice(&(public_key_offset as u16).to_le_bytes());
    data.extend_from_slice(&instruction_index.to_le_bytes());
    data.extend_from_slice(&(message_data_offset as u16).to_le_bytes());
    data.extend_from_slice(&(message.len() as u16).to_le_bytes());
    data.extend_from_slice(&instruction_index.to_le_bytes());

    data.extend_from_slice(public_key.as_ref());
    data.extend_from_slice(signature);
    data.extend_from_slice(message);

    let program_id = Pubkey::from_str("Ed25519SigVerify111111111111111111111111111")
        .map_err(|_| crate::error::IndrasError::InvalidInput)?;
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data,
    })
}

/// Verify ed25519 signature via CPI to ed25519_program
///
/// # Arguments
/// * `message_hash` - SHA256 hash of the message (32 bytes)
/// * `public_key` - ed25519 public key (32 bytes)
/// * `signature` - ed25519 signature (64 bytes)
///
/// # Returns
/// * `Ok(())` if signature is valid
/// * `Err(CoreError::InvalidInput)` if signature verification fails
///
/// # Feature Flags
/// * `skip-ed25519-verify` - Skips signature verification (for test validator)
#[cfg(feature = "skip-ed25519-verify")]
#[allow(unused_variables)]
pub fn verify_ed25519_signature(
    message_hash: &[u8; 32],
    public_key: &Pubkey,
    signature: &[u8; 64],
) -> Result<()> {
    msg!("⚠️  Signature verification UNCONDITIONALLY SKIPPED for testing purposes.");
    msg!("⚠️  Message hash: {:?}", message_hash);
    msg!("⚠️  Public key: {}", public_key);
    msg!("⚠️  Signature: {:?}", signature);
    Ok(())
}

#[cfg(not(feature = "skip-ed25519-verify"))]
pub fn verify_ed25519_signature(
    message_hash: &[u8; 32],
    public_key: &Pubkey,
    signature: &[u8; 64],
) -> Result<()> {
    let ix = build_ed25519_instruction(message_hash, signature, public_key)?;
    invoke(&ix, &[]).map_err(|_| crate::error::IndrasError::InvalidInput)?;
    Ok(())
}

#[cfg(all(test, feature = "skip-ed25519-verify"))]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    #[test]
    fn test_verify_ed25519_signature_always_passes() {
        // With skip-ed25519-verify feature, verification always passes
        let message_hash = [1u8; 32];
        let public_key = Pubkey::new_unique();
        let signature = [2u8; 64];
        
        let result = verify_ed25519_signature(&message_hash, &public_key, &signature);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_ed25519_signature_different_inputs() {
        // Should pass regardless of input values
        let message_hash1 = [1u8; 32];
        let message_hash2 = [2u8; 32];
        let public_key = Pubkey::new_unique();
        let signature = [0u8; 64];
        
        let result1 = verify_ed25519_signature(&message_hash1, &public_key, &signature);
        let result2 = verify_ed25519_signature(&message_hash2, &public_key, &signature);
        
        // Both should pass (skip-ed25519-verify feature)
        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_verify_ed25519_signature_zero_hash() {
        let message_hash = [0u8; 32];
        let public_key = Pubkey::new_unique();
        let signature = [0u8; 64];
        
        let result = verify_ed25519_signature(&message_hash, &public_key, &signature);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_ed25519_signature_max_hash() {
        let message_hash = [255u8; 32];
        let public_key = Pubkey::new_unique();
        let signature = [255u8; 64];
        
        let result = verify_ed25519_signature(&message_hash, &public_key, &signature);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_ed25519_signature_different_public_keys() {
        let message_hash = [1u8; 32];
        let public_key1 = Pubkey::new_unique();
        let public_key2 = Pubkey::new_unique();
        let signature = [0u8; 64];
        
        let result1 = verify_ed25519_signature(&message_hash, &public_key1, &signature);
        let result2 = verify_ed25519_signature(&message_hash, &public_key2, &signature);
        
        // Both should pass (skip-ed25519-verify feature)
        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_verify_ed25519_signature_multiple_calls() {
        let message_hash = [1u8; 32];
        let public_key = Pubkey::new_unique();
        let signature = [0u8; 64];
        
        // Multiple calls should all pass
        for _ in 0..10 {
            let result = verify_ed25519_signature(&message_hash, &public_key, &signature);
            assert!(result.is_ok());
        }
    }
}
