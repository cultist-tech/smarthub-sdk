#[macro_export]
macro_rules! impl_rent_core {
    ($contract:ident, $tokens:ident) => {
        use $crate::rent::{RentFeatureCore, RentFeatureResolve};

        #[near_bindgen]
        impl RentFeatureCore for $contract {
          //
          fn rent_token_is_locked(&self, contract_id: AccountId, token_id: $crate::rent::TokenId) -> bool {
            self.$tokens.rent_token_is_locked(contract_id, token_id)
          }
          fn rent_update(&mut self, contract_id: AccountId, token_id: $crate::rent::TokenId, ft_token_id: &AccountId, price_per_hour: near_sdk::json_types::U128, min_time: u64, max_time: u64) {
            self.$tokens.rent_update(contract_id, token_id, ft_token_id, price_per_hour, min_time, max_time)
          }
          fn rent_remove(&mut self, contract_id: AccountId, token_id: $crate::rent::TokenId) {
            self.$tokens.rent_remove(contract_id, token_id)
          }

          #[payable]
          fn rent_pay(&mut self, contract_id: AccountId, token_id: $crate::rent::TokenId, time: u64, receiver_id: AccountId) -> near_sdk::Promise {
            self.$tokens.rent_pay(contract_id, token_id, time, receiver_id)
          }
          fn rent_claim(&mut self, contract_id: AccountId, token_id: $crate::rent::TokenId) -> near_sdk::Promise {
            self.$tokens.rent_claim(contract_id, token_id)
          }
          fn rent_is_ended(&self, contract_id: AccountId, token_id: $crate::rent::TokenId) -> bool {
            self.$tokens.rent_is_ended(contract_id, token_id)
          }
          fn rent_total_supply(&self) -> u64 {
            self.$tokens.rent_total_supply()
          }
          fn rent_is_approved(&self, contract_id: AccountId, token_id: $crate::rent::TokenId, account_id: AccountId) -> bool {
            self.$tokens.rent_is_approved(contract_id, token_id, account_id)
          }
        }

          #[near_bindgen]
        impl RentFeatureResolve for $contract {
          //
          #[private]
          fn rent_resolve_pay(&mut self, contract_id: AccountId, token_id: $crate::rent::TokenId, buyer_id: AccountId, owner_id: AccountId, receiver_id: AccountId, time: u64, end_time: u64, ft_token_id: AccountId, price: near_sdk::json_types::U128) -> near_sdk::json_types::U128  {
            self.$tokens.rent_resolve_pay(contract_id, token_id, buyer_id, owner_id, receiver_id, time, end_time, ft_token_id, price)
          }
          #[private]
          fn rent_resolve_claim(&mut self, contract_id: AccountId, token_id: $crate::rent::TokenId, owner_id: AccountId, renter_id: AccountId) {
            self.$tokens.rent_resolve_claim(contract_id, token_id, owner_id, renter_id)
          }
        }
    };
}

#[macro_export]
macro_rules! impl_rent_enumeration {
    ($contract:ident, $tokens:ident) => {
        use $crate::rent::{RentFeatureEnumeration, JsonRent};

        #[near_bindgen]
        impl RentFeatureEnumeration for $contract {
          fn rents(&self, from_index: Option<near_sdk::json_types::U128>, limit: Option<u64>) -> Vec<JsonRent> {
           self.$tokens.rents(from_index, limit)
          }
          fn rents_for_account(&self, account_id: AccountId, from_index: Option<near_sdk::json_types::U128>, limit: Option<u64>) -> Vec<JsonRent> {
           self.$tokens.rents_for_account(account_id, from_index, limit)
          }
          fn rents_supply_for_account(&self, account_id: AccountId) -> near_sdk::json_types::U128 {
            self.$tokens.rents_supply_for_account(account_id)
          }
           fn rent(&self, contract_id: AccountId, token_id: $crate::rent::TokenId) -> Option<JsonRent> {
           self.$tokens.rent(contract_id, token_id)
          }
          fn rented_tokens_for_account(&self, account_id: AccountId, from_index: Option<near_sdk::json_types::U128>, limit: Option<u64>) -> Vec<JsonRent> {
            self.$tokens.rented_tokens_for_account(account_id, from_index, limit)
          }
          fn rented_tokens_supply_for_account(&self, account_id: AccountId) -> near_sdk::json_types::U128 {
            self.$tokens.rented_tokens_supply_for_account(account_id)
          }
          fn rents_by_ids(&self, ids: Vec<$crate::rent::TokenId>) -> Vec<JsonRent> {
            self.$tokens.rents_by_ids(ids)
          }

          fn rents_by_contract(&self, contract_id: AccountId, from_index: Option<near_sdk::json_types::U128>, limit: Option<u64>) -> Vec<JsonRent> {
            self.$tokens.rents_by_contract(contract_id, from_index, limit)
          }
          fn rents_supply_by_contract(&self, contract_id: AccountId) -> near_sdk::json_types::U128 {
            self.$tokens.rents_supply_by_contract(contract_id)
          }
        }
    };
}
