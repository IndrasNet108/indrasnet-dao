//! Proposal Amendment module
//!
//! Handles amendments to proposals during the Draft phase

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Proposal Amendment account structure
#[account]
#[derive(InitSpace)]
pub struct ProposalAmendment {
    pub amendment_id: u64,
    pub proposal_id: u64,
    pub author: Pubkey,
    #[max_len(2000)]
    pub content: String,
    pub created_at: i64,
    pub bump: u8,
}

impl ProposalAmendment {
    /// Create a new proposal amendment
    pub fn new(
        amendment_id: u64,
        proposal_id: u64,
        author: Pubkey,
        content: String,
        bump: u8,
    ) -> Result<Self> {
        Self::new_with_time(amendment_id, proposal_id, author, content, bump, Clock::get()?.unix_timestamp)
    }

    /// Create a new proposal amendment with specified time
    pub fn new_with_time(
        amendment_id: u64,
        proposal_id: u64,
        author: Pubkey,
        content: String,
        bump: u8,
        current_time: i64,
    ) -> Result<Self> {
        require!(!content.is_empty(), IndrasError::InvalidInput);
        require!(content.len() <= 2000, IndrasError::InvalidInput);

        Ok(Self {
            amendment_id,
            proposal_id,
            author,
            content,
            created_at: current_time,
            bump,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    #[test]
    fn test_proposal_amendment_new_with_time() {
        let author = create_test_pubkey(1);
        let amendment = ProposalAmendment::new_with_time(
            1,
            100,
            author,
            "Amendment content".to_string(),
            255,
            1000,
        ).unwrap();

        assert_eq!(amendment.amendment_id, 1);
        assert_eq!(amendment.proposal_id, 100);
        assert_eq!(amendment.author, author);
        assert_eq!(amendment.content, "Amendment content");
        assert_eq!(amendment.created_at, 1000);
    }

    #[test]
    fn test_proposal_amendment_validation_empty_content() {
        let author = create_test_pubkey(1);
        let result = ProposalAmendment::new_with_time(
            1,
            100,
            author,
            String::new(),
            255,
            1000,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_proposal_amendment_validation_content_too_long() {
        let author = create_test_pubkey(1);
        let content = "a".repeat(2001);
        let result = ProposalAmendment::new_with_time(
            1,
            100,
            author,
            content,
            255,
            1000,
        );
        assert!(result.is_err());
    }
}
