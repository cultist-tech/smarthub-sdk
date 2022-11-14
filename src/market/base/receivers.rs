use crate::market::base::MarketFeature;
use crate::market::{ MarketOnFtTransferArgs, MarketCore };
use crate::utils::contract_token_id;
use crate::metadata::{ FungibleTokenId };
use near_sdk::{ PromiseOrValue, AccountId };
use near_sdk::json_types::{ U128 };

impl MarketFeature {
    pub fn internal_on_ft_transfer(
        &mut self,
        args: &MarketOnFtTransferArgs,
        ft_token_id: &FungibleTokenId,
        amount: &U128,
        sender_id: &AccountId
    ) -> PromiseOrValue<U128> {
        let MarketOnFtTransferArgs { nft_contract_id, token_id } = args;

        let contract_and_token_id = contract_token_id(&nft_contract_id, &token_id);
        let mut sale = self.sales.get(&contract_and_token_id).expect("No sale in ft_on_transfer");

        assert_ne!(&sale.owner_id, sender_id, "Cannot buy your own sale.");

        let price = *sale.sale_conditions
            .get(&ft_token_id)
            .expect("Not for sale in that token type");

        assert!(amount.0 > 0, "Amount must be greater than 0");
        
        let fee = self.internal_market_fee(&price.0, &sender_id);
        
        let amount = &U128(amount.0 - fee);

        if !sale.is_auction && amount == &price {
            self.market_process_purchase(
                nft_contract_id.clone(),
                token_id.clone(),
                ft_token_id.clone(),
                price,
                sender_id.clone()
            ).into()
        } else {
            if sale.is_auction && price.0 > 0 {
                assert!(amount.0 >= price.0, "Amount must be greater than reserve price");
            }
            self.market_add_bid(
                contract_and_token_id,
                amount.0,
                ft_token_id.clone(),
                sender_id.clone(),
                &mut sale
            );

            PromiseOrValue::Value(U128(0))
        }
    }
}
