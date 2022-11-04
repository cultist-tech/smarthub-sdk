use near_sdk::{AccountId, env};
use near_sdk::json_types::{ U128 };
use crate::referral::{ReferralFeature, ReferralCode, InfluencerId, ContractId, ProgramId, ReferralInfo};
use crate::referral::utils::get_program_id;

impl ReferralCode for ReferralFeature {
    fn referral_program_code(
        &self,
        contract_id: ContractId,
        influencer_id: InfluencerId,
        program_id: ProgramId
    ) -> Option<String> {
        let id = get_program_id(&contract_id, &influencer_id, &program_id);

        self.code_by_program.get(&id)
    }

    fn referral_code_info(&self, code: String) -> Option<ReferralInfo> {
        self.info_by_code.get(&code)
    }

    fn referral_accept_code(
        &mut self,
        code: String
    ) {
        let info: ReferralInfo = self.info_by_code.get(&code).expect("Not found referral program");

        self.internal_accept_referral(
            &info.contract_id,
            &info.influencer_id,
            &info.program_id,
            &env::predecessor_account_id()
        )
    }
}
