use near_sdk::AccountId;
use near_sdk::json_types::{ U128 };
use crate::referral::{ ReferralFeature, ReferralEnumeration };

impl ReferralEnumeration for ReferralFeature {
    fn referrals_by_contract(&self, contract_id: AccountId) -> Vec<AccountId> {
        unimplemented!()
    }

    fn referrals_supply_by_contract(&self, contract_id: AccountId) -> U128 {
        unimplemented!()
    }

    fn referrals_by_program(
        &self,
        contract_id: AccountId,
        influencer_id: AccountId,
        program_id: String
    ) -> Vec<AccountId> {
        unimplemented!()
    }

    fn referrals_supply_by_program(
        &self,
        contract_id: AccountId,
        influencer_id: AccountId,
        program_id: String
    ) -> U128 {
        unimplemented!()
    }

    fn referrals_by_influencer(
        &self,
        contract_id: AccountId,
        program_id: String
    ) -> Vec<AccountId> {
        unimplemented!()
    }

    fn referrals_supply_by_influencer(&self, contract_id: AccountId, program_id: String) -> U128 {
        unimplemented!()
    }
}
