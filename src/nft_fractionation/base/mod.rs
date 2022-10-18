pub use base_impl::*;

pub mod base_impl;
mod internal;
mod macros;

pub use self::base_impl::NftFractionationFeature;
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use crate::nft_fractionation::metadata::{ Fractionation, FractionationId };

pub trait NonFungibleTokenFractionation {
    fn nft_fractionation_complete(&mut self, contract_id: AccountId, token_id: FractionationId);

    fn nft_fractionation(&self, contract_id: AccountId, token_id: FractionationId) -> Fractionation;
    fn nft_fractionations(
        &self,
        contract_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<Fractionation>;
    fn nft_fractionations_supply(&self, contract_id: AccountId) -> U128;
}