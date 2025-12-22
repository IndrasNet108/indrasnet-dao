//! TransferredRights structure for tracking rights transferred by author to e.V.

use anchor_lang::prelude::*;

/// Rights that author can transfer to e.V. without grant
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, InitSpace)]
pub struct TransferredRights {
    /// Can modify the idea (Bearbeitung)
    pub can_modify: bool,
    
    /// Can distribute the idea (Verbreitung)
    pub can_distribute: bool,
    
    /// Can reproduce the idea (Vervielfältigung)
    pub can_reproduce: bool,
    
    /// Can develop/improve the idea (Weiterentwicklung)
    pub can_develop: bool,
    
    /// Can sublicense the idea (Unterlizenzierung)
    pub can_sublicense: bool,
    
    /// Can gift the idea (Schenkung)
    pub can_gift: bool,
    
    /// Can bequeath the idea (Vermächtnis)
    pub can_bequeath: bool,
    
    /// Timestamp when rights were transferred
    pub transferred_at: i64,
    
    /// Author who transferred the rights
    pub transferred_by: Pubkey,
}

impl TransferredRights {
    /// Create new TransferredRights
    pub fn new(
        can_modify: bool,
        can_distribute: bool,
        can_reproduce: bool,
        can_develop: bool,
        can_sublicense: bool,
        can_gift: bool,
        can_bequeath: bool,
        transferred_by: Pubkey,
    ) -> Result<Self> {
        let current_time = Clock::get()?.unix_timestamp;
        
        Ok(Self {
            can_modify,
            can_distribute,
            can_reproduce,
            can_develop,
            can_sublicense,
            can_gift,
            can_bequeath,
            transferred_at: current_time,
            transferred_by,
        })
    }
    
    /// Create new TransferredRights with explicit timestamp
    pub fn new_with_time(
        can_modify: bool,
        can_distribute: bool,
        can_reproduce: bool,
        can_develop: bool,
        can_sublicense: bool,
        can_gift: bool,
        can_bequeath: bool,
        transferred_by: Pubkey,
        transferred_at: i64,
    ) -> Self {
        Self {
            can_modify,
            can_distribute,
            can_reproduce,
            can_develop,
            can_sublicense,
            can_gift,
            can_bequeath,
            transferred_at,
            transferred_by,
        }
    }
    
    /// Check if any rights were transferred
    pub fn has_any_rights(&self) -> bool {
        self.can_modify
            || self.can_distribute
            || self.can_reproduce
            || self.can_develop
            || self.can_sublicense
            || self.can_gift
            || self.can_bequeath
    }
    
