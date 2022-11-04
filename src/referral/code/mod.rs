mod code_impl;
mod macros;
mod internal;

use near_sdk::AccountId;
use crate::referral::{ProgramId, ContractId, ReferralInfo, ReferralProgramMetadata, ReferralProgram};
use crate::referral::metadata::{ InfluencerId, InfluencerRoyalty };

pub trait ReferralCode {
    fn referral_program_code(&self, contract_id: ContractId, influencer_id: InfluencerId, program_id: ProgramId) -> Option<String>;

    fn referral_code_info(&self, code: String) -> Option<ReferralInfo>;

    fn referral_accept_code(
        &mut self,
        code: String,
    );
}
