//! Test Fixtures for Solana Runtime
//!
//! This module provides fixtures and helpers for testing instructions
//! with solana-program-test, allowing real function calls and coverage.

use solana_program_test::*;
use solana_sdk::{
    account::Account,
    pubkey::Pubkey as SdkPubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use anchor_lang::prelude::Pubkey;
use anyhow::Result;

/// Program ID for testing
pub fn program_id() -> SdkPubkey {
    // Convert Anchor Pubkey to SDK Pubkey
    let program_id_bytes: [u8; 32] = crate::ID.to_bytes();
    SdkPubkey::from(program_id_bytes)
}

/// Test fixture for Solana program test environment
pub struct TestFixture {
    pub program_id: SdkPubkey,
    pub context: ProgramTestContext,
    pub authority: Keypair,
    pub user: Keypair,
}

impl TestFixture {
    /// Initialize test fixture with program test environment
    pub async fn new() -> Result<Self> {
        // Convert Anchor Pubkey to SDK Pubkey
        // Anchor Pubkey can be converted via its bytes
        let program_id_bytes: [u8; 32] = crate::ID.to_bytes();
        let program_id_sdk = SdkPubkey::from(program_id_bytes);
        
        // Use the compiled BPF program when available to avoid processor type mismatches.
        let mut program_test = ProgramTest::default();
        program_test.prefer_bpf(true);
        let program_path = std::path::Path::new("target/deploy/indrasnet_dao_core.so");
        if program_path.exists() {
            if let Some(out_dir) = program_path.parent() {
                std::env::set_var("BPF_OUT_DIR", out_dir);
            }
            program_test.add_program("indrasnet_dao_core", program_id_sdk, None);
        } else {
            return Err(anyhow::anyhow!(
                "Program binary not found at {}. Run `anchor build` before tests.",
                program_path.display()
            ));
        }
        
        let mut context = program_test.start_with_context().await;
        // Note: In solana-program-test, payer is already a Keypair
        // We can't clone it directly, so we'll use it as-is
        // For tests, we can create a new authority or use the payer
        let authority = Keypair::new();
        let user = Keypair::new();
        
        // Fund authority and user from the test payer.
        let lamports = 10_000_000_000; // 10 SOL
        let instructions = vec![
            system_instruction::transfer(&context.payer.pubkey(), &authority.pubkey(), lamports),
            system_instruction::transfer(&context.payer.pubkey(), &user.pubkey(), lamports),
        ];
        
        let transaction = Transaction::new_signed_with_payer(
            &instructions,
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );
        
        context
            .banks_client
            .process_transaction(transaction)
            .await
            .map_err(|e| anyhow::anyhow!("Airdrop failed: {:?}", e))?;
        
        Ok(Self {
            program_id: program_id_sdk,
            context,
            authority,
            user,
        })
    }
    
    /// Get mutable reference to context
    pub fn context_mut(&mut self) -> &mut ProgramTestContext {
        &mut self.context
    }
}

/// Helper to create a mock account with data
pub fn create_mock_account<T: anchor_lang::AccountSerialize>(
    owner: &SdkPubkey,
    lamports: u64,
    data: &T,
) -> Result<Account> {
    let mut account_data = Vec::new();
    data.try_serialize(&mut account_data)
        .map_err(|e| anyhow::anyhow!("Serialization failed: {:?}", e))?;
    
    Ok(Account {
        lamports,
        data: account_data,
        owner: *owner,
        executable: false,
        rent_epoch: 0,
    })
}

/// Helper to create a mock account with raw data
pub fn create_mock_account_raw(
    owner: &SdkPubkey,
    lamports: u64,
    data: Vec<u8>,
) -> Account {
    Account {
        lamports,
        data,
        owner: *owner,
        executable: false,
        rent_epoch: 0,
    }
}

/// Helper to find PDA
pub fn find_pda(seeds: &[&[u8]], program_id: &SdkPubkey) -> (SdkPubkey, u8) {
    SdkPubkey::find_program_address(seeds, program_id)
}

/// Helper to convert Account to AccountSharedData for set_account
pub fn account_to_shared(account: Account) -> solana_sdk::account::AccountSharedData {
    solana_sdk::account::AccountSharedData::from(account)
}

/// Helper to get Anchor Pubkey from Keypair
pub fn get_pubkey_from_keypair(keypair: &Keypair) -> Pubkey {
    let sdk_pubkey = keypair.pubkey();
    let bytes: [u8; 32] = sdk_pubkey.to_bytes();
    Pubkey::try_from(bytes.as_ref())
        .unwrap_or_else(|_| Pubkey::default())
}

/// Helper to create account with serialized data (with Anchor discriminator)
pub fn create_account_with_data<T: anchor_lang::AccountSerialize>(
    owner: &SdkPubkey,
    data: &T,
) -> Result<Account> {
    let mut serialized = Vec::new();
    data.try_serialize(&mut serialized)
        .map_err(|e| anyhow::anyhow!("Serialization failed: {:?}", e))?;
    
    // Add discriminator (8 bytes) - for Anchor accounts
    let mut account_data = vec![0u8; 8];
    account_data.extend_from_slice(&serialized);
    
    Ok(Account {
        lamports: 1_000_000_000, // 1 SOL
        data: account_data,
        owner: *owner,
        executable: false,
        rent_epoch: 0,
    })
}
