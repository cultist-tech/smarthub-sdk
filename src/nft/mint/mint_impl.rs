use crate::nft::metadata::TokenMetadata;
use crate::nft::mint::NonFungibleTokenMint;
use crate::nft::{NonFungibleToken, Token, TokenId, TokenRarity, TokenCollection, TokenType, TokenSubType};
use crate::nft::royalty::Royalty;
use near_sdk::{AccountId, env};

impl NonFungibleToken {}

impl NonFungibleTokenMint for NonFungibleToken {
    fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: Option<AccountId>,
        token_metadata: TokenMetadata,
        bind_to_owner: Option<bool>,
        perpetual_royalties: Option<Royalty>,
        reveal_at: Option<u64>,

        rarity: Option<TokenRarity>,
        collection: Option<TokenCollection>,
        token_type: Option<TokenType>,
        token_sub_type: Option<TokenSubType>,
    ) -> Token {
        self.internal_mint_nft(
            &token_id,
            Some(receiver_id.unwrap_or_else(|| env::current_account_id())),
            Some(token_metadata),
            bind_to_owner,
            perpetual_royalties,
            reveal_at,

            rarity,
            collection,
            token_type,
            token_sub_type,
        )
    }
}
