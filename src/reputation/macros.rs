// Reputation

#[macro_export]
macro_rules! impl_reputation_feature {
    ($contract:ident, $reputations:ident) => {
        use $crate::reputation::{ ContractReputation };

        #[near_bindgen]
        impl ContractReputation for $contract {        
          fn reputation(&self, account_id: AccountId) -> u32 {
            self.$reputations.reputation(account_id)
          }
        }
    };
}
