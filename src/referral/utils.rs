use near_sdk::AccountId;
use crate::referral::metadata::{ InfluencerId, InfluencerProgramId, ContractId, AccountContractId };
use crate::referral::ProgramId;

pub(crate) static DELIMETER: &str = "||";

pub(crate) fn influencer_program_id(
    contract_id: ContractId,
    influencer_id: &InfluencerId,
    program_id: &ProgramId
) -> InfluencerProgramId {
    format!("{}{}{}{}{}", contract_id, DELIMETER, influencer_id, DELIMETER, program_id)
}

pub(crate) fn contract_account_id(
    contract_id: ContractId,
    account_id: &AccountId
) -> AccountContractId {
    format!("{}{}{}", contract_id, DELIMETER, account_id)
}
