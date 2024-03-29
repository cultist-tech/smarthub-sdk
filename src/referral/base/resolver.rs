use near_sdk::{ AccountId, PromiseOrValue, ext_contract, is_promise_success, env };
use crate::referral::ReferralFeature;
use crate::storage::Storage;
use crate::utils::refund_deposit_to_account;

#[ext_contract(ext_self)]
pub trait ReferralResolver {
    // cross contract call
    fn resolve_on_referral_create(
        &mut self,
        contract_id: AccountId,
        influencer_id: AccountId,
        program_id: String,
        account_id: AccountId
    ) -> bool;
}

impl ReferralResolver for ReferralFeature {
    fn resolve_on_referral_create(
        &mut self,
        contract_id: AccountId,
        influencer_id: AccountId,
        program_id: String,
        account_id: AccountId
    ) -> bool {
        let is_success = is_promise_success();

        is_success
    }
}
