use crate::mt::{ MultiFungibleTokenCore, MtTransfer, MultiFungibleTokenResolver };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::{ LookupMap, TreeMap };
use near_sdk::json_types::U128;
use near_sdk::{
    BorshStorageKey,
    env,
    ext_contract,
    log,
    require,
    AccountId,
    Balance,
    Gas,
    IntoStorageKey,
    PromiseOrValue,
    PromiseResult,
    StorageUsage,
};
use crate::mt::utils::assert_at_least_one_yocto;
use crate::mt::base::metadata::{ MtToken };
use crate::mt::error::MtError;

const GAS_FOR_MT_RESOLVE_TRANSFER: Gas = Gas(5_000_000_000_000);
const GAS_FOR_FT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_MT_RESOLVE_TRANSFER.0);

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    AccountsPerToken {
        token_id: Vec<u8>,
    },
}

#[ext_contract(ext_self)]
pub trait ExtMultiFungibleTokenResolver {
    fn mt_resolve_transfer(
        &mut self,
        token_ids: Vec<AccountId>,
        sender_id: AccountId,
        receiver_id: AccountId,
        amounts: Vec<U128>
    ) -> Vec<U128>;
}

#[ext_contract(ext_fungible_token_receiver)]
pub trait MultiFungibleTokenReceiver {
    fn mt_on_transfer(
        &mut self,
        sender_id: AccountId,
        token_ids: Vec<AccountId>,
        amounts: Vec<U128>,
        msg: String
    ) -> PromiseOrValue<Vec<U128>>;
}

/// Implementation of a MultiFungibleToken standard.
/// Allows to include NEP-141 compatible token to any contract.
/// There are next traits that any contract may implement:
///     - MultiFungibleTokenCore -- interface with mt_transfer methods. MultiFungibleToken provides methods for it.
///     - MultiFungibleTokenMetaData -- return metadata for the token in NEP-148, up to contract to implement.
///     - StorageManager -- interface for NEP-145 for allocating storage per account. MultiFungibleToken provides methods for it.
///     - AccountRegistrar -- interface for an account to register and unregister
///
/// For example usage, see examples/fungible-token/src/lib.rs.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MultiFungibleToken {
    pub tokens: TreeMap<AccountId, MtToken>,

    /// The storage size in bytes for one account.
    pub account_storage_usage: StorageUsage,
}

impl MultiFungibleToken {
    pub fn new<S>(prefix: S) -> Self where S: IntoStorageKey {
        let mut this = Self {
            tokens: TreeMap::new(prefix),
            account_storage_usage: 0,
        };

        this.measure_account_storage_usage();
        this
    }

    pub(crate) fn internal_get_token(&self, token_id: &AccountId) -> MtToken {
        self.tokens.get(token_id).expect(&MtError::NotFoundToken.to_string())
    }

    fn measure_account_storage_usage(&mut self) {
        let tmp_token_id = AccountId::new_unchecked("b".repeat(64));

        self.tokens.insert(
            &tmp_token_id,
            &(MtToken {
                accounts: LookupMap::new(StorageKey::AccountsPerToken {
                    token_id: env::sha256(tmp_token_id.as_bytes()),
                }),
                total_supply: 0,
            })
        );

        let initial_storage_usage = env::storage_usage();
        let tmp_account_id = AccountId::new_unchecked("a".repeat(64));

        let mut mft = self.tokens.get(&tmp_token_id).expect("Not found token");

        mft.accounts.insert(&tmp_account_id, &0u128);
        self.account_storage_usage = env::storage_usage() - initial_storage_usage;
        mft.accounts.remove(&tmp_account_id);

        self.tokens.remove(&tmp_token_id);
    }

    pub fn internal_unwrap_balance_of(&self, mft: &MtToken, account_id: &AccountId) -> Balance {
        match mft.accounts.get(account_id) {
            Some(balance) => balance,
            None => {
                0
                // env::panic_str(format!("The account {} is not registered", &account_id).as_str())
            }
        }
    }

    pub fn internal_deposit(
        &mut self,
        token_id: &AccountId,
        account_id: &AccountId,
        amount: Balance
    ) {
        let mut mft = self.internal_get_token(token_id);
        let balance = self.internal_unwrap_balance_of(&mft, account_id);

        if let Some(new_balance) = balance.checked_add(amount) {
            mft.accounts.insert(account_id, &new_balance);

            mft.total_supply = mft.total_supply
                .checked_add(amount)
                .unwrap_or_else(|| env::panic_str("Total supply overflow"));

            self.tokens.insert(&token_id, &mft);
        } else {
            env::panic_str("Balance overflow");
        }
    }

