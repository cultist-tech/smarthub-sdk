use near_sdk::AccountId;
use near_sdk::json_types::{ U128 };
use crate::referral::{ ProgramId, ReferralCore };

pub struct ReferralFeature {}

impl ReferralCore for ReferralFeature {
    fn referral_by(&self, contract_id: AccountId, account_id: AccountId) -> Option<AccountId> {
        unimplemented!()
    }

    // payable
    fn referral_create_program(
        &mut self,
        contract_id: AccountId,
        influencer_id: AccountId,
        program_id: ProgramId,
        royalty_percent: u64
    ) {
        unimplemented!()
    }

    fn referral_accept(
        &mut self,
        contract_id: AccountId,
        influencer_id: AccountId,
        program_id: ProgramId
    ) {
        unimplemented!()
    }
}
