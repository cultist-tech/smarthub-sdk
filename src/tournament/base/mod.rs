use near_sdk::AccountId;
use near_sdk::json_types::{ U128 };
use crate::tournament::metadata::{ TournamentId };

mod tournament_core;
mod internal;
mod macros;
mod receivers;

pub use self::tournament_core::{ TournamentFactory, WinnerPlace };

pub trait TournamentFactoryCore {
    fn tournament_create(
        &mut self,
        tournament_id: TournamentId,
        players_number: u8,
        price: Option<U128>,
        name: String,
        media: Option<String>,
        summary: Option<String>,
        nft_access_contract: Option<AccountId>
    );

    fn tournament_start(&mut self, tournament_id: TournamentId);

    fn tournament_end(&mut self, tournament_id: TournamentId);

    //add player to the tournament with NEAR depositing
    fn tournament_join(&mut self, tournament_id: TournamentId, owner_id: AccountId);

    fn tournament_add_prize(
        &mut self,
        tournament_id: TournamentId,
        owner_id: AccountId,
        place_number: u8,
        prize_id: String
    );

    //refunds the prizes for the winners
    fn tournament_execute_reward(
        &mut self,
        tournament_id: TournamentId,
        winner_place: u8,
        account_id: AccountId,
    );

    fn tournament_add_whitelist_prize_owner(
        &mut self,
        tournament_id: TournamentId,
        account_id: AccountId
    );
}
