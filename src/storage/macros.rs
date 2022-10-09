/// Ensures that when fungible token storage grows by collections adding entries,
/// the storage is be paid by the caller. This ensures that storage cannot grow to a point
/// that the FT contract runs out of Ⓝ.
/// Takes name of the Contract struct, the inner field for the token and optional method name to
/// call when the account was closed.
#[macro_export]
macro_rules! impl_contract_storage {
    ($contract:ident, $token:ident) => {
        use $crate::storage::{
            StorageCore
        };

        #[near_bindgen]
        impl StorageCore for $contract {
            #[payable]
            fn storage_deposit(
                &mut self,
                account_id: Option<AccountId>,
            ) -> $crate::storage::StorageBalance {
                self.$token.storage_deposit(account_id)
            }

            #[payable]
            fn storage_withdraw(&mut self, amount: Option<U128>) -> $crate::storage::StorageBalance {
                self.$token.storage_withdraw(amount)
            }

            fn storage_balance_of(&self, account_id: AccountId) -> $crate::storage::StorageBalance {
                self.$token.storage_balance_of(account_id)
            }

             fn storage_cost(&self) -> u128 {
                self.$token.storage_cost()
            }
        }
    };
}
