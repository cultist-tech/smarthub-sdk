/// Tournament enumeration adds the extension standard offering
/// view-only methods.
#[macro_export]
macro_rules! impl_tournament_nft_access {
    ($contract:ident, $tournament:ident) => {
        use $crate::tournament::nft_access::TournamentFactoryNftAccess;

        #[near_bindgen]
        impl TournamentFactoryNftAccess for $contract {
            fn tournament_nft_access(&self, tournament_id: TournamentId, owner_id: AccountId) -> Vec<TokenId> {
                self.$tournament.tournament_nft_access(tournament_id, owner_id)
            }

            fn tournament_add_nft_access(&mut self, tournament_id: TournamentId, token_ids: Vec<TokenId>) {
              self.$tournament.tournament_add_nft_access(tournament_id, token_ids)
            }
        }
    };
}