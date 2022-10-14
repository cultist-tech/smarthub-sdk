/// The core methods for a basic tournament. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_tournament_core {
    ($contract:ident, $tournament:ident $(, $assert_access:ident)?) => {
        use $crate::tournament::base::{TournamentFactoryCore};

        #[near_bindgen]
        impl TournamentFactoryCore for $contract {
            #[payable]
            fn tournament_create(
                &mut self,
                  tournament_id: TournamentId,
                  players_number: u8,
                  price: Option<U128>,
                  name: String,
                  media: Option<String>,
                  summary: Option<String>,
                  nft_access_contract: Option<AccountId>,
            ) {
                $(self.$assert_access();)?
                self.$tournament.tournament_create(tournament_id, players_number, price, name, media, summary, nft_access_contract)
            }

            #[payable]
            fn tournament_join(
                &mut self,
                tournament_id: TournamentId, owner_id: AccountId
            ) {
                $(self.$assert_access();)?
                self.$tournament.tournament_join(tournament_id, owner_id)
            }

            fn tournament_start(&mut self, tournament_id: TournamentId) {
             $(self.$assert_access();)?
             self.$tournament.tournament_start(tournament_id)
            }

            fn tournament_end(&mut self, tournament_id: TournamentId) {
             $(self.$assert_access();)?
             self.$tournament.tournament_end(tournament_id)
            }

            #[payable]
            fn tournament_add_prize(
                &mut self,
                tournament_id: TournamentId,
                owner_id: AccountId,
                place_number: u8,
                prize_id: String,
            ) {
                $(self.$assert_access();)?
                self.$tournament.tournament_add_prize(tournament_id, owner_id, place_number, prize_id)
            }

            fn tournament_execute_reward(
                &mut self,
                tournament_id: TournamentId,
                winner_place: u8,
                account_id: AccountId,
            ) {
                $(self.$assert_access();)?
                self.$tournament.tournament_execute_reward(tournament_id, winner_place, account_id)
            }

            fn tournament_add_whitelist_prize_owner(&mut self, tournament_id: TournamentId, account_id: AccountId) {
              $(self.$assert_access();)?
              self.$tournament.tournament_add_whitelist_prize_owner(tournament_id, account_id)
            }
        }
    };
}
