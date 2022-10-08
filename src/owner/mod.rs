pub use owner_impl::*;
use near_sdk::AccountId;

pub mod owner_impl;
mod macros;

pub trait ContractOwner {
    fn get_owner(&self) -> AccountId;
}

pub trait ContractOwnerTransfer {
    fn set_owner(&mut self, account_id: AccountId) -> AccountId;
}