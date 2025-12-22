/*
 * Copyright (c) 2024 Captain Light. All rights reserved.
 * IndrasNet DAO AI - AI Analysis, Collective AI, AI Functions
 * Created by: Captain Light
 * Contact: info@indrasnet.ee
 * GitLab: https://gitlab.com/IndrasNet/indrasnet-dao-v3
 */

#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

// ===== PROGRAM ID =====
declare_id!("4sH3BtyuQhzqeYA7SiqFuPopg2szhNR3B5KAmQHn5YRC");

// ===== MODULE DECLARATIONS =====
pub mod error;
// pub mod constants; // Removed: empty module, not used
pub mod types;
pub mod events;
pub mod state;
// pub mod utils; // Removed: empty module, not used
pub mod ai;
pub mod ai_types;
pub mod collective_ai_functions;
pub mod voting_types;
pub mod instructions;

// ===== TYPES FOR ACCOUNTS STRUCTURES =====
// Import types that will be used in Accounts structs (must be before include!)
#[allow(unused_imports)]
use indrasnet_dao_core::state::{DaoConfig, Idea, Grant, Phenomenon};
#[allow(unused_imports)]
use crate::state::{AIAnalysis, Metaphenomenon};
pub use crate::state::security_analysis;
pub use crate::state::security_analysis::{SecurityAnalysisEnhancement, RiskLevel};

// ===== RE-EXPORTS FOR CRATE-WIDE USE =====
pub use crate::error::IndrasError;

// CPI program imports (for CPI accounts)
#[allow(unused_imports)]
use indrasnet_dao_core::program::IndrasnetDaoCore;
#[allow(unused_imports)]
use indrasnet_dao_security::program::IndrasnetDaoSecurity;

// ===== INCLUDE ACCOUNTS STRUCTURES =====
include!("instruction_accounts/ai_analysis.rs");
include!("instruction_accounts/phenomenon.rs");
include!("instruction_accounts/metaphenomenon.rs");
include!("instruction_accounts/risk_assessment.rs");
include!("instruction_accounts/performance_analytics.rs");
include!("instruction_accounts/security_enhancements.rs");
include!("instruction_accounts/security_enhancement.rs");
include!("instruction_accounts/collective_ai.rs");
include!("instruction_accounts/semantic_risk.rs");

// ===== PROGRAM MODULE =====
#[program]
pub mod indrasnet_dao_ai {
    use super::*;
    
    // Re-export account types for use in handlers
    pub use crate::EnhanceSecurityAnalysis;

    // ===== AI ANALYSIS INSTRUCTIONS =====

    /// Analyze idea for compliance with DAO norms
    ///
    /// Проверяет идею на соответствие нормативам ДАО:
    /// - Этические требования (ethics_compliance_score >= 50)
    /// - Юридические требования (legal_compliance_score >= 50)
    /// - Инновационность (impact_score >= 70, feasibility_score >= 70)
    /// - Уникальность (uniqueness_score >= 70) - КРИТИЧНО!
    /// - Реализуемость (feasibility_score >= 70 с артефактами)
    ///
    /// NOTE: Реальный AI анализ происходит оффчейн (Gemini API).
    /// Этот handler фиксирует результаты анализа ончейн.
    /// Analyze idea for compliance with DAO norms
    ///
    /// NOTE: Moved from Core program to AI program for modular architecture
    #[allow(clippy::too_many_arguments)]
    pub fn analyze_idea(
        ctx: Context<AnalyzeIdea>,
        idea_id: u64,
        charter_compliance_score: u8,
        governance_compliance_score: u8,
        ethics_compliance_score: u8,
        legal_compliance_score: u8,
        technical_feasibility_score: u8,
        uniqueness_score: u8,
        impact_score: u8,
        feasibility_score: u8,
        artifacts_hash: Option<[u8; 32]>,
        artifacts_verified: bool,
        detailed_feedback: String,
        checked_governance_docs: Vec<u64>,
        checked_legal_frameworks: Vec<u64>,
        checked_ethics_frameworks: Vec<u64>,
        input_hash: [u8; 32],
        output_hash: [u8; 32],
        model_id: String,
        model_version: String,
        model_hash: Option<[u8; 32]>,
        analyzer_pubkey: Pubkey,
        analyzer_signature: [u8; 64],
        nonce: u64,
        expires_at: Option<i64>,
        thresholds_profile: Option<u8>,
    ) -> Result<()> {
        instructions::analyze_idea_handler(
            ctx,
            idea_id,
            charter_compliance_score,
            governance_compliance_score,
            ethics_compliance_score,
            legal_compliance_score,
            technical_feasibility_score,
            uniqueness_score,
            impact_score,
            feasibility_score,
            artifacts_hash,
            artifacts_verified,
            detailed_feedback,
            checked_governance_docs,
            checked_legal_frameworks,
            checked_ethics_frameworks,
            input_hash,
            output_hash,
            model_id,
            model_version,
            model_hash,
            analyzer_pubkey,
            analyzer_signature,
            nonce,
            expires_at,
            thresholds_profile,
        )
    }

