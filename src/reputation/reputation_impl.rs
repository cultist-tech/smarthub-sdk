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
        if let Some(reputation) = reputations.get(&account_id) {
        
            if reputation < MAX_REPUTATION {
                let increased_reputation = reputation + amount;
                
                if increased_reputation < MAX_REPUTATION {
                    reputations.insert(&account_id, &increased_reputation);
                    return increased_reputation;
                } else {
                    reputations.insert(&account_id, &MAX_REPUTATION);
                    return MAX_REPUTATION;
                }
            }    
            return MAX_REPUTATION;            
        }

        reputations.insert(&account_id, &amount);
        
        amount.clone()
    }
    
    pub fn internal_sub_reputation(&mut self, account_id: &AccountId, amount: &u32) -> u32 {
        let reputations = &mut self.reputation_by_id;
        if let Some(reputation) = reputations.get(&account_id) {
        
            if reputation > MIN_REPUTATION {
            
                if reputation < *amount {
                    reputations.insert(&account_id, &MIN_REPUTATION);
                    return MIN_REPUTATION;
                } else {
                    let decreased_reputation = reputation - amount;
                    
                    reputations.insert(&account_id, &decreased_reputation);
                    return decreased_reputation;
                }
            }    
            return MIN_REPUTATION;            
        }        
        
        MIN_REPUTATION
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