    pub fn internal_withdraw(
        &mut self,
        token_id: &AccountId,
        account_id: &AccountId,
        amount: Balance
    ) {
        let mut mft = self.internal_get_token(token_id);
        let balance = self.internal_unwrap_balance_of(&mft, account_id);

        if let Some(new_balance) = balance.checked_sub(amount) {
            mft.accounts.insert(account_id, &new_balance);
            mft.total_supply = mft.total_supply
                .checked_sub(amount)
                .unwrap_or_else(|| env::panic_str("Total supply overflow"));

            self.tokens.insert(&token_id, &mft);
        } else {
            env::panic_str("The account doesn't have enough balance");
        }
    }

    pub fn internal_transfer_batch(
        &mut self,
        token_ids: &Vec<AccountId>,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        amounts: &Vec<U128>,
        memo: Option<String>
    ) {
        for i in 0..token_ids.len() {
            let _token_id = token_ids.get(i).expect("Invalid params");
            let _amount = amounts.get(i).expect("Invalid params");

            self.internal_transfer(&_token_id, &sender_id, &receiver_id, _amount.0, memo.clone());
        }

        (MtTransfer {
            token_ids,
            old_owner_id: sender_id,
            new_owner_id: receiver_id,
            amounts,
            memo: memo.as_deref(),
        }).emit();
    }

    pub fn internal_transfer(
        &mut self,
        token_id: &AccountId,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        amount: Balance,
        _memo: Option<String>
    ) {
        require!(sender_id != receiver_id, "Sender and receiver should be different");
        require!(amount > 0, "The amount should be a positive number");

        self.internal_withdraw(token_id, sender_id, amount);
        self.internal_deposit(token_id, receiver_id, amount);
    }

    pub fn internal_register_account(&mut self, token_id: &AccountId, account_id: &AccountId) {
        let mut mft = self.internal_get_token(token_id);

        if mft.accounts.insert(account_id, &0).is_some() {
            env::panic_str("The account is already registered");
        }
    }

    pub fn internal_on_ft_transfer(
        &mut self,
        ft_token_id: &AccountId,
        sender_id: &AccountId,
        amount: &U128
    ) -> PromiseOrValue<U128> {
        self.internal_deposit(&ft_token_id, &sender_id, amount.0);

        PromiseOrValue::Value(U128::from(0))
    }
}

impl MultiFungibleTokenCore for MultiFungibleToken {
    fn mt_batch_transfer(
        &mut self,
        receiver_id: AccountId,
        token_ids: Vec<AccountId>,
        amounts: Vec<U128>,
        memo: Option<String>
    ) {
        assert_at_least_one_yocto();

        let sender_id = env::predecessor_account_id();

        assert_eq!(token_ids.len(), amounts.len(), "Invalid params");

        self.internal_transfer_batch(&token_ids, &sender_id, &receiver_id, &amounts, memo)
    }

    fn mt_batch_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_ids: Vec<AccountId>,
        amounts: Vec<U128>,
        memo: Option<String>,
        msg: String
    ) -> PromiseOrValue<U128> {
        assert_at_least_one_yocto();

        assert_eq!(token_ids.len(), amounts.len(), "Invalid params");

        require!(
            env::prepaid_gas() > GAS_FOR_FT_TRANSFER_CALL + GAS_FOR_MT_RESOLVE_TRANSFER,
            "More gas is required"
        );

        let sender_id = env::predecessor_account_id();

        self.internal_transfer_batch(&token_ids, &sender_id, &receiver_id, &amounts, memo);

        ext_fungible_token_receiver
            ::ext(receiver_id.clone())
            .with_static_gas(env::prepaid_gas() - GAS_FOR_FT_TRANSFER_CALL)
            .mt_on_transfer(sender_id.clone(), token_ids.clone(), amounts.clone(), msg)
            .then(
                ext_self
                    ::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_MT_RESOLVE_TRANSFER)
                    .mt_resolve_transfer(token_ids.clone(), sender_id, receiver_id, amounts.clone())
            )
            .into()

        // Initiating receiver's call and the callback
        //   ext_fungible_token_receiver::mt_on_transfer(
        //       sender_id.clone(),
        //       token_ids.clone(),
        //       amounts.clone(),
        //       msg,
        //
        //       receiver_id.clone(),
        //       NO_DEPOSIT,
        //       env::prepaid_gas() - GAS_FOR_FT_TRANSFER_CALL,
        //   )
        //   .then(ext_self::mt_resolve_transfer(
        //       token_ids.clone(),
        //       sender_id,
        //       receiver_id,
        //       amounts.clone(),
        //       env::current_account_id(),
        //       NO_DEPOSIT,
        //       GAS_FOR_RESOLVE_TRANSFER,
        //   ))
        //   .into()
    }

    fn mt_total_supply(&self, token_id: AccountId) -> U128 {
        let mft = self.internal_get_token(&token_id);

        mft.total_supply.into()
    }

    fn mt_balance_of(&self, account_id: AccountId, token_id: AccountId) -> U128 {
        let mft = self.internal_get_token(&token_id);

        mft.accounts.get(&account_id).unwrap_or(0).into()
    }

    fn mt_add_token(&mut self, token_id: AccountId) {
        self.tokens.insert(
            &token_id,
            &(MtToken {
                accounts: LookupMap::new(StorageKey::AccountsPerToken {
                    token_id: env::sha256(token_id.as_bytes()),
                }),
                total_supply: 0,
            })
        );
    }
}

