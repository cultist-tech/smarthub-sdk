use crate::referral::base::base_impl::ReferralFeature;
use near_sdk::borsh::{ self, BorshSerialize };
use near_sdk::{ BorshStorageKey, CryptoHash, env, AccountId };
use crate::referral::{ContractId, InfluencerId, ProgramId, InfluencerRoyalty, ReferralInfo, ReferralProgramMetadata};
use near_sdk::collections::{ UnorderedSet, TreeMap };
use crate::referral::utils::{
    get_program_id,
    contract_account_id,
    assert_referral_program_money,
    assert_referral_money,
};
use crate::utils::refund_deposit_to_account;
use crate::storage::Storage;
use crate::referral::events::{ReferralAccept, ProgramCreate};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    ContractReferralsInner {
        contract_hash: Vec<u8>,
    },
    InfluencerReferralsInner {
        influencer_hash: Vec<u8>,
    },
    ProgramReferralsInner {
        program_hash: Vec<u8>,
    },
    ContractInfluencerInner {
        contract_hash: Vec<u8>,
    },
    ContractInfluencerProgramInner {
        influencer_hash: Vec<u8>,
        contract_hash: Vec<u8>,
    },
    InfluencerContractInner {
        influencer_hash: Vec<u8>,
    },
    InfluencerContractProgramInner {
        contract_hash: Vec<u8>,
        influencer_hash: Vec<u8>,
    },
}

impl ReferralFeature {
    pub(crate) fn internal_referral_by(
        &self,
        contract_id: &ContractId,
        account_id: &AccountId
    ) -> Option<InfluencerId> {
        self.influencer_by_id.get(&contract_account_id(&contract_id, &account_id))
    }

    pub(crate) fn internal_create_program(
        &mut self,
        contract_id: &ContractId,
        influencer_id: &InfluencerId,
        program_id: &ProgramId,
        royalty_percent: &Option<u64>,
        metadata: &Option<ReferralProgramMetadata>
    ) {
        let attached_deposit = env::attached_deposit();
        let mut storage = Storage::start();

        assert_referral_program_money();

        let id = get_program_id(contract_id, influencer_id, program_id);
        let code = self.internal_get_random_code();

        assert!(self.referrals_by_program.get(&id).is_none(), "Program already exists");
        assert!(self.info_by_code.get(&code).is_none(), "Code already used, please try again");

        // enumeration
        self.internal_add_program_to_contract(&contract_id, &influencer_id, &program_id);
        self.internal_add_program_to_influencer(&contract_id, &influencer_id, &program_id);

        // enumeration
        self.referrals_by_contract.insert(
            &contract_id,
            &UnorderedSet::new(StorageKey::ContractReferralsInner {
                contract_hash: env::sha256(contract_id.as_bytes()),
            })
        );
        self.referrals_by_influencer.insert(
            &influencer_id,
            &UnorderedSet::new(StorageKey::InfluencerReferralsInner {
                influencer_hash: env::sha256(influencer_id.as_bytes()),
            })
        );
        self.referrals_by_program.insert(
            &id,
            &UnorderedSet::new(StorageKey::ProgramReferralsInner {
                program_hash: env::sha256(id.as_bytes()),
            })
        );

        // royalty
        if let Some(royalty_percent) = royalty_percent {
            self.royalty_by_program.insert(&id, royalty_percent);
        }
        if let Some(metadata) = metadata {
            self.metadata_by_program.insert(&id, &metadata);
        }

        // code
        self.code_by_program.insert(&id, &code);
        self.info_by_code.insert(
            &code,
            &(ReferralInfo {
                contract_id: contract_id.clone(),
                influencer_id: influencer_id.clone(),
                program_id: program_id.clone(),
            })
        );

        ProgramCreate {
            contract_id: &contract_id,
            influencer_id: &influencer_id,
            program_id: &program_id,
            royalty_percent: &royalty_percent,
            code: &code,
            metadata: &metadata,
        }.emit();

        storage.refund(&attached_deposit);
    }

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

    pub(crate) fn internal_add_referral(
        &mut self,
        contract_id: &ContractId,
        influencer_id: &InfluencerId,
        program_id: &ProgramId,
        account_id: &AccountId
    ) {
        let contract_account = contract_account_id(&contract_id, &account_id);

        self.influencer_by_id.insert(&contract_account, &account_id);
        self.internal_add_referral_to_contract(&contract_id, &account_id);
        self.internal_add_referral_to_influencer(&influencer_id, &account_id);
        self.internal_add_referral_to_program(
            &contract_id,
            &influencer_id,
            &program_id,
            &account_id
        );


        ReferralAccept {
            contract_id: &contract_id,
            influencer_id: &influencer_id,
            program_id: &program_id,
            account_id: &account_id,
        }.emit();
    }

