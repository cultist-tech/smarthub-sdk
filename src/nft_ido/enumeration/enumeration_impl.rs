use crate::nft_ido::{NftIdoEnumeration, NftIdoFeature, JsonIdo, IdoId};
use crate::metadata::TokenId;
use near_sdk::{AccountId, require};
use crate::nft_ido::utils::contract_token_id;
use near_sdk::json_types::U128;

impl NftIdoEnumeration for NftIdoFeature {
  fn nft_idos(&self) -> Vec<JsonIdo> {
    self.idos_available
      .iter()
      .map(|contract_ido_id| { self.enum_get_ido(&contract_ido_id).unwrap() })
      .collect()
  }

  fn nft_ido(&self, contract_id: AccountId, ido_id: IdoId) -> Option<JsonIdo> {
    let id = contract_token_id(&contract_id, &ido_id);

    self.enum_get_ido(&id)
  }

  fn nft_ido_not_minted(&self, contract_id: AccountId, ido_id: IdoId) -> u64 {
    let id = contract_token_id(&contract_id, &ido_id);

    let ido = self.ido_by_id.get(&id).expect("Not found ido");
    let rand_tokens = self.ido_random_tokens.get(&id);

    if let Some(rand_tokens) = rand_tokens {
      return rand_tokens.len() as u64;
    }

    ido.amount
  }

  fn nft_ido_tokens(
    &self,
    contract_id: AccountId,
    ido_id: IdoId,
    from_index: Option<U128>,
    limit: Option<u64>
  ) -> Vec<TokenId> {
    let id = contract_token_id(&contract_id, &ido_id);

    let tokens_per_ido = &self.ido_tokens;
    let token_set = if let Some(token_set) = tokens_per_ido.get(&id) {
      token_set
    } else {
      return vec![];
    };

    let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
    require!(limit != 0, "Cannot provide limit of 0.");
    let start_index: u128 = from_index.map(From::from).unwrap_or_default();

    require!(
            (token_set.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );

    token_set.iter().collect()
  }

  fn nft_ido_account_minted(
    &self,
    contract_id: AccountId,
    ido_id: IdoId,
    account_id: AccountId
  ) -> u64 {
    let id = contract_token_id(&contract_id, &ido_id);
    let owner_minted = self.internal_mint_counter_by_ido(&account_id, &id);

    owner_minted
  }

  fn nft_idos_by_contract(
    &self,
    contract_id: AccountId,
    from_index: Option<U128>,
    limit: Option<u64>
  ) -> Vec<JsonIdo> {
    let ids = if let Some(ids) = self.ido_tokens_by_contract.get(&contract_id) {
      ids
    } else {
      return vec![];
    };

    let start = u128::from(from_index.unwrap_or(U128(0)));

    ids.as_vector()
      .iter()
      .skip(start as usize)
      .take(limit.unwrap_or(0) as usize)
      .map(|token_id| self.enum_get_ido(&contract_token_id(&contract_id, &token_id)).unwrap())
      .collect()
  }

  fn nft_idos_supply_by_contract(&self, contract_id: AccountId) -> U128 {
    if let Some(list) = self.ido_tokens_by_contract.get(&contract_id) {
      return U128::from(list.len() as u128);
    }

    U128::from(0)
  }
}
