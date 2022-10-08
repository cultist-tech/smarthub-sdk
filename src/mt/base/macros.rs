/// The core methods for a basic fungible token. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_multi_fungible_token_core {
    ($contract:ident, $token:ident, $assert_owner:ident, $assert_transfer:ident) => {
        use $crate::mt::base::{MultiFungibleTokenCore, MultiFungibleTokenResolver};

        #[near_bindgen]
        impl MultiFungibleTokenCore for $contract {
            #[payable]
            fn mt_batch_transfer(&mut self, receiver_id: AccountId, token_ids: Vec<AccountId>, amounts: Vec<U128>, memo: Option<String>) {
                self.$assert_transfer();
                self.$token.mt_batch_transfer(receiver_id, token_ids, amounts, memo)
            }

            #[payable]
            fn mt_batch_transfer_call(
                &mut self,
                receiver_id: AccountId,
                token_ids: Vec<AccountId>,
                amounts: Vec<U128>,
                memo: Option<String>,
                msg: String,
            ) -> PromiseOrValue<U128> {
                self.$assert_transfer();
                self.$token.mt_batch_transfer_call(receiver_id, token_ids, amounts, memo, msg)
            }

            fn mt_total_supply(&self, token_id: AccountId) -> U128 {
                self.$token.mt_total_supply(token_id)
            }

            fn mt_balance_of(&self, account_id: AccountId, token_id: AccountId) -> U128 {
                self.$token.mt_balance_of(account_id, token_id)
            }

            fn mt_add_token(
                &mut self,
                token_id: AccountId,
            )  {
                self.$assert_owner();
                self.$token.mt_add_token(token_id)
            }
        }

        #[near_bindgen]
        impl MultiFungibleTokenResolver for $contract {
            #[private]
            fn mt_resolve_transfer(
                &mut self,
                sender_id: AccountId,
                receiver_id: AccountId,
                token_ids: Vec<AccountId>,
                amounts: Vec<U128>,
            ) -> Vec<U128> {
                 let res = self.$token.internal_mt_resolve_transfer(&token_ids, &sender_id, receiver_id, &amounts);

                 res.iter().map(|el| U128::from(el.0)).collect()
            }
        }
    };
}

/// Ensures that when fungible token storage grows by collections adding entries,
/// the storage is be paid by the caller. This ensures that storage cannot grow to a point
/// that the FT contract runs out of Ⓝ.
/// Takes name of the Contract struct, the inner field for the token and optional method name to
/// call when the account was closed.
#[macro_export]
macro_rules! impl_multi_fungible_token_storage {
    ($contract:ident, $token:ident $(, $on_account_closed_fn:ident)?) => {
        use $crate::mt::{
            StorageManagement, StorageBalance, StorageBalanceBounds
        };

        #[near_bindgen]
        impl StorageManagement for $contract {
            #[payable]
            fn storage_deposit(
                &mut self,
                token_id: AccountId,
                account_id: Option<AccountId>,
                registration_only: Option<bool>,
            ) -> StorageBalance {
                self.$token.storage_deposit(token_id, account_id, registration_only)
            }

            #[payable]
            fn storage_withdraw(&mut self, token_id: AccountId, amount: Option<U128>) -> StorageBalance {
                self.$token.storage_withdraw(token_id, amount)
            }

            #[payable]
            fn storage_unregister(&mut self, token_id: AccountId, force: Option<bool>) -> bool {
                #[allow(unused_variables)]
                if let Some((account_id, balance)) = self.$token.internal_storage_unregister(token_id,force) {
                    $(self.$on_account_closed_fn(account_id, balance);)?
                    true
                } else {
                    false
                }
            }

            fn storage_balance_bounds(&self, token_id: AccountId) -> StorageBalanceBounds {
                self.$token.storage_balance_bounds(token_id)
            }

            fn storage_balance_of(&self, token_id: AccountId, account_id: AccountId) -> Option<StorageBalance> {
                self.$token.storage_balance_of(token_id, account_id)
            }
        }
    };
}