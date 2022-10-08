pub use whitelist_impl::*;
use near_sdk::AccountId;

pub mod whitelist_impl;
mod macros;

pub trait WhitelistFeatureCore {
    fn is_whitelist(&self, account_id: AccountId) -> bool;

    fn whitelist_add(&mut self, account_id: AccountId) -> bool;

    fn whitelist_remove(&mut self, account_id: AccountId) -> bool;
}