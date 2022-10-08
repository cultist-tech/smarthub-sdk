pub use burn_impl::*;
use crate::nft::TokenId;

pub mod burn_impl;
mod internal;
mod macros;

pub trait NonFungibleTokenBurnable {
    fn nft_burn(&mut self, token_id: &TokenId);
}
