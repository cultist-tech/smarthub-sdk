pub use base_impl::*;

pub mod base_impl;
mod internal;
mod macros;

pub use self::base_impl::NftFractionationFeature;
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use crate::nft_fractionation::metadata::{ Fractionation, FractionationId };

pub trait FractionationCore {
    fn nft_fractionation_complete(&mut self, contract_id: AccountId, token_id: FractionationId);

    fn nft_fractionation(&self, contract_id: AccountId, token_id: FractionationId) -> Option<Fractionation>;
}
