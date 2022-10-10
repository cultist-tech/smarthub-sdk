pub use base_impl::*;

pub mod base_impl;
mod internal;
mod macros;
mod resolvers;
mod receivers;

pub use self::base_impl::NftIdoFeature;
pub use self::resolvers::{NftIdoResolvers};
use crate::nft_ido::{ JsonIdo, IdoId, TokenId };
use near_sdk::json_types::U128;
use near_sdk::AccountId;

pub trait IdoCore {
    fn nft_ido_add(
        &mut self,
        contract_id: AccountId,
        ido_id: IdoId,
        name: String,
        amount: u64,
        price: U128,
        per_transaction_min: u64,
        per_transaction_max: u64,
        buy_max: u64,
        ft_token: Option<AccountId>,
        media: Option<String>
    ) -> JsonIdo;

    fn nft_ido_start(&mut self, contract_id: AccountId, ido_id: IdoId, date: u64) -> JsonIdo;

    fn nft_ido_update(
        &mut self,
        contract_id: AccountId,
        ido_id: IdoId,
        date: u64,
        per_transaction_min: u64,
        per_transaction_max: u64,
        buy_max: u64
    ) -> JsonIdo;

    fn nft_ido_pause(&mut self, contract_id: AccountId, ido_id: IdoId, pause: bool) -> JsonIdo;

    fn nft_ido_buy(
        &mut self,
        contract_id: AccountId,
        receiver_id: AccountId,
        ido_id: IdoId,
        amount: u64
    );
}
