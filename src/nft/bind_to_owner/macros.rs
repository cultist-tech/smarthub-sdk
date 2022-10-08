// Pause

#[macro_export]
macro_rules! impl_non_fungible_token_bind_to_owner {
    ($contract:ident, $instance:ident) => {
        use $crate::nft::bind_to_owner::{NonFungibleTokenBindToOwner};

        #[near_bindgen]
        impl NonFungibleTokenBindToOwner for $contract {
          fn nft_is_bind_to_owner(&self, token_id: String) -> bool {
            self.$instance.nft_is_bind_to_owner(token_id)
          }
        }
    };
}
