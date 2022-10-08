use crate::nft::TokenId;
pub use reveal_impl::*;

mod internal;
mod macros;
pub mod reveal_impl;
mod test;

pub trait NonFungibleTokenReveal {
    fn nft_reveal(&mut self, token_id: TokenId);
}
