use crate::nft_fractionation::{NftFractionationFeature, FractionationEnumeration, Fractionation};
use near_sdk::{AccountId, require};
use near_sdk::json_types::U128;

impl FractionationEnumeration for NftFractionationFeature {
    fn nft_fractionations(
        &self,
        contract_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<Fractionation> {
        let arr = &self.fractionations_by_contract
            .get(&contract_id)
            .expect("No fractionations for contract!");

        let start_index: u128 = from_index.map(From::from).unwrap_or_default();

        if (arr.len() as u128) <= start_index {
            return vec![];
        }

        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        require!(limit != 0, "Cannot provide limit of 0.");

        let res = arr
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|token_id| self.enum_fractionation(&contract_id, &token_id).unwrap())
            .collect();

        res
    }

    fn nft_fractionations_supply(&self, contract_id: AccountId) -> U128 {
        let count = if let Some(fractionations) = self.fractionations_by_contract.get(&contract_id) {
            fractionations.len()
        } else {
            0
        };

        U128::from(count as u128)
    }
}
