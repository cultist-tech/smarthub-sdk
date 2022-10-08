// Pause

#[macro_export]
macro_rules! impl_whitelist_feature {
    ($contract:ident, $instance:ident, $assert_owner:ident) => {
        use $crate::whitelist::{WhitelistFeatureCore};

        #[near_bindgen]
        impl WhitelistFeatureCore for $contract {
          fn is_whitelist(&self, account_id: AccountId) -> bool {
            self.$instance.is_whitelist(account_id)
          }

        fn whitelist_add(&mut self, account_id: AccountId) -> bool {
            self.$assert_owner();
            self.$instance.whitelist_add(account_id)
          }
        fn whitelist_remove(&mut self, account_id: AccountId) -> bool {
            self.$assert_owner();
            self.$instance.whitelist_remove(account_id)
          }
        }
    };
}