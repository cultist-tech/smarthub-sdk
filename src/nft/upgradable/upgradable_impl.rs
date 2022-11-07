use near_sdk::{ AccountId, env, Balance, Promise, PromiseOrValue };
use near_sdk::json_types::U128;
use crate::nft::{
    NonFungibleToken,
    TokenId,
    TokenRarity,
    TokenTypes,
    UpdateOnFtTransferArgs,
    TOKEN_TYPE,
    RARITY_MAX,
    PriceType
};
use crate::nft::metadata::UpgradePrice;
use crate::nft::upgradable::NonFungibleTokenUpgradable;
use crate::nft::events::{ NftUpgrade, NftSetUpgradePrice, NftRemoveUpgradePrice };
use crate::nft::utils::{ upgrade_key, types_str };
use crate::utils::near_ft;

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

    pub fn internal_price_for_token_upgrade(&self, token_id: &TokenId) -> Option<UpgradePrice> {
        let next_rarity = self.assert_next_rarity(&token_id);        
         
        let types = self.token_types_by_id.as_ref().unwrap().get(&token_id);
        
        let types_str = types_str(&types);

        let upgrade_key = upgrade_key(&types_str, &next_rarity);

        let price = self.upgrade_prices.as_ref().unwrap().get(&upgrade_key);

        price
    }   

    pub fn internal_set_upgrade_price(
        &mut self,
        types: &Option<TokenTypes>,
        rarity: &TokenRarity,
        price: &UpgradePrice
    ) {
        let types_str = types_str(types);
        let upgrade_key = upgrade_key(&types_str, rarity);

        self.upgrade_prices.as_mut().unwrap().insert(&upgrade_key, &price);
        
        (NftSetUpgradePrice {
            rarity: &rarity,
            types: &types,
            ft_token: &price.ft_token_id,
            price: &U128(price.price),
        }).emit();
    }

    pub fn internal_on_ft_transfer(
        &mut self,
        args: &UpdateOnFtTransferArgs,
        ft_token_id: &AccountId,
        amount: &u128,
        sender_id: &AccountId
    ) -> PromiseOrValue<U128> {
        let UpdateOnFtTransferArgs { token_id } = args;

        let owner_id = self.owner_by_id
            .get(token_id)
            .unwrap_or_else(|| env::panic_str("Not found token"));

        assert_eq!(&owner_id, sender_id, "Unauthorized upgrade");

        let price = self
            .internal_price_for_token_upgrade(&token_id)
            .expect("There is no price for upgrade");

        assert!(price.ft_token_id == *ft_token_id, "Price is in another FT token");

        assert!(
            *amount == price.price,
            "Price value is not deposited. Attached: {}, Required: {}",
            amount,
            price.price
        );

        self.internal_upgrade_token(&token_id, &owner_id);

        PromiseOrValue::Value(U128(0))
    }
}

impl NonFungibleTokenUpgradable for NonFungibleToken {
    fn nft_upgrade(&mut self, token_id: TokenId) {
        let owner_id = self.assert_token_holder(&token_id);

        let price = self
            .internal_price_for_token_upgrade(&token_id)
            .expect("There is no price for upgrade");

        assert!(price.ft_token_id == near_ft(), "Price is not in native token");

        let attached_deposit: Balance = env::attached_deposit();

        // check there is enough deposit attached for upgrade
        assert!(
            attached_deposit >= price.price,
            "Deposit is too small. Attached: {}, Required: {}",
            attached_deposit,
            price.price
        );

        //get the refund amount from the attached deposit - required cost
        let refund = attached_deposit - price.price;

        self.internal_upgrade_token(&token_id, &owner_id);

        //if the refund is greater than 1 yocto NEAR, we refund the predecessor that amount
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }

    fn nft_set_upgrade_price(
        &mut self,
        types: Option<TokenTypes>,
        rarity: TokenRarity,
        ft_token_id: AccountId,
        price: U128
    ) {
        assert!(rarity <= RARITY_MAX, "Given rarity is more then assumpted!");

        let upgrade_price = UpgradePrice {
            ft_token_id,
            price: price.into(),
        };

        self.internal_set_upgrade_price(&types, &rarity, &upgrade_price);
    }
    
    fn nft_remove_upgrade_price(
        &mut self,
        types: Option<TokenTypes>,
        rarity: TokenRarity,       
    ) {
        
        let types_str = types_str(&types);
         
        let upgrade_key = upgrade_key(&types_str, &rarity);
         
        assert!(self.upgrade_prices.as_mut().unwrap().remove(&upgrade_key).is_some(), "Price was not set");    
        
        (NftRemoveUpgradePrice {
            price_type: &PriceType::Upgradable, 
            rarity: &rarity,
            types: &types,            
        }).emit();
    }


    fn nft_upgrade_price(&self, token_id: TokenId) -> Option<(AccountId, U128)> {
        if let Some(price) = self.internal_price_for_token_upgrade(&token_id) {
            return Some((price.ft_token_id, U128(price.price)));
        }

        None
    }
}