impl MultiFungibleToken {
    /// Internal method that returns the amount of burned tokens in a corner case when the sender
    /// has deleted (unregistered) their account while the `mt_transfer_call` was still in flight.
    /// Returns (Used token amount, Burned token amount)
    pub fn internal_mt_resolve_transfer(
        &mut self,
        token_ids: &Vec<AccountId>,
        sender_id: &AccountId,
        receiver_id: AccountId,
        amounts: &Vec<U128>
    ) -> Vec<U128> {
        // let amount: Balance = amount.into();

        let unused_amounts = match env::promise_result(0) {
            PromiseResult::NotReady => env::abort(),
            PromiseResult::Successful(value) => {
                if let Ok(unused_amounts) = near_sdk::serde_json::from_slice::<Vec<U128>>(&value) {
                    // std::cmp::min(_amount, unused_amounts.0);

                    if unused_amounts.len() < amounts.len() {
                        amounts
                            .iter()
                            .map(|_| U128::from(0))
                            .collect()
                    } else {
                        amounts
                            .iter()
                            .enumerate()
                            .map(|(i, amount)| {
                                let zero = U128::from(0);
                                let unused_amount = unused_amounts.get(i).unwrap_or_else(|| &zero);

                                U128::from(std::cmp::min(amount.0, unused_amount.0))
                            })
                            .collect()
                    }
                } else {
                    amounts.clone()
                }
            }
            PromiseResult::Failed => amounts.clone(),
        };

        for i in 0..token_ids.len() {
            let _token_id = token_ids.get(i).expect("Invalid resolve params");
            let _amount = amounts.get(i).expect("Invalid resolve params").0;
            let unused_amount = unused_amounts.get(i).expect("Invalid unused params").0;

            let mut mft = self.internal_get_token(&_token_id);

            // Get the unused amount from the `mt_on_transfer` call result.

            if unused_amount > 0 {
                let receiver_balance = mft.accounts.get(&receiver_id).unwrap_or(0);
                if receiver_balance > 0 {
                    let refund_amount = std::cmp::min(receiver_balance, unused_amount);
                    mft.accounts.insert(&receiver_id, &(receiver_balance - refund_amount));

                    if let Some(sender_balance) = mft.accounts.get(sender_id) {
                        mft.accounts.insert(sender_id, &(sender_balance + refund_amount));
                        log!("Refund {} from {} to {}", refund_amount, receiver_id, sender_id);

                        // return (amount - refund_amount, 0);
                    } else {
                        // Sender's account was deleted, so we need to burn tokens.
                        mft.total_supply -= refund_amount;
                        log!("The account of the sender was deleted");
                        // return (amount, refund_amount);
                    }
                }
            }
        }

        unused_amounts
    }
}

impl MultiFungibleTokenResolver for MultiFungibleToken {
    fn mt_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_ids: Vec<AccountId>,
        amounts: Vec<U128>
    ) -> Vec<U128> {
        let res = self.internal_mt_resolve_transfer(&token_ids, &sender_id, receiver_id, &amounts);

        let result = res
            .iter()
            .map(|el| U128::from(el.0))
            .collect();

        result
    }
}
