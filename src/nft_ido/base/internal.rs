use near_sdk::{ AccountId, env, BorshStorageKey };
use near_sdk::collections::{ UnorderedSet, LookupMap };
use near_sdk::borsh::{ self, BorshSerialize };
use rand::Rng;
use crate::nft_ido::{ NftIdoFeature, TokenId, JsonIdo, ContractIdoId, IdoId };
use crate::nft_ido::utils::{ random_use, contract_token_id };
use crate::nft_ido::events::IdoBuyToken;
use crate::nft::base::external::{ ext_nft };
use crate::nft_ido::base::resolvers::{ ext_self };
use crate::nft::base::{GAS_FOR_NFT_TRANSFER_CALL, GAS_FOR_NFT_TRANSFER, GAS_FOR_RESOLVE_NFT_TRANSFER};

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    IdoTokensInner {
        ido_hash: Vec<u8>,
    },
    MintCounterPerIdo {
        ido_hash: Vec<u8>,
    },
    IdoTokensByContractInner {
        contract_hash: Vec<u8>,
    },
}

impl NftIdoFeature {
    pub(crate) fn assert_ido_not_locked(&self, contract_id: &AccountId, ido_id: &IdoId) {
        let id = contract_token_id(&contract_id, &ido_id);
        let is_available = self.idos_available.contains(&id);

        if !is_available {
            env::panic_str("Ido is locked");
        }
    }

    pub(crate) fn assert_ido_started(&self, contract_id: &AccountId, ido_id: &IdoId) {
        let id = contract_token_id(&contract_id, &ido_id);

        let date = self.ido_date_by_id.get(&id).expect("Not found ido");
        let now = env::block_timestamp();

        if &now < date {
            env::panic_str("Ido is not started");
        }
    }

    pub(crate) fn assert_ido_not_started(&self, contract_id: &AccountId, ido_id: &IdoId) {
        let id = contract_token_id(&contract_id, &ido_id);

        let date = self.ido_date_by_id.get(&id);

        if let Some(date) = date {
            let now = env::block_timestamp();

            if &now >= date {
                env::panic_str("Ido is already started");
            }
        }
    }

    pub(crate) fn enum_get_ido(&self, id: &ContractIdoId) -> Option<JsonIdo> {
        let ido = self.ido_by_id.get(id).expect("Not found ido");
        let locked = !self.idos_available.contains(id);
        let date = self.ido_date_by_id.get(id);
        let ft_token = self.ido_by_ft_token.get(id);
        let rand_tokens: Option<Vec<TokenId>> = self.ido_random_tokens.get(id);

        let mut not_minted: u64 = if date.is_some() {
          rand_tokens.as_ref().unwrap().len() as u64
        } else {
          ido.amount.clone()
        };

        let amount_ready = if let Some(rand_tokens) = &rand_tokens {
            rand_tokens.len() as u64
        } else {
            0
        };

        Some(JsonIdo {
            ido_id: ido.ido_id.clone(),
            contract_id: ido.contract_id.clone(),
            name: ido.name.clone(),
            media: ido.media.clone(),
            price: ido.price.clone(),
            buy_max: ido.buy_max,
            per_transaction_min: ido.per_transaction_min,
            per_transaction_max: ido.per_transaction_max,
            amount: ido.amount,
            amount_ready,
            not_minted,
            locked,
            start_date: date.cloned(),
            ft_token,
        })
    }

    pub fn internal_nft_ido_buy_ft(
        &mut self,
        sender_id: &AccountId,
        contract_id: &AccountId,
        ido_id: &IdoId,
        amount: u64,
        receiver_id: &AccountId,
        deposit: &u128
    ) {
        let id = contract_token_id(&contract_id, &ido_id);
        let is_available = self.idos_available.contains(&ido_id);

        if !is_available {
            env::panic_str(&"Ido is locked");
        }

        if self.ido_by_ft_token.get(&id).is_some() {
            env::panic_str("Ido only by FT");
        }

        let ido = self.ido_by_id.get(&id).expect("Not found ido");
        let price = ido.price.0;

        assert!(deposit.clone() >= price * (amount as u128), "Invalid attached deposit");

        self.internal_random_mint(&sender_id, &contract_id, &ido_id, &receiver_id, &amount)
    }

    pub fn internal_ido_add_token(
        &mut self,
        contract_id: &AccountId,
        ido_id: &IdoId,
        token_id: &TokenId
    ) {
        let id = contract_token_id(&contract_id, &ido_id);
        let ido = self.ido_by_id.get(&id).expect("Not found ido");
        let caller_id = env::signer_account_id();

        assert_eq!(&ido.contract_id, &caller_id, "Access denied");
        self.assert_ido_not_started(&contract_id, &ido_id);

        let ido_random = &mut self.ido_random_tokens.get(&id).unwrap_or_else(|| { vec![] });

        let ido_tokens = &mut self.ido_tokens.get(&id).unwrap_or_else(|| {
            UnorderedSet::new(StorageKey::IdoTokensInner {
                ido_hash: env::sha256(id.as_bytes()),
            })
        });

        assert!(ido_tokens.len() < ido.amount, "All nft already transferred");
        assert!(ido_random.len() < (ido.amount as usize), "All nft already transferred");

        ido_tokens.insert(&token_id);
        ido_random.push(token_id.clone());

        self.ido_tokens.insert(&id, &ido_tokens);
        self.ido_random_tokens.insert(&id, &ido_random);
        self.ido_by_token.insert(&token_id, &id);
    }

