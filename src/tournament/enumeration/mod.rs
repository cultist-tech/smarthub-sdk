use crate::tournament::{ JsonTournament, TournamentId };
use near_sdk::json_types::{ U128 };
use near_sdk::AccountId;
use std::collections::HashMap;
use crate::tournament::{ WinnerPlace, RewardPrize };

mod enumeration_impl;
mod macros;

pub trait TournamentFactoryEnumeration {
    fn tournament(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId
    ) -> Option<JsonTournament>;

    fn tournaments(
        &self,
        owner_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<JsonTournament>;

    fn tournament_players(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId
    ) -> Vec<AccountId>;

    fn tournament_prizes(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId
    ) -> HashMap<WinnerPlace, Vec<RewardPrize>>;

    fn tournament_free_places(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId
    ) -> Option<u64>;

    fn tournament_member(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId,
        account_id: AccountId
    ) -> bool;

    fn tournament_is_whitelist_prize_owner(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId,
        account_id: AccountId
    ) -> bool;

    fn tournament_whitelist_prize_owners(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId
    ) -> Vec<AccountId>;

    fn tournament_is_rewarded(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId,
        place: u8
    ) -> bool;
}
