use crate::metadata::FungibleTokenId;
use near_sdk::{env, require, AccountId, CryptoHash, Promise};
use rand::prelude::StdRng;
use rand::SeedableRng;
use std::convert::TryInto;

const PRICE: u128 = 10000000000000000000000;
const DELIMETER: &str = "||";

pub fn assert_tx_money() {
    require!(
        env::attached_deposit() == PRICE,
        "Requires attached deposit of 0.01 NEAR"
    )
}
/// Assert that at least 1 yoctoNEAR was attached.
pub fn assert_at_least_one_yocto() {
    require!(
        env::attached_deposit() >= 1,
        "Requires attached deposit of at least 1 yoctoNEAR"
    )
}

pub fn contract_token_id(contract_id: &AccountId, token_id: &String) -> String {
    format!("{}{}{}", contract_id, DELIMETER, token_id)
}

pub fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

pub fn near_ft() -> FungibleTokenId {
    AccountId::new_unchecked("near".to_string())
}

// custom
// https://github.com/ilblackdragon/dragonear/blob/main/src/dragon.rs
pub fn random_use() -> StdRng {
    let seed: [u8; 32] = env::random_seed().try_into().unwrap();
    let rng: StdRng = SeedableRng::from_seed(seed);

    rng
}

pub fn refund_deposit_to_account(refund: u128) {
  Promise::new(env::signer_account_id()).transfer(refund);
}
