use near_sdk::{ env, AccountId, IntoStorageKey, require };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::{ TreeMap, LookupMap, UnorderedSet };
use std::collections::HashMap;
use crate::nft_ido::metadata::{ IdoId, TokenId, Ido };
use crate::nft_ido::{ IdoCore, JsonIdo, IdoEnumeration, ContractIdoId };
use near_sdk::json_types::U128;
use crate::nft_ido::events::{ IdoStart, IdoCreate, IdoUpdate, IdoPause };
use crate::nft_ido::utils::{ contract_token_id, assert_tx_money };
use crate::nft_ido::base::internal::StorageKey;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftIdoFeature {
    pub ido_by_token: LookupMap<ContractIdoId, ContractIdoId>,
    pub ido_tokens: LookupMap<ContractIdoId, UnorderedSet<TokenId>>,
    pub idos_available: UnorderedSet<ContractIdoId>,
    pub ido_by_id: HashMap<ContractIdoId, Ido>,
    pub ido_date_by_id: HashMap<ContractIdoId, u64>,
    pub ido_random_tokens: LookupMap<ContractIdoId, Vec<TokenId>>,
    pub ido_mint_counter: LookupMap<ContractIdoId, LookupMap<AccountId, u64>>,
    pub ido_by_ft_token: LookupMap<ContractIdoId, AccountId>,
    pub ido_tokens_by_contract: TreeMap<AccountId, UnorderedSet<TokenId>>,
}

impl NftIdoFeature {
    pub fn new<S1, S2, S4, S5, S6, S7, S8>(
        ido_by_token_prefix: S1,
        tokens_per_ido_prefix: S2,
        random_tokens_prefix: S4,
        mint_counter_prefix: S5,
        idos_available_prefix: S6,
        ido_by_ft_prefix: S7,
        ido_tokens_by_contract_prefix: S8
    )
        -> Self
        where
            S1: IntoStorageKey,
            S2: IntoStorageKey,
            S4: IntoStorageKey,
            S5: IntoStorageKey,
            S6: IntoStorageKey,
            S7: IntoStorageKey,
            S8: IntoStorageKey
    {
        let this = Self {
            ido_by_token: LookupMap::new(ido_by_token_prefix),
            ido_tokens: LookupMap::new(tokens_per_ido_prefix),
            ido_by_id: HashMap::new(),
            ido_date_by_id: HashMap::new(),
            ido_random_tokens: LookupMap::new(random_tokens_prefix),
            ido_mint_counter: LookupMap::new(mint_counter_prefix),
            idos_available: UnorderedSet::new(idos_available_prefix),
            ido_by_ft_token: LookupMap::new(ido_by_ft_prefix),
            ido_tokens_by_contract: TreeMap::new(ido_tokens_by_contract_prefix),
        };

        this
    }
}

impl IdoCore for NftIdoFeature {
    fn nft_ido_add(
        &mut self,
        contract_id: AccountId,
        ido_id: IdoId,
        name: String,
        amount: u64,
        price: U128,
        per_transaction_min: u64,
        per_transaction_max: u64,
        buy_max: u64,
        ft_token: Option<AccountId>,
        media: Option<String>
    ) -> JsonIdo {
        assert_tx_money();

        assert_eq!(contract_id, env::predecessor_account_id());

        let id = contract_token_id(&contract_id, &ido_id);

        // TODO нужно убрать ограничение в 1 нфт, не срочная задача
        assert_eq!(per_transaction_min, 1, "Only one nft per tx");
        assert_eq!(per_transaction_max, 1, "Only one nft per tx");
        assert!(
            per_transaction_min <= amount && per_transaction_max <= amount && buy_max <= amount,
            "Invalid amount"
        );

        let ido = Ido {
            ido_id: ido_id.clone(),
            contract_id: contract_id.clone(),
            name,
            amount,
            price,
            per_transaction_min,
            per_transaction_max,
            buy_max,
            media,
        };
        assert!(self.ido_by_id.insert(id.clone(), ido).is_none(), "Ido already exists");

        if let Some(ft_token) = ft_token {
            self.ido_by_ft_token.insert(&id, &ft_token);
        }

        let mut contract_idos = self.ido_tokens_by_contract.get(&contract_id).unwrap_or_else(|| {
            UnorderedSet::new(StorageKey::IdoTokensByContractInner {
                contract_hash: env::sha256(contract_id.as_bytes()),
            })
        });
        contract_idos.insert(&ido_id);
        self.ido_tokens_by_contract.insert(&contract_id, &contract_idos);

        let json_ido = self.enum_get_ido(&id).unwrap();

        (IdoCreate {
            ido: &json_ido,
        }).emit();

        json_ido
    }

    fn nft_ido_start(&mut self, contract_id: AccountId, ido_id: IdoId, date: u64) -> JsonIdo {
        // 18 08 2022
        if date < 1663513608939000000 {
            env::panic_str("&Invalid date");
        }

        assert_eq!(contract_id, env::predecessor_account_id());

        let id = contract_token_id(&contract_id, &ido_id);

        let ido = self.ido_by_id.get(&id).expect("Not found ido");

        assert!(!self.idos_available.contains(&id), "Ido already unlocked");

        let tokens_per_ido = self.ido_tokens.get(&id).expect("Not all of tokens are minted").len();

        assert_eq!(u64::from(ido.amount), tokens_per_ido, "Not all of tokens are minted");

        self.idos_available.insert(&id);
        self.ido_date_by_id.insert(id.clone(), date);

        (IdoStart {
            ido_id: &ido_id,
            contract_id: &contract_id,
            date: &date,
        }).emit();

        self.enum_get_ido(&id).unwrap()
    }

