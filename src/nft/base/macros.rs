// Core

#[macro_export]
macro_rules! impl_non_fungible_token_core {
    ($contract:ident, $token:ident $(, $assert_transfer:ident)?) => {
        use $crate::nft::base::NonFungibleTokenCore;
        use $crate::nft::base::NonFungibleTokenResolver;

        #[near_bindgen]
        impl NonFungibleTokenCore for $contract {
            #[payable]
            fn nft_transfer(
                &mut self,
                receiver_id: AccountId,
                token_id: mfight_sdk::nft::TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
            ) {
                $(self.$assert_transfer();)?
                self.$token.nft_transfer(receiver_id, token_id, approval_id, memo)
            }

            #[payable]
            fn nft_transfer_call(
                &mut self,
                receiver_id: AccountId,
                token_id: mfight_sdk::nft::TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
                msg: String,
            ) -> PromiseOrValue<bool> {
                $(self.$assert_transfer();)?
                self.$token.nft_transfer_call(receiver_id, token_id, approval_id, memo, msg)
            }

            fn nft_token(&self, token_id: mfight_sdk::nft::TokenId) -> Option<mfight_sdk::nft::Token> {
                self.$token.nft_token(token_id)
            }
        }

        #[near_bindgen]
        impl NonFungibleTokenResolver for $contract {
            #[private]
            fn nft_resolve_transfer(
                &mut self,
                previous_owner_id: AccountId,
                receiver_id: AccountId,
                token_id: mfight_sdk::nft::TokenId,
                approved_account_ids: Option<std::collections::HashMap<AccountId, u64>>,
            ) -> bool {
                self.$token.nft_resolve_transfer(
                    previous_owner_id,
                    receiver_id,
                    token_id,
                    approved_account_ids,
                )
            }
        }
    };
}
