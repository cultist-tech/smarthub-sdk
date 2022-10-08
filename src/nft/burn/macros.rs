// Burn

#[macro_export]
macro_rules! impl_non_fungible_token_burn {
    ($contract:ident, $tokens:ident $(, $assert_burn:ident)?) => {
        use $crate::nft::{NonFungibleTokenBurnable};

        #[near_bindgen]
        impl NonFungibleTokenBurnable for $contract {
          #[payable]
          fn nft_burn(&mut self, token_id: &mfight_sdk::nft::TokenId) {
            $(self.$assert_burn();)?
            self.$tokens.nft_burn(token_id)
          }
        }
    };
}