    fn nft_ido_update(
        &mut self,
        contract_id: AccountId,
        ido_id: IdoId,
        date: u64,
        per_transaction_min: u64,
        per_transaction_max: u64,
        buy_max: u64
    ) -> JsonIdo {
        assert_eq!(contract_id, env::predecessor_account_id());

        let id = contract_token_id(&contract_id, &ido_id);

        self.assert_ido_not_started(&contract_id, &ido_id);

        // TODO нужно убрать ограничение в 1 нфт, не срочная задача
        assert_eq!(per_transaction_min, 1, "Only one nft per tx");
        assert_eq!(per_transaction_max, 1, "Only one nft per tx");

        let ido = self.ido_by_id.get(&id).expect("Not found ido");

        self.ido_date_by_id.insert(id.clone(), date);

        let new_ido = Ido {
            ido_id: ido.ido_id.clone(),
            contract_id: ido.contract_id.clone(),
            name: ido.name.clone(),
            amount: ido.amount.clone(),
            price: ido.price.clone(),
            buy_max: buy_max.clone(),
            per_transaction_min: per_transaction_min.clone(),
            per_transaction_max: per_transaction_max.clone(),
            media: ido.media.clone(),
        };

        self.ido_by_id.insert(id.clone(), new_ido);

        (IdoUpdate {
            ido_id: &ido_id,
            contract_id: &contract_id,
            date: &date,
            per_transaction_min: &per_transaction_min,
            per_transaction_max: &per_transaction_max,
            buy_max: &buy_max,
        }).emit();

        self.enum_get_ido(&id).unwrap()
    }

    fn nft_ido_pause(&mut self, contract_id: AccountId, ido_id: IdoId, pause: bool) -> JsonIdo {
        assert_eq!(contract_id, env::predecessor_account_id());

        let id = contract_token_id(&contract_id, &ido_id);

        let ido = self.enum_get_ido(&id).unwrap();

        if pause {
            self.idos_available.remove(&id);
        } else {
            self.idos_available.insert(&id);
        }

        (IdoPause {
            ido_id: &ido_id,
            contract_id: &contract_id,
            pause: &pause,
        }).emit();

        ido
    }

    fn nft_ido_buy(
        &mut self,
        contract_id: AccountId,
        receiver_id: AccountId,
        ido_id: IdoId,
        amount: u64
    ) {
        let sender_id = env::predecessor_account_id();
        let id = contract_token_id(&contract_id, &ido_id);

        let is_available = self.idos_available.contains(&id);

        if !is_available {
            env::panic_str(&"Ido is locked");
        }

        if self.ido_by_ft_token.get(&id).is_some() {
            env::panic_str("Ido only by FT");
        }

        let ido = self.ido_by_id.get(&id).expect("Not found ido");
        let deposit = env::attached_deposit();
        let price = ido.price.0;

        assert!(deposit >= price * (amount as u128), "Invalid attached deposit");

        self.internal_random_mint(&sender_id, &contract_id, &ido_id, &receiver_id, &amount)
    }
}

impl IdoEnumeration for NftIdoFeature {
    fn nft_idos(&self) -> Vec<JsonIdo> {
        self.idos_available
            .iter()
            .map(|contract_ido_id| { self.enum_get_ido(&contract_ido_id).unwrap() })
            .collect()
    }

    fn nft_ido(&self, contract_id: AccountId, ido_id: IdoId) -> Option<JsonIdo> {
        let id = contract_token_id(&contract_id, &ido_id);

        self.enum_get_ido(&id)
    }

    fn nft_ido_not_minted(&self, contract_id: AccountId, ido_id: IdoId) -> u64 {
        let id = contract_token_id(&contract_id, &ido_id);

        let ido = self.ido_by_id.get(&id).expect("Not found ido");
        let rand_tokens = self.ido_random_tokens.get(&id);

        if let Some(rand_tokens) = rand_tokens {
            return rand_tokens.len() as u64;
        }

        ido.amount
    }

    fn nft_ido_tokens(
        &self,
        contract_id: AccountId,
        ido_id: IdoId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<TokenId> {
        let id = contract_token_id(&contract_id, &ido_id);

        let tokens_per_ido = &self.ido_tokens;
        let token_set = if let Some(token_set) = tokens_per_ido.get(&id) {
            token_set
        } else {
            return vec![];
        };

        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        require!(limit != 0, "Cannot provide limit of 0.");
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();

        require!(
            (token_set.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );

        token_set.iter().collect()
    }

    fn nft_ido_account_minted(
        &self,
        contract_id: AccountId,
        ido_id: IdoId,
        account_id: AccountId
    ) -> u64 {
        let id = contract_token_id(&contract_id, &ido_id);
        let owner_minted = self.internal_mint_counter_by_ido(&account_id, &id);

        owner_minted
    }

    fn nft_idos_by_contract(
        &self,
        contract_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<JsonIdo> {
        let ids = if let Some(ids) = self.ido_tokens_by_contract.get(&contract_id) {
            ids
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        ids.as_vector()
            .iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .map(|token_id| self.enum_get_ido(&contract_token_id(&contract_id, &token_id)).unwrap())
            .collect()
    }

    fn nft_idos_supply_by_contract(&self, contract_id: AccountId) -> U128 {
        if let Some(list) = self.ido_tokens_by_contract.get(&contract_id) {
            return U128::from(list.len() as u128);
        }

        U128::from(0)
    }
}
