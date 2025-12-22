//! Collective AI Functions handlers
//!
//! Handlers for Collective AI Functions instructions
//!
//! On-chain: Validation and state management
//! Off-chain: Actual collective AI coordination (in separate service)

use anchor_lang::prelude::*;
use crate::collective_ai_functions::neuron::onchain as neuron_onchain;
use crate::collective_ai_functions::synapse::onchain as synapse_onchain;
use crate::collective_ai_functions::memory::onchain as memory_onchain;
use crate::collective_ai_functions::consciousness::onchain as consciousness_onchain;

/// Initialize neuron
///
/// Creates a neuron in the collective AI network
///
/// # Compute Units
/// Recommended: 30,000 CU
/// - Validation: ~5,000 CU
/// - Account initialization: ~25,000 CU
pub fn initialize_neuron_handler(
    ctx: Context<crate::InitializeNeuron>,
    neuron_id: u64,
) -> Result<()> {
    let neuron = &mut ctx.accounts.neuron;
    let owner = ctx.accounts.authority.key();
    let current_time = Clock::get()?.unix_timestamp;
    
    neuron_onchain::initialize_neuron_metadata(
        neuron,
        neuron_id,
        owner,
        current_time,
        ctx.bumps.neuron,
    )
}

/// Activate neuron
///
/// Activates a neuron in the collective AI network
///
/// # Compute Units
/// Recommended: 15,000 CU
/// - Validation: ~5,000 CU
/// - State update: ~10,000 CU
pub fn activate_neuron_handler(
    ctx: Context<crate::ActivateNeuron>,
) -> Result<()> {
    let neuron = &mut ctx.accounts.neuron;
    let current_time = Clock::get()?.unix_timestamp;
    
    neuron_onchain::activate_neuron(neuron, current_time)
}

/// Initialize synapse
///
/// Creates a connection between two neurons
///
/// # Compute Units
/// Recommended: 30,000 CU
/// - Validation: ~5,000 CU
/// - Account initialization: ~25,000 CU
pub fn initialize_synapse_handler(
    ctx: Context<crate::InitializeSynapse>,
    synapse_id: u64,
    source_neuron_id: u64,
    target_neuron_id: u64,
    synapse_type: crate::collective_ai_functions::SynapseType,
    weight: u8,
) -> Result<()> {
    let synapse = &mut ctx.accounts.synapse;
    let current_time = Clock::get()?.unix_timestamp;
    
    synapse_onchain::initialize_synapse_metadata(
        synapse,
        synapse_id,
        source_neuron_id,
        target_neuron_id,
        synapse_type,
        weight,
        current_time,
        ctx.bumps.synapse,
    )
}

/// Initialize memory
///
/// Creates a memory record in the collective AI
///
/// # Compute Units
/// Recommended: 35,000 CU
/// - Validation: ~10,000 CU
/// - Account initialization: ~25,000 CU
pub fn initialize_memory_handler(
    ctx: Context<crate::InitializeMemory>,
    memory_id: u64,
    memory_type: crate::collective_ai_functions::MemoryType,
    data_uri: String,
    data_hash: [u8; 32],
    data_size: u64,
) -> Result<()> {
    let memory = &mut ctx.accounts.memory;
    let current_time = Clock::get()?.unix_timestamp;
    
    memory_onchain::initialize_memory_metadata(
        memory,
        memory_id,
        memory_type,
        data_uri,
        data_hash,
        data_size,
        current_time,
        ctx.bumps.memory,
    )
}

/// Initialize consciousness
///
/// Creates a consciousness state record
///
/// # Compute Units
/// Recommended: 30,000 CU
/// - Validation: ~5,000 CU
/// - Account initialization: ~25,000 CU
pub fn initialize_consciousness_handler(
    ctx: Context<crate::InitializeConsciousness>,
    consciousness_id: u64,
) -> Result<()> {
    let consciousness = &mut ctx.accounts.consciousness;
    let current_time = Clock::get()?.unix_timestamp;
    
    consciousness_onchain::initialize_consciousness_metadata(
        consciousness,
        consciousness_id,
        current_time,
        ctx.bumps.consciousness,
    )
}

/// Update consciousness level
///
/// Updates the consciousness level based on network activity
///
/// # Compute Units
/// Recommended: 20,000 CU
/// - Validation: ~5,000 CU
/// - State update: ~15,000 CU
pub fn update_consciousness_level_handler(
    ctx: Context<crate::UpdateConsciousnessLevel>,
    active_neuron_count: u32,
    active_synapse_count: u32,
) -> Result<()> {
    let consciousness = &mut ctx.accounts.consciousness;
    let current_time = Clock::get()?.unix_timestamp;
    
    consciousness_onchain::update_consciousness_level(
        consciousness,
        active_neuron_count,
        active_synapse_count,
        current_time,
    )
}
