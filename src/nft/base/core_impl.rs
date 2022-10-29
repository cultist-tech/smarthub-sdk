use super::resolver::NonFungibleTokenResolver;
use crate::nft::bind_to_owner::BindToOwnerFeature;
use crate::nft::base::receiver::ext_receiver;
use crate::nft::base::NonFungibleTokenCore;
use crate::nft::metadata::{ TokenMetadata, UpgradeKey, UpgradePrice };
use crate::nft::token::{ Token, TokenId };
use crate::nft::utils::{ hash_account_id, refund_approved_account_ids };
use crate::nft::royalty::RoyaltyFeature;
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::{ LookupMap, TreeMap, UnorderedSet };
use near_sdk::json_types::Base64VecU8;
use near_sdk::{
    assert_one_yocto,
    env,
    ext_contract,
    log,
    require,
    AccountId,
    BorshStorageKey,
    CryptoHash,
    Gas,
    IntoStorageKey,
    PromiseOrValue,
    PromiseResult,
    StorageUsage,
};
use std::collections::HashMap;
use crate::nft::{TokenRarity, TokenCollection, TokenType, TokenSubType, NonFungibleTokenBindToOwner, TokenTypes};

pub const GAS_FOR_RESOLVE_NFT_TRANSFER: Gas = Gas(5_000_000_000_000);
pub const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_NFT_TRANSFER.0);
pub const GAS_FOR_NFT_TRANSFER: Gas = Gas(25_000_000_000_000);

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    TokensPerOwner {
        account_hash: Vec<u8>,
    },
    TokenPerOwnerInner {
        account_id_hash: CryptoHash,
    },
}

#[ext_contract(ext_self)]
trait NFTResolver {
    fn nft_resolve_transfer(
        &mut self,
        previous_owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: Option<HashMap<AccountId, u64>>
    ) -> bool;
}

/// Implementation of the non-fungible token standard.
/// Allows to include NEP-171 compatible token to any contract.
/// There are next traits that any contract may implement:
///     - NonFungibleTokenCore -- interface with nft_transfer methods. NonFungibleToken provides methods for it.
///     - NonFungibleTokenApproval -- interface with nft_approve methods. NonFungibleToken provides methods for it.
///     - NonFungibleTokenEnumeration -- interface for getting lists of tokens. NonFungibleToken provides methods for it.
///     - NonFungibleTokenMetadata -- return metadata for the token in NEP-177, up to contract to implement.
///
/// For example usage, see examples/non-fungible-token/src/lib.rs.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NonFungibleToken {
  // The storage size in bytes for each new token
  pub extra_storage_in_bytes_per_token: StorageUsage,

  // always required
  pub owner_by_id: TreeMap<TokenId, AccountId>,

  // required by metadata extension
  pub token_metadata_by_id: Option<LookupMap<TokenId, TokenMetadata>>,

  // required by enumeration extension
  pub tokens_per_owner: Option<LookupMap<AccountId, UnorderedSet<TokenId>>>,

  // required by approval extension
  pub approvals_by_id: Option<LookupMap<TokenId, HashMap<AccountId, u64>>>,
  pub next_approval_id_by_id: Option<LookupMap<TokenId, u64>>,

  // required by royalty and nft_payout extensions
  pub royalty: RoyaltyFeature,

  // required by bind_to_owner extension
  pub bind_to_owner: BindToOwnerFeature,

  // required by upgrade extension
  pub token_rarity_by_id: Option<LookupMap<TokenId, TokenRarity>>,
  pub token_collection_by_id: Option<LookupMap<TokenId, TokenCollection>>,
  pub token_type_by_id: Option<LookupMap<TokenId, TokenType>>,
  pub token_sub_type_by_id: Option<LookupMap<TokenId, TokenSubType>>,

  pub token_types_by_id: Option<LookupMap<TokenId, TokenTypes>>,

  // required by reveal extension
  pub token_hidden_metadata: UnorderedSet<TokenMetadata>,
  pub tokens_to_reveal: UnorderedSet<TokenId>,
  pub token_reveal_time_by_id: LookupMap<TokenId, u64>,

  // required by upgrade extension
  pub upgrade_prices: Option<LookupMap<UpgradeKey, UpgradePrice>>,
}

