use near_sdk::AccountId;
use near_sdk::json_types::{ U128 };
use crate::referral::{ ReferralFeature, ReferralEnumeration, InfluencerId, ContractId, ProgramId };
use crate::referral::utils::influencer_program_id;

impl ReferralEnumeration for ReferralFeature {
    fn referrals_by_contract(&self, contract_id: AccountId) -> Vec<AccountId> {
        if let Some(referrals) = self.referrals_by_contract.get(&contract_id) {
            referrals.to_vec()
        } else {
            vec![]
        }
    }

    fn referrals_supply_by_contract(&self, contract_id: AccountId) -> u128 {
        if let Some(referrals) = self.referrals_by_contract.get(&contract_id) {
            referrals.len() as u128
        } else {
            0
        }
    }

    fn referrals_by_program(
        &self,
        contract_id: AccountId,
        influencer_id: AccountId,
        program_id: String
    ) -> Vec<AccountId> {
        let id = influencer_program_id(&contract_id, &influencer_id, &program_id);

        if let Some(referrals) = self.referrals_by_program.get(&id) {
            referrals.to_vec()
        } else {
            vec![]
        }
    }

    fn referrals_supply_by_program(
        &self,
        contract_id: AccountId,
        influencer_id: AccountId,
        program_id: String
    ) -> u128 {
        let id = influencer_program_id(&contract_id, &influencer_id, &program_id);

        if let Some(referrals) = self.referrals_by_program.get(&id) {
            referrals.len() as u128
        } else {
            0
        }
    }

    fn referrals_by_influencer(&self, influencer_id: InfluencerId) -> Vec<AccountId> {
        if let Some(referrals) = self.referrals_by_contract.get(&influencer_id) {
            referrals.to_vec()
        } else {
            vec![]
        }
    }

    fn referrals_supply_by_influencer(&self, influencer_id: InfluencerId) -> u128 {
        if let Some(referrals) = self.referrals_by_contract.get(&influencer_id) {
            referrals.len() as u128
        } else {
            0
        }
    }

    fn referral_contracts_by_influencer(&self, influencer_id: InfluencerId) -> Vec<ContractId> {
        if let Some(contracts) = self.programs_by_influencer.get(&influencer_id) {
            contracts
                .iter()
                .map(|(key, value)| { key })
                .collect()
        } else {
            vec![]
        }
    }

    fn referral_programs_by_influencer(
        &self,
        influencer_id: InfluencerId,
        contract_id: ContractId
    ) -> Vec<ProgramId> {
        if let Some(contracts) = self.programs_by_influencer.get(&influencer_id) {
            if let Some(programs) = contracts.get(&contract_id) {
                programs.to_vec()
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    fn referral_influencers_by_contract(&self, contract_id: ContractId) -> Vec<InfluencerId> {
        if let Some(influencers) = self.programs_by_contract.get(&contract_id) {
            influencers
                .iter()
                .map(|(key, value)| { key })
                .collect()
        } else {
            vec![]
        }
    }

    fn referral_programs_by_contract(
        &self,
        contract_id: InfluencerId,
        influencer_id: InfluencerId
    ) -> Vec<ProgramId> {
      let influencers = self.programs_by_contract.get(&contract_id).expect("Not found");
        if let Some(programs) = influencers.get(&influencer_id) {
          programs.to_vec()
        } else {
          vec![]
        }
    }
}
