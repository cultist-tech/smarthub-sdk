use crate::mt::MultiFungibleToken;
use crate::mt::storage_management::{ StorageBalance, StorageBalanceBounds, StorageManagement };
use near_sdk::json_types::U128;
use near_sdk::{ assert_one_yocto, env, log, AccountId, Balance, Promise };
use crate::mt::base::metadata::{ MtToken };

impl MultiFungibleToken {
    /// Internal method that returns the Account ID and the balance in case the account was
    /// unregistered.
    pub fn internal_storage_unregister(
        &mut self,
        token_id: AccountId,
        force: Option<bool>
    ) -> Option<(AccountId, Balance)> {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        let force = force.unwrap_or(false);
        let mut mft = self.internal_get_token(&token_id);

        if let Some(balance) = mft.accounts.get(&account_id) {
            if balance == 0 || force {
                mft.accounts.remove(&account_id);
                mft.total_supply -= balance;
                Promise::new(account_id.clone()).transfer(self.storage_balance_bounds().min.0 + 1);
                Some((account_id, balance))
            } else {
                env::panic_str(
                    "Can't unregister the account with the positive balance without force"
                )
            }
        } else {
            log!("The account {} is not registered", &account_id);
            None
        }
    }

    fn internal_storage_balance_of(
        &self,
        mft: &MtToken,
        account_id: &AccountId
    ) -> Option<StorageBalance> {
        if mft.accounts.contains_key(account_id) {
            Some(StorageBalance { total: self.storage_balance_bounds().min, available: (0).into() })
        } else {
            None
        }
    }
}

impl StorageManagement for MultiFungibleToken {
    // `registration_only` doesn't affect the implementation for vanilla fungible token.
    #[allow(unused_variables)]
    fn storage_deposit(
        &mut self,
        token_id: AccountId,
        account_id: Option<AccountId>,
        registration_only: Option<bool>
    ) -> StorageBalance {
        let amount: Balance = env::attached_deposit();
        let account_id = account_id.unwrap_or_else(env::predecessor_account_id);

        let mft = self.internal_get_token(&token_id);

        if mft.accounts.contains_key(&account_id) {
            log!("The account is already registered, refunding the deposit");
            if amount > 0 {
                Promise::new(env::predecessor_account_id()).transfer(amount);
            }
        } else {
            let min_balance = self.storage_balance_bounds().min.0;
            if amount < min_balance {
                env::panic_str("The attached deposit is less than the minimum storage balance");
            }

            self.internal_register_account(&token_id, &account_id);
            let refund = amount - min_balance;
            if refund > 0 {
                Promise::new(env::predecessor_account_id()).transfer(refund);
            }
        }

        self.internal_storage_balance_of(&mft, &account_id).unwrap()
    }

    /// While storage_withdraw normally allows the caller to retrieve `available` balance, the basic
    /// MultiFungible Token implementation sets storage_balance_bounds.min == storage_balance_bounds.max,
    /// which means available balance will always be 0. So this implementation:
    /// * panics if `amount > 0`
    /// * never transfers Ⓝ to caller
    /// * returns a `storage_balance` struct if `amount` is 0
    fn storage_withdraw(&mut self, token_id: AccountId, amount: Option<U128>) -> StorageBalance {
        assert_one_yocto();

        let mft = self.internal_get_token(&token_id);

        let predecessor_account_id = env::predecessor_account_id();

        if
            let Some(storage_balance) = self.internal_storage_balance_of(
                &mft,
                &predecessor_account_id
            )
        {
            match amount {
                Some(amount) if amount.0 > 0 => {
                    env::panic_str("The amount is greater than the available storage balance");
                }
                _ => storage_balance,
            }
        } else {
            env::panic_str(
                format!("The account {} is not registered", &predecessor_account_id).as_str()
            );
        }
    }

    fn storage_unregister(&mut self, token_id: AccountId, force: Option<bool>) -> bool {
        self.internal_storage_unregister(token_id, force).is_some()
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        let required_storage_balance =
            Balance::from(self.account_storage_usage) * env::storage_byte_cost();
        StorageBalanceBounds {
            min: required_storage_balance.into(),
            max: Some(required_storage_balance.into()),
        }
    }

    fn storage_balance_of(
        &self,
        token_id: AccountId,
        account_id: AccountId
    ) -> Option<StorageBalance> {
        let mft = self.internal_get_token(&token_id);

        self.internal_storage_balance_of(&mft, &account_id)
    }
}