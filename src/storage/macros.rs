/// Ensures that when fungible token storage grows by collections adding entries,
/// the storage is be paid by the caller. This ensures that storage cannot grow to a point
/// that the FT contract runs out of Ⓝ.
/// Takes name of the Contract struct, the inner field for the token and optional method name to
/// call when the account was closed.
#[macro_export]
macro_rules! impl_contract_storage {
    ($contract:ident, $token:ident $(, $on_account_closed_fn:ident)?) => {
        use $crate::storage::{
            StorageManagement, StorageBalance, StorageBalanceBounds
        };

        #[near_bindgen]
        impl StorageManagement for $contract {
            #[payable]
            fn storage_deposit(
                &mut self,
                account_id: Option<AccountId>,
                registration_only: Option<bool>,
            ) -> StorageBalance {
                self.$token.storage_deposit(account_id, registration_only)
            }

            #[payable]
            fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
                self.$token.storage_withdraw(amount)
            }

            // #[payable]
            // fn storage_unregister(&mut self, force: Option<bool>) -> bool {
            //     #[allow(unused_variables)]
            //     if let Some((account_id, balance)) = self.$token.internal_storage_unregister(force) {
            //         // $(self.$on_account_closed_fn(account_id, balance);)?
            //         true
            //     } else {
            //         false
            //     }
            // }

            // fn storage_balance_bounds(&self) -> StorageBalanceBounds {
            //     self.$token.storage_balance_bounds()
            // }

            fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
                self.$token.storage_balance_of(account_id)
            }
        }
    };
}
