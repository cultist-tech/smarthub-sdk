/// Tournament enumeration adds the extension standard offering
/// view-only methods.
#[macro_export]
macro_rules! impl_tournament_nft_access {
    ($contract:ident, $tournament:ident $(, $assert_access:ident)?) => {
        use $crate::tournament::nft_access::TournamentFactoryNftAccess;

        #[near_bindgen]
        impl TournamentFactoryNftAccess for $contract {
            fn tournament_nft_access(&self, tournament_id: $crate::tournament::TournamentId, owner_id: AccountId) -> Vec<String> {
                self.$tournament.tournament_nft_access(tournament_id, owner_id)
            }

            fn tournament_add_nft_access(&mut self, tournament_id: $crate::tournament::TournamentId, token_ids: Vec<String>) {
              $(self.$assert_access();)?
              self.$tournament.tournament_add_nft_access(tournament_id, token_ids)
            }
        }
    };
}
