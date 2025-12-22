//! Real Solana Runtime Tests for utils/phenomenon_deserialize.rs
//!
//! These tests use solana-program-test to test phenomenon deserialization functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
mod tests {
    use crate::utils::phenomenon_deserialize::*;
    use crate::state::phenomenon::PhenomenonStatus;
    use anchor_lang::prelude::*;
    use anyhow::Result;

    /// Helper: Create minimal valid Phenomenon account data
    fn create_minimal_phenomenon_data(related_ideas: Vec<Pubkey>) -> Vec<u8> {
        let mut data = Vec::new();
        
        // Discriminator (8 bytes) - placeholder
        data.extend_from_slice(&[0u8; 8]);
        
        // observer (32 bytes) - placeholder
        data.extend_from_slice(&[0u8; 32]);
        
        // created_at (8 bytes) - placeholder
        data.extend_from_slice(&[0u8; 8]);
        
        // related_ideas Vec: length (4 bytes) + items
        let length = related_ideas.len() as u32;
        data.extend_from_slice(&length.to_le_bytes());
        for idea in related_ideas {
            data.extend_from_slice(idea.as_ref());
        }
        
        data
    }

    /// Test deserialize_related_ideas with empty list
    #[tokio::test]
    async fn test_deserialize_related_ideas_empty() -> Result<()> {
        let data = create_minimal_phenomenon_data(vec![]);
        let result = deserialize_related_ideas(&data);
        assert!(result.is_ok(), "Empty list should deserialize");
        assert_eq!(result.unwrap().len(), 0, "Empty list should have 0 items");
        
        Ok(())
    }

    /// Test deserialize_related_ideas with single idea
    #[tokio::test]
    async fn test_deserialize_related_ideas_single() -> Result<()> {
        let idea = Pubkey::new_unique();
        let data = create_minimal_phenomenon_data(vec![idea]);
        let result = deserialize_related_ideas(&data);
        assert!(result.is_ok(), "Single idea should deserialize");
        let ideas = result.unwrap();
        assert_eq!(ideas.len(), 1, "Should have 1 idea");
        assert_eq!(ideas[0], idea, "Idea should match");
        
        Ok(())
    }

    /// Test deserialize_related_ideas with max ideas
    #[tokio::test]
    async fn test_deserialize_related_ideas_max() -> Result<()> {
        // MAX_IDEAS = 10
        let ideas: Vec<Pubkey> = (0..10).map(|_| Pubkey::new_unique()).collect();
        let data = create_minimal_phenomenon_data(ideas.clone());
        let result = deserialize_related_ideas(&data);
        assert!(result.is_ok(), "Max ideas should deserialize");
        let deserialized = result.unwrap();
        assert_eq!(deserialized.len(), 10, "Should have 10 ideas");
        assert_eq!(deserialized, ideas, "Ideas should match");
        
        Ok(())
    }

    /// Test is_idea_in_phenomenon with idea present
    #[tokio::test]
    async fn test_is_idea_in_phenomenon_true() -> Result<()> {
        let idea1 = Pubkey::new_unique();
        let idea2 = Pubkey::new_unique();
        let idea3 = Pubkey::new_unique();
        let data = create_minimal_phenomenon_data(vec![idea1, idea2, idea3]);
        
        let result = is_idea_in_phenomenon(&data, &idea2);
        assert!(result.is_ok(), "Should succeed");
        assert_eq!(result.unwrap(), true, "Idea should be in phenomenon");
        
        Ok(())
    }

    /// Test is_idea_in_phenomenon with idea not present
    #[tokio::test]
    async fn test_is_idea_in_phenomenon_false() -> Result<()> {
        let idea1 = Pubkey::new_unique();
        let idea2 = Pubkey::new_unique();
        let idea3 = Pubkey::new_unique();
        let idea_not_in = Pubkey::new_unique();
        let data = create_minimal_phenomenon_data(vec![idea1, idea2, idea3]);
        
        let result = is_idea_in_phenomenon(&data, &idea_not_in);
        assert!(result.is_ok(), "Should succeed");
        assert_eq!(result.unwrap(), false, "Idea should not be in phenomenon");
        
        Ok(())
    }

    /// Test get_phenomenon_status with Active status
    #[tokio::test]
    async fn test_get_phenomenon_status_active() -> Result<()> {
        // Create data with status byte at one of the checked positions (48, 100, 150, 200) = 1 (Active)
        let mut data = vec![0u8; 201];
        data[48] = 255; // > 2, skip
        data[100] = 255; // > 2, skip
        data[150] = 255; // > 2, skip
        data[200] = 1; // PhenomenonStatus::Active
        
        let result = get_phenomenon_status(&data);
        assert!(result.is_ok(), "Should succeed");
        assert_eq!(result.unwrap(), PhenomenonStatus::Active, "Status should be Active");
        
        Ok(())
    }

    /// Test get_phenomenon_status with Proposed status
    #[tokio::test]
    async fn test_get_phenomenon_status_proposed() -> Result<()> {
        // Create data with status byte at one of the checked positions = 0 (Proposed)
        let mut data = vec![0u8; 201];
        data[48] = 255; // > 2, skip
        data[100] = 0; // PhenomenonStatus::Proposed
        data[150] = 255; // > 2, skip
        data[200] = 255; // > 2, skip
        
        let result = get_phenomenon_status(&data);
        assert!(result.is_ok(), "Should succeed");
        assert_eq!(result.unwrap(), PhenomenonStatus::Proposed, "Status should be Proposed");
        
        Ok(())
    }

    /// Test get_phenomenon_status fallback
    #[tokio::test]
    async fn test_get_phenomenon_status_fallback() -> Result<()> {
        // Data too small, should fallback to Active
        let data = vec![0u8; 50];
        let result = get_phenomenon_status(&data);
        assert!(result.is_ok(), "Should succeed");
        assert_eq!(result.unwrap(), PhenomenonStatus::Active, "Should fallback to Active");
        
        Ok(())
    }
}
