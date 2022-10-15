use near_sdk::borsh::{ self, BorshSerialize };
use near_sdk::{
    AccountId,
    env,
    CryptoHash,
    BorshStorageKey,
    Promise,
    is_promise_success,
    ext_contract,
    PromiseOrValue,
};
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::U128;
use crate::rent::{
    RentAdd,
    RentUpdate,
    RentRemove,
    RentFeature,
    contract_token_id,
    Rent,
    hash_account_id,
    TokenId,
    RentPay,
    time_get_minutes,
};
use crate::rent::meta::{ SaleConditions };
use crate::utils::near_ft;
use crate::nft::base::GAS_FOR_NFT_TRANSFER;
use crate::rent::base::GAS_FOR_RENT_PAY;
use crate::nft::base::external::ext_nft;

pub const RENT_TIME_MIN: u64 = 3600000000000; // min 15 min (15 * 60 * 1000 * 1_000_000)
pub const RENT_TIME_NAX: u64 = 8640000000000000; // max 100 days (60 * 60 * 1000 * 100 * 1_000_000)

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    RentTokensPerAccountInner {
        account_id_hash: CryptoHash,
    },
    RentsPerAccountInner {
        account_id_hash: CryptoHash,
    },
    RentsPerContractInner {
        contract_id_hash: CryptoHash,
    },
}

impl RentFeature {
    pub(crate) fn assert_approved(&self, nft_contract_id: &AccountId, token_id: &TokenId) {
        let sender_id = env::predecessor_account_id();
        let id = contract_token_id(&nft_contract_id, &token_id);

        let approve_id = self.approved_owner_by_id
            .as_ref()
            .unwrap()
            .get(&id)
            .expect("Not approved for sender");

        assert_eq!(approve_id, sender_id, "Not approved for sender");
    }

    pub(crate) fn assert_valid_time(&self, time: &u64) {
        assert!(time >= &RENT_TIME_MIN, "minimum 1 hour.");
        assert!(time <= &RENT_TIME_NAX, "maximum 100 days.");
    }

    //

    pub(crate) fn internal_rent_add(
        &mut self,
        nft_contract_id: &AccountId,
        token_id: &TokenId,
        owner_id: &AccountId,
        sale_conditions: &SaleConditions,
        min_time: &u64,
        max_time: &u64
    ) {
        let id = contract_token_id(&nft_contract_id, &token_id);
        let is_paid = self.rents_current.get(&id).is_some();
        let exist = self.rents_by_id.get(&id);

        assert!(!is_paid, "Token is already in rent");
        assert!(exist.is_none(), "Token already on market");

        self.assert_valid_time(&min_time);
        self.assert_valid_time(&max_time);

        let rent = Rent {
            owner_id: owner_id.clone(),
            contract_id: nft_contract_id.clone(),
            token_id: token_id.clone(),
            min_time: min_time.clone(),
            max_time: max_time.clone(),
            created_at: env::block_timestamp(),
            sale_conditions: sale_conditions.clone(),
        };

        self.approved_owner_by_id.as_mut().unwrap().insert(&id, &owner_id);
        self.rents_pending.insert(&id);
        self.rents_by_id.insert(&id, &rent);
        self.internal_add_rent_to_account(&owner_id, &id);
        self.internal_add_rent_to_contract(&nft_contract_id, &token_id);

        (RentAdd {
            token_id: &rent.token_id,
            contract_id: &nft_contract_id,
            owner_id: &rent.owner_id,
            sale_conditions: &rent.sale_conditions,
            min_time: &rent.min_time,
            max_time: &rent.max_time,
            created_at: &rent.created_at,
        }).emit();
    }

    pub(crate) fn internal_rent_update(
        &mut self,
        nft_contract_id: &AccountId,
        token_id: &TokenId,
        account_id: &AccountId,
        ft_token_id: &AccountId,
        price_per_hour: &U128,
        time: &u64,
        max_time: &u64
    ) {
        let is_paid = self.rents_current.get(&token_id).is_some();
        let id = contract_token_id(&nft_contract_id, &token_id);

        self.rents_pending.insert(&id);

        assert!(!is_paid, "Rent already started");

        let mut rent: Rent = self.rents_by_id.get(&id).expect("Not found rent");

        assert_eq!(account_id, &rent.owner_id, "Unauthorized");

        rent.sale_conditions.insert(ft_token_id.clone(), price_per_hour.clone());

        rent.min_time = time.clone();
        rent.max_time = max_time.clone();

        self.rents_by_id.insert(&id, &rent);

        (RentUpdate {
            token_id: &rent.token_id,
            contract_id: &nft_contract_id,
            owner_id: &rent.owner_id,
            ft_token_id: &ft_token_id,
            price: &price_per_hour,
            min_time: &rent.min_time,
            max_time: &rent.max_time,
            created_at: &rent.created_at,
        }).emit();
    }

