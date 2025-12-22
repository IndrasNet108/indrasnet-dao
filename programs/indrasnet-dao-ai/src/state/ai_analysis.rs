//! AI Analysis state structure
//!
//! AI анализ идеи на соответствие нормативам ДАО:
//! - Этические требования (ethics_compliance_score)
//! - Юридические требования (legal_compliance_score)
//! - Инновационность (impact_score + feasibility_score)
//! - Уникальность (uniqueness_score) - КРИТИЧНО!
//! - Реализуемость (feasibility_score с артефактами)

use anchor_lang::prelude::*;

/// Решение AI анализа
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy, Debug)]
pub enum AIReviewDecision {
    Approve,    // Одобрить
    Reject,     // Отклонить
    Appeal,     // Требует обжалования
}

impl anchor_lang::Space for AIReviewDecision {
    const INIT_SPACE: usize = 1;
}

/// Причина отклонения
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy, Debug)]
pub enum RejectionReason {
    NotCompliantWithCharter,      // Не соответствует Уставу
    NotCompliantWithGovernance,   // Не соответствует правилам управления
    NotCompliantWithCodeOfConduct, // Не соответствует кодексу поведения
    NotCompliantWithEthics,       // Не соответствует этическим принципам
    NotCompliantWithLegal,        // Не соответствует законодательству
    NotCompliantWithTechnical,    // Не соответствует техническим стандартам
    Duplicate,                    // Дубликат (низкая уникальность)
    NotFeasible,                  // Технически не реализуемо
    LowImpact,                    // Низкий потенциал импакта
    NotInnovative,                // Не инновационна
    NotUnique,                    // Не уникальна
    FeasibilityNotProven,         // Реализуемость не доказана (нет артефактов)
    Incomplete,                   // Неполная информация
    Other,                        // Другое
}

impl anchor_lang::Space for RejectionReason {
    const INIT_SPACE: usize = 1;
}

/// Причина одобрения
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy, Debug)]
pub enum ApprovalReason {
    UniqueIdea,                   // Уникальная идея
    HighImpact,                   // Высокий потенциал импакта
    CompliantWithCharter,         // Соответствует Уставу
    CompliantWithGovernance,      // Соответствует правилам управления
    CompliantWithCodeOfConduct,   // Соответствует кодексу поведения
    CompliantWithEthics,          // Соответствует этическим принципам
    CompliantWithLegal,           // Соответствует законодательству
    CompliantWithTechnical,       // Соответствует техническим стандартам
    TechnicallyFeasible,          // Технически реализуемо
    Innovative,                   // Инновационна
    FeasibilityProven,            // Реализуемость доказана (есть артефакты)
    WellDocumented,               // Хорошо документировано
    Other,                        // Другое
}

impl anchor_lang::Space for ApprovalReason {
    const INIT_SPACE: usize = 1;
}

/// AI анализ идеи
///
/// Проверяет идею на соответствие нормативам ДАО:
/// - Этические требования (ethics_compliance_score >= 50)
/// - Юридические требования (legal_compliance_score >= 50)
/// - Инновационность (impact_score >= 70, feasibility_score >= 70)
/// - Уникальность (uniqueness_score >= 70) - КРИТИЧНО!
/// - Реализуемость (feasibility_score >= 70 с артефактами)
#[account]
#[derive(InitSpace)]
pub struct AIAnalysis {
    pub idea_id: u64,
    pub reviewer: Pubkey,                    // ИИ или человек
    
    // Основное решение
    /// OPTIONAL: Contract can also determine itself
    pub decision: Option<AIReviewDecision>,
    
    // Детальные оценки по критериям
    pub charter_compliance_score: u8,        // Соответствие Уставу (0-100)
    pub governance_compliance_score: u8,     // Соответствие правилам управления (0-100)
    pub ethics_compliance_score: u8,         // Соответствие этическим принципам (0-100) - КРИТИЧНО!
    pub legal_compliance_score: u8,          // Соответствие законодательству (0-100) - КРИТИЧНО!
    pub technical_feasibility_score: u8,     // Техническая реализуемость (0-100)
    pub uniqueness_score: u8,                // Уникальность (0-100) - КРИТИЧНО!
    pub impact_score: u8,                    // Потенциал импакта / социальная значимость (0-100)
    pub feasibility_score: u8,               // Реализуемость с доказательствами (0-100) - КРИТИЧНО!
    
    // Общая оценка
    /// OPTIONAL: Contract can compute itself
    pub overall_score: Option<u8>,           // Общая оценка (0-100)
    
    // Причины решения
    #[max_len(10)]
    pub rejection_reasons: Vec<RejectionReason>,
    #[max_len(10)]
    pub approval_reasons: Vec<ApprovalReason>,
    
    // Ссылки на проверенные документы
    #[max_len(20)]
    pub checked_governance_docs: Vec<u64>,   // ID проверенных управляющих документов
    #[max_len(20)]
    pub checked_legal_frameworks: Vec<u64>,  // ID проверенных правовых рамок
    #[max_len(20)]
    pub checked_ethics_frameworks: Vec<u64>, // ID проверенных этических рамок
    
    // Детальная обратная связь
    #[max_len(2000)]
    pub detailed_feedback: String,
    
    // Хеш артефактов (доказательств реализуемости)
    // Хранится off-chain (IPFS, Arweave), хеш on-chain
    pub artifacts_hash: Option<[u8; 32]>,    // Хеш артефактов
    pub artifacts_verified: bool,            // Артефакты проверены
    
