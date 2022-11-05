// Fractionation

#[macro_export]
macro_rules! impl_fractionation_core {
    ($contract:ident, $tokens:ident, $assert_owner:ident) => {
        use $crate::nft_fractionation::{FractionationCore, Fractionation};

        #[near_bindgen]
        impl FractionationCore for $contract {
          fn nft_fractionation(&self, contract_id: AccountId, token_id: FractionationId) -> Option<$crate::nft_fractionation::Fractionation> {
            self.$tokens.nft_fractionation(contract_id, token_id)
          }
          fn nft_fractionation_complete(&mut self, contract_id: AccountId, token_id: FractionationId) {
            self.$tokens.nft_fractionation_complete(contract_id, token_id)
          }
        }
    };
}
