use near_sdk::{ env, AccountId, IntoStorageKey };
use near_sdk::collections::LookupMap;
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use crate::reputation::ContractReputation;

pub const MAX_REPUTATION: u32 = 100_000;
const MIN_REPUTATION: u32 = 0;

pub const BUY_INCREMENT: u32 = 10;
pub const SALE_INCREMENT: u32 = 5;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ReputationFeature {
    reputation_by_id: LookupMap<AccountId, u32>,
}

impl ReputationFeature {
    pub fn new<R>(prefix: R) -> Self  where R: IntoStorageKey{     
        let this = Self {
            reputation_by_id: LookupMap::new(prefix),
        };

        this
    }       

    pub fn internal_add_reputation(&mut self, account_id: &AccountId, amount: &u32) -> u32 {
        let reputations = &mut self.reputation_by_id;
        
        let next_reputation = if let Some(reputation) = reputations.get(&account_id){
            if reputation + amount > MAX_REPUTATION {
                MAX_REPUTATION
            } else {
                reputation + amount
            }
        } else {
            if *amount >= MAX_REPUTATION {
                MAX_REPUTATION
            } else {
                *amount
            }
        };
        
        reputations.insert(&account_id, &next_reputation);
        
        next_reputation        
    }
    
    pub fn internal_sub_reputation(&mut self, account_id: &AccountId, amount: &u32) -> u32 {
        let reputations = &mut self.reputation_by_id;
        
        let next_reputation = if let Some(reputation) = reputations.get(&account_id){
            if reputation > *amount {
                reputation - amount
            } else {
                MIN_REPUTATION
            }
        } else {
            MIN_REPUTATION
        };
        
        reputations.insert(&account_id, &next_reputation);
        
        next_reputation
    }
    
    pub fn internal_reputation(&self, account_id: &AccountId) -> u32 {
        self.reputation_by_id.get(&account_id).unwrap_or_else(|| 0)
    }
}

impl ContractReputation for ReputationFeature {
    fn reputation(&self, account_id: AccountId) -> u32 {
        self.internal_reputation(&account_id)
    }
}
