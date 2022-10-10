#[macro_export]
macro_rules! impl_nft_ido_enumeration {
    ($contract:ident, $tokens:ident) => {
        use $crate::nft_ido::{NftIdoEnumeration};

        #[near_bindgen]
        impl NftIdoEnumeration for $contract {

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
