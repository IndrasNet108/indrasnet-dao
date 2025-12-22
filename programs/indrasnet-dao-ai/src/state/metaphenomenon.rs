//! Metaphenomenon account structure
//!
//! Метафеномен - это аналитическое решение ИИ по объединению схожих феноменов
//! Метафеномены создаются ИИ для выявления паттернов на более высоком уровне абстракции
//! и стратегического управления группами феноменов
//!
//! Иерархия: Идеи → Феномены → Метафеномены

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Метафеномен (Metaphenomenon)
/// 
/// Метафеномен создается ИИ для объединения схожих феноменов
/// Метафеномен позволяет выявлять паттерны на более высоком уровне абстракции
/// и координировать стратегическое развитие группы феноменов
#[account]
#[derive(InitSpace)]
pub struct Metaphenomenon {
    pub observer: Pubkey,              // ИИ или пользователь, создавший метафеномен
    pub created_at: i64,
    /// Связанные феномены (максимум 20)
    /// Используем Vec для гибкости (InitSpace автоматически обработает)
    #[max_len(20)]
    pub related_phenomena: Vec<Pubkey>, // Феномены, входящие в метафеномен
    #[max_len(100)]
    pub name: String,                  // Название метафеномена
    #[max_len(500)]
    pub metadata_uri: String,          // URI метаданных (IPFS, Arweave и т.д.)
    pub ethics_score: u8,              // Средняя оценка этичности (0-255)
    pub strategic_importance: u8,      // Стратегическая важность (0-255)
    pub bump: u8,
}

impl Metaphenomenon {
    /// Максимальное количество связанных феноменов
    pub const MAX_RELATED_PHENOMENA: usize = 20;
    
    /// Добавить феномен в метафеномен
    pub fn add_phenomenon(&mut self, phenomenon: Pubkey) -> Result<()> {
        require!(
            self.related_phenomena.len() < Self::MAX_RELATED_PHENOMENA,
            IndrasError::InvalidInput
        );
        require!(
            !self.related_phenomena.contains(&phenomenon),
            IndrasError::InvalidInput
        );
        
        self.related_phenomena.push(phenomenon);
        Ok(())
    }
    
    /// Удалить феномен из метафеномена
    pub fn remove_phenomenon(&mut self, phenomenon: Pubkey) -> Result<()> {
        if let Some(index) = self.related_phenomena.iter().position(|&id| id == phenomenon) {
            self.related_phenomena.remove(index);
        }
        Ok(())
    }
    
    /// Проверить, содержит ли метафеномен феномен
    pub fn contains_phenomenon(&self, phenomenon: Pubkey) -> bool {
        self.related_phenomena.contains(&phenomenon)
    }
    
    /// Получить количество связанных феноменов
    pub fn phenomenon_count(&self) -> usize {
        self.related_phenomena.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    fn create_test_metaphenomenon(observer: Pubkey) -> Metaphenomenon {
        Metaphenomenon {
            observer,
            created_at: 1000,
            related_phenomena: Vec::new(),
            name: "Test Metaphenomenon".to_string(),
            metadata_uri: "https://example.com/metadata".to_string(),
            ethics_score: 100,
            strategic_importance: 150,
            bump: 255,
        }
    }

    #[test]
    fn test_metaphenomenon_add_phenomenon() {
        let observer = create_test_pubkey(1);
        let mut metaphenomenon = create_test_metaphenomenon(observer);
        let phenomenon = create_test_pubkey(2);
        
        assert!(metaphenomenon.add_phenomenon(phenomenon).is_ok());
        assert_eq!(metaphenomenon.phenomenon_count(), 1);
        assert!(metaphenomenon.contains_phenomenon(phenomenon));
    }

    #[test]
    fn test_metaphenomenon_add_phenomenon_duplicate() {
        let observer = create_test_pubkey(1);
        let mut metaphenomenon = create_test_metaphenomenon(observer);
        let phenomenon = create_test_pubkey(2);
        
        assert!(metaphenomenon.add_phenomenon(phenomenon).is_ok());
        // Try to add duplicate - should fail
        assert!(metaphenomenon.add_phenomenon(phenomenon).is_err());
    }

    #[test]
    fn test_metaphenomenon_add_phenomenon_max_limit() {
        let observer = create_test_pubkey(1);
        let mut metaphenomenon = create_test_metaphenomenon(observer);
        
        // Add 20 phenomena (max)
        for i in 0..20 {
            let phenomenon = create_test_pubkey(i + 2);
            assert!(metaphenomenon.add_phenomenon(phenomenon).is_ok());
        }
        
        // Try to add 21st phenomenon - should fail
        let phenomenon = create_test_pubkey(22);
        assert!(metaphenomenon.add_phenomenon(phenomenon).is_err());
    }

    #[test]
    fn test_metaphenomenon_remove_phenomenon() {
        let observer = create_test_pubkey(1);
        let mut metaphenomenon = create_test_metaphenomenon(observer);
        let phenomenon = create_test_pubkey(2);
        
        metaphenomenon.add_phenomenon(phenomenon).unwrap();
        assert_eq!(metaphenomenon.phenomenon_count(), 1);
        
        metaphenomenon.remove_phenomenon(phenomenon).unwrap();
        assert_eq!(metaphenomenon.phenomenon_count(), 0);
        assert!(!metaphenomenon.contains_phenomenon(phenomenon));
    }
}
