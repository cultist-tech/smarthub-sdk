pub use royalty_impl::*;
use near_sdk::AccountId;

pub mod royalty_impl;
pub mod macros;

pub use self::macros::*;

pub trait NonFungibleTokenRoyalty {
    fn set_royalty_value(&mut self, contract_royalty: u32);
    fn set_royalty_account(&mut self, account_id: AccountId) -> AccountId;

    fn nft_royalty_value(&self) -> u32;
    fn nft_royalty_account(&self) -> AccountId;
}
