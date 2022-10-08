use near_sdk::{AccountId, PromiseOrValue, ext_contract};

#[ext_contract(ext_receiver)]
pub trait ReferralReceiver {
  // cross contract call
  fn refferal_on_create(&self, account_id: AccountId, influencer_id: AccountId, program_id: String) -> PromiseOrValue<bool>;
}
