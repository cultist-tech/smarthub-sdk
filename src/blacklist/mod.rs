use near_sdk::AccountId;

pub use blacklist_impl::*;
pub use macros::*;

pub mod blacklist_impl;
pub mod macros;

pub trait ContractBlacklistCore {
    fn is_blacklist(&self, account_id: AccountId) -> bool;

    fn blacklist_add(&mut self, account_id: AccountId) -> bool;

    fn blacklist_remove(&mut self, account_id: AccountId) -> bool;
}