use crate::nft::metadata::TokenMetadata;
use crate::nft::{Token, TokenId, TokenRarity };
use crate::nft::royalty::Royalty;
pub use mint_impl::*;
use near_sdk::AccountId;

mod internal;
mod macros;
pub mod mint_impl;

pub trait NonFungibleTokenMint {
    fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: Option<AccountId>,
        token_metadata: TokenMetadata,
        bind_to_owner: Option<bool>,
        perpetual_royalties: Option<Royalty>,
        reveal_at: Option<u64>,

        rarity: Option<TokenRarity>,
        collection: Option<String>,
        token_type: Option<String>,
        token_sub_type: Option<String>,
    ) -> Token;
}
