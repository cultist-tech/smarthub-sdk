use near_sdk::AccountId;

pub use reputation_impl::*;
pub use macros::*;

pub mod reputation_impl;
pub mod macros;

pub trait ContractReputation {
    fn reputation(&self, account_id: AccountId) -> u32;
}

pub trait ReputationSharing {
    fn reputation_share(&mut self, account_id: AccountId, amount: u32) -> u32;
    
    fn reputation_shares_left(&self, account_id: AccountId) -> u32;
}
