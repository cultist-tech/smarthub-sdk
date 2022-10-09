use near_sdk::{ AccountId, env, IntoStorageKey, Balance, Promise, assert_one_yocto };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::{ LookupMap };
use near_sdk::json_types::{ U128 };
use crate::storage::{ StorageBalance, StorageCore };

#[derive(BorshDeserialize, BorshSerialize)]
pub struct StorageFeature {
    cost: u128,
    amount_by_account: LookupMap<AccountId, u128>,
    used_by_account: LookupMap<AccountId, u128>,
}

impl StorageFeature {
    pub fn new<T, T1>(cost: u128, prefix_used: T, prefix_amount: T1) -> Self
        where T: IntoStorageKey, T1: IntoStorageKey
    {
        Self {
            cost,
            amount_by_account: LookupMap::new(prefix_amount),
            used_by_account: LookupMap::new(prefix_used),
        }
    }

    fn internal_balance_of(&self, account_id: &AccountId) -> StorageBalance {
        let amount = self.amount_by_account.get(account_id).unwrap_or_else(|| 0);
        let used = self.used_by_account.get(&account_id).unwrap_or_else(|| 0);

        StorageBalance {
            total: U128::from(amount.clone()),
            available: U128::from(amount - used),
        }
    }

    pub fn internal_available(&self, account_id: &AccountId) -> u128 {
        let amount = self.amount_by_account.get(account_id).unwrap_or_else(|| 0);
        let used = self.used_by_account.get(&account_id).unwrap_or_else(|| 0);

        amount - used
    }

    pub fn internal_decrease(&mut self, account_id: &AccountId, value: u128) {
        let used = self.used_by_account.get(account_id).unwrap_or_else(|| 0);

        if used >= value {
            self.used_by_account.insert(&account_id, &(used - value));
        }
    }

    pub fn internal_increase(&mut self, account_id: &AccountId, value: u128) {
        let amount = self.amount_by_account.get(account_id).unwrap_or_else(|| 0);
        let used = self.used_by_account.get(account_id).unwrap_or_else(|| 0);

        let next = used + value;

        assert!(amount >= next, "Storage is too small");

        self.used_by_account.insert(&account_id, &next);
    }

    pub fn internal_update_cost_unguarded(&mut self, cost: u128) {
        self.cost = cost;
    }
}

impl StorageCore for StorageFeature {
    // `registration_only` doesn't affect the implementation for vanilla fungible token.
    #[allow(unused_variables)]
    fn storage_deposit(&mut self, account_id: Option<AccountId>) -> StorageBalance {
        let amount: Balance = env::attached_deposit();
        let account_id = account_id.unwrap_or_else(env::predecessor_account_id);

        let balance: u128 = self.amount_by_account
            .get(&account_id)
            .unwrap_or_else(|| u128::from(0 as u64));

        self.amount_by_account.insert(&account_id, &(balance + amount));

        self.internal_balance_of(&account_id)
    }

    /// While storage_withdraw normally allows the caller to retrieve `available` balance, the basic
    /// Fungible Token implementation sets storage_balance_bounds.min == storage_balance_bounds.max,
    /// which means available balance will always be 0. So this implementation:
    /// * panics if `amount > 0`
    /// * never transfers Ⓝ to caller
    /// * returns a `storage_balance` struct if `amount` is 0
    fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();

        let balance = self.internal_balance_of(&account_id);
        let amount_to_withdraw: U128 = amount.unwrap_or_else(|| balance.available);

        assert!(amount_to_withdraw <= balance.available, "To large amount to withdraw");

        self.amount_by_account.insert(&account_id, &(balance.total.0 - amount_to_withdraw.0));
        Promise::new(env::predecessor_account_id()).transfer(amount_to_withdraw.0);

        self.internal_balance_of(&account_id)
    }

    fn storage_balance_of(&self, account_id: AccountId) -> StorageBalance {
        self.internal_balance_of(&account_id)
    }

    fn storage_cost(&self) -> u128 {
        self.cost
    }
}
