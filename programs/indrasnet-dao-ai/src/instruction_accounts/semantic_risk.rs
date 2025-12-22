// NOTE: ai_models удалён (оффчейн логика)
// Semantic risk анализ теперь использует только результаты анализа идеи

#[derive(Accounts)]
pub struct AnalyzeSemanticRisk<'info> {
    #[account(mut)]
    pub idea_analysis: Account<'info, AIAnalysis>,
    // NOTE: ai_model удалён - модель работает оффчейн, результаты фиксируются в idea_analysis
    // Add other accounts as needed for DBSCAN validation
}