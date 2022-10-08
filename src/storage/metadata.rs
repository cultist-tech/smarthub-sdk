use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::json_types::U128;
use near_sdk::serde::{ Deserialize, Serialize };
use schemars::JsonSchema;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageBalance {
    pub total: U128,
    pub available: U128,
}
