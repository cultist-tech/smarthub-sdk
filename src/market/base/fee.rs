use near_sdk::AccountId;
use crate::market::base::{ MarketFeature, MARKET_BASE_FEE, MARKET_REDUCED_FEE, MARKET_MIN_FEE };
use crate::reputation::MAX_REPUTATION;

impl MarketFeature {
    pub fn internal_market_fee(
        &mut self,
        price: &u128,
        account_id: &AccountId,        
    ) -> u128 {
        let mut fee_percent = MARKET_BASE_FEE;  
        
        if self.reputation.is_some() {
            let reputation = self.reputation.as_ref().unwrap().internal_reputation(&account_id);
            
            if reputation > MAX_REPUTATION/2 && reputation != MAX_REPUTATION {
                fee_percent = MARKET_REDUCED_FEE;
            } 
            
            if reputation == MAX_REPUTATION {
                fee_percent = MARKET_MIN_FEE;
            }
        }
        let fee: u128 = price * fee_percent as u128 / 10_000u128;
        
        fee
    }
} 
