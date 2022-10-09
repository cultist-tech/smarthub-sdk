use near_sdk::AccountId;
use crate::referral::{InfluencerId, ContractId, ProgramId};

mod enumeration_impl;
mod internal;
mod macros;

pub trait ReferralEnumeration {
  // get referrals by contract
  fn referrals_by_contract(&self, contract_id: AccountId) -> Vec<AccountId>;
  // get count of referrals by contract
  fn referrals_supply_by_contract(&self, contract_id: AccountId) -> u128;

  // get referrals by program
  fn referrals_by_program(&self, contract_id: AccountId, influencer_id: AccountId, program_id: String) -> Vec<AccountId>;
  // get count of referrals by program
  fn referrals_supply_by_program(&self, contract_id: AccountId, influencer_id: AccountId, program_id: String) -> u128;

  // get referrals by influencer
  fn referrals_by_influencer(&self, influencer_id: InfluencerId) -> Vec<AccountId>;
  // get counts of referrals by influencer
  fn referrals_supply_by_influencer(&self, influencer_id: InfluencerId) -> u128;

  // get contracts by influencer
  fn referral_contracts_by_influencer(&self, influencer_id: InfluencerId) -> Vec<ContractId>;
  // get programs by influencer
  fn referral_programs_by_influencer(&self, influencer_id: InfluencerId, contract_id: ContractId) -> Vec<ProgramId>;

  // get influencers by contract
  fn referral_influencers_by_contract(&self, contract_id: ContractId) -> Vec<InfluencerId>;
  // get programs by contract
  fn referral_programs_by_contract(&self, contract_id: InfluencerId, influencer_id: InfluencerId) -> Vec<ProgramId>;
}
