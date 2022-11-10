// Reputation

#[macro_export]
macro_rules! impl_reputation_feature {
    ($contract:ident, $market:ident) => {
        use $crate::reputation::{ ContractReputation };

        #[near_bindgen]
        impl ContractReputation for $contract {        
          fn reputation(&self, account_id: AccountId) -> u32 {
            self.$market.reputation.as_ref().expect("Reputation is not implemented in contract").reputation(account_id)
          }
        }
    };
}
