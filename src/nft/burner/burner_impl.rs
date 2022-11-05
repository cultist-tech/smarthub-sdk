use crate::nft::{ NonFungibleToken, TokenId, TokenRarity, TokenTypes, BurnerPrice, RARITY_MAX };
use crate::nft::burner::NonFungibleTokenBurner;
use crate::nft::utils::upgrade_key;
use crate::nft::utils::types_str;

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
    }
}

impl NonFungibleTokenBurner for NonFungibleToken {
    fn nft_burner_upgrade(&mut self, token_id: TokenId, burning_tokens: Vec<TokenId>) {
        let owner_id = self.assert_token_holder(&token_id);

        let price = self
            .internal_burner_price(&token_id)
            .expect("There is no price for burner upgrade");

        assert!(
            (burning_tokens.len() as u8) == price.price,
            "Deposit tokens number is too small. Attached: {}, Required: {}",
            burning_tokens.len(),
            price.price
        );

        burning_tokens.iter().for_each(|burning_token_id| {
            assert_eq!(
                self.token_rarity_by_id.as_ref().unwrap().get(&burning_token_id).unwrap(),
                price.burning_rarity,
                "Burning tokens must have price defined rarity"
            );

            self.internal_burn_token(&owner_id, &burning_token_id);
        });

        self.internal_upgrade_token(&token_id, &owner_id);
    }

    fn nft_set_burner_upgrade_price(
        &mut self,
        types: Option<TokenTypes>,
        rarity: TokenRarity,
        price: u8,
        burning_rarity: TokenRarity
    ) {
        assert!(rarity <= RARITY_MAX, "Given rarity is more then assumpted!");

        let upgrade_price = BurnerPrice {
            burning_rarity,
            price,
        };

        self.internal_set_burner_price(&types, &rarity, &upgrade_price);
    }

    fn nft_remove_burner_upgrade_price(&mut self, types: Option<TokenTypes>, rarity: TokenRarity) {
        let types_str = types_str(&types);

        let upgrade_key = upgrade_key(&types_str, &rarity);

        assert!(
            self.burner_upgrade_prices.as_mut().unwrap().remove(&upgrade_key).is_some(),
            "Price was not set"
        );
    }

    fn nft_burner_upgrade_price(&self, token_id: TokenId) -> Option<BurnerPrice> {
        self.internal_burner_price(&token_id)
    }
}