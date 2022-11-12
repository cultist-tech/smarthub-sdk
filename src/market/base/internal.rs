use crate::market::base::{ GAS_FOR_FT_TRANSFER, MarketFeature, MARKET_BASE_FEE, MARKET_REDUCED_FEE, MARKET_MIN_FEE };
use crate::market::metadata::{ TokenId, Bids, MarketOnNftApproveArgs };
use near_sdk::collections::UnorderedSet;
use std::collections::HashMap;
use near_sdk::{AccountId, env, CryptoHash, Promise, BorshStorageKey, PromiseOrValue};
use crate::market::{ Sale, MarketRemoveSale, MarketCreateSale };
use near_sdk::borsh::{ self, BorshSerialize };
use crate::ft::base::external::ext_ft;
use crate::utils::{ contract_token_id, hash_account_id, near_ft };
use crate::reputation::MAX_REPUTATION;
use near_sdk::json_types::{ U128 };

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    ByOwnerIdInner {
        account_id_hash: CryptoHash,
    },
    ByNFTContractIdInner {
        account_id_hash: CryptoHash,
    },
}

impl MarketFeature {
    /// refund the last bid of each token type, don't update sale because it's already been removed

    pub(crate) fn refund_all_bids(&mut self, bids: &Bids) {
        for (bid_ft, bid_vec) in bids {
            let bid = &bid_vec[bid_vec.len() - 1];
            let fee = self.market_fees_paid.remove(&bid.owner_id).unwrap_or_else(|| 0);
            if bid_ft == &near_ft() {
                Promise::new(bid.owner_id.clone()).transfer(u128::from(bid.price) + fee);
            } else {
                ext_ft
                    ::ext(bid_ft.clone())
                    .with_static_gas(GAS_FOR_FT_TRANSFER)
                    .with_attached_deposit(1)
                    .ft_transfer(bid.owner_id.clone(), U128(bid.price.0 + fee), None);
            }
        }
    }

    pub(crate) fn internal_remove_sale(
        &mut self,
        nft_contract_id: &AccountId,
        token_id: &TokenId
    ) -> Sale {
        let contract_and_token_id = contract_token_id(&nft_contract_id, &token_id);
        let sale = self.sales.remove(&contract_and_token_id).expect("No sale");

        let mut by_owner_id = self.by_owner_id.get(&sale.owner_id).expect("No sale by_owner_id");
        by_owner_id.remove(&contract_and_token_id);
        if by_owner_id.is_empty() {
            self.by_owner_id.remove(&sale.owner_id);
        } else {
            self.by_owner_id.insert(&sale.owner_id, &by_owner_id);
        }

        let mut by_nft_contract_id = self.by_nft_contract_id
            .get(&nft_contract_id)
            .expect("No sale by nft_contract_id");
        by_nft_contract_id.remove(&token_id);
        if by_nft_contract_id.is_empty() {
            self.by_nft_contract_id.remove(&nft_contract_id);
        } else {
            self.by_nft_contract_id.insert(&nft_contract_id, &by_nft_contract_id);
        }

        (MarketRemoveSale {
            owner_id: &sale.owner_id,
            nft_contract_id: &nft_contract_id,
            token_id: &token_id,
        }).emit();

        sale
    }

    pub fn internal_on_nft_approve(
        &mut self,
        args: &MarketOnNftApproveArgs,
        nft_contract_id: &AccountId,
        token_id: &TokenId,
        owner_id: &AccountId,
        approval_id: &u64
    ) -> PromiseOrValue<String> {
        let MarketOnNftApproveArgs { is_auction, sale_conditions } = args;

        for (ft_token_id, _price) in sale_conditions.clone() {
            if !self.ft_token_ids.contains(&ft_token_id) {
                env::panic_str(
                    &format!("Token {} not supported by this market", ft_token_id).to_string()
                );
            }
        }

        let bids = HashMap::new();

        let contract_and_token_id = contract_token_id(nft_contract_id, token_id);

        let exists = self.sales.get(&contract_and_token_id);

        if exists.is_some() {
            env::panic_str("Token already listed");
        }

        let sale = Sale {
            owner_id: owner_id.clone().into(),
            approval_id: approval_id.clone(),
            nft_contract_id: nft_contract_id.clone(),
            token_id: token_id.clone(),
            sale_conditions: sale_conditions.clone(),
            bids,
            created_at: env::block_timestamp(),
            is_auction: is_auction.unwrap_or(false),
        };
        self.sales.insert(&contract_and_token_id, &sale);

        // extra for views

        let mut by_owner_id = self.by_owner_id.get(&owner_id).unwrap_or_else(|| {
            UnorderedSet::new(
                (StorageKey::ByOwnerIdInner {
                    account_id_hash: hash_account_id(&owner_id),
                })
                    .try_to_vec()
                    .unwrap()
            )
        });

        by_owner_id.insert(&contract_and_token_id);
        self.by_owner_id.insert(&owner_id, &by_owner_id);

        let mut by_nft_contract_id = self.by_nft_contract_id
            .get(&nft_contract_id)
            .unwrap_or_else(|| {
                UnorderedSet::new(
                    (StorageKey::ByNFTContractIdInner {
                        account_id_hash: hash_account_id(&nft_contract_id),
                    })
                        .try_to_vec()
                        .unwrap()
                )
            });
        by_nft_contract_id.insert(&token_id);
        self.by_nft_contract_id.insert(&nft_contract_id, &by_nft_contract_id);

        (MarketCreateSale {
            owner_id: &owner_id,
            nft_contract_id: &nft_contract_id,
            token_id: &token_id,
            sale: &sale,
        }).emit();

        PromiseOrValue::Value("true".to_string())
    }
    
    pub fn internal_market_fee(
        &mut self,
        price: &u128,
        account_id: &AccountId,        
    ) -> u128 {
        let mut fee_percent = MARKET_BASE_FEE;  
        
        if self.reputation.is_some() {
            let reputation = self.reputation.as_ref().unwrap().internal_reputation(&account_id);
            
            if reputation > MAX_REPUTATION/2 && reputation != MAX_REPUTATION {
                fee_percent = MARKET_REDUCED_FEE;
            } 
            
            if reputation == MAX_REPUTATION {
                fee_percent = MARKET_MIN_FEE;
            }
        }
        let fee: u128 = price * fee_percent as u128 / 10_000u128;
        
        fee
    }
}
