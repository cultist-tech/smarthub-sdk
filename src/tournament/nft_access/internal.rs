use crate::tournament::{ TournamentFactory, TournamentId, TokenId };
use near_sdk::{ AccountId };
use crate::tournament::utils::contract_tournament_id;

impl TournamentFactory {
    pub(crate) fn internal_nft_access(
        &self,
        tournament_id: &TournamentId,
        owner_id: &AccountId
    ) -> Vec<TokenId> {
        let id = contract_tournament_id(&owner_id, &tournament_id);

        self.tournament_access_nft.get(&id).unwrap().to_vec()
    }

    pub(crate) fn internal_use_nft_access(
        &mut self,
        tournament_id: &TournamentId,
        owner_id: &AccountId,
        token_id: &TokenId,
        account_id: &AccountId,
        nft_contract_id: &AccountId
    ) {
        let id = contract_tournament_id(&owner_id, &tournament_id);
        let tournament = self.tournaments_by_id.get(&id).expect("Not found");
        let access_nft = tournament.access_nft_contract.expect("Nft access not available");

        assert_eq!(&access_nft, nft_contract_id, "Invalid contract");

        self.assert_nft_access(&tournament_id, &owner_id, &token_id);
        self.assert_tournament_not_started(&id);
        self.assert_tournament_players(&id);

        let is_added = self.internal_add_player_to_tournament(
            &tournament_id,
            &owner_id,
            &account_id
        );
        assert!(is_added, "Already in the tournament");
    }

    //

    pub(crate) fn assert_nft_access(&self, tournament_id: &TournamentId, owner_id: &AccountId, token_id: &TokenId) {
        let id = contract_tournament_id(&owner_id, &tournament_id);

        let nft_list = self.tournament_access_nft
            .get(&id)
            .expect("Not found Tournament");

        assert!(nft_list.contains(&token_id), "Invalid Access");
    }
}
