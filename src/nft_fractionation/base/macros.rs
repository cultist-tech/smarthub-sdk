// Fractionation

#[macro_export]
macro_rules! impl_non_fungible_token_fractionation {
    ($contract:ident, $tokens:ident, $assert_owner:ident) => {
        use $crate::nft_fractionation::{NonFungibleTokenFractionation, Fractionation};

        #[near_bindgen]
        impl NonFungibleTokenFractionation for $contract {
          fn nft_fractionation(&self, contract_id: AccountId, token_id: FractionationId) -> Fractionation {
            self.$tokens.nft_fractionation(contract_id, token_id)
          }
          fn nft_fractionations(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Fractionation> {
            self.$tokens.nft_fractionations(from_index, limit)
          }
          fn nft_fractionations_supply(&self) -> U128 {
            self.$tokens.nft_fractionations_supply()
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