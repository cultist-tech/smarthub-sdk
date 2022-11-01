use near_sdk::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

pub type ProgramId = String;
pub type InfluencerId = AccountId;
pub type ContractId = AccountId;
pub type AccountContractId = String;
pub type InfluencerProgramId = String;
pub type ContractProgramId = String;
pub type InfluencerRoyalty = u64;

#[derive(
    BorshDeserialize,
    BorshSerialize,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    JsonSchema
)]
#[serde(crate = "near_sdk::serde")]
pub struct ReferralInfo {
    pub contract_id: ContractId,
    pub influencer_id: InfluencerId,
    pub program_id: ProgramId,
}

#[derive(
    BorshDeserialize,
    BorshSerialize,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    JsonSchema
)]
#[serde(crate = "near_sdk::serde")]
pub struct ReferralProgramMetadata {
    pub title: Option<String>,
    pub media: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(
BorshSerialize,
Serialize,
Clone,
Debug,
PartialEq,
JsonSchema
)]
#[serde(crate = "near_sdk::serde")]
pub struct ReferralProgram {
    pub contract_id: ContractId,
    pub influencer_id: InfluencerId,
    pub program_id: ProgramId,
    pub metadata: Option<ReferralProgramMetadata>,
    pub code: String,
}
