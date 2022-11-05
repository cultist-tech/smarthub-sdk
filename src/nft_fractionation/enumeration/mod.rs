mod enumeration_impl;
mod macros;

use near_sdk::AccountId;
use near_sdk::json_types::U128;
use crate::nft_fractionation::Fractionation;

pub trait FractionationEnumeration {
    fn nft_fractionations(
        &self,
        contract_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<Fractionation>;
    fn nft_fractionations_supply(&self, contract_id: AccountId) -> U128;
}