    pub(crate) fn internal_remove_pending_rent(
        &mut self,
        nft_contract_id: &AccountId,
        token_id: &TokenId,
        account_id: &AccountId
    ) {
        let id = contract_token_id(&nft_contract_id, &token_id);
        let is_paid = self.rents_current.get(&id).is_some();

        assert!(!is_paid, "Token is already in rent");

        let rent = self.rents_by_id.get(&id).expect("Not found rent");

        assert_eq!(rent.owner_id, account_id.clone(), "Only owner can remove rent");

        self.rents_by_id.remove(&id);
        self.rents_pending.remove(&id);
        self.internal_remove_rent_from_account(&account_id, &nft_contract_id, &token_id);
        self.internal_remove_rent_from_contract(&nft_contract_id, &token_id);

        (RentRemove {
            token_id: &token_id,
            contract_id: &nft_contract_id,
            account_id: &account_id,
        }).emit();
    }

    pub(crate) fn internal_add_token_to_account(
        &mut self,
        account_id: &AccountId,
        contract_id: &AccountId,
        token_id: &TokenId
    ) {
      let id = contract_token_id(&contract_id, &token_id);

        let mut tokens_set = self.rent_tokens_per_account.get(&account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                (StorageKey::RentTokensPerAccountInner {
                    account_id_hash: hash_account_id(&account_id),
                })
                    .try_to_vec()
                    .unwrap()
            )
        });
        tokens_set.insert(&id);
        self.rent_tokens_per_account.insert(&account_id, &tokens_set);
    }

    pub(crate) fn internal_remove_token_from_account(
        &mut self,
        account_id: &AccountId,
        nft_contract_id: &AccountId,
        token_id: &TokenId
    ) {
        let id = contract_token_id(&nft_contract_id, &token_id);

        let mut tokens_set = self.rent_tokens_per_account
            .get(&account_id)
            .expect("Token should be owned by the sender");

        tokens_set.remove(&id);

        if tokens_set.is_empty() {
            self.rent_tokens_per_account.remove(&account_id);
        } else {
            self.rent_tokens_per_account.insert(&account_id, &tokens_set);
        }
    }

    //

    pub(crate) fn internal_add_rent_to_account(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId
    ) {
        let mut rents_set = self.rents_per_account.get(account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                (StorageKey::RentsPerAccountInner {
                    account_id_hash: hash_account_id(&account_id),
                })
                    .try_to_vec()
                    .unwrap()
            )
        });
        rents_set.insert(token_id);
        self.rents_per_account.insert(account_id, &rents_set);
    }

    pub(crate) fn internal_add_rent_to_contract(
        &mut self,
        contract_id: &AccountId,
        token_id: &TokenId
    ) {
        let mut rents_set = self.rent_tokens_by_contract.get(contract_id).unwrap_or_else(|| {
            UnorderedSet::new(
                (StorageKey::RentsPerContractInner {
                    contract_id_hash: hash_account_id(&contract_id),
                })
                    .try_to_vec()
                    .unwrap()
            )
        });
        rents_set.insert(token_id);
        self.rent_tokens_by_contract.insert(contract_id, &rents_set);
    }

    pub(crate) fn internal_remove_current_rent(
        &mut self,
        _account_id: &AccountId,
        nft_contract_id: &AccountId,
        token_id: &TokenId
    ) {
        let id = contract_token_id(&nft_contract_id, &token_id);

        self.rents_by_id.remove(&id);
        self.rents_current.remove(&id);

        self.rents_end_by_id.remove(&id);
    }

    pub(crate) fn internal_remove_rent_from_account(
        &mut self,
        account_id: &AccountId,
        nft_contract_id: &AccountId,
        token_id: &TokenId
    ) {
        let id = contract_token_id(&nft_contract_id, &token_id);

        let mut rents_set = self.rents_per_account
            .get(account_id)
            .expect("Rent should be owned by the sender");

        rents_set.remove(&id);

        if rents_set.is_empty() {
            self.rents_per_account.remove(account_id);
        } else {
            self.rents_per_account.insert(account_id, &rents_set);
        }
    }

    pub(crate) fn internal_remove_rent_from_contract(
        &mut self,
        contract_id: &AccountId,
        token_id: &TokenId
    ) {
        let mut rents_set = self.rent_tokens_by_contract
            .get(contract_id)
            .expect("Not found nft contract");

        rents_set.remove(&token_id);

        if rents_set.is_empty() {
            self.rent_tokens_by_contract.remove(contract_id);
        } else {
            self.rent_tokens_by_contract.insert(contract_id, &rents_set);
        }
    }

    //


}
