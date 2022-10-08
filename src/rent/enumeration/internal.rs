use crate::rent::{RentFeature, TokenId, JsonRent, contract_token_id};
use near_sdk::{AccountId, env};

impl RentFeature {
  pub(crate) fn internal_rent_is_ended(&self, id: &String) -> bool {
    let rent_end_at = self.rents_end_by_id.get(&id).expect(&format!("Not found {}", id.to_string()));
    let now = env::block_timestamp();

    now > rent_end_at
  }

  pub(crate) fn enum_rent(&self, contract_token_id: &TokenId) -> Option<JsonRent> {
    if let Some(rent) = self.rents_by_id.get(&contract_token_id) {
      Some(JsonRent {
        token_id: rent.token_id,
        contract_id: rent.contract_id,
        owner_id: rent.owner_id,
        sale_conditions: rent.sale_conditions,
        min_time: rent.min_time,
        max_time: rent.max_time,
        ended_at: self.rents_end_by_id.get(&contract_token_id),
        renter_id: self.rents_current.get(&contract_token_id),
        created_at: rent.created_at,
      })
    } else {
      None
    }
  }

  pub(crate) fn internal_rent_token_is_locked(
    &self,
    nft_contract_id: &AccountId,
    token_id: &TokenId
  ) -> bool {
    self.rents_current.get(&contract_token_id(&nft_contract_id, &token_id)).is_some()
  }

  pub(crate) fn internal_rent_token_ids_for_account(
    &self,
    account_id: &AccountId
  ) -> Vec<TokenId> {
    let tokens_account = self.rent_tokens_per_account.get(&account_id);
    let tokens = if let Some(tokens_account) = tokens_account {
      tokens_account
    } else {
      return vec![];
    };

    tokens
      .iter()
      .filter(|token_id| {

        !self.internal_rent_is_ended(&token_id)
      })
      .collect()
  }

  pub(crate) fn internal_tokens_for_account(&self, account_id: &AccountId) -> Vec<TokenId> {
    let rents_account = self.rents_per_account.get(&account_id);
    let rents = if let Some(rents_account) = rents_account {
      rents_account
    } else {
      return vec![];
    };

    rents.as_vector().to_vec()
  }
}
