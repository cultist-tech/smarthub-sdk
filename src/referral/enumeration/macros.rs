/// The core methods for a referral. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_referral_enumeration {
    ($contract:ident, $tokens:ident) => {
        use $crate::referral::{ReferralEnumeration};

        #[near_bindgen]
        impl ReferralEnumeration for $contract {
          // get referrals by contract
          fn referrals_by_contract(&self, contract_id: $crate::referral::ContractId) -> Vec<AccountId> {
            self.$tokens.referrals_by_contract(contract_id)
          }
          // get count of referrals by contract
          fn referrals_supply_by_contract(&self, contract_id: $crate::referral::ContractId) -> u128 {
           self.$tokens.referrals_supply_by_contract(contract_id)
          }

          // get referrals by program
          fn referrals_by_program(&self, contract_id: AccountId, influencer_id: AccountId, program_id: String) -> Vec<AccountId> {
           self.$tokens.referrals_by_program(contract_id, influencer_id, program_id)
          }
          // get count of referrals by program
          fn referrals_supply_by_program(&self, contract_id: AccountId, influencer_id: AccountId, program_id: String) -> u128 {
            self.$tokens.referrals_supply_by_program(contract_id, influencer_id, program_id)
          }

          // get referrals by influencer
          fn referrals_by_influencer(&self, influencer_id: $crate::referral::InfluencerId) -> Vec<AccountId> {
           self.$tokens.referrals_by_influencer(influencer_id)
          }
          // get counts of referrals by influencer
          fn referrals_supply_by_influencer(&self, influencer_id: $crate::referral::InfluencerId) -> u128 {
           self.$tokens.referrals_supply_by_influencer(influencer_id)
          }

          // get contracts by influencer
          fn referral_contracts_by_influencer(&self, influencer_id: $crate::referral::InfluencerId) -> Vec<$crate::referral::ContractId> {
            self.$tokens.referral_contracts_by_influencer(influencer_id)
          }
          // get programs by influencer
          fn referral_programs_by_influencer(&self, influencer_id: $crate::referral::InfluencerId, contract_id: $crate::referral::ContractId) -> Vec<$crate::referral::ProgramId> {
            self.$tokens.referral_programs_by_influencer(influencer_id, contract_id)
          }

          // get influencers by contract
          fn referral_influencers_by_contract(&self, contract_id: $crate::referral::ContractId) -> Vec<$crate::referral::InfluencerId> {
            self.$tokens.referral_influencers_by_contract(contract_id)
          }
          // get programs by contract
          fn referral_programs_by_contract(&self, contract_id: $crate::referral::ContractId, influencer_id: $crate::referral::InfluencerId) -> Vec<$crate::referral::ProgramId> {
            self.$tokens.referral_programs_by_contract(contract_id, influencer_id)
          }
        }
    };
}
