use near_sdk::{ AccountId, env, require };
use crate::referral::metadata::{ InfluencerId, InfluencerProgramId, ContractId, AccountContractId };
use crate::referral::ProgramId;

pub(crate) static DELIMETER: &str = "||";
pub(crate) const PRICE_PER_PROGRAM: u128 = 100000000000000000000000;
pub(crate) const PRICE_PER_REFERRAL: u128 = 20000000000000000000000;

pub(crate) fn get_program_id(
    contract_id: &ContractId,
    influencer_id: &InfluencerId,
    program_id: &ProgramId
) -> InfluencerProgramId {
    format!("{}{}{}{}{}", contract_id, DELIMETER, influencer_id, DELIMETER, program_id)
}

pub(crate) fn contract_account_id(
    contract_id: &ContractId,
    account_id: &AccountId
) -> AccountContractId {
    format!("{}{}{}", contract_id, DELIMETER, account_id)
}

pub fn assert_referral_program_money() {
    require!(env::attached_deposit() >= PRICE_PER_PROGRAM, "Requires attached deposit of 0.1 NEAR")
}

pub fn assert_referral_money() {
    require!(env::attached_deposit() >= PRICE_PER_REFERRAL, "Requires attached deposit of 0.02 NEAR")
}
