use near_sdk::{ AccountId, env, IntoStorageKey };
use crate::blacklist::ContractBlacklistCore;
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::{ UnorderedSet };

#[derive(BorshDeserialize, BorshSerialize)]
pub struct BlacklistFeature {
    blocked_account_id: UnorderedSet<AccountId>,
}

impl BlacklistFeature {
    pub fn new<Q>(prefix: Q) -> Self where Q: IntoStorageKey {
        let this = Self {
            blocked_account_id: UnorderedSet::new(prefix),
        };

        this
    }

    pub fn internal_is_blocked(&self, account_id: &AccountId) -> bool {
        self.blocked_account_id.contains(&account_id)
    }

    pub fn assert_not_blocked(&self, account_id: &AccountId) {
        let is_blocked = self.internal_is_blocked(account_id);

        if is_blocked {
            env::panic_str("Address is blocked");
        }
    }
}

impl ContractBlacklistCore for BlacklistFeature {
    fn is_blacklist(&self, account_id: AccountId) -> bool {
        self.internal_is_blocked(&account_id)
    }

    fn blacklist_add(&mut self, account_id: AccountId) -> bool {
        self.blocked_account_id.insert(&account_id)
    }

    fn blacklist_remove(&mut self, account_id: AccountId) -> bool {
        self.blocked_account_id.remove(&account_id)
    }
}