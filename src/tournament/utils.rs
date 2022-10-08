use crate::tournament::TournamentId;
use near_sdk::{ CryptoHash, env, AccountId, require };
use crate::tournament::metadata::TokenId;

const PRICE: u128 = 100_000_000_000_000_000_000_000;

pub(crate) static DELIMETER: &str = "||";

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_tournament_id(tournament_id: &TournamentId, shift: &String) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();

    //we hash the tournament ID with shift and return it
    hash.copy_from_slice(&env::sha256((tournament_id.to_owned() + shift).as_bytes()));

    hash
}

pub(crate) fn contract_tournament_id(owner_id: &AccountId, tournament_id: &TournamentId) -> TournamentId {
    format!("{}{}{}", owner_id, DELIMETER, tournament_id)
}

pub(crate) fn tournament_prize_id(
    owner_id: &AccountId,
    tournament_id: &TournamentId,
    place: &u8,
    prize_id: &TokenId
) -> TokenId {
    format!(
        "{}{}{}{}{}{}{}",
        owner_id,
        DELIMETER,
        tournament_id,
        DELIMETER,
        place,
        DELIMETER,
        prize_id
    )
}

pub(crate) fn tournament_place_id(
    owner_id: &AccountId,
    tournament_id: &TournamentId,
    place: &u8
) -> String {
    format!("{}{}{}{}{}", owner_id, DELIMETER, tournament_id, DELIMETER, place.to_string())
}

pub fn assert_tx_money() {
    require!(env::attached_deposit() == PRICE, "Requires attached deposit of 0.1 NEAR")
}
