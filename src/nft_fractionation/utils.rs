use near_sdk::{ AccountId };
use crate::nft_fractionation::TokenId;

pub(crate) static DELIMETER: &str = "||";

pub(crate) fn contract_token_id(contract_id: &AccountId, token_id: &TokenId) -> TokenId {
    format!("{}{}{}", contract_id, DELIMETER, token_id)
}
