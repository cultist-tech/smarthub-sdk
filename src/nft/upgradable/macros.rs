// Upgradable

#[macro_export]
macro_rules! impl_non_fungible_token_upgradable {
    ($contract:ident, $tokens:ident $(, $assert_owner:ident)?) => {
        use $crate::nft::NonFungibleTokenUpgradable;

        #[near_bindgen]
        impl NonFungibleTokenUpgradable for $contract {
            #[payable]
            fn nft_upgrade(&mut self, token_id: mfight_sdk::nft::TokenId) {                
                self.$tokens.nft_upgrade(token_id)
            }
            
            fn nft_upgrade_price(&mut self, token_type: mfight_sdk::nft::TokenType, rarity: mfight_sdk::nft::TokenRarity, price: U128) {
                $(self.$assert_owner();)?
                self.$tokens.nft_upgrade_price(token_type, rarity, price)
            }
        }
    };
}
