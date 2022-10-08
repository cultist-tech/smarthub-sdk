use near_sdk::{ env, AccountId };
use crate::nft_fractionation::TokenId;

pub(crate) static DELIMETER: &str = "||";

pub(crate) fn date_now() -> u64 {
    env::block_timestamp() / 1000000
}

pub(crate) fn contract_token_id(contract_id: &AccountId, token_id: &TokenId) -> TokenId {
    format!("{}{}{}", contract_id, DELIMETER, token_id)
}