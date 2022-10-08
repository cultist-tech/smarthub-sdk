use near_sdk::json_types::U128;
use near_sdk::AccountId;
use crate::rent::{ RentFeatureEnumeration, RentFeature, JsonRent, contract_token_id, TokenId };

impl RentFeatureEnumeration for RentFeature {
    fn rents(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonRent> {
        let keys = self.rents_pending.as_vector().to_vec();
        let start = u128::from(from_index.unwrap_or(U128(0)));

        keys.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .map(|token_id| self.enum_rent(&token_id).unwrap())
            .collect()
    }

    fn rents_for_account(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<JsonRent> {
        let ids = self.internal_tokens_for_account(&account_id);

        let start = u128::from(from_index.unwrap_or(U128(0)));
        ids.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .map(|token_id| self.enum_rent(&token_id).expect(&format!("{}", token_id.to_string())))
            .collect()
    }

    fn rents_by_ids(&self, ids: Vec<TokenId>) -> Vec<JsonRent> {
        ids.iter()
            .map(|token_id| self.enum_rent(&token_id).unwrap())
            .collect()
    }

    fn rents_supply_for_account(&self, account_id: AccountId) -> U128 {
        self.rents_per_account
            .get(&account_id)
            .map(|account_rents| U128::from(account_rents.len() as u128))
            .unwrap_or(U128(0))
    }

    fn rent(&self, contract_id: AccountId, token_id: TokenId) -> Option<JsonRent> {
        let id = contract_token_id(&contract_id, &token_id);

        self.enum_rent(&id)
    }

    fn rented_tokens_for_account(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<JsonRent> {
        let ids = self.internal_rent_token_ids_for_account(&account_id);

        let start = u128::from(from_index.unwrap_or(U128(0)));

        ids.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .map(|token_id| self.enum_rent(&token_id).unwrap())
            .collect()
    }

    fn rented_tokens_supply_for_account(&self, account_id: AccountId) -> U128 {
        U128::from(self.internal_rent_token_ids_for_account(&account_id).len() as u128)
    }

    fn rents_by_contract(
        &self,
        contract_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<JsonRent> {
        let ids = if let Some(ids) = self.rent_tokens_by_contract.get(&contract_id) {
            ids
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        ids.as_vector()
            .iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .map(|token_id| self.enum_rent(&contract_token_id(&contract_id, &token_id)).unwrap())
            .collect()
    }
    fn rents_supply_by_contract(&self, contract_id: AccountId) -> U128 {
        if let Some(list) = self.rent_tokens_by_contract.get(&contract_id) {
            return U128::from(list.len() as u128);
        }

        U128::from(0)
    }
}
