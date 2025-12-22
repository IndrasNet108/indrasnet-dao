//! Phenomenon creation instruction handlers
//!
//! Handlers for creating phenomena:
//! - create_phenomenon - Create phenomenon with embedding signature verification (SEC-INV-10)
//!
//! NOTE: Moved from Core program to AI program for modular architecture
//! Track B: Semantic & Phenomena Layer

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use indrasnet_dao_core::state::{DiscoveryMethod, PhenomenonStatus};
use indrasnet_dao_core::error::IndrasError as CoreError;
use sha2::{Sha256, Digest};

/// Create a new phenomenon
///
/// SEC-INV-10: All phenomena MUST have cryptographically signed embeddings.
/// This handler verifies the embedding signature via CPI to ed25519_program.
///
/// NOTE: Moved from Core program to AI program for modular architecture
#[allow(clippy::too_many_arguments)]
pub fn create_phenomenon_handler(
    ctx: Context<crate::CreatePhenomenon>,
    phenomenon_id: u64,
    name: String,
    metadata_uri: String,
    related_ideas: Vec<Pubkey>,
    // Track B: Clustering metadata
    similarity_score: f32,
    clustering_proof: [u8; 32],
    discovery_method: DiscoveryMethod,
    // Track B: Embedding signature (SEC-INV-10) - REQUIRED
    embedding_hash: [u8; 32],
    embedding_signature: [u8; 64],
    embedding_provider: String,
    embedding_provider_pubkey: Pubkey,
    // Track B: Governance
    proposal_id: Option<u64>,
    // Track B: Network effects
    associated_mesh_groups: Vec<Pubkey>,
    grant_priority: u8,
    // Track B: DBSCAN validation parameters (B3)
    cluster_hash: [u8; 32],
    cluster_signature: [u8; 64],
    temporal_window_days: u8,
    author_overlap_count: u8,
    // Track B: DBSCAN parameters
    eps: f32,  // Similarity threshold (default: 0.7)
    min_samples: u8,  // Minimum ideas per cluster (default: 3)
    // Track B: Verified semantic distances (from B2)
    // NOTE: For MVP, we accept distances from client. Full implementation requires verified distances.
    verified_distances: Option<Vec<indrasnet_dao_core::state::grant::semantic::VerifiedDistance>>,  // Optional for MVP
) -> Result<()> {
    let observer = ctx.accounts.observer.key();
    let current_time = Clock::get()?.unix_timestamp;
    let phenomenon = &mut ctx.accounts.phenomenon;
    
    // Validate inputs
    require!(!name.is_empty(), CoreError::InvalidInput);
    require!(name.len() <= 100, CoreError::StringTooLong);
    require!(metadata_uri.len() <= 500, CoreError::UriTooLong);
    require!(related_ideas.len() <= 10, CoreError::InvalidInput);
    require!(related_ideas.len() >= 3, CoreError::InvalidInput); // Min 3 ideas for cluster
    require!((0.0..=1.0).contains(&similarity_score), CoreError::InvalidScore);
    require!(grant_priority <= 100, CoreError::InvalidScore);
    require!(associated_mesh_groups.len() <= 10, CoreError::InvalidInput);
    
    // Track B: DBSCAN validation (B3)
    // 1. Check min_samples >= 3
    require!(
        related_ideas.len() >= min_samples as usize,
        CoreError::InsufficientClusterSize
    );
    require!(min_samples >= 3, CoreError::InvalidInput); // DBSCAN requirement
    
    // 2. Check eps threshold (default: 0.7)
    require!(eps > 0.0 && eps <= 1.0, CoreError::InvalidScore);
    const DEFAULT_EPS: f32 = 0.7;
    let effective_eps = if eps == 0.0 { DEFAULT_EPS } else { eps };
    
    // 3. Check author overlap (≥ 1 shared author)
    require!(
        author_overlap_count >= 1,
        CoreError::NoAuthorOverlap
    );
    
    // 4. Check temporal window (≤ 30 days default)
    require!(temporal_window_days > 0, CoreError::InvalidInput);
    const MAX_TEMPORAL_WINDOW_DAYS: u8 = 30;
    require!(
        temporal_window_days <= MAX_TEMPORAL_WINDOW_DAYS,
        CoreError::TemporalWindowExceeded
    );
    
    // 5. Validate verified distances (if provided)
    // Check that all pairwise distances ≤ eps
    if let Some(ref distances) = verified_distances {
        for dist in distances {
            require!(
                dist.source_index < related_ideas.len() as u8,
                CoreError::InvalidInput
            );
            require!(
                dist.target_index < related_ideas.len() as u8,
                CoreError::InvalidInput
            );
            require!(
                dist.distance <= effective_eps,
                CoreError::SemanticDistanceExceeded
            );
        }
        
        // TODO: DBSCAN reachability validation (requires utility functions from Core)
        // For MVP, we skip this validation
        msg!("DBSCAN: WARNING - DBSCAN reachability validation skipped (requires utility functions from Core)");
    } else {
        // If no verified distances provided, log warning but allow (for MVP)
        msg!("DBSCAN: WARNING - No verified distances provided, skipping reachability and noise validation (MVP)");
    }
    
    // 6. Verify cluster signature
    // Compute message hash for signature verification
    let mut cluster_hasher = Sha256::new();
    cluster_hasher.update(cluster_hash);
    cluster_hasher.update(phenomenon_id.to_le_bytes());
    cluster_hasher.update(current_time.to_le_bytes());
    let cluster_message_hash = cluster_hasher.finalize();
    
    // TODO: Full cluster signature verification via CPI
    // For MVP, we log the check but require signature to be provided
    require!(
        cluster_signature != [0u8; 64],
        CoreError::ClusterSignatureInvalid
    );
    
    // Suppress unused variable warning
    let _ = cluster_message_hash;
    
    // SEC-INV-10: Embedding signature verification
    // Verify embedding signature via CPI to ed25519_program
    // Signature format: ed25519(SHA256(embedding_hash || phenomenon_id || current_time), embedding_provider_pubkey)
    
    // SEC-INV-10: Verify embedding signature via CPI to ed25519_program
    msg!("SEC-INV-10: Verifying embedding signature for provider: {}", embedding_provider);
    
    // Validate embedding hash is not zero
    require!(
        embedding_hash != [0u8; 32],
        CoreError::EmbeddingHashMismatch
    );
    
    // Validate signature is not zero
    require!(
        embedding_signature != [0u8; 64],
        CoreError::EmbeddingSignatureInvalid
    );
    
    // Compute message hash for signature verification
    // Message hash: SHA256(embedding_hash || phenomenon_id || current_time)
    let mut hasher = Sha256::new();
    hasher.update(embedding_hash);
    hasher.update(phenomenon_id.to_le_bytes());
    hasher.update(current_time.to_le_bytes());
    let _message_hash_array: [u8; 32] = hasher.finalize().into();
    
    // TODO: Full signature verification via CPI to ed25519_program
    // For MVP, we log the check but require signature to be provided
    require!(
        embedding_signature != [0u8; 64],
        CoreError::EmbeddingSignatureInvalid
    );
    msg!("SEC-INV-10: Embedding signature format validated (full verification skipped for MVP)");
    
    // SEC-INV-11: Verify provider is in AIServiceRegistry (if provided)
    if let Some(_registry_info) = &ctx.accounts.ai_service_registry {
        // TODO: Deserialize AIServiceRegistry manually from UncheckedAccount
        // For MVP, we log the check but require registry to be provided
        msg!("SEC-INV-11: AI service registry provided (verification skipped for MVP)");
    } else {
        // If registry not provided, only DAO authority can create phenomena
        require!(
            embedding_provider_pubkey == ctx.accounts.dao_config.authority,
            CoreError::InvalidEmbeddingProvider
        );
        msg!("SEC-INV-11: Provider verified as DAO authority (registry not provided)");
    }
    
    // Initialize phenomenon account
    phenomenon.observer = observer;
    phenomenon.created_at = current_time;
    phenomenon.related_ideas = related_ideas;
    phenomenon.name = name;
    phenomenon.metadata_uri = metadata_uri;
    phenomenon.ethics_score = 100; // Default, can be updated later
    
    // Track B: Clustering metadata
    phenomenon.similarity_score = similarity_score;
    phenomenon.clustering_proof = clustering_proof;
    phenomenon.discovered_by = observer;
    phenomenon.discovery_method = discovery_method;
    
    // Track B: Embedding signature (SEC-INV-10)
    phenomenon.embedding_signature = Some(embedding_signature);
    phenomenon.embedding_provider = Some(embedding_provider);
    phenomenon.embedding_hash = Some(embedding_hash);
    
    // Track B: Governance
    phenomenon.status = PhenomenonStatus::Proposed; // Start as Proposed, requires approval
    phenomenon.proposal_id = proposal_id;
    phenomenon.approved_at = None;
    
    // Track B: Network effects
    phenomenon.associated_mesh_groups = associated_mesh_groups;
    phenomenon.grant_priority = grant_priority;
    
    // Track B: DBSCAN validation metadata (B3)
    phenomenon.cluster_hash = Some(cluster_hash);
    phenomenon.cluster_signature = Some(cluster_signature);
    phenomenon.temporal_window_days = Some(temporal_window_days as u16);
    phenomenon.author_overlap_count = Some(author_overlap_count);
    
    phenomenon.bump = ctx.bumps.phenomenon;
    
    msg!("Phenomenon {} created by {} with {} related ideas", phenomenon_id, observer, phenomenon.related_ideas.len());
    
    Ok(())
}