impl NonFungibleToken {
    pub fn new<Q, R, S, T, R1, B, RM, RT, RTM, E1, E2, E3, E4, E5, U1>(
        owner_by_id_prefix: Q,
        token_metadata_prefix: Option<R>,
        enumeration_prefix: Option<S>,
        approval_prefix: Option<T>,

        nft_royalty_prefix: R1,

        bind_to_owner_prefix: B,

        reveal_hidden_meta_prefix: RM,
        reveal_tokens_prefix: RT,
        reveal_time_prefix: RTM,

        token_rarity_prefix: Option<E1>,
        token_collection_prefix: Option<E2>,
        token_type_prefix: Option<E3>,
        token_sub_type_prefix: Option<E4>,
        token_types_prefix: Option<E5>,

        upgrade_prefix: Option<U1>,
    )
        -> Self
        where
            Q: IntoStorageKey,
            R: IntoStorageKey,
            S: IntoStorageKey,
            T: IntoStorageKey,
            R1: IntoStorageKey,
            B: IntoStorageKey,
            RM: IntoStorageKey,
            RT: IntoStorageKey,
            RTM: IntoStorageKey,
            E1: IntoStorageKey,
            E2: IntoStorageKey,
            E3: IntoStorageKey,
            E4: IntoStorageKey,
            E5: IntoStorageKey,
            U1: IntoStorageKey
    {
        let (approvals_by_id, next_approval_id_by_id) = if let Some(prefix) = approval_prefix {
            let prefix: Vec<u8> = prefix.into_storage_key();
            (
                Some(LookupMap::new(prefix.clone())),
                Some(LookupMap::new([prefix, "n".into()].concat())),
            )
        } else {
            (None, None)
        };
        let mut this = Self {
            extra_storage_in_bytes_per_token: 0,
            owner_by_id: TreeMap::new(owner_by_id_prefix),
            token_metadata_by_id: token_metadata_prefix.map(LookupMap::new),
            tokens_per_owner: enumeration_prefix.map(LookupMap::new),
            approvals_by_id,
            next_approval_id_by_id,

            royalty: RoyaltyFeature::new(env::current_account_id(), 0, nft_royalty_prefix),
            bind_to_owner: BindToOwnerFeature::new(bind_to_owner_prefix),

            token_hidden_metadata: UnorderedSet::new(reveal_hidden_meta_prefix),
            tokens_to_reveal: UnorderedSet::new(reveal_tokens_prefix),
            token_reveal_time_by_id: LookupMap::new(reveal_time_prefix),

            token_rarity_by_id: token_rarity_prefix.map(LookupMap::new),
            token_collection_by_id: token_collection_prefix.map(LookupMap::new),
            token_type_by_id: token_type_prefix.map(LookupMap::new),
            token_sub_type_by_id: token_sub_type_prefix.map(LookupMap::new),
            token_types_by_id: token_types_prefix.map(LookupMap::new),

            upgrade_prices: upgrade_prefix.map(LookupMap::new),
        };
        this.measure_min_token_storage_cost();
        this
    }

    // TODO: does this seem reasonable?
    fn measure_min_token_storage_cost(&mut self) {
        let initial_storage_usage = env::storage_usage();
        // 64 Length because this is the max account id length
        let tmp_token_id = "a".repeat(64);
        let tmp_owner_id = AccountId::new_unchecked("a".repeat(64));

        // 1. set some dummy data
        self.owner_by_id.insert(&tmp_token_id, &tmp_owner_id);
        if let Some(token_metadata_by_id) = &mut self.token_metadata_by_id {
            token_metadata_by_id.insert(
                &tmp_token_id,
                &(TokenMetadata {
                    title: Some("a".repeat(64)),
                    description: Some("a".repeat(64)),
                    media: Some("a".repeat(64)),
                    media_hash: Some(Base64VecU8::from("a".repeat(64).as_bytes().to_vec())),
                    copies: Some(1),
                    issued_at: None,
                    expires_at: None,
                    starts_at: None,
                    updated_at: None,
                    extra: None,
                    reference: None,
                    reference_hash: None,
                })
            );
        }
        if let Some(tokens_per_owner) = &mut self.tokens_per_owner {
            let u = &mut UnorderedSet::new(StorageKey::TokensPerOwner {
                account_hash: env::sha256(tmp_owner_id.as_bytes()),
            });
            u.insert(&tmp_token_id);
            tokens_per_owner.insert(&tmp_owner_id, u);
        }
        if let Some(approvals_by_id) = &mut self.approvals_by_id {
            let mut approvals = HashMap::new();
            approvals.insert(tmp_owner_id.clone(), 1u64);
            approvals_by_id.insert(&tmp_token_id, &approvals);
        }
        if let Some(next_approval_id_by_id) = &mut self.next_approval_id_by_id {
            next_approval_id_by_id.insert(&tmp_token_id, &1u64);
        }
        let u = UnorderedSet::new(
            (StorageKey::TokenPerOwnerInner {
                account_id_hash: hash_account_id(&tmp_owner_id),
            })
                .try_to_vec()
                .unwrap()
        );
        if let Some(tokens_per_owner) = &mut self.tokens_per_owner {
            tokens_per_owner.insert(&tmp_owner_id, &u);
        }

        // 2. see how much space it took
        self.extra_storage_in_bytes_per_token = env::storage_usage() - initial_storage_usage;

        // 3. roll it all back
        if let Some(next_approval_id_by_id) = &mut self.next_approval_id_by_id {
            next_approval_id_by_id.remove(&tmp_token_id);
        }
        if let Some(approvals_by_id) = &mut self.approvals_by_id {
            approvals_by_id.remove(&tmp_token_id);
        }
        if let Some(tokens_per_owner) = &mut self.tokens_per_owner {
            tokens_per_owner.remove(&tmp_owner_id);
        }
        if let Some(token_metadata_by_id) = &mut self.token_metadata_by_id {
            token_metadata_by_id.remove(&tmp_token_id);
        }
        if let Some(tokens_per_owner) = &mut self.tokens_per_owner {
            tokens_per_owner.remove(&tmp_owner_id);
        }
        self.owner_by_id.remove(&tmp_token_id);
    }
}

