// Fractionation

#[macro_export]
macro_rules! impl_fractionation_enumeration {
    ($contract:ident, $tokens:ident) => {
        use $crate::nft_fractionation::{FractionationEnumeration};

        #[near_bindgen]
        impl FractionationEnumeration for $contract {
          fn nft_fractionations(&self, contract_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<$crate::nft_fractionation::Fractionation> {
            self.$tokens.nft_fractionations(contract_id, from_index, limit)
          }
          fn nft_fractionations_supply(&self, contract_id: AccountId) -> U128 {
            self.$tokens.nft_fractionations_supply(contract_id)
          }
        }
    };
}
