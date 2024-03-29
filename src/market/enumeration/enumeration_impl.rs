use crate::market::base::{ MarketFeature };
use near_sdk::{ AccountId };
use crate::market::{ Sale, TokenId, MarketEnumeration };
use std::cmp::min;
use near_sdk::json_types::U64;
use crate::utils::contract_token_id;

impl MarketEnumeration for MarketFeature {
    fn market_supply_sales(&self) -> U64 {
        U64(self.sales.len())
    }

    fn market_supply_by_owner_id(&self, account_id: AccountId) -> U64 {
        let by_owner_id = self.by_owner_id.get(&account_id);
        if let Some(by_owner_id) = by_owner_id {
            U64(by_owner_id.len())
        } else {
            U64(0)
        }
    }

    fn market_sales_by_owner_id(
        &self,
        account_id: AccountId,
        from_index: U64,
        limit: u64
    ) -> Vec<Sale> {
        let mut tmp = vec![];
        let by_owner_id = self.by_owner_id.get(&account_id);
        let sales = if let Some(by_owner_id) = by_owner_id {
            by_owner_id
        } else {
            return vec![];
        };
        let keys = sales.as_vector();
        let start = u64::from(from_index);
        let end = min(start + limit, sales.len());
        for i in start..end {
            tmp.push(self.sales.get(&keys.get(i).unwrap()).unwrap());
        }
        tmp
    }

    fn market_supply_by_nft_contract_id(&self, nft_contract_id: AccountId) -> U64 {
        let by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id);
        if let Some(by_nft_contract_id) = by_nft_contract_id {
            U64(by_nft_contract_id.len())
        } else {
            U64(0)
        }
    }

    fn market_sales_by_nft_contract_id(
        &self,
        nft_contract_id: AccountId,
        from_index: U64,
        limit: u64
    ) -> Vec<Sale> {
        let mut tmp = vec![];
        let by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id);
        let sales = if let Some(by_nft_contract_id) = by_nft_contract_id {
            by_nft_contract_id
        } else {
            return vec![];
        };
        let keys = sales.as_vector();
        let start = u64::from(from_index);
        let end = min(start + limit, sales.len());
        for i in start..end {
            let id = contract_token_id(&nft_contract_id, &keys.get(i).unwrap());
            tmp.push(self.sales.get(&id).unwrap());
        }
        tmp
    }

    fn market_sale(&self, contract_id: AccountId, token_id: TokenId) -> Option<Sale> {
        self.sales.get(&format!("{}{}{}", contract_id, "||".to_string(), token_id))
    }
}
