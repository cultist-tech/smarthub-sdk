use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::AccountId;
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::json_types::U128;
use schemars::JsonSchema;
use std::collections::HashMap;

pub type TokenId = String;
pub type SaleConditions = HashMap<AccountId, U128>;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Rent {
    pub token_id: TokenId,
    pub contract_id: AccountId,
    pub owner_id: AccountId,
    pub sale_conditions: SaleConditions,
    pub min_time: u64,
    pub max_time: u64,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonRent {
    pub token_id: TokenId,
    pub contract_id: AccountId,
    pub owner_id: AccountId,
    pub sale_conditions: SaleConditions,
    pub min_time: u64,
    pub max_time: u64,
    pub created_at: u64,
    pub ended_at: Option<u64>,
    pub renter_id: Option<AccountId>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct RentOnNftApproveArgs {
    pub sale_conditions: SaleConditions,
    pub min_time: u64,
    pub max_time: u64,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct RentOnFtTransferArgs {
    pub token_id: TokenId,
    pub contract_id: AccountId,
    pub receiver_id: AccountId,
    pub time: u64,
}
