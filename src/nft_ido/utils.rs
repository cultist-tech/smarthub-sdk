use rand::{ SeedableRng };
use std::convert::TryInto;
use near_sdk::{ env, require, AccountId };
use rand::prelude::StdRng;
use crate::nft_ido::{ TokenId, ContractIdoId };

const PRICE: u128 = 100_000_000_000_000_000_000_000;

pub(crate) static DELIMETER: &str = "||";

// custom
// https://github.com/ilblackdragon/dragonear/blob/main/src/dragon.rs
pub(crate) fn random_use() -> StdRng {
    let seed: [u8; 32] = env::random_seed().try_into().unwrap();
    let rng: StdRng = SeedableRng::from_seed(seed);

    rng
}

pub(crate) fn contract_token_id(contract_id: &AccountId, token_id: &TokenId) -> ContractIdoId {
    format!("{}{}{}", contract_id, DELIMETER, token_id)
}

pub fn assert_tx_money() {
    require!(env::attached_deposit() == PRICE, "Requires attached deposit of 0.1 NEAR")
}