    pub(crate) fn internal_mint_counter_change(
        &mut self,
        contract_id: &AccountId,
        owner_id: &AccountId,
        ido_id: &IdoId,
        value: &u64
    ) {
        let id = contract_token_id(&contract_id, &ido_id);

        let mint_counter = &mut self.ido_mint_counter;

        let mut ido_accounts = mint_counter.get(&id).unwrap_or_else(|| {
            LookupMap::new(StorageKey::MintCounterPerIdo {
                ido_hash: env::sha256(id.as_bytes()),
            })
        });
        ido_accounts.insert(&owner_id, &value);
        mint_counter.insert(&id, &ido_accounts);
    }

    pub(crate) fn internal_mint_counter_by_ido(
        &self,
        owner_id: &AccountId,
        contract_ido_id: &ContractIdoId
    ) -> u64 {
        let ido_accounts = self.ido_mint_counter.get(&contract_ido_id).unwrap_or_else(||
            LookupMap::new(StorageKey::MintCounterPerIdo {
                ido_hash: env::sha256(contract_ido_id.as_bytes()),
            })
        );

        ido_accounts.get(&owner_id).unwrap_or_else(|| 0)
    }

    pub(crate) fn internal_random_tokens(
        &mut self,
        contract_id: &AccountId,
        ido_id: &IdoId,
        amount: &u64
    ) -> Vec<TokenId> {
        let id = contract_token_id(&contract_id, &ido_id);
        let mut random_tokens = self.ido_random_tokens.get(&id).expect("Not found ido");

        let mut index = 0;
        let mut tokens: Vec<TokenId> = Vec::new();
        let mut rnd = random_use();

        loop {
            if &index == amount {
                break;
            }

            let rand_index = rnd.gen_range(0, random_tokens.len().clone());
            let token_id = random_tokens.get(rand_index).expect("Invalid token index").clone();

            random_tokens.remove(rand_index);
            self.ido_random_tokens.insert(&id, &random_tokens);

            tokens.push(token_id);
            index = index + 1;
        }

        tokens
    }

    pub(crate) fn internal_random_mint(
        &mut self,
        sender_id: &AccountId,
        contract_id: &AccountId,
        ido_id: &IdoId,
        receiver_id: &AccountId,
        amount: &u64
    ) {
        let id = contract_token_id(&contract_id, &ido_id);
        let _amount = amount.clone() as u64;

        self.assert_ido_not_locked(&contract_id, &ido_id);
        self.assert_ido_started(&contract_id, &ido_id);

        let ido = self.ido_by_id.get(&id).expect("Not found ido");
        let buy_max = ido.buy_max;
        let per_transaction_min = ido.per_transaction_min;
        let per_transaction_max = ido.per_transaction_max;

        let rest_amount = self.ido_random_tokens.get(&id).expect("Not found ido").len() as u64;
        let owner_minted = self.internal_mint_counter_by_ido(&receiver_id, &id);

        if _amount > rest_amount {
            env::panic_str("Insufficient amount of nft");
        }
        assert!(owner_minted + _amount <= buy_max, "Mint limit");
        assert!(_amount <= per_transaction_max, "Invalid mint max amount");
        assert!(_amount >= per_transaction_min, "Invalid mint min amount");

        let tokens = self.internal_random_tokens(&contract_id, &ido_id, &_amount);

        tokens.iter().for_each(|token_id| {
            self.internal_call_nft_transfer(
                &sender_id,
                &contract_id,
                &token_id,
                &receiver_id,
                &ido_id
            );
            
            (IdoBuyToken {
                ido_id: &ido_id,
                contract_id: &contract_id,
                token_id: &token_id,
                receiver_id: &receiver_id,                
            }).emit();
        });

        let next_minted = u64::from(owner_minted + _amount);
        self.internal_mint_counter_change(&contract_id, &receiver_id, ido_id, &next_minted);
    }

    pub fn internal_call_nft_transfer(
        &self,
        sender_id: &AccountId,
        contract_id: &AccountId,
        token_id: &TokenId,
        receiver_id: &AccountId,
        ido_id: &IdoId
    ) {
        ext_nft
            ::ext(contract_id.clone())
            .with_static_gas(GAS_FOR_NFT_TRANSFER)
            .with_attached_deposit(1)
            .nft_transfer(
                receiver_id.clone(),
                token_id.clone(),
                None,
                Some("First buy".to_string())
            )
            .then(
                ext_self
                    ::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_RESOLVE_NFT_TRANSFER)
                    .resolve_nft_transfer(
                        sender_id.clone(),
                        receiver_id.clone(),
                        token_id.clone(),
                        ido_id.clone(),
                        contract_id.clone()
                    )
            );

        // ext_nft::nft_transfer(
        //   receiver_id.clone(),
        //   token_id.clone(),
        //   None,
        //   Some("First buy".to_string()),
        //
        //   contract_id.clone(),
        //   1,
        //   GAS_FOR_NFT_TRANSFER_CALL,
        // ).then(ext_self::resolve_nft_transfer(
        //   sender_id.clone(),
        //   receiver_id.clone(),
        //   token_id.clone(),
        //   ido_id.clone(),
        //   contract_id.clone(),
        //
        //   env::current_account_id(),
        //   NO_DEPOSIT,
        //   env::prepaid_gas() - GAS_FOR_NFT_TRANSFER_CALL,
        // ));
    }
}
