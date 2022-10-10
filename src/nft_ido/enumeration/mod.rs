use crate::nft_ido::{JsonIdo, IdoId};
use near_sdk::AccountId;
use crate::metadata::TokenId;
use near_sdk::json_types::U128;

mod macros;
mod enumeration_impl;

pub trait NftIdoEnumeration {
  fn nft_idos(&self) -> Vec<JsonIdo>;

  fn nft_ido(&self, contract_id: AccountId, ido_id: IdoId) -> Option<JsonIdo>;

  fn nft_ido_not_minted(&self, contract_id: AccountId, ido_id: IdoId) -> u64;

  fn nft_ido_tokens(
    &self,
    contract_id: AccountId,
    ido_id: IdoId,
    from_index: Option<U128>,
    limit: Option<u64>
  ) -> Vec<TokenId>;

  fn nft_ido_account_minted(
    &self,
    contract_id: AccountId,
    ido_id: IdoId,
    account_id: AccountId
  ) -> u64;

  fn nft_idos_by_contract(
    &self,
    contract_id: AccountId,
    from_index: Option<U128>,
    limit: Option<u64>
  ) -> Vec<JsonIdo>;

  fn nft_idos_supply_by_contract(&self, contract_id: AccountId) -> U128;
}
