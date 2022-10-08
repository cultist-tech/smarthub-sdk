use near_sdk::{AccountId, PromiseOrValue, ext_contract, Gas, env};
use crate::referral::ReferralFeature;
use crate::referral::base::resolver::{ext_self};

pub const GAS_FOR_RESOLVE_REFERRAL_CREATE: Gas = Gas(5_000_000_000_000);
pub const GAS_FOR_REFERRAL_CREATE: Gas = Gas(
  25_000_000_000_000 + GAS_FOR_RESOLVE_REFERRAL_CREATE.0
);


#[ext_contract(ext_receiver)]
pub trait ReferralReceiver {
  // cross contract call
  fn refferal_on_create(&self, account_id: AccountId, influencer_id: AccountId, program_id: String) -> PromiseOrValue<bool>;
}

impl ReferralFeature {
  pub(crate) fn internal_call_on_referral(
    &self,
    contract_id: &AccountId,
    account_id: &AccountId,
    influencer_id: &AccountId,
    program_id: &String
  ) -> PromiseOrValue<bool> {
    ext_receiver
    ::ext(contract_id.clone())
      .with_static_gas(env::prepaid_gas() - GAS_FOR_REFERRAL_CREATE)
      .refferal_on_create(account_id.clone(), influencer_id.clone(), program_id.clone())
      .then(
        ext_self
        ::ext(env::current_account_id())
          .with_static_gas(GAS_FOR_RESOLVE_REFERRAL_CREATE)
          .resolve_on_referral_create(contract_id.clone(), account_id.clone(), influencer_id.clone(), program_id.clone())
      )
      .into()
  }
}
