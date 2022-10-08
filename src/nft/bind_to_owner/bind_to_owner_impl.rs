use near_sdk::collections::LookupMap;
use crate::nft::bind_to_owner::NonFungibleTokenBindToOwner;
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::IntoStorageKey;
use crate::nft::NonFungibleToken;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct BindToOwnerFeature {
    pub token_bind_by_id: LookupMap<String, bool>,
}

impl BindToOwnerFeature {
    pub fn new<Q>(prefix: Q) -> Self where Q: IntoStorageKey {
        let this = Self {
            token_bind_by_id: LookupMap::new(prefix),
        };

        this
    }

    pub fn assert_bind_to_player(&self, token_id: &String) {
        let is_bind = self.internal_is_bind_to_owner(&token_id);

        assert!(!&is_bind, "Token is bind to account");
    }

    pub fn internal_is_bind_to_owner(&self, token_id: &String) -> bool {
        self.token_bind_by_id.get(&token_id).unwrap_or_else(|| false)
    }

    pub fn internal_token_bind_to_owner(&mut self, token_id: &String, bind_to_owner: &bool) {
        self.token_bind_by_id.insert(&token_id, &bind_to_owner);
    }
}


impl NonFungibleTokenBindToOwner for NonFungibleToken {
  fn nft_is_bind_to_owner(&self, token_id: String) -> bool {
    self.bind_to_owner.internal_is_bind_to_owner(&token_id)
  }
}