    // Hashing and verification (MVP v1.0.0 specification)
    /// Hash of input data (idea_text): sha256(idea_text)
    /// REQUIRED: Critical for verification and audit
    pub input_hash: [u8; 32],
    /// Hash of output data (json_response): sha256(json_response)
    /// REQUIRED: Critical for verification and audit
    pub output_hash: [u8; 32],
    /// AI model identifier (e.g., "gemini-2.5-flash")
    /// REQUIRED: Critical for verification and audit
    #[max_len(100)]
    pub model_id: String,
    /// AI model version (e.g., "1.0.0")
    /// REQUIRED: Ensures determinism
    #[max_len(50)]
    pub model_version: String,
    /// Model hash: SHA256(model_id || model_version || model_weights_hash)
    /// SEC-INV-7: Optional for MVP, required if ModelRegistry provided
    pub model_hash: Option<[u8; 32]>,
    /// Analyzer public key (analyzer_pubkey)
    /// REQUIRED: Governance requirement
    pub analyzer_pubkey: Pubkey,
    /// Analyzer signature: ed25519(bundle_hash, analyzer_key)
    /// bundle_hash = sha256(input_hash || output_hash || model_id || model_version)
    /// REQUIRED: Without signature anchor is empty
    pub analyzer_signature: [u8; 64],
    /// Threshold profile for decision making (enum for different profiles)
    /// OPTIONAL: Can be defaulted
    pub thresholds_profile: Option<u8>,
    
    // Метаданные
    pub created_at: i64,
    pub analysis_duration_ms: u64,          // Время анализа в миллисекундах
    
    // Replay attack protection (REQUIRED)
    /// Unique nonce per analysis (replay attack protection)
    pub nonce: u64,
    /// Optional expiration timestamp
    pub expires_at: Option<i64>,
    
    pub bump: u8,
}

impl AIAnalysis {
    /// Проверяет, соответствует ли идея этическим требованиям
    pub fn is_ethics_compliant(&self) -> bool {
        self.ethics_compliance_score >= 50
    }
    
    /// Проверяет, соответствует ли идея юридическим требованиям
    pub fn is_legal_compliant(&self) -> bool {
        self.legal_compliance_score >= 50
    }
    
    /// Проверяет, является ли идея инновационной
    /// Инновационность = (impact_score + feasibility_score) / 2 >= 70
    pub fn is_innovative(&self) -> bool {
        let innovation_score = (self.impact_score as u16 + self.feasibility_score as u16) / 2;
        innovation_score >= 70
    }
    
    /// Проверяет, является ли идея уникальной
    /// КРИТИЧНО: uniqueness_score >= 70 для авторства
    pub fn is_unique(&self) -> bool {
        self.uniqueness_score >= 70
    }
    
    /// Get decision with default
    /// If decision = None, calculates based on scores
    pub fn get_decision(&self) -> AIReviewDecision {
        self.decision.unwrap_or_else(|| {
            // Calculate decision based on scores if not set
            if self.uniqueness_score < 70 {
                AIReviewDecision::Reject
            } else if self.is_ethics_compliant() 
                && self.is_legal_compliant() 
                && self.is_innovative() 
                && self.is_unique() 
                && self.is_feasibility_proven() {
                AIReviewDecision::Approve
            } else {
                AIReviewDecision::Reject
            }
        })
    }
    
    /// Get overall_score with default
    /// If overall_score = None, calculates based on weighted scores
    pub fn get_overall_score(&self) -> u8 {
        self.overall_score.unwrap_or_else(|| {
            // Calculate weighted score
            let weights = [15u32, 10u32, 10u32, 15u32, 10u32, 30u32, 10u32];
            let score_values = [
                self.charter_compliance_score,
                self.governance_compliance_score,
                self.ethics_compliance_score,
                self.legal_compliance_score,
                self.technical_feasibility_score,
                self.uniqueness_score,
                self.impact_score,
            ];
            
            let mut weighted_sum: u32 = 0;
            for (score, weight) in score_values.iter().zip(weights.iter()) {
                weighted_sum += (*score as u32) * weight;
            }
            
            (weighted_sum / 100) as u8
        })
    }
    
    /// Проверяет, доказана ли реализуемость
    /// Реализуемость = feasibility_score >= 70 И artifacts_verified = true
    pub fn is_feasibility_proven(&self) -> bool {
        self.feasibility_score >= 70 && self.artifacts_verified
    }
    
    /// Проверяет, может ли идея получить авторство
    /// Требования для авторства:
    /// - Инновационность (impact_score >= 70, feasibility_score >= 70)
    /// - Уникальность (uniqueness_score >= 70)
    /// - Реализуемость доказана (feasibility_score >= 70, artifacts_verified = true)
    pub fn can_get_authorship(&self) -> bool {
        self.is_innovative() && self.is_unique() && self.is_feasibility_proven()
    }
    
    /// Проверяет, может ли идея попасть в мэш-группу
    /// Требования:
    /// - Этические требования (ethics_compliance_score >= 50)
    /// - Юридические требования (legal_compliance_score >= 50)
    /// - Инновационность (impact_score >= 70, feasibility_score >= 70)
    /// - Уникальность (uniqueness_score >= 70)
    /// - Реализуемость доказана (feasibility_score >= 70, artifacts_verified = true)
    pub fn can_enter_mesh_group(&self) -> bool {
        self.is_ethics_compliant() 
            && self.is_legal_compliant() 
            && self.is_innovative() 
            && self.is_unique() 
            && self.is_feasibility_proven()
    }
}
