use near_sdk::{ AccountId, env, IntoStorageKey };
use crate::whitelist::WhitelistFeatureCore;
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::{ UnorderedSet };

#[derive(BorshDeserialize, BorshSerialize)]
pub struct WhitelistFeature {
    whitelist_ids: UnorderedSet<AccountId>,
}

impl WhitelistFeature {
    pub fn new<Q>(prefix: Q) -> Self where Q: IntoStorageKey {
        let this = Self {
            whitelist_ids: UnorderedSet::new(prefix),
        };

        this
    }

    pub fn internal_is_whitelist(&self, account_id: &AccountId) -> bool {
        self.whitelist_ids.contains(&account_id)
    }

    pub fn assert_whitelist(&self, account_id: &AccountId) {
        let is_whitelist = self.internal_is_whitelist(account_id);

        if !is_whitelist {
            env::panic_str("Whitelist does not include");
        }
    }

  pub fn internal_list(&self) -> Vec<AccountId> {
    self.whitelist_ids.to_vec()
  }
}

impl WhitelistFeatureCore for WhitelistFeature {
    fn is_whitelist(&self, account_id: AccountId) -> bool {
        self.internal_is_whitelist(&account_id)
    }

    fn whitelist_add(&mut self, account_id: AccountId) -> bool {
        self.whitelist_ids.insert(&account_id)
    }

    fn whitelist_remove(&mut self, account_id: AccountId) -> bool {
        self.whitelist_ids.remove(&account_id)
    }
}
