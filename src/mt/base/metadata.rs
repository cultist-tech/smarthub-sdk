use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::json_types::Base64VecU8;
use near_sdk::{ require, AccountId, Balance };
use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::collections::LookupMap;

pub const MT_METADATA_SPEC: &str = "mft-1.0.0";

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MtToken {
    /// AccountID -> Account balance.
    pub accounts: LookupMap<AccountId, Balance>,

    /// Total supply of the all token.
    pub total_supply: Balance,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MultiFungibleTokenMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
    pub decimals: u8,
}

pub trait MultiFungibleTokenMetadataProvider {
    fn mt_metadata(&self, token_id: String) -> MultiFungibleTokenMetadata;
}

impl MultiFungibleTokenMetadata {
    pub fn assert_valid(&self) {
        require!(self.spec == MT_METADATA_SPEC);
        require!(self.reference.is_some() == self.reference_hash.is_some());

        if let Some(reference_hash) = &self.reference_hash {
            require!(reference_hash.0.len() == 32, "Hash has to be 32 bytes");
        }
    }
}