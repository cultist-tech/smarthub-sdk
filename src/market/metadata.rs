use near_sdk::AccountId;
use std::collections::HashMap;
use near_sdk::json_types::{ U128 };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::serde_json::to_string;
use schemars::JsonSchema;
use crate::metadata::FungibleTokenId;

pub type SaleConditions = HashMap<FungibleTokenId, U128>;
pub type Bids = HashMap<FungibleTokenId, Vec<Bid>>;
pub type TokenId = String;

pub type ContractAndTokenId = String;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    pub owner_id: AccountId,
    pub price: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct Sale {
    pub owner_id: AccountId,
    pub approval_id: u64,
    pub nft_contract_id: AccountId,
    pub token_id: String,
    pub sale_conditions: SaleConditions,
    pub bids: Bids,
    pub created_at: u64,
    pub is_auction: bool,
}

impl ToString for Sale {
    fn to_string(&self) -> String {
        to_string(
            &(Sale {
                owner_id: self.owner_id.clone(),
                approval_id: self.approval_id.clone(),
                nft_contract_id: self.nft_contract_id.clone(),
                token_id: self.token_id.clone(),
                sale_conditions: self.sale_conditions.clone(),
                bids: self.bids.clone(),
                created_at: self.created_at.clone(),
                is_auction: self.is_auction.clone(),
            })
        )
            .ok()
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct MarketOnFtTransferArgs {
    pub nft_contract_id: AccountId,
    pub token_id: TokenId,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct MarketOnNftApproveArgs {
    pub sale_conditions: SaleConditions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auction: Option<bool>,
}
