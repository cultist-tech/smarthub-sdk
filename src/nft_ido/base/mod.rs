pub use base_impl::*;

pub mod base_impl;
mod internal;
mod macros;
mod resolvers;
mod receivers;
mod external;

pub use self::base_impl::NftIdoFeature;
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

pub trait IdoEnumeration {
    fn nft_idos(&self) -> Vec<JsonIdo>;

    fn nft_ido(&self, contract_id: AccountId, ido_id: IdoId) -> Option<JsonIdo>;

    fn nft_ido_not_minted(&self, contract_id: AccountId, ido_id: IdoId) -> u64;

    fn nft_ido_tokens(
        &self,
        contract_id: AccountId,
        ido_id: IdoId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<TokenId>;

    fn nft_ido_account_minted(
        &self,
        contract_id: AccountId,
        ido_id: IdoId,
        account_id: AccountId
    ) -> u64;

    fn nft_idos_by_contract(
        &self,
        contract_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<JsonIdo>;

    fn nft_idos_supply_by_contract(&self, contract_id: AccountId) -> U128;
}

pub trait NftIdoResolvers {
    fn resolve_nft_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        ido_id: IdoId,
        contract_id: AccountId
    ) -> bool;
}
