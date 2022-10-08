// Reveal

#[macro_export]
macro_rules! impl_non_fungible_token_reveal {
    ($contract:ident, $tokens:ident) => {
        use $crate::nft::NonFungibleTokenReveal;

        #[near_bindgen]
        impl NonFungibleTokenReveal for $contract {
            #[payable]
            fn nft_reveal(&mut self, token_id: mfight_sdk::nft::TokenId) {
                self.$tokens.nft_reveal(token_id)
            }
        }
    };
}
