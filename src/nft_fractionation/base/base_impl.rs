use near_sdk::{ env, AccountId, IntoStorageKey, require };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::{ TreeMap, LookupMap, UnorderedSet };
use near_sdk::json_types::U128;
use crate::nft_fractionation::{ TokenId, FractionationId };
use crate::nft_fractionation::base::NonFungibleTokenFractionation;
use crate::nft_fractionation::metadata::{ Fractionation, ContractFractionationId, ContractId };
use crate::nft_fractionation::events::FractionationComplete;
use crate::nft_fractionation::utils::{ date_now, contract_token_id };

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftFractionationFeature {
    pub nft_contract_id: AccountId,

    // get fractionation by token
    pub fractionation_token_by_id: LookupMap<ContractFractionationId, FractionationId>,
    // get fractionation tokens
    pub fractionation_by_id: TreeMap<ContractFractionationId, UnorderedSet<TokenId>>,
    // list of fractionations
    pub fractionation_ids: UnorderedSet<ContractFractionationId>,
    // is fractionation available
    pub fractionation_available_ids: UnorderedSet<ContractFractionationId>,
    // is fractionation completed
    pub fractionation_completed_by_id: LookupMap<ContractFractionationId, u64>,
    // get contract id by fractionation id
    pub fractionation_contract_by_id: LookupMap<ContractFractionationId, AccountId>,

    // tokens by owner
    pub tokens_per_owner: LookupMap<AccountId, TreeMap<ContractId, UnorderedSet<TokenId>>>,
}

impl NftFractionationFeature {
    pub fn new<F1, F2, F3, F4, F5, T1, F6>(
        nft_contract_id: AccountId,
        fractionation_prefix: F1,
        fractionation_tokens_prefix: F2,
        fractionations_prefix: F3,
        fractionation_completed_prefix: F4,
        fractionation_available_prefix: F5,
        tokens_per_owner_prefix: T1,
        fractionation_contract_by_id_prefix: F6
    )
        -> Self
        where
            F1: IntoStorageKey,
            F2: IntoStorageKey,
            F3: IntoStorageKey,
            F4: IntoStorageKey,
            F5: IntoStorageKey,
            T1: IntoStorageKey,
            F6: IntoStorageKey
    {
        let this = Self {
            nft_contract_id,
            fractionation_by_id: TreeMap::new(fractionation_prefix),
            fractionation_token_by_id: LookupMap::new(fractionation_tokens_prefix),
            fractionation_ids: UnorderedSet::new(fractionations_prefix),
            fractionation_available_ids: UnorderedSet::new(fractionation_available_prefix),
            fractionation_completed_by_id: LookupMap::new(fractionation_completed_prefix),
            tokens_per_owner: LookupMap::new(tokens_per_owner_prefix),
            fractionation_contract_by_id: LookupMap::new(fractionation_contract_by_id_prefix),
        };

        this
    }
}

impl NonFungibleTokenFractionation for NftFractionationFeature {
    fn nft_fractionation(&self, contract_id: AccountId, token_id: TokenId) -> Fractionation {
        let id = contract_token_id(&contract_id, &token_id);

        self.enum_fractionation(&id)
    }

    fn nft_fractionations(
        &self,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<Fractionation> {
        let arr = &self.fractionation_by_id;

        let start_index: u128 = from_index.map(From::from).unwrap_or_default();

        if (arr.len() as u128) <= start_index {
            return vec![];
        }

        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        require!(limit != 0, "Cannot provide limit of 0.");

        let res = arr
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|(token_id, _entries)| self.enum_fractionation(&token_id))
            .collect();

        res
    }

    fn nft_fractionations_supply(&self) -> U128 {
        let count = self.fractionation_by_id.len();

        U128::from(count as u128)
    }

    fn nft_fractionation_complete(&mut self, contract_id: AccountId, token_id: FractionationId) {
        let id = contract_token_id(&contract_id, &token_id);
        let signer_id = env::predecessor_account_id();

        let fractionation_tokens = self.fractionation_by_id
            .get(&id)
            .expect("Not found fractionation")
            .to_vec();

        self.assert_tokens_holder(&signer_id, &contract_id, &fractionation_tokens);

        // burn items
        self.internal_remove_tokens(&signer_id, &contract_id, &fractionation_tokens);

        // lock fractionation
        self.internal_remove_fractionation(&id);

        // transfer new token
        self.internal_call_nft_transfer(&contract_id, &token_id, &signer_id);

        let date = date_now();
        self.fractionation_completed_by_id.insert(&id, &date);

        (FractionationComplete {
            contract_id: &contract_id,
            token_id: &token_id,
            receiver_id: &signer_id,
        }).emit();
    }
}