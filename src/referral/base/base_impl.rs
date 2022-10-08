use near_sdk::AccountId;
use near_sdk::json_types::{ U128 };
use crate::referral::{ ProgramId, ReferralCore };
use crate::referral::metadata::{
    InfluencerId,
    AccountContractId,
    InfluencerProgramId,
    ContractId,
    InfluencerRoyalty,
};
use near_sdk::collections::{ LookupMap, TreeMap, UnorderedSet };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ReferralFeature {
    pub influencer_by_id: LookupMap<AccountContractId, InfluencerId>,
    pub referrals_by_contract: TreeMap<ContractId, UnorderedSet<AccountId>>,
    pub referrals_by_influencer: TreeMap<InfluencerId, UnorderedSet<AccountId>>,
    pub referrals_by_program: TreeMap<InfluencerProgramId, UnorderedSet<AccountId>>,
    pub royalty_by_program: LookupMap<InfluencerProgramId, InfluencerRoyalty>,
}

impl ReferralFeature {}

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

    fn referral_program_royalty(
        &self,
        contract_id: AccountId,
        influencer_id: InfluencerId,
        program_id: ProgramId
    ) -> Option<u64> {
        unimplemented!()
    }
}
