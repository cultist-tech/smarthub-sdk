use near_sdk::{ AccountId, IntoStorageKey, env };
use near_sdk::json_types::{ U128 };
use crate::referral::{ ProgramId, ReferralCore, ReferralInfo };
use crate::referral::metadata::{
    InfluencerId,
    AccountContractId,
    InfluencerProgramId,
    ContractId,
    InfluencerRoyalty,
};
use near_sdk::collections::{ LookupMap, TreeMap, UnorderedSet };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use crate::storage::StorageFeature;
use crate::referral::utils::{ contract_account_id, influencer_program_id };

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ReferralFeature {
    pub influencer_by_id: LookupMap<AccountContractId, InfluencerId>,
    pub referrals_by_contract: TreeMap<ContractId, UnorderedSet<AccountId>>,
    pub referrals_by_influencer: TreeMap<InfluencerId, UnorderedSet<AccountId>>,
    pub referrals_by_program: TreeMap<InfluencerProgramId, UnorderedSet<AccountId>>,
    pub royalty_by_program: LookupMap<InfluencerProgramId, InfluencerRoyalty>,

    pub programs_by_contract: TreeMap<ContractId, TreeMap<InfluencerId, UnorderedSet<ProgramId>>>,
    pub programs_by_influencer: TreeMap<InfluencerId, TreeMap<ContractId, UnorderedSet<ProgramId>>>,

    pub code_by_program: LookupMap<InfluencerProgramId, String>,
    pub info_by_code: LookupMap<String, ReferralInfo>,
}

impl ReferralFeature {
    pub fn new<T1, T2, T3, T4, T5, T6, T7, T8, T9>(
        prefix1: T1,
        prefix2: T2,
        prefix3: T3,
        prefix4: T4,
        prefix5: T5,
        prefix6: T6,
        prefix7: T7,
        prefix8: T8,
        prefix9: T9
    )
        -> Self
        where
            T1: IntoStorageKey,
            T2: IntoStorageKey,
            T3: IntoStorageKey,
            T4: IntoStorageKey,
            T5: IntoStorageKey,
            T6: IntoStorageKey,
            T7: IntoStorageKey,
            T8: IntoStorageKey,
            T9: IntoStorageKey
    {
        let mut this = Self {
            influencer_by_id: LookupMap::new(prefix1),
            referrals_by_contract: TreeMap::new(prefix2),
            referrals_by_influencer: TreeMap::new(prefix3),
            referrals_by_program: TreeMap::new(prefix4),
            royalty_by_program: LookupMap::new(prefix5),
            programs_by_contract: TreeMap::new(prefix6),
            programs_by_influencer: TreeMap::new(prefix7),
            code_by_program: LookupMap::new(prefix8),
            info_by_code: LookupMap::new(prefix9),
        };

        this
    }

    pub fn measure_min_storage_cost(&mut self) -> u128 {
        let tmp_program = "a".repeat(64);
        let tmp_account = AccountId::new_unchecked("a".repeat(64));
        let tmp_account2 = AccountId::new_unchecked("b".repeat(64));

        self.internal_add_referral(&tmp_account, &tmp_account, &tmp_program, &tmp_account2);

        let prev = env::storage_usage();

        self.internal_add_referral(&tmp_account, &tmp_account, &tmp_program, &tmp_account);

        let next = env::storage_usage();
        let diff = next - prev;

        self.internal_remove_referral(&tmp_account, &tmp_account, &tmp_program, &tmp_account);
        self.internal_remove_referral(&tmp_account, &tmp_account, &tmp_program, &tmp_account2);

        diff as u128
    }
}

impl ReferralCore for ReferralFeature {
    fn referral_by(&self, contract_id: AccountId, account_id: AccountId) -> Option<AccountId> {
        self.internal_referral_by(&contract_id, &account_id)
    }

    fn referral_program_code(
        &self,
        contract_id: ContractId,
        influencer_id: InfluencerId,
        program_id: ProgramId
    ) -> Option<String> {
        let id = influencer_program_id(&contract_id, &influencer_id, &program_id);

        self.code_by_program.get(&id)
    }

    // payable
    fn referral_create_program(
        &mut self,
        influencer_id: AccountId,
        program_id: ProgramId,
        royalty_percent: InfluencerRoyalty
    ) {
        let contract_id = env::predecessor_account_id();

        self.internal_create_program(&contract_id, &influencer_id, &program_id, &royalty_percent)
    }

    fn referral_accept(
        &mut self,
        contract_id: AccountId,
        influencer_id: AccountId,
        program_id: ProgramId
    ) {
        self.internal_accept_referral(
            &contract_id,
            &influencer_id,
            &program_id,
            &env::predecessor_account_id()
        )
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

    fn referral_program_royalty(
        &self,
        contract_id: AccountId,
        influencer_id: InfluencerId,
        program_id: ProgramId
    ) -> Option<u64> {
        let id = influencer_program_id(&contract_id, &influencer_id, &program_id);

        self.royalty_by_program.get(&id)
    }
}
