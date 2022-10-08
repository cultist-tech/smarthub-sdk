use crate::nft::base::StorageKey;
use crate::nft::events::{ NftCreate, NftMint };
use crate::nft::metadata::TokenMetadata;
use crate::nft::metadata::HIDDEN_TOKEN;
use crate::nft::{
    NonFungibleToken,
    Token,
    TokenId,
    TokenRarity,
    TokenCollection,
    TokenType,
    TokenSubType,
};
use crate::nft::royalty::Royalty;
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::Base64VecU8;
use near_sdk::{ env, AccountId };
use std::collections::HashMap;

impl NonFungibleToken {
    pub fn internal_mint_nft(
        &mut self,
        token_id: &TokenId,
        token_owner_id: Option<AccountId>,
        token_metadata: Option<TokenMetadata>,
        bind_to_owner: Option<bool>,
        perpetual_royalties: Option<Royalty>,
        reveal_at: Option<u64>,

        rarity: Option<TokenRarity>,
        collection: Option<TokenCollection>,
        token_type: Option<TokenType>,
        token_sub_type: Option<TokenSubType>
    ) -> Token {
        //While hidden token minting metadata to hide must be provided
        if reveal_at.is_some() {
            assert!(token_metadata.is_some(), "Metadata to hide not provided");
        }

        let token_id = if let Some(reveal_time) = reveal_at {
            format!("_r{}", token_id.clone())
        } else {
            token_id.to_string()
        };
        let metadata = if let Some(reveal_time) = reveal_at {
            Some(TokenMetadata {
                title: Some(format!("{} #{}", HIDDEN_TOKEN.to_string(), token_id.clone())),
                description: Some(
                    format!("Hidden token that may be revealed in {} unix time", reveal_time)
                ),
                media: Some("__hidden.png".to_string()),
                media_hash: None,
                copies: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            })
        } else {
            token_metadata.clone()
        };

        let token = self.internal_create_nft_with_refund(
            &token_id,
            token_owner_id,
            metadata.clone(),
            bind_to_owner,
            perpetual_royalties,
            reveal_at,

            rarity,
            collection,
            token_type,
            token_sub_type,
            Some(env::predecessor_account_id())
        );

        if let Some(reveal_time) = reveal_at {
            self.token_hidden_metadata.insert(token_metadata.as_ref().unwrap());
            self.tokens_to_reveal.insert(&token_id);
            self.token_reveal_time_by_id.insert(&token_id, &reveal_time);
        }

        token
    }

    /// Mint a new token without checking:
    /// * Whether the caller id is equal to the `owner_id`
    /// * `refund_id` will transfer the left over balance after storage costs are calculated to the provided account.
    ///   Typically the account will be the owner. If `None`, will not refund. This is useful for delaying refunding
    ///   until multiple tokens have been minted.
    ///
    /// Returns the newly minted token
    pub fn internal_create_nft_with_refund(
        &mut self,
        token_id: &TokenId,
        token_owner_id: Option<AccountId>,
        token_metadata: Option<TokenMetadata>,
        bind_to_owner: Option<bool>,
        perpetual_royalties: Option<Royalty>,
        reveal_at: Option<u64>,

        rarity: Option<TokenRarity>,
        collection: Option<TokenCollection>,
        token_type: Option<TokenType>,
        token_sub_type: Option<TokenSubType>,

        _refund_id: Option<AccountId>
    ) -> Token {
        // let prev_storage = env::storage_usage();
        // Remember current storage usage if refund_id is Some
        // let _initial_storage_usage = refund_id.map(|account_id| (account_id, env::storage_usage()));

        //

        if self.token_metadata_by_id.is_some() && token_metadata.is_none() {
            env::panic_str("Must provide metadata");
        }
        if self.owner_by_id.get(&token_id).is_some() {
            env::panic_str("token_id must be unique");
        }

        let mut owner_id = env::current_account_id();

        if let Some(token_owner_id) = token_owner_id {
            owner_id = token_owner_id;
        }

        // Core behavior: every token must have an owner
        self.owner_by_id.insert(&token_id, &owner_id);

        // Metadata extension: Save metadata, keep variable around to return later.
        // Note that check above already panicked if metadata extension in use but no metadata
        // provided to call.
        self.token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, token_metadata.as_ref().unwrap()));

        // bind to owner extension
        if let Some(bind_to_owner) = &bind_to_owner {
            self.bind_to_owner.internal_token_bind_to_owner(&token_id, &bind_to_owner);
        }
        // royalty extension
        let royalty = self.royalty.royalty_calculate(perpetual_royalties);

        // Enumeration extension: Record tokens_per_owner for use with enumeration view methods.
        if let Some(tokens_per_owner) = &mut self.tokens_per_owner {
            let mut token_ids = tokens_per_owner.get(&owner_id).unwrap_or_else(|| {
                UnorderedSet::new(StorageKey::TokensPerOwner {
                    account_hash: env::sha256(owner_id.as_bytes()),
                })
            });
            token_ids.insert(&token_id);
            tokens_per_owner.insert(&owner_id, &token_ids);
        }

        // Approval Management extension: return empty HashMap as part of Token
        let approved_account_ids = if self.approvals_by_id.is_some() {
            Some(HashMap::new())
        } else {
            None
        };

        // extra fields
        if let Some(rarity) = &rarity {
            self.token_rarity_by_id.as_mut().unwrap().insert(&token_id, &rarity);
        }
        if let Some(collection) = &collection {
            self.token_collection_by_id.as_mut().unwrap().insert(&token_id, &collection);
        }
        if let Some(token_type) = &token_type {
            self.token_type_by_id.as_mut().unwrap().insert(&token_id, &token_type);
        }
        if let Some(token_sub_type) = &token_sub_type {
            self.token_sub_type_by_id.as_mut().unwrap().insert(&token_id, &token_sub_type);
        }

        // if let Some((id, storage_usage)) = initial_storage_usage {
        // refund_deposit_to_account(env::storage_usage() - storage_usage, id)
        // }
        // Return any extra attached deposit not used for storage

        let token = Token {
            token_id: token_id.clone(),
            owner_id: owner_id.clone(),
            metadata: token_metadata,
            approved_account_ids,
            royalty: Some(royalty),
            bind_to_owner,
            reveal_at,

            rarity: rarity.clone(),
            token_sub_type: token_sub_type.clone(),
            token_type: token_type.clone(),
            collection: collection.clone(),
        };

        (NftCreate { token: &token }).emit();
        (NftMint {
            owner_id: &owner_id,
            token_ids: &vec![token_id.clone()],
            memo: None,
        }).emit();

        token
    }
}
