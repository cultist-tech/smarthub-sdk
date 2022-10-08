// Pause

#[macro_export]
macro_rules! impl_non_fungible_token_royalty {
    ($contract:ident, $instance:ident, $assert_owner:ident) => {
        use $crate::nft::royalty::{NonFungibleTokenRoyalty};

        #[near_sdk::near_bindgen]
        impl NonFungibleTokenRoyalty for $contract {
          fn set_royalty_value(&mut self, contract_royalty: u32) {
            self.$assert_owner();
            self.$instance.set_royalty_value(contract_royalty)
          }
          fn set_royalty_account(&mut self, account_id: near_sdk::AccountId) -> near_sdk::AccountId {
            self.$assert_owner();
            self.$instance.set_royalty_account(account_id)
          }

          fn nft_royalty_value(&self) -> u32 {
            self.$instance.nft_royalty_value()
          }
          fn nft_royalty_account(&self) -> near_sdk::AccountId {
            self.$instance.nft_royalty_account()
          }
        }
    };
}
