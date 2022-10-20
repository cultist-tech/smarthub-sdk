use crate::nft::{NonFungibleToken, TokenId, TokenRarity, TokenType};
use crate::nft::upgradable::NonFungibleTokenUpgradable;
use near_sdk::{ AccountId, env };
use near_sdk::json_types::U128;
use crate::nft::events::NftUpgrade;

const ONE_NEAR: u128 = 1000000000000000000000000;

impl NonFungibleToken {
    pub fn internal_upgrade_token(&mut self, token_id: &TokenId) {
        let owner_id = self.assert_token_holder(&token_id);
        let next_rarity = self.assert_next_rarity(&token_id);

        self.internal_upgrade_token_unguarded(&owner_id, token_id, &next_rarity);
    }

    pub fn assert_next_rarity(&self, token_id: &TokenId) -> TokenRarity {
        let rarity = self.token_rarity_by_id
            .as_ref()
            .unwrap()
            .get(token_id)
            .expect("Not found rarity");

        let next = match rarity {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 3,
            3 => 5,
            5 => 6,
            6 => env::panic_str("Token fully upgraded"),
            _ => env::panic_str("Token fully upgraded"),
        };

        next
    }

    pub fn internal_upgrade_price(
        &self,
        token_type: &TokenType,
        rarity: &TokenRarity
    ) -> U128 {
        let price = match token_type {
            TokenType::Armor => {
                match rarity {
                    1 => U128::from(ONE_NEAR * 3),
                    2 => U128::from(ONE_NEAR * 11),
                    3 => U128::from(ONE_NEAR * 37),
                    3 => U128::from(ONE_NEAR * 100_000),
                    5 => U128::from(ONE_NEAR * 100_000),
                    6 => U128::from(ONE_NEAR * 100_000),
                    _ => unimplemented!(),
                }
            }
            TokenType::Weapon => {
                match rarity {
                    1 => U128::from(ONE_NEAR * 8),
                    2 => U128::from(ONE_NEAR * 28),
                    3 => U128::from(ONE_NEAR * 98),
                    3 => U128::from(ONE_NEAR * 100_000),
                    5 => U128::from(ONE_NEAR * 100_000),
                    6 => U128::from(ONE_NEAR * 100_000),
                    _ => unimplemented!(),
                }
            }
            TokenType::Shield => {
                match rarity {
                    1 => U128::from(ONE_NEAR * 4),
                    2 => U128::from(ONE_NEAR * 14),
                    3 => U128::from(ONE_NEAR * 49),
                    3 => U128::from(ONE_NEAR * 100_000),
                    5 => U128::from(ONE_NEAR * 100_000),
                    6 => U128::from(ONE_NEAR * 100_000),
                    _ => unimplemented!(),
                }
            }
            TokenType::Pet => {
                match rarity {
                    1 => U128::from(ONE_NEAR * 8),
                    2 => U128::from(ONE_NEAR * 28),
                    3 => U128::from(ONE_NEAR * 98),
                    3 => U128::from(ONE_NEAR * 100_000),
                    5 => U128::from(ONE_NEAR * 100_000),
                    6 => U128::from(ONE_NEAR * 100_000),
                    _ => unimplemented!(),
                }
            }
            TokenType::Jewelry => {
                match rarity {
                    1 => U128::from(ONE_NEAR * 4),
                    2 => U128::from(ONE_NEAR * 14),
                    3 => U128::from(ONE_NEAR * 49),
                    3 => U128::from(ONE_NEAR * 100_000),
                    5 => U128::from(ONE_NEAR * 100_000),
                    6 => U128::from(ONE_NEAR * 100_000),
                    _ => unimplemented!(),
                }
            }
            TokenType::Class => {
                match rarity {
                    1 => U128::from(ONE_NEAR * 8),
                    2 => U128::from(ONE_NEAR * 28),
                    3 => U128::from(ONE_NEAR * 98),
                    3 => U128::from(ONE_NEAR * 100_000),
                    5 => U128::from(ONE_NEAR * 100_000),
                    6 => U128::from(ONE_NEAR * 100_000),
                    _ => unimplemented!(),
                }
            }
            TokenType::Race => {
                match rarity {
                    1 => U128::from(ONE_NEAR * 6),
                    2 => U128::from(ONE_NEAR * 21),
                    3 => U128::from(ONE_NEAR * 74),
                    3 => U128::from(ONE_NEAR * 100_000),
                    5 => U128::from(ONE_NEAR * 100_000),
                    6 => U128::from(ONE_NEAR * 100_000),
                    _ => unimplemented!(),
                }
            }
            _ => {
                unimplemented!();
            }
        };

        price
    }

    pub fn internal_upgrade_token_unguarded(
        &mut self,
        owner_id: &AccountId,
        token_id: &TokenId,
        rarity: &TokenRarity
    ) {
        let next_rarity = self.assert_next_rarity(&token_id);

        assert_eq!(next_rarity, rarity.clone(), "Invalid rarity upgrade");

        self.token_rarity_by_id.as_mut().unwrap().insert(&token_id, &next_rarity);

        (NftUpgrade {
            owner_id: &owner_id,
            token_id: &token_id,
            rarity: &next_rarity,
        }).emit();
    }
}

impl NonFungibleTokenUpgradable for NonFungibleToken {
    fn nft_upgrade(&mut self, token_id: TokenId) {
        unimplemented!();

        // self.internal_upgrade_token(&token_id, &U128::from(0));
    }
}
