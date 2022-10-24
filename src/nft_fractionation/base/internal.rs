use near_sdk::collections::{ UnorderedSet, TreeMap };
use near_sdk::borsh::{ self, BorshSerialize };
use near_sdk::{ BorshStorageKey, env, AccountId };
use crate::nft_fractionation::{
    NftFractionationFeature,
    TokenId,
    FractionationId,
    FractionationNftOnTransferArgs,
    ContractId,
};
use crate::nft_fractionation::metadata::Fractionation;
use crate::nft_fractionation::events::{ FractionationCreate, FractionationProcess };
use crate::nft_fractionation::utils::contract_token_id;
use crate::nft::base::GAS_FOR_NFT_TRANSFER;
use crate::nft::base::external::ext_nft;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    FractionationTokensInner {
        token_hash: Vec<u8>,
    },
    FractionationContractsPerOwnerInner {
        account_hash: Vec<u8>,
    },
    FractionationTokensPerContractInner {
        contract_hash: Vec<u8>,
    },
    FractionationsPerContractInner {
        token_hash: Vec<u8>,
    },
}

impl NftFractionationFeature {
    pub(crate) fn assert_tokens_holder(
        &self,
        account_id: &AccountId,
        contract_id: &ContractId,
        token_ids: &Vec<TokenId>
    ) {
        let contracts = self.tokens_per_owner.get(&account_id).unwrap();
        let tokens = contracts.get(&contract_id).unwrap();

        token_ids
            .iter()
            .for_each(|token_id| {
                assert!(tokens.contains(&token_id), "Not all of tokens is owned")
            });
    }

    pub(crate) fn internal_remove_tokens(
        &mut self,
        account_id: &AccountId,
        contract_id: &AccountId,
        token_ids: &Vec<TokenId>
    ) {
        let mut contracts = self.tokens_per_owner.get(&account_id).unwrap();
        let mut tokens = contracts.get(&contract_id).unwrap();

        token_ids.iter().for_each(|token_id| {
            tokens.remove(&token_id);
        });

        contracts.insert(&contract_id, &tokens);
        self.tokens_per_owner.insert(&account_id, &contracts);
    }

    pub fn enum_fractionation(
        &self,
        contract_id: &AccountId,
        fractionation_id: &FractionationId
    ) -> Fractionation {
        let id = contract_token_id(&contract_id, &fractionation_id);
        let entries = self.fractionation_by_id.get(&id).expect("Not found fractionation");
        
        Fractionation {
            contract_id: contract_id.clone(),
            token_id: fractionation_id.clone(),
            entries: entries.to_vec(),            
        }
    }

    pub fn internal_remove_fractionation(
        &mut self,
        contract_id: &AccountId,
        fractionation_id: &FractionationId
    ) {
        let mut fractionations = self.fractionations_by_contract.get(&contract_id).unwrap();

        fractionations.remove(&fractionation_id);
        self.fractionations_by_contract.insert(&contract_id, &fractionations);

        let id = contract_token_id(&contract_id, &fractionation_id);

        self.fractionations_owners.remove(&id);
        self.fractionation_by_id.remove(&id);
    }

    pub fn internal_add_token_to_fractionation(
        &mut self,
        contract_id: &AccountId,
        fractionation_id: &FractionationId,
        token_id: &TokenId
    ) {
        let id = contract_token_id(&contract_id, &fractionation_id);
        let fractionation_by_id = &mut self.fractionation_by_id;

        let mut fractionation = fractionation_by_id.get(&id).expect("Not found fractionation");

        fractionation.insert(&token_id);
        fractionation_by_id.insert(&id, &fractionation);
    }

    pub fn internal_create_fractionation(
        &mut self,
        contract_id: &AccountId,
        token_id: &FractionationId,
        entries: &Vec<TokenId>,
        owner_id: &AccountId
    ) -> Fractionation {
        let id = contract_token_id(&contract_id, &token_id);

        let fractionation_by_id = &mut self.fractionation_by_id;

        assert_eq!(fractionation_by_id.contains_key(&id), false, "Fractionation already exists");

        self.fractionations_owners.insert(&id, &owner_id);

        let fractionation_tokens = UnorderedSet::new(StorageKey::FractionationTokensInner {
            token_hash: env::sha256(id.as_bytes()),
        });

        fractionation_by_id.insert(&id, &fractionation_tokens);
        let fractionation_id = token_id.clone();

        let mut fractionations = self.fractionations_by_contract
            .get(&contract_id)
            .unwrap_or_else(|| {
                UnorderedSet::new(StorageKey::FractionationsPerContractInner {
                    token_hash: env::sha256(fractionation_id.as_bytes()),
                })
            });

        fractionations.insert(&fractionation_id);
        self.fractionations_by_contract.insert(&contract_id, &fractionations);

        entries.iter().for_each(|token_id| {
            self.internal_add_token_to_fractionation(&contract_id, &fractionation_id, &token_id);
        });

        (FractionationCreate {
            token_id: &token_id,
            entries: &entries,
            contract_id: &contract_id,
        }).emit();

        Fractionation {
            contract_id: contract_id.clone(),
            token_id: token_id.clone(),
            entries: entries.clone(),            
        }
    }

    pub fn internal_call_nft_transfer(
        &self,
        nft_contract_id: &AccountId,
        token_id: &TokenId,
        receiver_id: &AccountId
    ) {
        ext_nft
            ::ext(nft_contract_id.clone())
            .with_static_gas(GAS_FOR_NFT_TRANSFER)
            .with_attached_deposit(1)
            .nft_transfer(
                receiver_id.clone(),
                token_id.clone(),
                None,
                Some("Received by fractionation".to_string())
            );        
    }

    pub fn internal_add_token_to_user(
        &mut self,
        contract_id: &AccountId,
        token_id: &TokenId,
        account_id: &AccountId
    ) {
        let mut owner_contracts = self.tokens_per_owner.get(&account_id).unwrap_or_else(|| {
            TreeMap::new(StorageKey::FractionationContractsPerOwnerInner {
                account_hash: env::sha256(account_id.as_bytes()),
            })
        });
        let mut tokens = owner_contracts.get(&contract_id).unwrap_or_else(|| {
            UnorderedSet::new(StorageKey::FractionationTokensPerContractInner {
                contract_hash: env::sha256(contract_id.as_bytes()),
            })
        });

        tokens.insert(&token_id);
        owner_contracts.insert(&contract_id, &tokens);

        self.tokens_per_owner.insert(&account_id, &owner_contracts);
    }    

    pub fn internal_on_nft_transfer(
        &mut self,
        args: &FractionationNftOnTransferArgs,
        contract_id: &AccountId,
        token_id: &TokenId,
        owner_id: &AccountId
    ) {
        let FractionationNftOnTransferArgs { fractionation_tokens } = args;

        if let Some(fractionation_tokens) = fractionation_tokens {
            self.internal_create_fractionation(
                &contract_id,
                &token_id,
                &fractionation_tokens,
                &owner_id
            );
        } else {
            self.internal_add_token_to_user(&contract_id, &token_id, &owner_id);
            
            (FractionationProcess {
                token_id: &token_id,
                contract_id: &contract_id,                
                account_id: &owner_id,
            }).emit();
        }
    }
}
