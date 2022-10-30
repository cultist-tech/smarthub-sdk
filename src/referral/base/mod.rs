mod base_impl;
mod receiver;
mod internal;
mod resolver;
mod macros;

pub use self::base_impl::{ ReferralFeature };
pub use self::receiver::{ ReferralReceiver };
pub use self::resolver::{ ReferralResolver };
use near_sdk::AccountId;
use crate::referral::{ProgramId, ContractId};
use crate::referral::metadata::{ InfluencerId, InfluencerRoyalty };

pub trait ReferralCore {
    // get influencer address by account
    fn referral_by(&self, contract_id: AccountId, account_id: AccountId) -> Option<AccountId>;

    fn referral_program_code(&self, contract_id: ContractId, influencer_id: InfluencerId, program_id: ProgramId) -> Option<String>;

    // create program for contract (by influencer)
    fn referral_create_program(
        &mut self,
        influencer_id: AccountId,
        program_id: ProgramId,
        royalty_percent: u64
    );

    fn referral_accept(
        &mut self,
        contract_id: AccountId,
        influencer_id: AccountId,
        program_id: ProgramId
    );

    fn referral_accept_code(
        &mut self,
        code: String,
    );

    fn referral_program_royalty(
        &self,
        contract_id: AccountId,
        influencer_id: InfluencerId,
        program_id: ProgramId
    ) -> Option<InfluencerRoyalty>;
}
