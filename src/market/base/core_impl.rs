use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::{ LookupMap, UnorderedMap, UnorderedSet, TreeMap };
use near_sdk::json_types::{ U128 };
use near_sdk::serde::{ Serialize };
use near_sdk::{AccountId, Balance, Gas, CryptoHash, BorshStorageKey, Promise, promise_result_as_success, env, ext_contract, IntoStorageKey, assert_self};
use crate::market::metadata::{ ContractAndTokenId, TokenId };
use crate::market::{ Sale, MarketCore, Bid, MarketUpdateSale, MarketOffer };
use crate::utils::{ contract_token_id, near_ft };
use crate::nft::base::external::{ ext_nft };
use crate::ft::base::external::{ ext_ft };
use crate::nft::royalty::Payout;
use crate::metadata::FungibleTokenId;

// TODO check seller supports storage_deposit at ft_token_id they want to post sale in

pub(crate) const GAS_FOR_FT_TRANSFER: Gas = Gas(5_000_000_000_000);
/// greedy max Tgas for resolve_purchase
pub(crate) const GAS_FOR_ROYALTIES: Gas = Gas(115_000_000_000_000);
pub(crate) const GAS_FOR_NFT_TRANSFER: Gas = Gas(18_000_000_000_000);
pub(crate) const BID_HISTORY_LENGTH_DEFAULT: u8 = 1;

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageBalanceBounds {
    pub min: U128,
    pub max: Option<U128>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MarketFeature {
    pub owner_id: AccountId,
    pub sales: UnorderedMap<ContractAndTokenId, Sale>,
    pub by_owner_id: TreeMap<AccountId, UnorderedSet<ContractAndTokenId>>,
    pub by_nft_contract_id: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub ft_token_ids: UnorderedSet<AccountId>,
    pub storage_deposits: LookupMap<AccountId, Balance>,
    pub bid_history_length: u8,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Sales,
    ByOwnerId,
    ByOwnerIdInner {
        account_id_hash: CryptoHash,
    },
    ByNFTContractId,
    ByNFTContractIdInner {
        account_id_hash: CryptoHash,
    },
    FTTokenIds,
    StorageDeposits,
}

impl MarketFeature {
    pub fn new<M1, M2, M3, M4, M5>(
        owner_id: AccountId,
        ft_token_ids: Option<Vec<FungibleTokenId>>,
        bid_history_length: Option<u8>,
        sales_prefix: M1,
        by_owner_prefix: M2,
        by_contract_prefix: M3,
        ft_tokens_prefix: M4,
        storage_prefix: M5
    )
        -> Self
        where
            M1: IntoStorageKey,
            M2: IntoStorageKey,
            M3: IntoStorageKey,
            M4: IntoStorageKey,
            M5: IntoStorageKey
    {
        let mut this = Self {
            owner_id: owner_id.into(),
            sales: UnorderedMap::new(sales_prefix),
            by_owner_id: TreeMap::new(by_owner_prefix),
            by_nft_contract_id: LookupMap::new(by_contract_prefix),
            ft_token_ids: UnorderedSet::new(ft_tokens_prefix),
            storage_deposits: LookupMap::new(storage_prefix),
            bid_history_length: bid_history_length.unwrap_or(BID_HISTORY_LENGTH_DEFAULT),
        };
        // support NEAR by default
        this.ft_token_ids.insert(&near_ft());

        if let Some(ft_token_ids) = ft_token_ids {
            for ft_token_id in ft_token_ids {
                this.ft_token_ids.insert(&ft_token_id);
            }
        }

        this
    }
}

impl MarketCore for MarketFeature {
    /// only owner
    fn market_add_ft_token(&mut self, nft_contract_id: AccountId) -> bool {
        assert_self();

        self.ft_token_ids.insert(&nft_contract_id);

        true
    }

    /// views

    fn supported_ft_token_ids(&self) -> Vec<AccountId> {
        self.ft_token_ids.to_vec()
    }

    /// for add sale see: nft_callbacks.rs

    /// TODO remove without redirect to wallet? panic reverts
    // #[payable]
    fn market_remove_sale(&mut self, nft_contract_id: AccountId, token_id: String) {
        // assert_at_least_one_yocto();

        let contract_and_token_id = contract_token_id(&nft_contract_id, &token_id);
        let sale = self.sales.get(&contract_and_token_id);

        if sale.is_none() {
            env::panic_str("Not found sale");
        }

        if let Some(sale) = sale {
            let owner_id = env::predecessor_account_id();
            assert_eq!(owner_id, sale.owner_id, "Must be sale owner");

            self.internal_remove_sale(&nft_contract_id, &token_id);

            self.refund_all_bids(&sale.bids);
        }
    }

    // #[payable]
    fn market_update_price(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        ft_token_id: AccountId,
        price: U128
    ) {
        // assert_at_least_one_yocto();
        let contract_and_token_id = contract_token_id(&nft_contract_id, &token_id);
        let mut sale = self.sales.get(&contract_and_token_id).expect("No sale");
        assert_eq!(env::predecessor_account_id(), sale.owner_id, "Must be sale owner");

        if !self.ft_token_ids.contains(&ft_token_id) {
            env::panic_str(
                &format!("Token {} not supported by this market", ft_token_id).to_string()
            );
        }
        sale.sale_conditions.insert(ft_token_id.clone(), price);
        self.sales.insert(&contract_and_token_id, &sale);

        (MarketUpdateSale {
            owner_id: &sale.owner_id,
            nft_contract_id: &nft_contract_id,
            token_id: &token_id.to_string(),
            ft_token_id: &ft_token_id,
            price: &price,
        }).emit();
    }

    // #[payable]
    fn market_offer(&mut self, nft_contract_id: AccountId, token_id: String) {
        let contract_and_token_id = contract_token_id(&nft_contract_id, &token_id);

        let mut sale = self.sales
            .get(&contract_and_token_id)
            .expect(&format!("No sale {}", contract_and_token_id.to_string()));
        let buyer_id = env::predecessor_account_id();
        assert_ne!(sale.owner_id, buyer_id, "Cannot bid on your own sale.");
        let ft_token_id = near_ft();
        let price = sale.sale_conditions.get(&ft_token_id).expect("Not for sale in NEAR").0;

        let deposit = env::attached_deposit();
        assert!(deposit > 0, "Attached deposit must be greater than 0");

        if !sale.is_auction && deposit == price {
            self.market_process_purchase(
                nft_contract_id.clone(),
                token_id,
                ft_token_id,
                U128(deposit),
                buyer_id
            );
        } else {
            if sale.is_auction && price > 0 {
                assert!(deposit >= price, "Attached deposit must be greater than reserve price");
            }
            self.market_add_bid(contract_and_token_id, deposit, ft_token_id, buyer_id, &mut sale);
        }
    }

    // #[private]
    fn market_add_bid(
        &mut self,
        contract_and_token_id: ContractAndTokenId,
        amount: Balance,
        ft_token_id: AccountId,
        buyer_id: AccountId,
        sale: &mut Sale
    ) {
        // store a bid and refund any current bid lower
        let new_bid = Bid {
            owner_id: buyer_id,
            price: U128(amount),
        };

        let bids_for_token_id = sale.bids.entry(ft_token_id.clone()).or_insert_with(Vec::new);

        if !bids_for_token_id.is_empty() {
            let current_bid = &bids_for_token_id[bids_for_token_id.len() - 1];
            assert!(
                amount > current_bid.price.0,
                "Can't pay less than or equal to current bid price: {}",
                current_bid.price.0
            );
            if ft_token_id == near_ft() {
                Promise::new(current_bid.owner_id.clone()).transfer(u128::from(current_bid.price));
            } else {
                ext_ft
                    ::ext(ft_token_id.clone())
                    .with_static_gas(GAS_FOR_FT_TRANSFER)
                    .with_attached_deposit(1)
                    .ft_transfer(current_bid.owner_id.clone(), current_bid.price, None);
            }
        }

        bids_for_token_id.push(new_bid);
        if bids_for_token_id.len() > (self.bid_history_length as usize) {
            bids_for_token_id.remove(0);
        }

        self.sales.insert(&contract_and_token_id, &sale);
    }

    fn market_accept_offer(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        ft_token_id: AccountId
    ) {
        let contract_and_token_id = contract_token_id(&nft_contract_id, &token_id);
        // remove bid before proceeding to process purchase
        let mut sale = self.sales.get(&contract_and_token_id).expect("No sale");
        let bids_for_token_id = sale.bids.remove(&ft_token_id).expect("No bids");
        let bid = &bids_for_token_id[bids_for_token_id.len() - 1];
        self.sales.insert(&contract_and_token_id, &sale);

        // panics at `self.internal_remove_sale` and reverts above if predecessor is not sale.owner_id
        self.market_process_purchase(
            nft_contract_id.clone(),
            token_id,
            ft_token_id.into(),
            bid.price,
            bid.owner_id.clone()
        );
    }

    // #[private]
    fn market_process_purchase(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        ft_token_id: AccountId,
        price: U128,
        buyer_id: AccountId
    ) -> Promise {
        let sale = self.internal_remove_sale(&nft_contract_id, &token_id);

        ext_nft
            ::ext(nft_contract_id.clone())
            .with_static_gas(GAS_FOR_NFT_TRANSFER)
            .with_attached_deposit(1)
            .nft_transfer_payout(
                buyer_id.clone(),
                token_id,
                sale.approval_id,
                price,
                10,
                Some("payout from market".to_string())
            )
            .then(
                ext_self
                    ::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_ROYALTIES)
                    .market_resolve_purchase(ft_token_id, buyer_id, sale, price)
            )
    }

    /// self callback

    // #[private]

    fn market_resolve_purchase(
        &mut self,
        ft_token_id: FungibleTokenId,
        buyer_id: AccountId,
        sale: Sale,
        price: U128
    ) -> U128 {
        // checking for payout information
        let payout_option = promise_result_as_success().and_then(|value| {
            // None means a bad payout from bad NFT contract
            near_sdk::serde_json
                ::from_slice::<Payout>(&value)
                .ok()
                .and_then(|payout| {
                    // gas to do 10 FT transfers (and definitely 10 NEAR transfers)
                    if payout.payout.len() + sale.bids.len() > 10 || payout.payout.is_empty() {
                        env::log_str(
                            &format!(
                                "Cannot have more than 10 royalties and sale.bids refunds"
                            ).to_string()
                        );
                        None
                    } else {
                        // TODO off by 1 e.g. payouts are fractions of 3333 + 3333 + 3333
                        let mut remainder = price.0;
                        for &value in payout.payout.values() {
                            remainder = remainder.checked_sub(value.0)?;
                        }
                        if remainder == 0 || remainder == 1 {
                            Some(payout)
                        } else {
                            None
                        }
                    }
                })
        });
        // is payout option valid?
        let payout: Payout = if let Some(payout_option) = payout_option {
            payout_option
        } else {
            if ft_token_id == AccountId::new_unchecked("near".to_string()) {
                Promise::new(buyer_id).transfer(u128::from(price));
            }
            // leave function and return all FTs in ft_resolve_transfer
            return price;
        };
        // Going to payout everyone, first return all outstanding bids (accepted offer bid was already removed)
        self.refund_all_bids(&sale.bids);

        // NEAR payouts
        if ft_token_id == near_ft() {
            for (receiver_id, amount) in payout.payout.clone() {
                Promise::new(receiver_id).transfer(amount.0);
            }

            (MarketOffer {
                owner_id: &sale.owner_id,
                receiver_id: &buyer_id,
                nft_contract_id: &sale.nft_contract_id,
                token_id: &sale.token_id.to_string(),
                payout: &payout.payout,
                ft_token_id: &ft_token_id,
                price: &price,
            }).emit();
            // refund all FTs (won't be any)
            price
        } else {
            // FT payouts
            for (receiver_id, amount) in payout.payout.clone() {
                ext_ft
                    ::ext(ft_token_id.clone())
                    .with_static_gas(GAS_FOR_FT_TRANSFER)
                    .with_attached_deposit(1)
                    .ft_transfer(receiver_id.clone(), amount, None);
            }

            (MarketOffer {
                owner_id: &sale.owner_id,
                receiver_id: &buyer_id,
                nft_contract_id: &sale.nft_contract_id,
                token_id: &sale.token_id.to_string(),
                payout: &payout.payout,
                ft_token_id: &ft_token_id,
                price: &price,
            }).emit();

            // keep all FTs (already transferred for payouts)
            U128(0)
        }
    }
}

/// self call

#[ext_contract(ext_self)]
trait ExtSelf {
    fn market_resolve_purchase(
        &mut self,
        ft_token_id: AccountId,
        buyer_id: AccountId,
        sale: Sale,
        price: U128
    ) -> Promise;
}
