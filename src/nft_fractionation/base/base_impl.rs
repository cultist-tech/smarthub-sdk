use near_sdk::{ env, AccountId, IntoStorageKey, require };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::{ TreeMap, LookupMap, UnorderedSet };
use near_sdk::json_types::U128;
use crate::nft_fractionation::{ TokenId, FractionationId };
use crate::nft_fractionation::base::NonFungibleTokenFractionation;
use crate::nft_fractionation::metadata::{ Fractionation, ContractFractionationId, ContractId };
use crate::nft_fractionation::events::FractionationComplete;
use crate::nft_fractionation::utils::contract_token_id;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftFractionationFeature {
    // get fractionation owners
    pub fractionations_owners: LookupMap<ContractFractionationId, AccountId>,
    // get fractionation tokens
    pub fractionation_by_id: TreeMap<ContractFractionationId, UnorderedSet<TokenId>>,
    // get fractionations by contract
    pub fractionations_by_contract: TreeMap<ContractId, UnorderedSet<FractionationId>>,    
    // tokens by owner
    pub tokens_per_owner: LookupMap<AccountId, TreeMap<ContractId, UnorderedSet<TokenId>>>,
}

impl NftFractionationFeature {
    pub fn new<F1, F2, F3, T1>(
        fractionations_owners_prefix: F1,
        fractionation_prefix: F2,
        fractionations_prefix: F3,        
        tokens_per_owner_prefix: T1
    )
        -> Self
        where
            F1: IntoStorageKey,
            F2: IntoStorageKey,
            F3: IntoStorageKey,            
            T1: IntoStorageKey
    {
        let this = Self {
            fractionations_owners: LookupMap::new(fractionations_owners_prefix),
            fractionation_by_id: TreeMap::new(fractionation_prefix),
            fractionations_by_contract: TreeMap::new(fractionations_prefix),            
            tokens_per_owner: LookupMap::new(tokens_per_owner_prefix),
        };

        this
    }
}

impl NonFungibleTokenFractionation for NftFractionationFeature {
    fn nft_fractionation(&self, contract_id: AccountId, token_id: TokenId) -> Fractionation {
        self.enum_fractionation(&contract_id, &token_id)
    }

    fn nft_fractionations(
        &self,
        contract_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<Fractionation> {
        let arr = &self.fractionations_by_contract
            .get(&contract_id)
            .expect("No fractionations for contract!");

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
            .map(|token_id| self.enum_fractionation(&contract_id, &token_id))
            .collect();

        res
    }

    fn nft_fractionations_supply(&self, contract_id: AccountId) -> U128 {
        let count = if let Some(fractionations) = self.fractionations_by_contract.get(&contract_id) {
            fractionations.len()
        } else {
            0
        };

        U128::from(count as u128)
    }

    fn nft_fractionation_complete(&mut self, contract_id: AccountId, token_id: FractionationId) {
        let id = contract_token_id(&contract_id, &token_id);
        let signer_id = env::predecessor_account_id();

        let owner_id = self.fractionations_owners
            .get(&id)
            .expect("Wrong token to coplete fractionation");
        assert_ne!(signer_id, owner_id, "Fractionation creater can not complete it!");

        let fractionation_tokens = self.fractionation_by_id
            .get(&id)
            .expect("Not found fractionation")
            .to_vec();

        self.assert_tokens_holder(&signer_id, &contract_id, &fractionation_tokens);

        // burn items
        self.internal_remove_tokens(&signer_id, &contract_id, &fractionation_tokens);

        // lock fractionation
        self.internal_remove_fractionation(&contract_id, &token_id);

        // transfer new token
        self.internal_call_nft_transfer(&contract_id, &token_id, &signer_id);        

        (FractionationComplete {
            contract_id: &contract_id,
            token_id: &token_id,
            receiver_id: &signer_id,
        }).emit();
    }
}