impl NonFungibleTokenCore for NonFungibleToken {
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>
    ) {
        assert_one_yocto();
        let sender_id = env::predecessor_account_id();

        self.internal_transfer(&sender_id, &receiver_id, &token_id, approval_id, memo);
    }

    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String
    ) -> PromiseOrValue<bool> {
        assert_one_yocto();

        require!(
            env::prepaid_gas() > GAS_FOR_NFT_TRANSFER_CALL + GAS_FOR_RESOLVE_NFT_TRANSFER,
            "More gas is required"
        );
        let sender_id = env::predecessor_account_id();
        let (old_owner, old_approvals) = self.internal_transfer(
            &sender_id,
            &receiver_id,
            &token_id,
            approval_id,
            memo
        );

        ext_receiver
            ::ext(receiver_id.clone())
            .with_static_gas(env::prepaid_gas() - GAS_FOR_NFT_TRANSFER_CALL)
            .nft_on_transfer(sender_id, old_owner.clone(), token_id.clone(), msg)
            .then(
                ext_self
                    ::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_RESOLVE_NFT_TRANSFER)
                    .nft_resolve_transfer(old_owner, receiver_id, token_id, old_approvals)
            )
            .into()
    }

    fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        let owner_id = self.owner_by_id.get(&token_id)?;
        let token = self.enum_get_token(owner_id.clone(), token_id.clone());

        Some(token)
    }
}

impl NonFungibleTokenResolver for NonFungibleToken {
    /// Returns true if token was successfully transferred to `receiver_id`.
    fn nft_resolve_transfer(
        &mut self,
        previous_owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: Option<HashMap<AccountId, u64>>
    ) -> bool {
        // Get whether token should be returned
        let must_revert = match env::promise_result(0) {
            PromiseResult::NotReady => env::abort(),
            PromiseResult::Successful(value) => {
                if let Ok(yes_or_no) = near_sdk::serde_json::from_slice::<bool>(&value) {
                    yes_or_no
                } else {
                    true
                }
            }
            PromiseResult::Failed => true,
        };

        // if call succeeded, return early
        if !must_revert {
            return true;
        }

        // OTHERWISE, try to set owner back to previous_owner_id and restore approved_account_ids

        // Check that receiver didn't already transfer it away or burn it.
        if let Some(current_owner) = self.owner_by_id.get(&token_id) {
            if current_owner != receiver_id {
                // The token is not owned by the receiver anymore. Can't return it.
                return true;
            }
        } else {
            // The token was burned and doesn't exist anymore.
            // Refund storage cost for storing approvals to original owner and return early.
            if let Some(approved_account_ids) = approved_account_ids {
                refund_approved_account_ids(previous_owner_id, &approved_account_ids);
            }
            return true;
        }

        log!("Return token {} from @{} to @{}", token_id, receiver_id, previous_owner_id);

        self.internal_transfer_unguarded(&token_id, &receiver_id, &previous_owner_id);

        // If using Approval Management extension,
        // 1. revert any approvals receiver already set, refunding storage costs
        // 2. reset approvals to what previous owner had set before call to nft_transfer_call
        if let Some(by_id) = &mut self.approvals_by_id {
            if let Some(receiver_approvals) = by_id.get(&token_id) {
                refund_approved_account_ids(receiver_id, &receiver_approvals);
            }
            if let Some(previous_owner_approvals) = approved_account_ids {
                by_id.insert(&token_id, &previous_owner_approvals);
            }
        }

        false
    }
}
