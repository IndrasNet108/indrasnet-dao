//! Security role registry state

use anchor_lang::prelude::*;

/// Security role registry (independent from Core roles)
#[account]
#[derive(InitSpace)]
pub struct SecurityRoleRegistry {
    /// Registry authority (root admin)
    pub authority: Pubkey,
    /// Security admins
    #[max_len(50)]
    pub security_admins: Vec<Pubkey>,
    /// Compliance admins
    #[max_len(50)]
    pub compliance_admins: Vec<Pubkey>,
    /// Analytics admins
    #[max_len(50)]
    pub analytics_admins: Vec<Pubkey>,
    /// PDA bump
    pub bump: u8,
}

impl SecurityRoleRegistry {
    pub fn is_security_admin(&self, key: &Pubkey) -> bool {
        self.security_admins.contains(key)
    }

    pub fn is_compliance_admin(&self, key: &Pubkey) -> bool {
        self.compliance_admins.contains(key)
    }

    pub fn is_analytics_admin(&self, key: &Pubkey) -> bool {
        self.analytics_admins.contains(key)
    }

    pub fn upsert_security_admin(&mut self, key: Pubkey) {
        if !self.security_admins.contains(&key) {
            self.security_admins.push(key);
        }
    }

    pub fn remove_security_admin(&mut self, key: &Pubkey) {
        self.security_admins.retain(|k| k != key);
    }

    pub fn upsert_compliance_admin(&mut self, key: Pubkey) {
        if !self.compliance_admins.contains(&key) {
            self.compliance_admins.push(key);
        }
    }

    pub fn remove_compliance_admin(&mut self, key: &Pubkey) {
        self.compliance_admins.retain(|k| k != key);
    }

    pub fn upsert_analytics_admin(&mut self, key: Pubkey) {
        if !self.analytics_admins.contains(&key) {
            self.analytics_admins.push(key);
        }
    }

    pub fn remove_analytics_admin(&mut self, key: &Pubkey) {
        self.analytics_admins.retain(|k| k != key);
    }
}
