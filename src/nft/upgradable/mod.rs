pub use upgradable_impl::*;
use near_sdk::AccountId;
use near_sdk::json_types::U128;
use crate::nft::{ TokenId, TokenRarity /*, TokenType*/ };
use crate::nft::metadata::UpgradePrice;

pub mod upgradable_impl;
mod macros;

pub trait NonFungibleTokenUpgradable {
    fn nft_upgrade(&mut self, token_id: TokenId);

    fn nft_set_upgrade_price(
        &mut self,
        token_type: String,
        rarity: TokenRarity,
        ft_token_id: AccountId,
        price: U128
    );

    fn nft_upgrade_price(&self, token_id: TokenId) -> Option<(AccountId, U128)>;
}