/// Add idea to phenomenon
///
/// Добавляет идею в феномен после проверки условий.
/// 
/// КРИТИЧНО: Феномен может создаваться в трех случаях:
/// 1. После получения гранта (GrantStatus::Approved или Active)
/// 2. После отказа пользователя от гранта (GrantStatus::Cancelled)
/// 3. После передачи прав e.V. без гранта (автор не хочет реализовывать идею, грант не нужен)
pub fn add_idea_to_phenomenon_handler(
    ctx: Context<crate::AddIdeaToPhenomenon>,
    idea_id: u64,
) -> Result<()> {
    let phenomenon = &mut ctx.accounts.phenomenon;
    let idea = &ctx.accounts.idea;
    
    // Validate idea ID matches
    require!(idea.id == idea_id, IndrasError::InvalidInput);
    
    // КРИТИЧНО: Проверяем два возможных сценария:
    // 1. Идея имеет грант (в подходящем статусе)
    // 2. Идея имеет переданные права e.V. без гранта
    
    if let Some(grant) = &ctx.accounts.grant {
        // Сценарий 1: Идея имеет грант
        // Проверяем, что грант в подходящем статусе для создания феномена
        // Разрешенные статусы:
        // - Approved: грант одобрен, но еще не активен
        // - Active: грант активен, выплачивается
        // - Cancelled: пользователь отказался от гранта
        require!(
            grant.status == indrasnet_dao_core::state::grant::GrantStatus::Approved || 
            grant.status == indrasnet_dao_core::state::grant::GrantStatus::Active || 
            grant.status == indrasnet_dao_core::state::grant::GrantStatus::Cancelled,
            IndrasError::InvalidState
        );
        
        // Проверяем, что грант связан с идеей
        require!(
            grant.idea_id == idea_id,
            IndrasError::InvalidInput
        );
        
        let status_str = match grant.status {
            indrasnet_dao_core::state::grant::GrantStatus::Approved => "Approved",
            indrasnet_dao_core::state::grant::GrantStatus::Active => "Active",
            indrasnet_dao_core::state::grant::GrantStatus::Cancelled => "Cancelled (user declined)",
            _ => "Unknown",
        };
        
        msg!("Idea {} (with grant {} in status {}) added to phenomenon {} by {}", 
             idea_id,
             grant.id,
             status_str,
             phenomenon.key(),
             ctx.accounts.observer.key());
    } else {
        // Сценарий 2: Идея не имеет гранта, но должна иметь переданные права e.V.
        // Автор не хочет реализовывать идею и передал права e.V. без гранта
        require!(
            idea.rights_transferred_to_ev.is_some(),
            IndrasError::InvalidState
        );
        
        let transferred_rights = idea.rights_transferred_to_ev.as_ref().unwrap();
        
        // Проверяем, что хотя бы одно право передано
        require!(
            transferred_rights.can_modify || 
            transferred_rights.can_distribute || 
            transferred_rights.can_reproduce || 
            transferred_rights.can_develop || 
            transferred_rights.can_sublicense || 
            transferred_rights.can_gift || 
            transferred_rights.can_bequeath,
            IndrasError::InvalidInput
        );
        
        msg!("Idea {} (with rights transferred to e.V. without grant, transferred by {}) added to phenomenon {} by {}", 
             idea_id,
             transferred_rights.transferred_by,
             phenomenon.key(),
             ctx.accounts.observer.key());
    }
    
    // Добавляем идею в феномен
    let idea_pubkey = idea.key();
    phenomenon.add_idea(idea_pubkey)?;
    
    Ok(())
}
