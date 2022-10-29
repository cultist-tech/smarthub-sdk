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

pub type TokenRarity = u8;


#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    BorshDeserialize,
    BorshSerialize,
    JsonSchema
)]
#[serde(crate = "near_sdk::serde")]
pub enum TokenCollection {
    Fantasy,
    Medieval,
    Nordic,
    PostApoc,
    SteamPunk,
    Asian,
    CyberPunk,
    Unknown,
}

impl std::fmt::Display for TokenCollection {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    BorshDeserialize,
    BorshSerialize,
    JsonSchema
)]
#[serde(crate = "near_sdk::serde")]
pub enum TokenType {
    Sketch,
    Badge,
    //
    Skin,
    Avatar,
    Pet,
    Race,
    Class,
    //
    Weapon,
    Armor,
    Jewelry,
    Shield,
    //
    Access,
    Present,
}

impl std::fmt::Display for TokenType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}


#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    BorshDeserialize,
    BorshSerialize,
    JsonSchema
)]
#[serde(crate = "near_sdk::serde")]
pub enum TokenSubType {
    Unknown,

    // Jewelry
    Ring,
    Earring,
    Necklace,

    // Armor
    Helmet,
    HelmetLight,
    HelmetHeavy,
    Body,
    BodyLight,
    BodyHeavy,
    Pants,
    PantsLight,
    PantsHeavy,
    Boots,
    BootsLight,
    BootsHeavy,
    Gloves,
    GlovesLight,
    GlovesHeavy,
    Cloak,
    Wristband,
    WristbandLight,
    WristbandHeavy,
    Belt,
    BeltLight,
    BeltHeavy,

    // Weapon
    Wand,
    Castet,
    Knife,
    Sword,
    Sword2,
    Hatchet,
    Hatchet2,
    Cudgel,
    Cudgel2,
    Staff,
    // Gloves

    Shield,

    Pet,
    Race,
    Class,

    // Class
    MagCrit,
    MagDodge,
    Tank,
    Warrior,
    MonkBuff,
    MonkParry,

    // Pet
    // MagCrit,
    // MagDodge,
    // Tank,
    // Warrior,
    // MonkBuff,
    // MonkParry,

    // Skin
    Bear,
    Boar,
    Chicken,
    Wolf,
    Bandit,
    Raptor,

    // Access
    Tester,
    Ladder,
    PreAlphaTester,
    AlphaTester,
    BetaTester,

    // Present
    Cup,
    Pen,
    Camera,
    Mask,
    Coin,
    Brush,

    // Race
    Human,
    Elf,
    Dwarf,
    Giant,
    BeastMan,
    Werewolf,
}

impl std::fmt::Display for TokenSubType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

