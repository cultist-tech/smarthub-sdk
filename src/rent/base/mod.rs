use near_sdk::{ AccountId, Promise };
use near_sdk::json_types::U128;

mod base_impl;

mod internal;
mod receivers;
mod resolvers;

pub use self::base_impl::*;
use crate::rent::TokenId;

pub trait RentFeatureCore {
    //
    fn rent_token_is_locked(&self, contract_id: AccountId, token_id: TokenId) -> bool;

    fn rent_update(
        &mut self,
        contract_id: AccountId,
        token_id: TokenId,
        ft_token_id: &AccountId,
        price_per_hour: U128,
        min_time: u64,
        max_time: u64
    );
    fn rent_remove(&mut self, contract_id: AccountId, token_id: TokenId);

    // payable
    fn rent_pay(
        &mut self,
        contract_id: AccountId,
        token_id: TokenId,
        time: u64,
        receiver_id: AccountId
    ) -> Promise;
    fn rent_claim(&mut self, contract_id: AccountId, token_id: TokenId) -> Promise;

    fn rent_is_ended(&self, contract_id: AccountId, token_id: TokenId) -> bool;
    fn rent_total_supply(&self) -> u64;

    fn rent_is_approved(
        &self,
        contract_id: AccountId,
        token_id: TokenId,
        account_id: AccountId
    ) -> bool;
}
pub trait RentFeatureResolve {
    fn rent_resolve_pay(
        &mut self,
        contract_id: AccountId,
        token_id: TokenId,
        buyer_id: AccountId,
        owner_id: AccountId,
        receiver_id: AccountId,
        time: u64,
        end_time: u64,
        ft_token_id: AccountId,
        price: U128
    ) -> U128;
    fn rent_resolve_claim(
        &mut self,
        contract_id: AccountId,
        token_id: TokenId,
        owner_id: AccountId,
        renter_id: AccountId
    );
}
