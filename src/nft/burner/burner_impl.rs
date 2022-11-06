use crate::nft::{ NonFungibleToken, TokenId, TokenRarity, TokenTypes, BurnerPrice, RARITY_MAX, PriceType };
use crate::nft::burner::NonFungibleTokenBurner;
use crate::nft::utils::{ upgrade_key, types_str };
use crate::nft::events::{ NftSetBurnerPrice, NftRemoveUpgradePrice };

impl NonFungibleToken {
    pub fn internal_burner_price(&self, token_id: &TokenId) -> Option<BurnerPrice> {
        let next_rarity = self.assert_next_rarity(&token_id);

        let types = self.token_types_by_id.as_ref().unwrap().get(&token_id);

        let types_str = types_str(&types);

        let upgrade_key = upgrade_key(&types_str, &next_rarity);

        let price = self.burner_upgrade_prices.as_ref().unwrap().get(&upgrade_key);

        price
    }

    pub fn internal_set_burner_price(
        &mut self,
        types: &Option<TokenTypes>,
        rarity: &TokenRarity,
        price: &BurnerPrice
    ) {
        let types_str = types_str(types);

        let upgrade_key = upgrade_key(&types_str, rarity);

        self.burner_upgrade_prices.as_mut().unwrap().insert(&upgrade_key, &price);

        (NftSetBurnerPrice {
            rarity: &rarity,
            types: &types,
            burning_rarity_sum: &price,            
        }).emit();
    }
}

impl NonFungibleTokenBurner for NonFungibleToken {
    fn nft_burner_upgrade(&mut self, token_id: TokenId, burning_tokens: Vec<TokenId>) {
        let owner_id = self.assert_token_holder(&token_id);

        let price = self
            .internal_burner_price(&token_id)
            .expect("There is no price for burner upgrade");

        let mut provided_rarity_sum = 0;

        burning_tokens.iter().for_each(|burning_token_id| {
            
            self.assert_token_holder(&burning_token_id);
            
            let rarity = self.token_rarity_by_id.as_ref().unwrap().get(&burning_token_id).unwrap();
            
            provided_rarity_sum += rarity + 1;
        });
        
        assert_eq!(provided_rarity_sum, price, "Provided burning tokens rarity sum is not equal to upgrade price");
        
        burning_tokens.iter().for_each(|burning_token_id| {
            self.internal_burn_token_unguarded(&owner_id, &burning_token_id);
        });

        self.internal_upgrade_token(&token_id, &owner_id);
    }

    fn nft_set_burner_upgrade_price(
        &mut self,
        types: Option<TokenTypes>,
        rarity: TokenRarity,       
        burning_rarity_sum: u8,
    ) {
        assert!(rarity <= RARITY_MAX, "Given rarity is more then assumpted!");

        self.internal_set_burner_price(&types, &rarity, &burning_rarity_sum);
    }

    fn nft_remove_burner_upgrade_price(&mut self, types: Option<TokenTypes>, rarity: TokenRarity) {
        let types_str = types_str(&types);

        let upgrade_key = upgrade_key(&types_str, &rarity);

        assert!(
            self.burner_upgrade_prices.as_mut().unwrap().remove(&upgrade_key).is_some(),
            "Price was not set"
        );

        (NftRemoveUpgradePrice {
            price_type: &PriceType::Burner,
            rarity: &rarity,
            types: &types,
        }).emit();
    }

    fn nft_burner_upgrade_price(&self, token_id: TokenId) -> Option<BurnerPrice> {
        self.internal_burner_price(&token_id)
    }
}
