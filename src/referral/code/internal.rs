use crate::referral::{ReferralFeature, ContractId, InfluencerId, ProgramId, ReferralInfo};
use near_sdk::{AccountId, env};
use crate::referral::utils::{assert_referral_money, contract_account_id};
use crate::storage::Storage;

impl ReferralFeature {
    pub(crate) fn internal_accept_referral(
        &mut self,
        contract_id: &ContractId,
        influencer_id: &InfluencerId,
        program_id: &ProgramId,
        account_id: &AccountId
    ) {
        assert_referral_money();

        let attached_deposit = env::attached_deposit();
        let contract_account = contract_account_id(&contract_id, &account_id);
        assert!(self.influencer_by_id.get(&contract_account).is_none(), "Referral already exists");

        let mut storage = Storage::start();

        self.internal_add_referral(&contract_id, &influencer_id, &program_id, &account_id);

        storage.refund(&attached_deposit);

        self.internal_call_on_referral(&contract_id, &influencer_id, &program_id, &account_id);
    }

    pub(crate) fn internal_get_info_by_code(&self, code: String) -> ReferralInfo {
        self.info_by_code.get(&code).expect("Not found info")
    }

}
