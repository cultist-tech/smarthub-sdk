#[macro_export]
macro_rules! impl_owner_feature {
    ($contract:ident, $instance:ident) => {
        use $crate::owner::{ContractOwner, ContractOwnerTransfer};

        #[near_bindgen]
        impl ContractOwner for $contract {
          fn get_owner(&self) -> AccountId {
            self.$instance.get_owner()
          }
        }

        #[near_bindgen]
        impl ContractOwnerTransfer for $contract {
          fn set_owner(&mut self, account_id: AccountId) -> AccountId {
            self.$instance.set_owner(account_id)
          }
        }
    };
}