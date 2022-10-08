use near_sdk::{ AccountId, env };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use crate::owner::{ ContractOwner, ContractOwnerTransfer };

pub const OWNER_KEY: &[u8; 7] = b"__OWNER";

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OwnerFeature {
    owner_id: AccountId,
}

impl OwnerFeature {
    pub fn new(owner_id: AccountId) -> Self {
        let mut this = Self {
            owner_id: owner_id.clone(),
        };

        this.internal_set_owner(&owner_id);

        this
    }

    pub(crate) fn internal_set_owner(&mut self, account_id: &AccountId) -> AccountId {
        self.assert_owner();

        env::storage_write(OWNER_KEY, &account_id.try_to_vec().expect("INTERNAL_FAIL"));

        self.internal_get_owner()
    }

    pub fn assert_owner(&self) {
        if self.internal_get_owner() != env::predecessor_account_id() {
            env::panic_str("Access Denied");
        }
    }

    pub fn internal_get_owner(&self) -> AccountId {
        env::storage_read(OWNER_KEY)
            .map(|value| AccountId::try_from_slice(&value).unwrap())
            .unwrap_or_else(|| env::current_account_id())
    }
}

impl ContractOwner for OwnerFeature {
    fn get_owner(&self) -> AccountId {
        self.internal_get_owner()
    }
}

impl ContractOwnerTransfer for OwnerFeature {
    fn set_owner(&mut self, account_id: AccountId) -> AccountId {
        self.internal_set_owner(&account_id)
    }
}
