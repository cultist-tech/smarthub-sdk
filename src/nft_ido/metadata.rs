use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::AccountId;
use near_sdk::json_types::U128;
use schemars::JsonSchema;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Ido {
    pub ido_id: IdoId,
    pub contract_id: AccountId,

    pub name: String,
    pub media: Option<String>,
    pub amount: u64,
    pub price: U128,
    pub buy_max: u64,
    pub per_transaction_min: u64,
    pub per_transaction_max: u64,
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
pub struct JsonIdo {
    pub ido_id: IdoId,
    pub contract_id: AccountId,

    pub name: String,
    pub media: Option<String>,
    pub amount: u64,
    pub amount_ready: u64,
    pub price: U128,
    pub buy_max: u64,
    pub per_transaction_min: u64,
    pub per_transaction_max: u64,
    pub not_minted: u64,
    pub locked: bool,
    pub start_date: Option<u64>,
    pub ft_token: Option<AccountId>,
}

pub type IdoId = String;
pub type ContractIdoId = String;
pub type TokenId = String;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct NftIdoOnFtTransferArgs {
    // mint
    pub contract_id: AccountId,
    pub ido_id: IdoId,
    pub receiver_id: AccountId,
    pub mint_amount: u64,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct NftIdoOnNftTransferArgs {
    pub ido_id: IdoId,
}
