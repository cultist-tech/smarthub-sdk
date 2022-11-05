// Fractionation

#[macro_export]
macro_rules! impl_non_fungible_token_fractionation {
    ($contract:ident, $tokens:ident, $assert_owner:ident) => {
        use $crate::nft_fractionation::{NonFungibleTokenFractionation, Fractionation};

        #[near_bindgen]
        impl NonFungibleTokenFractionation for $contract {
          fn nft_fractionation(&self, contract_id: AccountId, token_id: FractionationId) -> Option<Fractionation> {
            self.$tokens.nft_fractionation(contract_id, token_id)
          }
          fn nft_fractionations(&self, contract_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<Fractionation> {
            self.$tokens.nft_fractionations(contract_id, from_index, limit)
          }
          fn nft_fractionations_supply(&self, contract_id: AccountId) -> U128 {
            self.$tokens.nft_fractionations_supply(contract_id)
          }
          fn nft_fractionation_complete(&mut self, contract_id: AccountId, token_id: FractionationId) {
            self.$tokens.nft_fractionation_complete(contract_id, token_id)
          }
          // fn nft_fractionation_create(&mut self, token_id: FractionationId, entries: Vec<TokenId>) -> Fractionation {
          //   self.$assert_owner();
          //   self.$tokens.nft_fractionation_create(token_id, entries)
          // }
        }
    };
}
