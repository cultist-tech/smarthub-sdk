// Upgradable

#[macro_export]
macro_rules! impl_non_fungible_token_burner {
    ($contract:ident, $tokens:ident, $assert_owner:ident $(, $assert_upgrade:ident)?) => {
        use $crate::nft::NonFungibleTokenBurner;

        #[near_bindgen]
        impl NonFungibleTokenBurner for $contract {            
            fn nft_burner_upgrade(&mut self, token_id: mfight_sdk::nft::TokenId, burning_tokens: Vec<mfight_sdk::nft::TokenId>) { 
                $(self.$assert_upgrade();)?
                self.$tokens.nft_burner_upgrade(token_id,burning_tokens)
            }
            
            fn nft_set_burner_upgrade_price(&mut self, types: Option<mfight_sdk::nft::TokenTypes>, rarity: mfight_sdk::nft::TokenRarity,  price: u8, burning_rarity: mfight_sdk::nft::TokenRarity) {
                self.$assert_owner();
                self.$tokens.nft_set_burner_upgrade_price(types, rarity, price, burning_rarity)
            }
            
            fn nft_remove_burner_upgrade_price(&mut self, types: Option< mfight_sdk::nft::TokenTypes>, rarity: mfight_sdk::nft::TokenRarity) {
                self.$assert_owner();
                self.$tokens.nft_remove_burner_upgrade_price(types, rarity)
            }
            
            fn nft_burner_upgrade_price(&self, token_id: mfight_sdk::nft::TokenId) -> Option<mfight_sdk::nft::BurnerPrice> {
                self.$tokens.nft_burner_upgrade_price(token_id)            
            }
        }
    };
}