    pub(crate) fn internal_add_referral_to_contract(
        &mut self,
        contract_id: &ContractId,
        account_id: &AccountId
    ) {
        let mut list = self.referrals_by_contract.get(&contract_id).unwrap_or_else(||
            UnorderedSet::new(StorageKey::ContractReferralsInner {
                contract_hash: env::sha256(contract_id.as_bytes()),
            })
        );

        list.insert(&account_id);

        self.referrals_by_contract.insert(&contract_id, &list);
    }

    pub(crate) fn internal_add_referral_to_influencer(
        &mut self,
        influencer_id: &InfluencerId,
        account_id: &AccountId
    ) {
        let mut list = self.referrals_by_influencer.get(&influencer_id).unwrap_or_else(||
            UnorderedSet::new(StorageKey::InfluencerReferralsInner {
                influencer_hash: env::sha256(influencer_id.as_bytes()),
            })
        );

        list.insert(&account_id);

        self.referrals_by_influencer.insert(&influencer_id, &list);
    }

    pub(crate) fn internal_add_referral_to_program(
        &mut self,
        contract_id: &ContractId,
        influencer_id: &InfluencerId,
        program_id: &ProgramId,
        account_id: &AccountId
    ) {
        let id = get_program_id(contract_id, influencer_id, program_id);

        let mut list = self.referrals_by_program.get(&id).unwrap_or_else(||
            UnorderedSet::new(StorageKey::ProgramReferralsInner {
                program_hash: env::sha256(id.as_bytes()),
            })
        );

        list.insert(&account_id);

        self.referrals_by_program.insert(&id, &list);
    }

    pub(crate) fn internal_remove_referral(
        &mut self,
        contract_id: &ContractId,
        influencer_id: &InfluencerId,
        program_id: &ProgramId,
        account_id: &AccountId
    ) {
        let contract_account = contract_account_id(&contract_id, &account_id);
        let id = get_program_id(contract_id, influencer_id, program_id);

        self.influencer_by_id.remove(&contract_account);

        if let Some(mut referrals_by_contract) = self.referrals_by_contract.get(&contract_id) {
            referrals_by_contract.remove(&account_id);

            if referrals_by_contract.len() == 0 {
                self.referrals_by_contract.remove(&contract_id);
            } else {
                self.referrals_by_contract.insert(&contract_id, &referrals_by_contract);
            }
        }
        if let Some(mut referrals_by_influencer) = self.referrals_by_influencer.get(&influencer_id) {
            referrals_by_influencer.remove(&account_id);

            if referrals_by_influencer.len() == 0 {
                self.referrals_by_influencer.remove(&influencer_id);
            } else {
                self.referrals_by_influencer.insert(&influencer_id, &referrals_by_influencer);
            }
        }
        if let Some(mut referrals_by_program) = self.referrals_by_program.get(&id) {
            referrals_by_program.remove(&account_id);

            if referrals_by_program.len() == 0 {
                self.referrals_by_program.remove(&id);
            } else {
                self.referrals_by_program.insert(&id, &referrals_by_program);
            }
        }
    }

    pub(crate) fn internal_add_program_to_contract(
        &mut self,
        contract_id: &ContractId,
        influencer_id: &InfluencerId,
        program_id: &ProgramId
    ) {
        let mut influencers = self.programs_by_contract.get(&contract_id).unwrap_or_else(||
            TreeMap::new(StorageKey::ContractInfluencerInner {
                contract_hash: env::sha256(contract_id.as_bytes()),
            })
        );
        let mut programs = influencers.get(&influencer_id).unwrap_or_else(||
            UnorderedSet::new(StorageKey::ContractInfluencerProgramInner {
                influencer_hash: env::sha256(influencer_id.as_bytes()),
                contract_hash: env::sha256(contract_id.as_bytes()),
            })
        );

        programs.insert(&program_id);
        influencers.insert(&influencer_id, &programs);

        self.programs_by_contract.insert(&contract_id, &influencers);
    }

    pub(crate) fn internal_add_program_to_influencer(
        &mut self,
        contract_id: &ContractId,
        influencer_id: &InfluencerId,
        program_id: &ProgramId
    ) {
        let mut contracts = self.programs_by_influencer.get(&influencer_id).unwrap_or_else(||
            TreeMap::new(StorageKey::InfluencerContractInner {
                influencer_hash: env::sha256(influencer_id.as_bytes()),
            })
        );
        let mut programs = contracts.get(&contract_id).unwrap_or_else(||
            UnorderedSet::new(StorageKey::InfluencerContractProgramInner {
                contract_hash: env::sha256(contract_id.as_bytes()),
                influencer_hash: env::sha256(influencer_id.as_bytes()),
            })
        );

        programs.insert(&program_id);
        contracts.insert(&contract_id, &programs);

        self.programs_by_influencer.insert(&influencer_id, &contracts);
    }

    pub(crate) fn internal_get_info_by_code(&self, code: String) -> ReferralInfo {
        self.info_by_code.get(&code).expect("Not found info")
    }

    pub(crate) fn internal_get_random_code(&self) -> String {
        env::block_timestamp().to_string()
    }
}