    /// Update idea status based on AI analysis
    ///
    /// Обновляет статус идеи на основе AI анализа:
    /// - Если Approved → IdeaStatus::Approved (идея может попасть в мэш-группу)
    /// - Если Reject → IdeaStatus::Rejected
    /// - Если Appeal → IdeaStatus::UnderReview (требует человеческого обжалования)
    pub fn update_idea_status_from_analysis(
        ctx: Context<UpdateIdeaStatusFromAnalysis>,
        idea_id: u64,
    ) -> Result<()> {
        instructions::update_idea_status_handler(ctx, idea_id)
    }

    // ===== PHENOMENON CREATION INSTRUCTIONS =====

    /// Create phenomenon from ideas with grants
    ///
    /// КРИТИЧНО: Феномены создаются ИИ ПОСЛЕ гранта для аналитики.
    /// Согласно документации: феномены создаются ПОСЛЕ получения гранта, а не ДО.
    ///
    /// Феномен объединяет схожие идеи с грантами для аналитики.
    /// Create phenomenon with embedding signature verification (SEC-INV-10)
    ///
    /// NOTE: Moved from Core program to AI program for modular architecture
    #[allow(clippy::too_many_arguments)]
    pub fn create_phenomenon(
        ctx: Context<CreatePhenomenon>,
        phenomenon_id: u64,
        name: String,
        metadata_uri: String,
        related_ideas: Vec<Pubkey>,
        similarity_score: f32,
        clustering_proof: [u8; 32],
        discovery_method: indrasnet_dao_core::state::DiscoveryMethod,
        embedding_hash: [u8; 32],
        embedding_signature: [u8; 64],
        embedding_provider: String,
        embedding_provider_pubkey: Pubkey,
        proposal_id: Option<u64>,
        associated_mesh_groups: Vec<Pubkey>,
        grant_priority: u8,
        cluster_hash: [u8; 32],
        cluster_signature: [u8; 64],
        temporal_window_days: u8,
        author_overlap_count: u8,
        eps: f32,
        min_samples: u8,
        verified_distances: Option<Vec<indrasnet_dao_core::state::grant::semantic::VerifiedDistance>>,
    ) -> Result<()> {
        instructions::create_phenomenon_handler(
            ctx,
            phenomenon_id,
            name,
            metadata_uri,
            related_ideas,
            similarity_score,
            clustering_proof,
            discovery_method,
            embedding_hash,
            embedding_signature,
            embedding_provider,
            embedding_provider_pubkey,
            proposal_id,
            associated_mesh_groups,
            grant_priority,
            cluster_hash,
            cluster_signature,
            temporal_window_days,
            author_overlap_count,
            eps,
            min_samples,
            verified_distances,
        )
    }

    /// Add idea to phenomenon
    ///
    /// Добавляет идею в феномен после проверки условий.
    /// КРИТИЧНО: Идея должна соответствовать одному из условий:
    /// 1. Иметь грант со статусом Approved, Active или Cancelled (отказ пользователя)
    /// 2. Иметь переданные права e.V. без гранта (автор не хочет реализовывать идею, грант не нужен)
    /// 
    /// Феномен может создаваться:
    /// - После получения гранта
    /// - После отказа от гранта
    /// - После передачи прав e.V. без гранта
    pub fn add_idea_to_phenomenon(
        ctx: Context<AddIdeaToPhenomenon>,
        idea_id: u64,
    ) -> Result<()> {
        instructions::add_idea_to_phenomenon_handler(ctx, idea_id)
    }

