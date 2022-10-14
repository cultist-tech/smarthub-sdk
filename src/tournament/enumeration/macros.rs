/// Tournament enumeration adds the extension standard offering
/// view-only methods.
#[macro_export]
macro_rules! impl_tournament_enumeration {
    ($contract:ident, $tournament:ident) => {
        use $crate::tournament::enumeration::TournamentFactoryEnumeration;

        #[near_bindgen]
        impl TournamentFactoryEnumeration for $contract {

            fn tournament(
                &self,
                tournament_id: $crate::tournament::TournamentId, owner_id: AccountId
            ) -> Option<$crate::tournament::JsonTournament> {
                self.$tournament.tournament(tournament_id, owner_id)
            }

            fn tournaments(
                &self,
                owner_id: AccountId,
                from_index: Option<U128>,
                limit: Option<u64>
            ) -> Vec<$crate::tournament::JsonTournament> {
                self.$tournament.tournaments(owner_id, from_index, limit)
            }

            fn tournament_players(&self, tournament_id: $crate::tournament::TournamentId, owner_id: AccountId) -> Vec<AccountId> {
              self.$tournament.tournament_players(tournament_id, owner_id)
            }

             fn tournament_prizes(&self, tournament_id: $crate::tournament::TournamentId, owner_id: AccountId) -> std::collections::HashMap<$crate::tournament::WinnerPlace, Vec<$crate::tournament::RewardPrize>> {
                self.$tournament.tournament_prizes(tournament_id, owner_id)
            }

            fn tournament_free_places(
                &self,
                tournament_id: $crate::tournament::TournamentId, owner_id: AccountId
            ) -> Option<u64> {
                self.$tournament.tournament_free_places(tournament_id, owner_id)
            }

            fn tournament_member(&self, tournament_id: $crate::tournament::TournamentId, owner_id: AccountId, account_id: AccountId) -> bool {
             self.$tournament.tournament_member(tournament_id, owner_id, account_id)
            }

            fn tournament_is_whitelist_prize_owner(&self, tournament_id: $crate::tournament::TournamentId, owner_id: AccountId, account_id: AccountId) -> bool {
              self.$tournament.tournament_is_whitelist_prize_owner(tournament_id, owner_id, account_id)
            }

            fn tournament_whitelist_prize_owners(
                &self,
                tournament_id: $crate::tournament::TournamentId,
                owner_id: AccountId
            ) -> Vec<AccountId> {
              self.$tournament.tournament_whitelist_prize_owners(tournament_id, owner_id)
            }

            fn tournament_is_rewarded(
              &self,
              tournament_id: $crate::tournament::TournamentId,
              owner_id: AccountId,
              place: u8,
            ) -> bool {
             self.$tournament.tournament_is_rewarded(tournament_id, owner_id, place)
            }
        }
    };
}
