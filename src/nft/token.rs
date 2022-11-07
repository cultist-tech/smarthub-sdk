use crate::nft::metadata::{ TokenMetadata };
use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::AccountId;
use std::collections::HashMap;
use crate::nft::royalty::Royalty;
use schemars::JsonSchema;

/// Note that token IDs for NFTs are strings on NEAR. It's still fine to use autoincrementing numbers as unique IDs if desired, but they should be stringified. This is to make IDs more future-proof as chain-agnostic conventions and standards arise, and allows for more flexibility with considerations like bridging NFTs across chains, etc.
pub type TokenId = String;

pub type TokenTypes = HashMap<String, String>;

pub const TOKEN_COLLECTION: &str = "token_collection";
pub const TOKEN_TYPE: &str = "token_type";
pub const TOKEN_SUB_TYPE: &str = "token_sub_type";

pub type TokenRarity = u8;

pub const RARITY_MAX: u8 = 6;

/// In this implementation, the Token struct takes two extensions standards (metadata and approval) as optional fields, as they are frequently used in modern NFTs.
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
pub struct Token {
    // core
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: Option<TokenMetadata>,
    pub approved_account_ids: Option<HashMap<AccountId, u64>>,

    // royalty extension
    pub royalty: Option<Royalty>,

    // bind to owner extension
    pub bind_to_owner: Option<bool>,

    pub reveal_at: Option<u64>,

    // extra fields
    pub rarity: Option<TokenRarity>,
    pub types: Option<TokenTypes>,
}