    // ===== METAPHENOMENON CREATION INSTRUCTIONS =====

    /// Create metaphenomenon from similar phenomena
    ///
    /// Метафеномен создается ИИ для объединения схожих феноменов.
    /// Это позволяет выявлять паттерны на более высоком уровне абстракции
    /// и координировать стратегическое развитие группы феноменов.
    ///
    /// Иерархия: Идеи → Феномены → Метафеномены
    pub fn create_metaphenomenon(
        ctx: Context<CreateMetaphenomenon>,
        metaphenomenon_id: u64,
        name: String,
        metadata_uri: String,
        related_phenomenon_ids: Vec<u64>,
        ethics_score: u8,
        strategic_importance: u8,
    ) -> Result<()> {
        instructions::create_metaphenomenon_handler(
            ctx,
            metaphenomenon_id,
            name,
            metadata_uri,
            related_phenomenon_ids,
            ethics_score,
            strategic_importance,
        )
    }

    /// Add phenomenon to metaphenomenon
    ///
    /// Добавляет феномен в метафеномен после проверки условий.
    /// КРИТИЧНО: Феномен должен существовать и быть валидным.
    pub fn add_phenomenon_to_metaphenomenon(
        ctx: Context<AddPhenomenonToMetaphenomenon>,
        phenomenon_id: u64,
    ) -> Result<()> {
        instructions::add_phenomenon_to_metaphenomenon_handler(ctx, phenomenon_id)
    }

    // ===== RISK ASSESSMENT INSTRUCTIONS =====

    /// Create risk assessment
    ///
    /// Creates a risk assessment record for an entity (idea, proposal, etc.)
    ///
    /// # Compute Units
    /// Recommended: 40,000 CU
    pub fn create_risk_assessment(
        ctx: Context<CreateRiskAssessment>,
        assessment_id: u64,
        entity_id: u64,
        entity_type: String,
        risk_scores: ai::risk_assessment::RiskScores,
        metadata_uri: String,
        assessment_hash: [u8; 32],
    ) -> Result<()> {
        instructions::create_risk_assessment_handler(
            ctx,
            assessment_id,
            entity_id,
            entity_type,
            risk_scores,
            metadata_uri,
            assessment_hash,
        )
    }

    /// Analyze semantic risk of an idea
    ///
    /// Performs semantic risk analysis on an idea using DBSCAN clustering.
    ///
    /// # Compute Units
    /// Recommended: 30,000 CU
    pub fn analyze_semantic_risk(
        ctx: Context<AnalyzeSemanticRisk>,
    ) -> Result<()> {
        instructions::analyze_semantic_risk_handler(ctx)
    }

    // ===== PERFORMANCE ANALYTICS INSTRUCTIONS =====

    /// Create performance analytics
    ///
    /// Creates a performance analytics record for an entity (idea, mesh group, etc.)
    ///
    /// # Compute Units
    /// Recommended: 40,000 CU
    pub fn create_performance_analytics(
        ctx: Context<CreatePerformanceAnalytics>,
        analytics_id: u64,
        entity_id: u64,
        entity_type: String,
        performance_scores: ai::performance_analytics::PerformanceScores,
        metadata_uri: String,
        analytics_hash: [u8; 32],
    ) -> Result<()> {
        instructions::create_performance_analytics_handler(
            ctx,
            analytics_id,
            entity_id,
            entity_type,
            performance_scores,
            metadata_uri,
            analytics_hash,
        )
    }

    // ===== SECURITY ENHANCEMENTS INSTRUCTIONS =====

    /// Create security enhancement recommendation
    ///
    /// Creates a security enhancement recommendation for an entity
    ///
    /// # Compute Units
    /// Recommended: 40,000 CU
    pub fn create_security_enhancement(
        ctx: Context<CreateSecurityEnhancement>,
        enhancement_id: u64,
        entity_id: u64,
        entity_type: String,
        title: String,
        description: String,
        priority: ai::security_enhancements::EnhancementPriority,
        score_improvement: u8,
        metadata_uri: String,
        enhancement_hash: [u8; 32],
    ) -> Result<()> {
        instructions::create_security_enhancement_handler(
            ctx,
            enhancement_id,
            entity_id,
            entity_type,
            title,
            description,
            priority,
            score_improvement,
            metadata_uri,
            enhancement_hash,
        )
    }

