use near_sdk::{ env, AccountId, IntoStorageKey, BorshStorageKey };
use near_sdk::collections::LookupMap;
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use crate::reputation::{ ContractReputation, ReputationSharing };
use std::cmp;

pub const MAX_REPUTATION: u32 = 100_000;
const MIN_REPUTATION: u32 = 0;

pub const BUY_INCREMENT: u32 = 10;
pub const SALE_INCREMENT: u32 = 5;

const DAILY_SHARE_CAP: u32 = 6;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ReputationFeature {
    reputation_by_id: LookupMap<AccountId, u32>,
    shares_by_id: Option<LookupMap<AccountId, LookupMap<u64, u32>>>,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    SharesPerAccount {
        account_hash: Vec<u8>,
    },
}

pub(crate) fn current_day() -> u64 {
    let now = env::block_timestamp() as u64;
    let day = 86400000 * 1000000;

    now / day
}

impl ReputationFeature {
    pub fn new<R>(prefix: R) -> Self where R: IntoStorageKey {
        let prefix: Vec<u8> = prefix.into_storage_key();
        let sharing_prefix = Some([prefix.clone() , "shares".into()].concat());
        let this = Self {
            reputation_by_id: LookupMap::new(prefix),
            shares_by_id: sharing_prefix.map(LookupMap::new),             
        };

        this
    }

    pub fn internal_add_reputation(&mut self, account_id: &AccountId, amount: &u32) -> u32 {
        let reputations = &mut self.reputation_by_id;

        let next_reputation = if let Some(reputation) = reputations.get(&account_id) {
            if reputation + amount > MAX_REPUTATION { MAX_REPUTATION } else { reputation + amount }
        } else {
            if *amount >= MAX_REPUTATION { MAX_REPUTATION } else { *amount }
        };

        reputations.insert(&account_id, &next_reputation);

        next_reputation
    }

    pub fn internal_sub_reputation(&mut self, account_id: &AccountId, amount: &u32) -> u32 {
        let reputations = &mut self.reputation_by_id;

        let next_reputation = if let Some(reputation) = reputations.get(&account_id) {
            if reputation > *amount { reputation - amount } else { MIN_REPUTATION }
        } else {
            MIN_REPUTATION
        };

        reputations.insert(&account_id, &next_reputation);

        next_reputation
    }

    pub fn internal_reputation(&self, account_id: &AccountId) -> u32 {
        self.reputation_by_id.get(&account_id).unwrap_or_else(|| 0)
    }

    pub(crate) fn internal_decrease_shares(&mut self, sender_id: &AccountId, amount: &u32) -> u32 {
        let day = current_day();
        
        let shares_by_id = self.shares_by_id.as_mut().expect("Reputation sharing is not implemented");

        let mut account_shares = shares_by_id.get(&sender_id).unwrap_or_else(||
            LookupMap::new(StorageKey::SharesPerAccount {
                account_hash: env::sha256(sender_id.as_bytes()),
            })
        );

        let used = account_shares.get(&day).unwrap_or_else(|| 0);
        let left_for_share = DAILY_SHARE_CAP - used;

        if left_for_share < amount.clone() {
            env::panic_str("All daily votes used");
        }

        let next_used = used + amount;
        account_shares.insert(&day, &next_used);
        shares_by_id.insert(&sender_id, &account_shares);

        left_for_share - amount
    }
    
    pub(crate) fn internal_shares_left(&self, account_id: &AccountId) -> u32 {
        let day = current_day();

        let reputation = self.internal_reputation(&account_id);  
        
        let shares_by_id = self.shares_by_id.as_ref().expect("Reputation sharing is not implemented");
        
        let shares_left = if let Some(account_shares) = shares_by_id.get(&account_id) {
            let used = account_shares.get(&day).unwrap_or_else(|| 0);
            let left_for_share = DAILY_SHARE_CAP - used;
        
            cmp::min(reputation, left_for_share)
        } else {
            cmp::min(reputation, DAILY_SHARE_CAP)
        };

        shares_left        
    }
}

impl ContractReputation for ReputationFeature {
    fn reputation(&self, account_id: AccountId) -> u32 {
        self.internal_reputation(&account_id)
    }
}

impl ReputationSharing for ReputationFeature {
    fn reputation_share(&mut self, receiver_id: AccountId, amount: u32) -> u32 {
        let sender_id = env::predecessor_account_id();

        let reputation = self.internal_reputation(&sender_id);

        assert!(reputation >= amount, "Don't have enough reputation for sharing");

        assert_ne!(sender_id, receiver_id, "Self sharing not allowed");

        let left_for_share = self.internal_decrease_shares(&sender_id, &amount);

        self.internal_sub_reputation(&sender_id, &amount);
        self.internal_add_reputation(&receiver_id, &amount);

        left_for_share
    }
    
    fn reputation_shares_left(&self, account_id: AccountId) -> u32 {
        self.internal_shares_left(&account_id)        
    }
}
