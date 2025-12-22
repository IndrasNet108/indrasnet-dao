// Accounts structures for Collective AI Functions instructions
// NOTE: anchor_lang types are imported in lib.rs before include!()

use crate::collective_ai_functions::neuron::NeuronMetadata;
use crate::collective_ai_functions::synapse::SynapseMetadata;
use crate::collective_ai_functions::memory::MemoryMetadata;
use crate::collective_ai_functions::consciousness::ConsciousnessMetadata;

#[derive(Accounts)]
#[instruction(neuron_id: u64)]
pub struct InitializeNeuron<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + NeuronMetadata::INIT_SPACE,
        seeds = [b"neuron", neuron_id.to_le_bytes().as_ref()],
        bump
    )]
    pub neuron: Account<'info, NeuronMetadata>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(neuron_id: u64)]
pub struct ActivateNeuron<'info> {
    #[account(
        mut,
        seeds = [b"neuron", neuron_id.to_le_bytes().as_ref()],
        bump = neuron.bump
    )]
    pub neuron: Account<'info, NeuronMetadata>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(synapse_id: u64)]
pub struct InitializeSynapse<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + SynapseMetadata::INIT_SPACE,
        seeds = [b"synapse", synapse_id.to_le_bytes().as_ref()],
        bump
    )]
    pub synapse: Account<'info, SynapseMetadata>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(memory_id: u64)]
pub struct InitializeMemory<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + MemoryMetadata::INIT_SPACE,
        seeds = [b"memory", memory_id.to_le_bytes().as_ref()],
        bump
    )]
    pub memory: Account<'info, MemoryMetadata>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(consciousness_id: u64)]
pub struct InitializeConsciousness<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ConsciousnessMetadata::INIT_SPACE,
        seeds = [b"consciousness", consciousness_id.to_le_bytes().as_ref()],
        bump
    )]
    pub consciousness: Account<'info, ConsciousnessMetadata>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(consciousness_id: u64)]
pub struct UpdateConsciousnessLevel<'info> {
    #[account(
        mut,
        seeds = [b"consciousness", consciousness_id.to_le_bytes().as_ref()],
        bump = consciousness.bump
    )]
    pub consciousness: Account<'info, ConsciousnessMetadata>,
    
    pub authority: Signer<'info>,
}