    /// Check if specific right was transferred
    pub fn has_right(&self, right: &str) -> bool {
        match right {
            "modify" => self.can_modify,
            "distribute" => self.can_distribute,
            "reproduce" => self.can_reproduce,
            "develop" => self.can_develop,
            "sublicense" => self.can_sublicense,
            "gift" => self.can_gift,
            "bequeath" => self.can_bequeath,
            _ => false,
        }
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
    fn test_transferred_rights_new_with_time() {
        let author = create_test_pubkey(1);
        let rights = TransferredRights::new_with_time(
            true,
            true,
            false,
            true,
            false,
            false,
            false,
            author,
            1000,
        );
        
        assert!(rights.can_modify);
        assert!(rights.can_distribute);
        assert!(!rights.can_reproduce);
        assert!(rights.can_develop);
        assert!(!rights.can_sublicense);
        assert_eq!(rights.transferred_by, author);
        assert_eq!(rights.transferred_at, 1000);
    }

    #[test]
    fn test_transferred_rights_has_any_rights() {
        let author = create_test_pubkey(1);
        let rights = TransferredRights::new_with_time(
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            author,
            1000,
        );
        
        assert!(rights.has_any_rights());
    }

    #[test]
    fn test_transferred_rights_has_no_rights() {
        let author = create_test_pubkey(1);
        let rights = TransferredRights::new_with_time(
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            author,
            1000,
        );
        
        assert!(!rights.has_any_rights());
    }

    #[test]
    fn test_transferred_rights_has_right() {
        let author = create_test_pubkey(1);
        let rights = TransferredRights::new_with_time(
            true,
            true,
            false,
            true,
            false,
            false,
            false,
            author,
            1000,
        );
        
        assert!(rights.has_right("modify"));
        assert!(rights.has_right("distribute"));
        assert!(rights.has_right("develop"));
        assert!(!rights.has_right("reproduce"));
        assert!(!rights.has_right("sublicense"));
        assert!(!rights.has_right("gift"));
        assert!(!rights.has_right("bequeath"));
    }

    #[test]
    fn test_transferred_rights_has_right_unknown() {
        let author = create_test_pubkey(1);
        let rights = TransferredRights::new_with_time(
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            author,
            1000,
        );
        
        // Unknown right should return false
        assert!(!rights.has_right("unknown"));
    }

    #[test]
    fn test_transferred_rights_all_rights_true() {
        let author = create_test_pubkey(1);
        let rights = TransferredRights::new_with_time(
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            author,
            1000,
        );
        
        assert!(rights.has_any_rights());
        assert!(rights.has_right("modify"));
        assert!(rights.has_right("distribute"));
        assert!(rights.has_right("reproduce"));
        assert!(rights.has_right("develop"));
        assert!(rights.has_right("sublicense"));
        assert!(rights.has_right("gift"));
        assert!(rights.has_right("bequeath"));
    }

    #[test]
    fn test_transferred_rights_structure() {
        let author = create_test_pubkey(42);
        let rights = TransferredRights::new_with_time(
            true,
            false,
            true,
            false,
            true,
            false,
            true,
            author,
            2000,
        );
        
        assert_eq!(rights.can_modify, true);
        assert_eq!(rights.can_distribute, false);
        assert_eq!(rights.can_reproduce, true);
        assert_eq!(rights.can_develop, false);
        assert_eq!(rights.can_sublicense, true);
        assert_eq!(rights.can_gift, false);
        assert_eq!(rights.can_bequeath, true);
        assert_eq!(rights.transferred_by, author);
        assert_eq!(rights.transferred_at, 2000);
    }

    #[test]
    fn test_transferred_rights_different_authors() {
        let author1 = create_test_pubkey(1);
        let author2 = create_test_pubkey(2);
        
        let rights1 = TransferredRights::new_with_time(
            true, false, false, false, false, false, false,
            author1, 1000,
        );
        
        let rights2 = TransferredRights::new_with_time(
            true, false, false, false, false, false, false,
            author2, 1000,
        );
        
        assert_ne!(rights1.transferred_by, rights2.transferred_by);
    }

    #[test]
    fn test_transferred_rights_different_timestamps() {
        let author = create_test_pubkey(1);
        
        let rights1 = TransferredRights::new_with_time(
            true, false, false, false, false, false, false,
            author, 1000,
        );
        
        let rights2 = TransferredRights::new_with_time(
            true, false, false, false, false, false, false,
            author, 2000,
        );
        
        assert_ne!(rights1.transferred_at, rights2.transferred_at);
    }

    #[test]
    fn test_transferred_rights_has_right_all_strings() {
        let author = create_test_pubkey(1);
        let rights = TransferredRights::new_with_time(
            true, true, true, true, true, true, true,
            author, 1000,
        );
        
        assert!(rights.has_right("modify"));
        assert!(rights.has_right("distribute"));
        assert!(rights.has_right("reproduce"));
        assert!(rights.has_right("develop"));
        assert!(rights.has_right("sublicense"));
        assert!(rights.has_right("gift"));
        assert!(rights.has_right("bequeath"));
    }

    #[test]
    fn test_transferred_rights_clone() {
        let author = create_test_pubkey(1);
        let rights1 = TransferredRights::new_with_time(
            true, false, true, false, true, false, true,
            author, 1000,
        );
        
        let rights2 = rights1.clone();
        assert_eq!(rights1, rights2);
    }

    #[test]
    fn test_transferred_rights_equality() {
        let author = create_test_pubkey(1);
        let rights1 = TransferredRights::new_with_time(
            true, false, true, false, true, false, true,
            author, 1000,
        );
        
        let rights2 = TransferredRights::new_with_time(
            true, false, true, false, true, false, true,
            author, 1000,
        );
        
        assert_eq!(rights1, rights2);
    }

    #[test]
    fn test_transferred_rights_inequality() {
        let author = create_test_pubkey(1);
        let rights1 = TransferredRights::new_with_time(
            true, false, false, false, false, false, false,
            author, 1000,
        );
        
        let rights2 = TransferredRights::new_with_time(
            false, true, false, false, false, false, false,
            author, 1000,
        );
        
        assert_ne!(rights1, rights2);
    }

    #[test]
    fn test_transferred_rights_space() {
        // 7 bool (1 byte each) + 1 i64 (8 bytes) + 1 Pubkey (32 bytes) = 47 bytes
        // InitSpace may add padding, so we check it's at least the minimum size
        let space = <TransferredRights as anchor_lang::Space>::INIT_SPACE;
        assert!(space >= 47, "Space should be at least 47 bytes");
    }

    #[test]
    fn test_transferred_rights_all_fields() {
        let author = create_test_pubkey(10);
        let rights = TransferredRights::new_with_time(
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            author,
            5000,
        );
        
        assert!(rights.can_modify);
        assert!(rights.can_distribute);
        assert!(rights.can_reproduce);
        assert!(rights.can_develop);
        assert!(rights.can_sublicense);
        assert!(rights.can_gift);
        assert!(rights.can_bequeath);
        assert_eq!(rights.transferred_by, author);
        assert_eq!(rights.transferred_at, 5000);
    }

    #[test]
    fn test_transferred_rights_has_any_rights_single() {
        let author = create_test_pubkey(1);
        let rights = vec![
            TransferredRights::new_with_time(true, false, false, false, false, false, false, author, 1000),
            TransferredRights::new_with_time(false, true, false, false, false, false, false, author, 1000),
            TransferredRights::new_with_time(false, false, true, false, false, false, false, author, 1000),
            TransferredRights::new_with_time(false, false, false, true, false, false, false, author, 1000),
            TransferredRights::new_with_time(false, false, false, false, true, false, false, author, 1000),
            TransferredRights::new_with_time(false, false, false, false, false, true, false, author, 1000),
            TransferredRights::new_with_time(false, false, false, false, false, false, true, author, 1000),
        ];
        
        for right in &rights {
            assert!(right.has_any_rights());
        }
    }
}
