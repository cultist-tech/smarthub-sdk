use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::json_types::{ U128 };
use near_sdk::{ AccountId };
use crate::tournament::WinnerPlace;
use schemars::JsonSchema;

pub type TournamentId = String;
pub type TournamentPrizeId = String;
pub type TournamentPlaceId = String;
pub type TokenId = String;
pub type PrizeId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentFactoryMetadata {
    pub name: String,
    pub icon: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentMetadata {
    pub name: String,
    pub media: Option<String>,
    pub summary: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Tournament {
    pub tournament_id: TournamentId,
    pub owner_id: AccountId,
    pub access_nft_contract: Option<AccountId>,

    pub players_number: u8,
    pub price: Option<U128>,

    pub started_at: Option<u64>,
    pub ended_at: Option<u64>,
    pub created_at: u64,
}

//The Json tournament is what will be returned from view calls.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonTournament {
    //tournament ID
    pub tournament_id: TournamentId,
    //owner of the tournament
    pub owner_id: AccountId,
    //tournament metadata
    pub metadata: TournamentMetadata,

    pub access_nft_contract: Option<AccountId>,
    pub price: Option<U128>,

    pub players_total: u8,
    pub players_current: u8,

    pub started_at: Option<u64>,
    pub ended_at: Option<u64>,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentOnFtTransferArgs {
    // upgradable
    pub tournament_id: TournamentId,
    pub owner_id: AccountId,
    pub place: Option<u8>,
    pub prize_id: Option<PrizeId>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentOnNftTransferArgs {
    pub tournament_id: TournamentId,
    pub owner_id: AccountId,
    pub place: Option<WinnerPlace>,
    pub prize_id: Option<PrizeId>,
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
pub enum RewardPrize {
    Near {
        amount: U128,
        owner_id: Option<AccountId>,
    },
    Ft {
        ft_contract_id: AccountId,
        amount: U128,
        owner_id: Option<AccountId>,
    },
    Nft {
        nft_contract_id: AccountId,
        token_id: TokenId,
        owner_id: Option<AccountId>,
    },
}
