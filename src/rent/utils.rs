use near_sdk::{ CryptoHash, AccountId, env };
use crate::rent::TokenId;

pub(crate) static DELIMETER: &str = "||";

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

// 600000 * 1000000
pub(crate) fn time_get_minutes(time: u64) -> u64 {
    time / 60000
}

pub(crate) fn contract_token_id(contract_id: &AccountId, token_id: &TokenId) -> TokenId {
    format!("{}{}{}", contract_id, DELIMETER, token_id)
}
