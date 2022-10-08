use std::collections::HashMap;
use near_sdk::{ AccountId, Balance, IntoStorageKey };
use near_sdk::json_types::U128;
use near_sdk::serde::{ Deserialize, Serialize };

use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use crate::nft::royalty::NonFungibleTokenRoyalty;
use near_sdk::collections::TreeMap;
use schemars::{JsonSchema};
use crate::nft::NonFungibleToken;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RoyaltyFeature {
    pub amount: u32,
    pub receiver_id: AccountId,

    pub royalty_by_id: TreeMap<String, Royalty>,
}

pub const MINTER_ROYALTY_CAP: u32 = 2000;
pub const CONTRACT_ROYALTY_CAP: u32 = 1000;

pub type Royalty = HashMap<AccountId, u32>;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

pub(crate) fn royalty_to_payout(a: u32, b: Balance) -> U128 {
    U128(((a as u128) * b) / 10_000u128)
}

impl RoyaltyFeature {
    pub fn new<Q>(receiver_id: AccountId, amount: u32, token_royalty_prefix: Q) -> Self
        where Q: IntoStorageKey
    {
        let this = Self {
          royalty_by_id: TreeMap::new(token_royalty_prefix),
            amount,
            receiver_id,
        };

        this
    }

    pub fn royalty_calculate(
        &self,
        perpetual_royalties: Option<Royalty>
    ) -> HashMap<AccountId, u32> {
        let mut royalty = HashMap::new();
        let mut total_perpetual = 0;

        // user added perpetual_royalties (percentage paid with every transfer)
        if let Some(perpetual_royalties) = perpetual_royalties {
            assert!(
                perpetual_royalties.len() < 7,
                "Cannot add more than 6 perpetual royalty amounts"
            );
            for (account, amount) in perpetual_royalties {
                royalty.insert(account, amount);
                total_perpetual += amount;
            }
        }

        assert!(
            total_perpetual <= MINTER_ROYALTY_CAP,
            "Perpetual royalties cannot be more than 20%"
        );

        royalty
    }

    pub fn to_payout(
        &self,
        balance: U128,
        max_len_payout: u32,
        token_id: String,
        owner_id: AccountId
    ) -> Payout {
        let royalty = self.royalty_by_id.get(&token_id);

        // compute payouts based on balance option
        // adds in contract_royalty and computes previous owner royalty from remainder
        let mut total_perpetual = 0;
        let balance_u128 = u128::from(balance);
        let mut payout: Payout = Payout {
            payout: HashMap::new(),
        };

        if let Some(royalty) = royalty {
            assert!(
                (royalty.len() as u32) <= max_len_payout,
                "Market cannot payout to that many receivers"
            );

            for (k, v) in royalty.iter() {
                let key = k.clone();

                if key != owner_id {
                    payout.payout.insert(key, royalty_to_payout(*v, balance_u128));
                    total_perpetual += *v;
                }
            }
        }

        // payout to contract owner - may be previous token owner, they get remainder of balance
        if self.amount > 0 && self.receiver_id != owner_id {
            payout.payout.insert(
                self.receiver_id.clone(),
                royalty_to_payout(self.amount, balance_u128)
            );
            total_perpetual += self.amount;
        }

        assert!(
            total_perpetual <= MINTER_ROYALTY_CAP + CONTRACT_ROYALTY_CAP,
            "Royalties should not be more than caps"
        );
        // payout to previous owner
        payout.payout.insert(owner_id, royalty_to_payout(10000 - total_perpetual, balance_u128));

        payout
    }
}

impl NonFungibleTokenRoyalty for RoyaltyFeature {
    fn set_royalty_value(&mut self, contract_royalty: u32) {
        assert!(
            contract_royalty <= CONTRACT_ROYALTY_CAP,
            "Contract royalties limited to 10% for owner"
        );

        self.amount = contract_royalty;
    }

    fn set_royalty_account(&mut self, account_id: AccountId) -> AccountId {
        self.receiver_id = account_id.clone();
        self.receiver_id.clone()
    }

    fn nft_royalty_value(&self) -> u32 {
        self.amount
    }

    fn nft_royalty_account(&self) -> AccountId {
        self.receiver_id.clone()
    }
}

impl NonFungibleTokenRoyalty for NonFungibleToken {
  fn set_royalty_value(&mut self, contract_royalty: u32) {
    self.royalty.set_royalty_value(contract_royalty)
  }

  fn set_royalty_account(&mut self, account_id: AccountId) -> AccountId {
    self.royalty.set_royalty_account(account_id)
  }

  fn nft_royalty_value(&self) -> u32 {
    self.royalty.nft_royalty_value()
  }

  fn nft_royalty_account(&self) -> AccountId {
    self.royalty.nft_royalty_account()
  }
}
