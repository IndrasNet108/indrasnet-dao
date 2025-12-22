//! Partnership role registry state

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PartnershipRoleRegistry {
    pub authority: Pubkey,
    #[max_len(32)]
    pub partnership_admins: Vec<Pubkey>,
    #[max_len(32)]
    pub revenue_admins: Vec<Pubkey>,
    #[max_len(32)]
    pub metrics_admins: Vec<Pubkey>,
    pub bump: u8,
}

impl PartnershipRoleRegistry {
    pub fn is_partnership_admin(&self, key: &Pubkey) -> bool {
        self.partnership_admins.contains(key)
    }

    pub fn is_revenue_admin(&self, key: &Pubkey) -> bool {
        self.revenue_admins.contains(key)
    }

    pub fn is_metrics_admin(&self, key: &Pubkey) -> bool {
        self.metrics_admins.contains(key)
    }

    pub fn upsert_partnership_admin(&mut self, key: Pubkey) {
        if !self.partnership_admins.contains(&key) {
            self.partnership_admins.push(key);
        }
    }

    pub fn remove_partnership_admin(&mut self, key: &Pubkey) {
        self.partnership_admins.retain(|k| k != key);
    }

    pub fn upsert_revenue_admin(&mut self, key: Pubkey) {
        if !self.revenue_admins.contains(&key) {
            self.revenue_admins.push(key);
        }
    }

    pub fn remove_revenue_admin(&mut self, key: &Pubkey) {
        self.revenue_admins.retain(|k| k != key);
    }

    pub fn upsert_metrics_admin(&mut self, key: Pubkey) {
        if !self.metrics_admins.contains(&key) {
            self.metrics_admins.push(key);
        }
    }

    pub fn remove_metrics_admin(&mut self, key: &Pubkey) {
        self.metrics_admins.retain(|k| k != key);
    }
}
