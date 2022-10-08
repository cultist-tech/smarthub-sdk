// Pause

#[macro_export]
macro_rules! impl_blacklist_feature {
    ($contract:ident, $instance:ident, $assert_owner:ident) => {
        use $crate::blacklist::{ContractBlacklistCore};

        #[near_bindgen]
        impl ContractBlacklistCore for $contract {
          fn is_blacklist(&self, account_id: AccountId) -> bool {
            self.$instance.is_blacklist(account_id)
          }

        fn blacklist_add(&mut self, account_id: AccountId) -> bool {
            self.$assert_owner();
            self.$instance.blacklist_add(account_id)
          }

        fn blacklist_remove(&mut self, account_id: AccountId) -> bool {
          self.$assert_owner();
          self.$instance.blacklist_remove(account_id)
        }
        }
    };
}