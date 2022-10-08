mod enumeration_impl;
mod internal;

use near_sdk::json_types::U128;
use near_sdk::AccountId;
use crate::rent::{ JsonRent, TokenId };

pub trait RentFeatureEnumeration {
    fn rents(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonRent>;
    fn rents_by_ids(&self, ids: Vec<TokenId>) -> Vec<JsonRent>;

    fn rents_for_account(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<JsonRent>;
    fn rents_supply_for_account(&self, account_id: AccountId) -> U128;
    fn rented_tokens_for_account(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<JsonRent>;
    fn rented_tokens_supply_for_account(&self, account_id: AccountId) -> U128;

    fn rents_by_contract(
        &self,
        contract_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<JsonRent>;
    fn rents_supply_by_contract(&self, contract_id: AccountId) -> U128;

    fn rent(&self, contract_id: AccountId, token_id: TokenId) -> Option<JsonRent>;
}
