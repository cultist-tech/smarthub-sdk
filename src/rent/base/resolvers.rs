use near_sdk::{Promise, AccountId, env, is_promise_success, ext_contract};
use crate::rent::{contract_token_id, Rent, time_get_minutes, RentFeature, RentPay, RentFeatureResolve, RentClaim};
use crate::metadata::TokenId;
use crate::nft::base::GAS_FOR_NFT_TRANSFER;
use crate::rent::base::GAS_FOR_RENT_PAY;
use crate::utils::near_ft;
use near_sdk::json_types::U128;
use crate::nft::base::external::ext_nft;
use crate::ft::base::external::ext_ft;
use crate::ft::base::core_impl::GAS_FOR_FT_TRANSFER;

#[ext_contract(ext_self)]
trait ExtSelf {
  fn rent_resolve_pay(
    &mut self,
    contract_id: AccountId,
    token_id: TokenId,
    buyer_id: AccountId,
    owner_id: AccountId,
    receiver_id: AccountId,
    time: u64,
    end_time: u64,
    ft_token_id: AccountId,
    price: U128
  );
  fn rent_resolve_claim(
    &mut self,
    contract_id: AccountId,
    token_id: TokenId,
    owner_id: AccountId,
    renter_id: AccountId
  );
}

impl RentFeature {
  pub fn internal_process_purchase(
    &mut self,
    contract_id: &AccountId,
    token_id: &TokenId,
    buyer_id: &AccountId,
    receiver_id: &AccountId,
    time: &u64,
    ft_token_id: &AccountId,
    deposit: &U128
  ) -> Promise {
    let id = contract_token_id(&contract_id, &token_id);
    let is_paid = self.rents_current.get(&id).is_some();
    let rent: Rent = self.rents_by_id.get(&id).expect("Token is not available for rent");

    assert!(!is_paid, "Token is already in rent");
    assert_ne!(receiver_id, &rent.owner_id, "Not rent owned token");

    let minutes = time_get_minutes(time.clone()) as u128;
    let price_per_hour = rent.sale_conditions.get(&ft_token_id).expect("Ft not available");

    let price = ((price_per_hour.0 * minutes) / 60 / 1_000_000) as u128;

    assert!(
      deposit.0 == price,
      "Invalid attached deposit {}, price {}",
      deposit.0.to_string(),
      price.to_string()
    );

    let now = env::block_timestamp();
    let end_time = now + time;

    assert!(time >= &rent.min_time && time <= &rent.max_time, "Invalid rent time");

    self.rents_pending.remove(&id);
    // self.internal_remove_rent_from_account(&rent.owner_id, &contract_id, &token_id);
    // self.internal_remove_rent_from_contract(&contract_id, &token_id);

    ext_nft
    ::ext(contract_id.clone())
      .with_static_gas(GAS_FOR_NFT_TRANSFER)
      .with_attached_deposit(1)
      .nft_transfer(env::current_account_id(), token_id.clone(), None, None)
      .then(
        ext_self
        ::ext(env::current_account_id())
          .with_static_gas(env::prepaid_gas() - GAS_FOR_RENT_PAY)
          .rent_resolve_pay(
            contract_id.clone(),
            token_id.clone(),
            buyer_id.clone(),
            rent.owner_id.clone(),
            receiver_id.clone(),
            time.clone(),
            end_time.clone(),
            ft_token_id.clone(),
            deposit.clone()
          )
      )
  }

  pub fn internal_resolve_purchase(
    &mut self,
    contract_id: &AccountId,
    token_id: &TokenId,
    buyer_id: &AccountId,
    owner_id: &AccountId,
    receiver_id: &AccountId,
    time: &u64,
    end_time: &u64,
    ft_token_id: &AccountId,
    price: &U128
  ) -> U128 {
    let id = contract_token_id(&contract_id, &token_id);

    if !is_promise_success() {
      self.rents_pending.insert(&id);

      if ft_token_id == &near_ft() {

        Promise::new(buyer_id.clone()).transfer(u128::from(price.clone()));
      }

      return price.clone();
    }

    self.rents_current.insert(&id, &receiver_id);
    self.rents_end_by_id.insert(&id, &end_time);
    self.internal_add_token_to_account(&receiver_id, &contract_id, &token_id);

    if ft_token_id == &near_ft() {
      Promise::new(owner_id.clone()).transfer(u128::from(price.clone()));
    } else {
      ext_ft
      ::ext(ft_token_id.clone())
        .with_static_gas(GAS_FOR_FT_TRANSFER)
        .with_attached_deposit(1)
        .ft_transfer(owner_id.clone(), price.clone(), None);
    }

    (RentPay {
      token_id: &token_id,
      contract_id: &contract_id,
      owner_id: &owner_id,
      time: &time,
      end_time: &end_time,
      price: &price,
      receiver_id: &receiver_id,
    }).emit();

    U128(0)
  }
}

impl RentFeatureResolve for RentFeature {
  fn rent_resolve_pay(
    &mut self,
    contract_id: AccountId,
    token_id: TokenId,
    buyer_id: AccountId,
    owner_id: AccountId,
    receiver_id: AccountId,
    time: u64,
    end_time: u64,
    ft_token_id: AccountId,
    price: U128
  ) -> U128 {
    self.internal_resolve_purchase(
      &contract_id,
      &token_id,
      &buyer_id,
      &owner_id,
      &receiver_id,
      &time,
      &end_time,
      &ft_token_id,
      &price
    )
  }

  fn rent_resolve_claim(
    &mut self,
    contract_id: AccountId,
    token_id: TokenId,
    owner_id: AccountId,
    renter_id: AccountId
  ) {
    let is_success = is_promise_success();

    if !is_success {
      env::panic_str("Error during transfer nft");
    }

    self.internal_remove_current_rent(&owner_id, &contract_id, &token_id);
    self.internal_remove_token_from_account(&renter_id, &contract_id, &token_id);
    self.internal_remove_rent_from_contract(&contract_id, &token_id);
    self.internal_remove_rent_from_account(&owner_id, &contract_id, &token_id);

    (RentClaim {
      token_id: &token_id,
      contract_id: &contract_id,
      owner_id: &owner_id,
      renter_id: &renter_id,
    }).emit();
  }
}