    /// Enhance security analysis with AI insights (CPI callable)
    ///
    /// Called by Security module via CPI to get AI-enhanced threat interpretation
    ///
    /// # Compute Units
    /// Recommended: 50,000 CU
    pub fn enhance_security_analysis(
        ctx: Context<EnhanceSecurityAnalysis>,
        threat_data: String,
        threat_score: u8,
        anomaly_score: u8,
        context: String,
    ) -> Result<security_analysis::SecurityAnalysisEnhancement> {
        instructions::security_enhancement::enhance_security_analysis_handler(
            ctx,
            threat_data,
            threat_score,
            anomaly_score,
            context,
        )
    }

    /// Update security enhancement status
    ///
    /// Updates the status of a security enhancement
    ///
    /// # Compute Units
    /// Recommended: 20,000 CU
    pub fn update_security_enhancement_status(
        ctx: Context<UpdateSecurityEnhancementStatus>,
        status: ai::security_enhancements::EnhancementStatus,
    ) -> Result<()> {
        instructions::update_security_enhancement_status_handler(ctx, status)
    }

    // ===== COLLECTIVE AI FUNCTIONS INSTRUCTIONS =====

    /// Initialize neuron
    ///
    /// Creates a neuron in the collective AI network
    ///
    /// # Compute Units
    /// Recommended: 30,000 CU
    pub fn initialize_neuron(
        ctx: Context<InitializeNeuron>,
        neuron_id: u64,
    ) -> Result<()> {
        instructions::initialize_neuron_handler(ctx, neuron_id)
    }

    /// Activate neuron
    ///
    /// Activates a neuron in the collective AI network
    ///
    /// # Compute Units
    /// Recommended: 15,000 CU
    pub fn activate_neuron(
        ctx: Context<ActivateNeuron>,
    ) -> Result<()> {
        instructions::activate_neuron_handler(ctx)
    }

    /// Initialize synapse
    ///
    /// Creates a connection between two neurons
    ///
    /// # Compute Units
    /// Recommended: 30,000 CU
    pub fn initialize_synapse(
        ctx: Context<InitializeSynapse>,
        synapse_id: u64,
        source_neuron_id: u64,
        target_neuron_id: u64,
        synapse_type: collective_ai_functions::SynapseType,
        weight: u8,
    ) -> Result<()> {
        instructions::initialize_synapse_handler(
            ctx,
            synapse_id,
            source_neuron_id,
            target_neuron_id,
            synapse_type,
            weight,
        )
    }

    /// Initialize memory
    ///
    /// Creates a memory record in the collective AI
    ///
    /// # Compute Units
    /// Recommended: 35,000 CU
    pub fn initialize_memory(
        ctx: Context<InitializeMemory>,
        memory_id: u64,
        memory_type: collective_ai_functions::MemoryType,
        data_uri: String,
        data_hash: [u8; 32],
        data_size: u64,
    ) -> Result<()> {
        instructions::initialize_memory_handler(
            ctx,
            memory_id,
            memory_type,
            data_uri,
            data_hash,
            data_size,
        )
    }

    /// Initialize consciousness
    ///
    /// Creates a consciousness state record
    ///
    /// # Compute Units
    /// Recommended: 30,000 CU
    pub fn initialize_consciousness(
        ctx: Context<InitializeConsciousness>,
        consciousness_id: u64,
    ) -> Result<()> {
        instructions::initialize_consciousness_handler(ctx, consciousness_id)
    }

    /// Update consciousness level
    ///
    /// Updates the consciousness level based on network activity
    ///
    /// # Compute Units
    /// Recommended: 20,000 CU
    pub fn update_consciousness_level(
        ctx: Context<UpdateConsciousnessLevel>,
        active_neuron_count: u32,
        active_synapse_count: u32,
    ) -> Result<()> {
        instructions::update_consciousness_level_handler(
            ctx,
            active_neuron_count,
            active_synapse_count,
        )
    }
}
