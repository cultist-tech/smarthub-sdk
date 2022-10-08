use near_sdk::{ require, AccountId };
use crate::tournament::{
    TournamentFactory,
    TournamentId,
    JsonTournament,
    TournamentFactoryEnumeration,
    WinnerPlace,
    RewardPrize,
};
use crate::tournament::utils::{contract_tournament_id, tournament_prize_id, tournament_place_id};
use near_sdk::json_types::U128;
use std::collections::HashMap;
use crate::whitelist::{WhitelistFeatureCore, WhitelistFeature};

impl TournamentFactoryEnumeration for TournamentFactory {
    //get the information for a specific tournament ID
    fn tournament(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId
    ) -> Option<JsonTournament> {
        let id = contract_tournament_id(&owner_id, &tournament_id);

        self.enum_tournament(&id)
    }

    //Query for  tournaments on the contract regardless of the ID using pagination
    fn tournaments(
        &self,
        owner_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<JsonTournament> {
        let tournaments = if let Some(tournaments) = self.tournaments_by_owner.get(&owner_id) {
            tournaments
        } else {
            return vec![];
        };

        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        require!(
            (tournaments.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        require!(limit != 0, "Cannot provide limit of 0.");

        tournaments
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|tournament_id|
                self.enum_tournament(&contract_tournament_id(&owner_id, &tournament_id)).unwrap()
            )
            .collect()
    }

    fn tournament_players(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId
    ) -> Vec<AccountId> {
        let id = contract_tournament_id(&owner_id, &tournament_id);

        self.players_per_tournament.get(&id).as_ref().unwrap().to_vec()
    }

    fn tournament_prizes(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId
    ) -> HashMap<WinnerPlace, Vec<RewardPrize>> {
        let id = contract_tournament_id(&owner_id, &tournament_id);

        let mut map = HashMap::new();

        if let Some(prizes) = self.prizes_by_tournament.get(&id) {
            prizes.iter().for_each(|(place, arr)| {
                let rewards = arr
                    .iter()
                    .map(|prize_id| {
                        self.prize_by_id
                            .get(&tournament_prize_id(&owner_id, &tournament_id, &place, &prize_id))
                            .unwrap()
                    })
                    .collect();

                map.insert(place, rewards);
            });
        }

        map
    }

    // get free places in the tournament
    fn tournament_free_places(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId
    ) -> Option<u64> {
        let id = contract_tournament_id(&owner_id, &tournament_id);

        let tournament = self.tournaments_by_id.get(&id).expect("Not found");

        //calculate free places
        let free_places =
            tournament.players_number - self.internal_get_players_number_in_tournament(&id);

        //return free places
        Some((free_places as u64).into())
    }

    fn tournament_member(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId,
        account_id: AccountId
    ) -> bool {
        let id = contract_tournament_id(&owner_id, &tournament_id);
        self.players_per_tournament.get(&id).expect("Not found").contains(&account_id)
    }

    fn tournament_is_whitelist_prize_owner(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId,
        account_id: AccountId
    ) -> bool {
        let id = contract_tournament_id(&owner_id, &tournament_id);

        if let Some(whitelist) = self.whitelist_prize_owners.get(&id) {
            return whitelist.is_whitelist(account_id);
        }

        false
    }

  fn tournament_whitelist_prize_owners(&self, tournament_id: TournamentId, owner_id: AccountId) -> Vec<AccountId> {
    let id = contract_tournament_id(&owner_id, &tournament_id);

    if let Some(whitelist) = self.whitelist_prize_owners.get(&id) {
      whitelist.internal_list()
    } else {
      vec![]
    }
  }

  fn tournament_is_rewarded(&self, tournament_id: TournamentId, owner_id: AccountId, place: u8) -> bool {
    let tournament_place_id = tournament_place_id(&owner_id, &tournament_id, &place);

    self.prizes_per_place_rewarded.contains(&tournament_place_id)
  }
}
