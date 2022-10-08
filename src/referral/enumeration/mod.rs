use near_sdk::AccountId;
use near_sdk::json_types::{U128};

mod enumeration_impl;

pub trait ReferralEnumeration {
  // get referrals by contract
  fn referrals_by_contract(&self, contract_id: AccountId) -> Vec<AccountId>;
  // get count of referrals by contract
  fn referrals_supply_by_contract(&self, contract_id: AccountId) -> U128;

  // get referrals by program
  fn referrals_by_program(&self, contract_id: AccountId, influencer_id: AccountId, program_id: String) -> Vec<AccountId>;
  // get count of referrals by program
  fn referrals_supply_by_program(&self, contract_id: AccountId, influencer_id: AccountId, program_id: String) -> U128;

  // get referrals by influencer
  fn referrals_by_influencer(&self, contract_id: AccountId, program_id: String) -> Vec<AccountId>;
  // get counts of referrals by influencer
  fn referrals_supply_by_influencer(&self, contract_id: AccountId, program_id: String) -> U128;
}
