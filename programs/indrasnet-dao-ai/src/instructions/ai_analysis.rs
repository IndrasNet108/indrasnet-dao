//! AI Analysis instruction handlers
//!
//! Handlers for AI analysis of ideas:
//! - analyze_idea - полный анализ идеи на соответствие нормативам ДАО
//! - update_idea_status - обновление статуса идеи на основе анализа
//!
//! NOTE: Moved from Core program to AI program for modular architecture

use anchor_lang::prelude::*;
use crate::state::ai_analysis::{AIReviewDecision, RejectionReason, ApprovalReason};
use indrasnet_dao_core::state::enums::IdeaStatus;
use indrasnet_dao_core::error::IndrasError as CoreError;
use sha2::{Sha256, Digest};
use hex;
// Security CPI types (one-way dependency: AI → Security, safe)
use indrasnet_dao_security::cpi::accounts::CheckProposalSecurity;
use indrasnet_dao_security::cpi::check_proposal_security_cpi;
use indrasnet_dao_core::cpi::accounts::RegisterAiAnalysis;
use indrasnet_dao_core::cpi::register_ai_analysis;

/// Analyze idea for compliance with DAO norms
///
/// Проверяет идею на соответствие нормативам ДАО:
/// - Этические требования (ethics_compliance_score >= 50)
/// - Юридические требования (legal_compliance_score >= 50)
/// - Инновационность (impact_score >= 70, feasibility_score >= 70)
/// - Уникальность (uniqueness_score >= 70) - КРИТИЧНО!
/// - Реализуемость (feasibility_score >= 70 с артефактами)
///
/// # Compute Units
/// Recommended: 40,000 CU
/// - Validation: ~10,000 CU
/// - Score calculations: ~10,000 CU
/// - Account initialization: ~20,000 CU
///
/// # Notes
/// - Реальный AI анализ происходит оффчейн (Gemini API).
/// - Этот handler фиксирует результаты анализа ончейн.
/// Analyze idea for compliance with DAO norms
///
/// NOTE: Moved from Core program to AI program for modular architecture
#[allow(clippy::too_many_arguments)]
pub fn analyze_idea_handler(
    ctx: Context<crate::AnalyzeIdea>,
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
    // New parameters for hashing and verification (REQUIRED)
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
    // SEC-INV-X: Rate limit check - 1 analysis per 5 minutes per analyzer
    let current_time = Clock::get()?.unix_timestamp;
    let rate_limit_tracker = &mut ctx.accounts.rate_limit_tracker;

    if rate_limit_tracker.user == Pubkey::default() {
        rate_limit_tracker.user = ctx.accounts.analyzer.key();
        rate_limit_tracker.operation_type = "analyze_idea".to_string();
        rate_limit_tracker.last_operation_at = 0;
        rate_limit_tracker.operation_count = 0;
        rate_limit_tracker.window_start = current_time;
    } else {
        require!(
            rate_limit_tracker.user == ctx.accounts.analyzer.key(),
            CoreError::InvalidInput
        );
        if rate_limit_tracker.operation_type.is_empty() {
            rate_limit_tracker.operation_type = "analyze_idea".to_string();
        } else {
            require!(
                rate_limit_tracker.operation_type == "analyze_idea",
                CoreError::InvalidInput
            );
        }
    }

    rate_limit_tracker.check_time_based_rate_limit(current_time, 5 * 60)?;
    msg!(
        "SEC-INV-X: Rate limit check passed for analyzer {} (window: {}s)",
        ctx.accounts.analyzer.key(),
        5 * 60
    );

    let idea = &ctx.accounts.idea;
    let analysis = &mut ctx.accounts.analysis;
    
    // Validate idea ID matches
    require!(idea.id == idea_id, CoreError::InvalidInput);
    
    // Validate idea is in Draft or UnderReview status
    require!(
        idea.status == IdeaStatus::Draft || idea.status == IdeaStatus::UnderReview,
        CoreError::InvalidState
    );
    
    // Validate scores (0-100)
    let scores = [
        charter_compliance_score,
        governance_compliance_score,
        ethics_compliance_score,
        legal_compliance_score,
        technical_feasibility_score,
        uniqueness_score,
        impact_score,
        feasibility_score,
    ];
    
    for score in scores.iter() {
        require!(*score <= 100, CoreError::InvalidScore);
    }
    
    // Validate vectors length
    require!(checked_governance_docs.len() <= 20, CoreError::InvalidInput);
    require!(checked_legal_frameworks.len() <= 20, CoreError::InvalidInput);
    require!(checked_ethics_frameworks.len() <= 20, CoreError::InvalidInput);
    require!(detailed_feedback.len() <= 2000, CoreError::StringTooLong);
    
    // Validate new parameters (REQUIRED)
    require!(model_id.len() <= 100, CoreError::StringTooLong);
    require!(!model_id.is_empty(), CoreError::InvalidInput);
    require!(model_version.len() <= 50, CoreError::StringTooLong);
    require!(!model_version.is_empty(), CoreError::InvalidInput);
    
    // SECURITY: Verify analyzer_pubkey matches analyzer account
    require!(
        analyzer_pubkey == ctx.accounts.analyzer.key(),
        CoreError::InvalidInput
    );
    
    // SECURITY: Check signature format (64 bytes for Ed25519)
    require!(analyzer_signature.len() == 64, CoreError::InvalidInput);
    
    // Compute bundle_hash on-chain for signature verification
    // bundle_hash = sha256(input_hash || output_hash || model_id || model_version || timestamp || nonce)
    let mut hasher = Sha256::new();
    hasher.update(input_hash);
    hasher.update(output_hash);
    hasher.update(model_id.as_bytes());
    hasher.update(model_version.as_bytes());
    hasher.update(current_time.to_le_bytes());
    hasher.update(nonce.to_le_bytes());
    let bundle_hash = hasher.finalize();
    
    // SECURITY: Check that bundle_hash is not empty
    require!(!bundle_hash.iter().all(|&b| b == 0), CoreError::InvalidInput);
    
    // TODO: Full signature verification via CPI to ed25519_program
    // For MVP, we log the check but require signature to be provided
    require!(
        analyzer_signature != [0u8; 64],
        CoreError::InvalidInput
    );
    
    msg!("Hash format validated, signature format validated (64 bytes)");
    msg!("Bundle hash computed: {}", hex::encode(bundle_hash));
    
    // Verify AI service authenticity (if registry provided)
    // NOTE: Registry is optional for MVP - if not provided, only DAO authority can submit
    if let Some(_registry_info) = &ctx.accounts.ai_service_registry {
        // TODO: Deserialize AIServiceRegistry manually from UncheckedAccount
        // For MVP, we log the check but require registry to be provided
        msg!("AI service registry provided (verification skipped for MVP)");
    }
    
    // Verify model version (if registry provided)
    // NOTE: Registry is optional for MVP - if not provided, any model version is accepted
    if let Some(_model_registry_info) = &ctx.accounts.model_registry {
        // TODO: Deserialize ModelRegistry manually from UncheckedAccount
        // For MVP, we log the check but require registry to be provided
        msg!("Model registry provided (verification skipped for MVP)");
    }
    
    // Check input_hash == idea.idea_hash (reuse attack protection)
    if let Some(idea_hash) = idea.idea_hash {
        require!(
            input_hash == idea_hash,
            CoreError::InvalidInput
        );
        msg!("Input hash verified against idea.idea_hash");
    } else {
        msg!("WARNING: idea.idea_hash not set, skipping input_hash verification (backward compatibility)");
    }
    
    // Replay attack protection (nonce + timestamp)
    if let Some(expires_at_val) = expires_at {
        require!(
            expires_at_val > current_time,
            CoreError::InvalidInput
        );
        msg!("Analysis expiration set: {}", expires_at_val);
    }
    
    require!(
        nonce > 0,
        CoreError::InvalidInput
    );
    
    msg!("Replay attack protection: nonce={}, timestamp={}, expires_at={:?}", 
         nonce, current_time, expires_at);
    
    // Security check via CPI (if Security program is provided)
    if let Some(security_program) = &ctx.accounts.security_program {
        // Prepare AI analysis description for security check
        let analysis_description = format!(
            "AI Analysis: Idea {} - Ethics: {}%, Legal: {}%, Impact: {}%, Uniqueness: {}%",
            idea_id,
            ethics_compliance_score,
            legal_compliance_score,
            impact_score,
            uniqueness_score
        );
        
        // Call Security program via CPI to check AI analysis security
        let security_check_result = check_proposal_security_cpi(
            CpiContext::new(
                security_program.to_account_info(),
                CheckProposalSecurity {
                    caller: ctx.accounts.analyzer.to_account_info(),
                    ai_program: None, // AI program not used in AI → Security CPI (would create cycle)
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
            ),
            format!("AI Analysis: {}", idea.title),
            analysis_description,
        );
        
        // Handle security check result
        if let Ok(return_value) = security_check_result {
            let security_check = return_value.get();
            // Log security check result
            msg!(
                "AI analysis security check: safe={}, requires_vote={}, risk_level={:?}",
                security_check.is_safe,
                security_check.requires_dao_vote,
                security_check.risk_flag.as_ref().map(|rf| rf.risk_level.clone())
            );
            
            // If Security module requires DAO vote, analysis will need review
            if security_check.requires_dao_vote {
                msg!("WARNING: Security module flagged AI analysis as requiring DAO review");
            }
        } else {
            // Log error but don't block AI analysis
            msg!("Security check failed for AI analysis (proceeding anyway)");
        }
    }
    
    // Calculate overall score (weighted)
    // Веса: charter(15%), governance(10%), ethics(10%), legal(15%), technical(10%), uniqueness(30%), impact(10%)
    // uniqueness - самое важное (30%)
    let weights = [15u32, 10u32, 10u32, 15u32, 10u32, 30u32, 10u32];
    let score_values = [
        charter_compliance_score,
        governance_compliance_score,
        ethics_compliance_score,
        legal_compliance_score,
        technical_feasibility_score,
        uniqueness_score,
        impact_score,
    ];
    
    let weighted_sum: u32 = score_values.iter()
        .zip(weights.iter())
        .map(|(score, weight)| (*score as u32) * (*weight as u32))
        .sum();
    
    let overall_score = (weighted_sum / 100) as u8;
    
    // Determine decision and reasons
    let (decision, rejection_reasons, approval_reasons) = determine_decision(
        charter_compliance_score,
        governance_compliance_score,
        ethics_compliance_score,
        legal_compliance_score,
        technical_feasibility_score,
        uniqueness_score,
        impact_score,
        feasibility_score,
        overall_score,
    );
    
    // Initialize analysis
    analysis.idea_id = idea_id;
    analysis.reviewer = ctx.accounts.analyzer.key();
    analysis.decision = Some(decision);
    analysis.overall_score = Some(overall_score);
    analysis.charter_compliance_score = charter_compliance_score;
    analysis.governance_compliance_score = governance_compliance_score;
    analysis.ethics_compliance_score = ethics_compliance_score;
    analysis.legal_compliance_score = legal_compliance_score;
    analysis.technical_feasibility_score = technical_feasibility_score;
    analysis.uniqueness_score = uniqueness_score;
    analysis.impact_score = impact_score;
    analysis.feasibility_score = feasibility_score;
    analysis.rejection_reasons = rejection_reasons;
    analysis.approval_reasons = approval_reasons;
    analysis.checked_governance_docs = checked_governance_docs;
    analysis.checked_legal_frameworks = checked_legal_frameworks;
    analysis.checked_ethics_frameworks = checked_ethics_frameworks;
    analysis.detailed_feedback = detailed_feedback;
    analysis.artifacts_hash = artifacts_hash;
    analysis.artifacts_verified = artifacts_verified;
    
    // Set new fields (REQUIRED)
    analysis.input_hash = input_hash;
    analysis.output_hash = output_hash;
    analysis.model_id = model_id;
    analysis.model_version = model_version;
    analysis.model_hash = model_hash;
    analysis.analyzer_pubkey = analyzer_pubkey;
    analysis.analyzer_signature = analyzer_signature;
    
    // Optional fields
    analysis.thresholds_profile = thresholds_profile;
    
    // Replay attack protection
    analysis.nonce = nonce;
    analysis.expires_at = expires_at;
    
    analysis.created_at = current_time;
    analysis.analysis_duration_ms = 0; // Будет установлено при завершении анализа
    analysis.bump = ctx.bumps.analysis;
    
    msg!("AI Analysis completed for idea {}: decision={:?}, overall_score={}, uniqueness={}, ethics={}, legal={}, feasibility={}, model={}:{}", 
         idea_id, 
         decision,
         overall_score,
         uniqueness_score,
         ethics_compliance_score,
         legal_compliance_score,
         feasibility_score,
         analysis.model_id,
         analysis.model_version);

    // Register AI analysis in Core via CPI (CPI-guarded)
    let signer_seeds: &[&[u8]] = &[b"ai_cpi_authority", &[ctx.bumps.ai_cpi_authority]];
    register_ai_analysis(
        CpiContext::new_with_signer(
            ctx.accounts.core_program.to_account_info(),
            RegisterAiAnalysis {
                dao_config: ctx.accounts.dao_config.to_account_info(),
                idea: ctx.accounts.idea.to_account_info(),
                analysis: ctx.accounts.analysis.to_account_info(),
                analysis_record: ctx.accounts.analysis_record.to_account_info(),
                ai_program: ctx.accounts.ai_program.to_account_info(),
                ai_cpi_authority: ctx.accounts.ai_cpi_authority.to_account_info(),
                analyzer: ctx.accounts.analyzer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[signer_seeds],
        ),
        idea_id,
    )?;

    Ok(())
}

/// Update idea status based on AI analysis
///
/// Обновляет статус идеи на основе AI анализа:
/// - Если Approved → IdeaStatus::Approved (идея может попасть в мэш-группу)
/// - Если Reject → IdeaStatus::Rejected
/// - Если Appeal → IdeaStatus::UnderReview (требует человеческого обжалования)
///
/// КРИТИЧНО: Проверяет все критерии перед одобрением:
/// - Этические требования (ethics_compliance_score >= 50)
/// - Юридические требования (legal_compliance_score >= 50)
/// - Инновационность (impact_score >= 70, feasibility_score >= 70)
/// - Уникальность (uniqueness_score >= 70)
/// - Реализуемость (feasibility_score >= 70, artifacts_verified = true)
pub fn update_idea_status_handler(
    ctx: Context<crate::UpdateIdeaStatusFromAnalysis>,
    idea_id: u64,
) -> Result<()> {
    let idea = &mut ctx.accounts.idea;
    let analysis = &ctx.accounts.analysis;
    
    // Validate idea ID matches
    require!(idea.id == idea_id, CoreError::InvalidInput);
    require!(analysis.idea_id == idea_id, CoreError::InvalidInput);
    
    // Validate idea is in Draft or UnderReview status
    require!(
        idea.status == IdeaStatus::Draft || idea.status == IdeaStatus::UnderReview,
        CoreError::InvalidState
    );
    
    // Update status based on AI decision
    // decision is now Option, use get_decision() to get value
    let decision = analysis.get_decision();
    match decision {
        AIReviewDecision::Approve => {
            // КРИТИЧНО: Проверяем все критерии перед одобрением
            require!(
                analysis.is_ethics_compliant(),
                CoreError::ComplianceViolation
            );
            require!(
                analysis.is_legal_compliant(),
                CoreError::ComplianceViolation
            );
            require!(
                analysis.is_innovative(),
                CoreError::IdeaNotInnovative
            );
            require!(
                analysis.is_unique(),
                CoreError::IdeaNotUnique
            );
            require!(
                analysis.is_feasibility_proven(),
                CoreError::FeasibilityNotProven
            );
            
            // Все критерии выполнены - одобряем
            idea.status = IdeaStatus::Approved;
            
            msg!("Idea {} approved by AI analysis (ethics: {}, legal: {}, innovation: {}, uniqueness: {}, feasibility: {})", 
                 idea_id,
                 analysis.ethics_compliance_score,
                 analysis.legal_compliance_score,
                 if analysis.is_innovative() { "yes" } else { "no" },
                 analysis.uniqueness_score,
                 if analysis.is_feasibility_proven() { "proven" } else { "not proven" });
        }
        AIReviewDecision::Reject => {
            idea.status = IdeaStatus::Rejected;
            msg!("Idea {} rejected by AI analysis (reasons: {:?})", 
                 idea_id, 
                 analysis.rejection_reasons);
        }
        AIReviewDecision::Appeal => {
            idea.status = IdeaStatus::UnderReview;
            msg!("Idea {} requires human appeal (decision: Appeal)", idea_id);
        }
    }
    
    Ok(())
}

/// Determine decision based on scores
///
/// Определяет решение на основе оценок с приоритетом уникальности.
fn determine_decision(
    charter: u8,
    governance: u8,
    ethics: u8,
    legal: u8,
    technical: u8,
    uniqueness: u8,
    impact: u8,
    feasibility: u8,
    overall: u8,
) -> (AIReviewDecision, Vec<RejectionReason>, Vec<ApprovalReason>) {
    let mut rejection_reasons = Vec::new();
    let mut approval_reasons = Vec::new();
    
    // КРИТИЧЕСКИЙ ФАКТОР: Уникальность - самое важное!
    if uniqueness < 70 {
        rejection_reasons.push(RejectionReason::NotUnique);
    } else if uniqueness >= 70 {
        approval_reasons.push(ApprovalReason::UniqueIdea);
    }
    
    // Проверка соответствия критическим критериям
    if charter < 50 {
        rejection_reasons.push(RejectionReason::NotCompliantWithCharter);
    } else {
        approval_reasons.push(ApprovalReason::CompliantWithCharter);
    }
    
    if governance < 50 {
        rejection_reasons.push(RejectionReason::NotCompliantWithGovernance);
    } else {
        approval_reasons.push(ApprovalReason::CompliantWithGovernance);
    }
    
    if ethics < 50 {
        rejection_reasons.push(RejectionReason::NotCompliantWithEthics);
    } else {
        approval_reasons.push(ApprovalReason::CompliantWithEthics);
    }
    
    if legal < 50 {
        rejection_reasons.push(RejectionReason::NotCompliantWithLegal);
    } else {
        approval_reasons.push(ApprovalReason::CompliantWithLegal);
    }
    
    if technical < 30 {
        rejection_reasons.push(RejectionReason::NotFeasible);
    } else {
        approval_reasons.push(ApprovalReason::TechnicallyFeasible);
    }
    
    // Проверка инновационности
    let innovation_score = (impact as u16 + feasibility as u16) / 2;
    if innovation_score < 70 {
        rejection_reasons.push(RejectionReason::NotInnovative);
    } else {
        approval_reasons.push(ApprovalReason::Innovative);
    }
    
    if impact < 30 {
        rejection_reasons.push(RejectionReason::LowImpact);
    } else if impact >= 70 {
        approval_reasons.push(ApprovalReason::HighImpact);
    }
    
    // Проверка реализуемости
    if feasibility < 70 {
        rejection_reasons.push(RejectionReason::FeasibilityNotProven);
    } else {
        approval_reasons.push(ApprovalReason::FeasibilityProven);
    }
    
    // Определение решения с приоритетом уникальности
    let decision = if uniqueness < 70 {
        // Если уникальность низкая - автоматически отклоняем
        AIReviewDecision::Reject
    } else if !rejection_reasons.is_empty() {
        // Если есть другие критические нарушения
        AIReviewDecision::Reject
    } else if uniqueness >= 70 && innovation_score >= 70 && feasibility >= 70 && overall >= 70 {
        // Если уникальность высокая, инновационность высокая, реализуемость доказана и общая оценка высокая - одобряем
        AIReviewDecision::Approve
    } else if overall >= 70 {
        // Если общая оценка высокая
        AIReviewDecision::Approve
    } else if overall >= 50 {
        // Если общая оценка средняя - на обжалование
        AIReviewDecision::Appeal
    } else {
        AIReviewDecision::Reject
    };
    
    (decision, rejection_reasons, approval_reasons)
}
