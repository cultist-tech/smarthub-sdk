// Reputation

#[macro_export]
macro_rules! impl_reputation_feature {
    ($contract:ident, $market:ident, $reputation:ident) => {
        use $crate::reputation::{ ContractReputation };

        #[near_bindgen]
        impl ContractReputation for $contract {        
          fn reputation(&self, account_id: AccountId) -> u32 {
            self.$market.$reputation.as_ref().expect("Reputation is not implemented in contract").reputation(account_id)
          }
        }
    };
}

#[macro_export]
macro_rules! impl_reputation_sharing {
    ($contract:ident, $market:ident, $reputation:ident) => {
        use $crate::reputation::{ ReputationSharing };

        #[near_bindgen]
        impl ReputationSharing for $contract {        
          fn reputation_share(&mut self, receiver_id: AccountId, amount: u32) -> u32 {
            self.$market.$reputation.as_mut().expect("Reputation sharing is not implemented in contract").reputation_share(receiver_id, amount)
          }
          
          fn reputation_shares_left(&self, account_id: AccountId) -> u32 {
            self.$market.$reputation.as_ref().expect("Reputation sharing is not implemented in contract").reputation_shares_left(account_id)
          }
        }
    };
}
