use near_sdk::AccountId;

pub use reputation_impl::*;
pub use macros::*;

pub mod reputation_impl;
pub mod macros;

pub trait ContractReputation {
    fn reputation(&self, account_id: AccountId) -> u32;
}
