use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::AccountId;
use schemars::JsonSchema;

pub type TokenId = String;
pub type FractionationId = String;
pub type ContractFractionationId = String;
pub type ContractId = AccountId;

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
pub struct Fractionation {
    pub token_id: FractionationId,
    pub contract_id: AccountId,
    pub entries: Vec<TokenId>,    
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct FractionationNftOnTransferArgs {
    pub fractionation_tokens: Option<Vec<TokenId>>,
}
