pub use burner_impl::*;
use near_sdk::AccountId;
use near_sdk::json_types::U128;
use crate::nft::{ TokenId, TokenRarity, TokenTypes };
use crate::nft::metadata::BurnerPrice;

pub mod burner_impl;
mod macros;

pub trait NonFungibleTokenBurner {
    fn nft_burner_upgrade(&mut self, token_id: TokenId, burning_tokens: Vec<TokenId>);

    fn nft_set_burner_upgrade_price(
        &mut self,
        types: Option<TokenTypes>,
        rarity: TokenRarity,
        price: u8,
        burning_rarity: TokenRarity
    );

    fn nft_remove_burner_upgrade_price(&mut self, types: Option<TokenTypes>, rarity: TokenRarity);

    fn nft_burner_upgrade_price(&self, token_id: TokenId) -> Option<BurnerPrice>;
}