pub use base_impl::*;
use near_sdk::{ AccountId, ext_contract };
use near_sdk::json_types::U128;
use crate::escrow::metadata::{ EscrowOfferId, JsonEscrow, EscrowEnum };

pub mod base_impl;
mod macros;
mod internal;
mod receivers;
mod resolvers;

pub trait EscrowCore {
    fn escrow_remove_offer(&mut self, offer_id: EscrowOfferId);
}

pub trait EscrowEnumeration {
  fn escrow_offer(&self, offer_id: EscrowOfferId) -> Option<JsonEscrow>;
  fn escrow_offers_by_owner(
    &self,
    account_id: AccountId,
    limit: Option<u64>,
    offset: Option<U128>
  ) -> Vec<JsonEscrow>;
  fn escrow_offers_for_owner(
    &self,
    account_id: AccountId,
    limit: Option<u64>,
    offset: Option<U128>
  ) -> Vec<JsonEscrow>;
  fn escrow_offers_total_by_owner(&self, account_id: AccountId) -> u64;
  fn escrow_offers_total_for_owner(&self, account_id: AccountId) -> u64;
}

#[ext_contract(ext_self)]
pub trait EscrowResolver {
    fn resolve_accept_offer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        offer: EscrowEnum,
        offer_id: EscrowOfferId
    ) -> bool;
    fn resolve_remove_offer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        offer: EscrowEnum,
        offer_id: EscrowOfferId
    ) -> bool;
}
