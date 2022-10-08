// Mint

#[macro_export]
macro_rules! impl_non_fungible_token_mint {
    ($contract:ident, $tokens:ident, $assert_owner:ident) => {
        use $crate::nft::{NonFungibleTokenMint, Royalty};

        #[near_bindgen]
        impl NonFungibleTokenMint for $contract {
            fn nft_mint(
                &mut self,
                token_id: mfight_sdk::nft::TokenId,
                receiver_id: Option<AccountId>,
                token_metadata: mfight_sdk::nft::TokenMetadata,

                bind_to_owner: Option<bool>,
                perpetual_royalties: Option<Royalty>,
                reveal_at: Option<u64>,

                rarity: Option<mfight_sdk::nft::TokenRarity>,
                collection: Option<mfight_sdk::nft::TokenCollection>,
                token_type: Option<mfight_sdk::nft::TokenType>,
                token_sub_type: Option<mfight_sdk::nft::TokenSubType>,
            ) -> mfight_sdk::nft::Token {
                self.$assert_owner();

                self.$tokens.nft_mint(
                    token_id,
                    receiver_id,
                    token_metadata,
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
    };
}
