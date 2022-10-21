pub use upgradable_impl::*;
use near_sdk::AccountId;
use near_sdk::json_types::U128;
use crate::nft::{ TokenId, TokenRarity, TokenType };

pub mod upgradable_impl;
mod macros;

pub trait NonFungibleTokenUpgradable {
    fn nft_upgrade(&mut self, token_id: TokenId);
    
    fn nft_upgrade_price(&mut self, token_type: TokenType, rarity: TokenRarity,      price: U128);
}
