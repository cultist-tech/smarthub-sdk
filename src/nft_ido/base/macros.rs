// Ido

#[macro_export]
macro_rules! impl_nft_ido_core {
    ($contract:ident, $tokens:ident, $assert_access:ident) => {
        use $crate::nft_ido::{IdoCore, NftIdoResolvers, JsonIdo};

        #[near_bindgen]
        impl IdoCore for $contract {
          #[payable]
          fn nft_ido_add(&mut self, contract_id: AccountId, ido_id: $crate::nft_ido::IdoId, name: String, amount: u64, price: U128, per_transaction_min: u64, per_transaction_max: u64, buy_max: u64, ft_token: Option<AccountId>, media: Option<String>) -> JsonIdo {
            self.$assert_access();
            self.$tokens.nft_ido_add(contract_id, ido_id, name, amount, price, per_transaction_min, per_transaction_max, buy_max, ft_token, media)
          }

          fn nft_ido_start(&mut self, contract_id: AccountId, ido_id: $crate::nft_ido::IdoId, date: u64) -> JsonIdo {
            self.$assert_access();
            self.$tokens.nft_ido_start(contract_id, ido_id, date)
          }

          fn nft_ido_update(&mut self, contract_id: AccountId, ido_id: $crate::nft_ido::IdoId, date: u64, per_transaction_min: u64, per_transaction_max: u64, buy_max: u64) -> JsonIdo {
            self.$assert_access();
            self.$tokens.nft_ido_update(contract_id, ido_id, date, per_transaction_min, per_transaction_max, buy_max)
          }

           fn nft_ido_pause(&mut self, contract_id: AccountId, ido_id: $crate::nft_ido::IdoId, pause: bool) -> JsonIdo {
            self.$assert_access();
            self.$tokens.nft_ido_pause(contract_id, ido_id, pause)
          }

             #[payable]
            fn nft_ido_buy(&mut self, contract_id: AccountId, receiver_id: AccountId, ido_id: $crate::nft_ido::IdoId, amount: u64) {
              self.$tokens.nft_ido_buy(contract_id, receiver_id, ido_id, amount)
            }
        }

        #[near_bindgen]
        impl NftIdoResolvers for $contract {

          #[private]
          fn resolve_nft_transfer(&mut self, sender_id: AccountId, receiver_id: AccountId, token_id: TokenId, ido_id: $crate::nft_ido::IdoId, contract_id: AccountId) -> bool {
            self.$tokens.resolve_nft_transfer(sender_id, receiver_id, token_id, ido_id, contract_id)
          }
        }
    };
}

#[macro_export]
macro_rules! impl_nft_ido_enumeration {
    ($contract:ident, $tokens:ident) => {
        use $crate::nft_ido::{IdoEnumeration};

        #[near_bindgen]
        impl IdoEnumeration for $contract {

          fn nft_idos(&self) -> Vec<JsonIdo> {
            self.$tokens.nft_idos()
          }

          fn nft_ido(&self, contract_id: AccountId, ido_id: IdoId) -> Option<JsonIdo> {
            self.$tokens.nft_ido(contract_id, ido_id)
          }

          fn nft_ido_not_minted(&self, contract_id: AccountId, ido_id: IdoId) -> u64 {
            self.$tokens.nft_ido_not_minted(contract_id, ido_id)
          }

          fn nft_ido_tokens(&self, contract_id: AccountId, ido_id: IdoId, from_index: Option<near_sdk::json_types::U128>, limit: Option<u64>) -> Vec<TokenId> {
             self.$tokens.nft_ido_tokens(contract_id, ido_id, from_index, limit)
          }

          fn nft_ido_account_minted(&self, contract_id: AccountId, ido_id: IdoId, account_id: AccountId) -> u64 {
            self.$tokens.nft_ido_account_minted(contract_id, ido_id, account_id)
          }

          fn nft_idos_by_contract(&self, contract_id: AccountId, from_index: Option<U128>, limit: Option<u64>,) -> Vec<JsonIdo> {
             self.$tokens.nft_idos_by_contract(contract_id, from_index, limit)
          }

          fn nft_idos_supply_by_contract(&self, contract_id: AccountId) -> near_sdk::json_types::U128 {
             self.$tokens.nft_idos_supply_by_contract(contract_id)
          }
        }
    };
}
