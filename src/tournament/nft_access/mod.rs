use crate::tournament::{ TournamentId, TokenId };
use near_sdk::AccountId;

mod internal;
mod nft_access_impl;
mod macros;

pub trait TournamentFactoryNftAccess {
    fn tournament_nft_access(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId
    ) -> Vec<TokenId>;

    fn tournament_add_nft_access(&mut self, tournament_id: TournamentId, token_ids: Vec<TokenId>);
}