use near_sdk::{ AccountId, env, Balance, Promise };
use near_sdk::json_types::U128;
use crate::nft::{ NonFungibleToken, TokenId, TokenRarity };

use crate::nft::upgradable::NonFungibleTokenUpgradable;
use crate::nft::events::NftUpgrade;

const RARITY_MAX: u8 = 6;

impl NonFungibleToken {
    pub fn internal_upgrade_token(&mut self, token_id: &TokenId, owner_id: &AccountId) {
        let next_rarity = self.assert_next_rarity(&token_id);

        self.token_rarity_by_id.as_mut().unwrap().insert(&token_id, &next_rarity);

        (NftUpgrade {
            owner_id: &owner_id,
            token_id: &token_id,
            rarity: &next_rarity,
        }).emit();
    }

    pub fn assert_next_rarity(&self, token_id: &TokenId) -> TokenRarity {
        let rarity = self.token_rarity_by_id
            .as_ref()
            .unwrap()
            .get(token_id)
            .expect("Not found rarity");

        if rarity >= RARITY_MAX {
            env::panic_str("Token fully upgraded");
        }

        let next = rarity + 1;

        next
    }

    pub fn internal_price_for_token_upgrade(&self, token_id: &TokenId) -> u128 {
        let next_rarity = self.assert_next_rarity(&token_id);

        let upgrade_key = next_rarity.to_string();

        let price = self.entity_upgrade_price
            .as_ref()
            .unwrap()
            .get(&upgrade_key)
            .expect("There is no price for entity");

        price
    }

    pub fn internal_set_upgrade_price(&mut self, rarity: &TokenRarity, price: &u128) {
        let upgrade_key = rarity.to_string();

        self.entity_upgrade_price.as_mut().unwrap().insert(&upgrade_key, &price);
    }
}

impl NonFungibleTokenUpgradable for NonFungibleToken {
    fn nft_upgrade(&mut self, token_id: TokenId) {
        let owner_id = self.assert_token_holder(&token_id);

        let price = self.internal_price_for_token_upgrade(&token_id);

        let attached_deposit: Balance = env::attached_deposit();

        // check there is enough deposit attached for upgrade
        assert!(
            attached_deposit >= price,
            "Deposit is too small. Attached: {}, Required: {}",
            attached_deposit,
            price
        );

        //get the refund amount from the attached deposit - required cost
        let refund = attached_deposit - price;

        self.internal_upgrade_token(&token_id, &owner_id);

        //if the refund is greater than 1 yocto NEAR, we refund the predecessor that amount
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }

    fn nft_set_upgrade_price(&mut self, rarity: TokenRarity, price: U128) {
        assert!(rarity <= RARITY_MAX, "Given rarity is more then assumpted!");

        self.internal_set_upgrade_price(&rarity, &price.into());
    }

    fn nft_upgrade_price(&self, token_id: TokenId) -> U128 {
        let price = self.internal_price_for_token_upgrade(&token_id);

        price.into()
    }
}
