use near_sdk::{ AccountId, PromiseOrValue, ext_contract };
use crate::referral::ReferralFeature;

#[ext_contract(ext_self)]
pub trait ReferralResolver {
    // cross contract call
    fn resolve_on_referral_create(
        &mut self,
        contract_id: AccountId,
        account_id: AccountId,
        influencer_id: AccountId,
        program_id: String
    ) -> bool;
}

impl ReferralResolver for ReferralFeature {
    fn resolve_on_referral_create(
        &mut self,
        contract_id: AccountId,
        account_id: AccountId,
        influencer_id: AccountId,
        program_id: String
    ) -> bool {
        unimplemented!()
    }
}
