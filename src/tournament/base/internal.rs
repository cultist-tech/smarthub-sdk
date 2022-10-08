use near_sdk::{ AccountId, env };
use crate::tournament::utils::{ hash_tournament_id, contract_tournament_id, tournament_prize_id };
use near_sdk::collections::{ TreeMap, UnorderedSet };
use crate::tournament::{
    TournamentFactory,
    TournamentId,
    TournamentJoin,
    Tournament,
    TournamentAddPrize,
    JsonTournament,
    RewardPrize,
};
use crate::tournament::base::tournament_core::StorageKey;
use crate::whitelist::{WhitelistFeature, WhitelistFeatureCore};

impl TournamentFactory {
    //add a player to the set of players the tournament has
    pub(crate) fn internal_add_player_to_tournament(
        &mut self,
        tournament_id: &TournamentId,
        owner_id: &AccountId,
        account_id: &AccountId
    ) -> bool {
        let id = contract_tournament_id(&owner_id, &tournament_id);

        //get the set of players for the given tournament
        let mut players_set = self.players_per_tournament.get(&id).unwrap_or_else(|| {
            //if the tournament doesn't have any players, we create a new unordered set
            UnorderedSet::new(StorageKey::PlayersPerTournamentInner {
                //we get a new unique prefix for the collection
                tournament_id_hash: hash_tournament_id(&id, &"s".to_string()),
            })
        });

        //we insert the player ID into the set
        let new_one = players_set.insert(account_id);

        //we insert that set for the given tournament ID.
        self.players_per_tournament.insert(&id, &players_set);

        if new_one {
            (TournamentJoin {
                account_id: &account_id,
                tournament_id: &tournament_id,
                owner_id: &owner_id,
            }).emit();
        }

        new_one
    }

    //get number of players already in the tournament
    pub(crate) fn internal_get_players_number_in_tournament(&self, id: &TournamentId) -> u8 {
        //get the set of tokens for the given account
        if let Some(players_set) = self.players_per_tournament.get(id) {
            players_set.len() as u8
        } else {
            0
        }
    }

    pub(crate) fn enum_tournament(&self, id: &TournamentId) -> Option<JsonTournament> {
        if let Some(tournament) = self.tournaments_by_id.get(&id) {
            //we'll get the metadata for that token
            let metadata = self.tournament_metadata_by_id.get(&id).unwrap();
            let players_current = match self.players_per_tournament.get(&id) {
                Some(players) => players.len() as u8,
                None => 0,
            };

            //we return the JsonToken (wrapped by Some since we return an option)
            return Some(JsonTournament {
                tournament_id: tournament.tournament_id,
                owner_id: tournament.owner_id,
                access_nft_contract: tournament.access_nft_contract,
                metadata,
                started_at: tournament.started_at,
                ended_at: tournament.ended_at,
                created_at: tournament.created_at,
                players_total: tournament.players_number,
                players_current,
                price: tournament.price,
            });
        }

        None
    }

    //

    pub(crate) fn assert_tournament_not_started(&self, id: &TournamentId) -> Tournament {
        let tournament = self.tournaments_by_id.get(id).expect("Not found tournament");

        assert!(tournament.started_at.is_none(), "Tournament already started");

        tournament
    }

    pub(crate) fn assert_tournament_started(&self, id: &TournamentId) -> Tournament {
        let tournament = self.tournaments_by_id.get(id).expect("Not found tournament");

        assert!(tournament.started_at.is_some(), "Tournament not started");

        tournament
    }

    // pub(crate) fn assert_tournament_ended(&self, id: &TournamentId) -> Tournament {
    //   let tournament = self.tournaments_by_id.get(id).expect("Not found tournament");
    //
    //   assert!(tournament.started_at.is_some(), "Tournament is not started");
    //   assert!(tournament.ended_at.is_some(), "Tournament is not ended");
    //
    //   tournament
    // }

    pub(crate) fn assert_tournament_players(&self, id: &TournamentId) -> Tournament {
        let tournament = self.tournaments_by_id.get(id).expect("Not found tournament");

        //Check there are some free playses for the players in the tournament
        assert!(
            tournament.players_number - self.internal_get_players_number_in_tournament(id) > 0,
            "Tournament is already full of players"
        );

        tournament
    }

    pub(crate) fn internal_tournament_add_prize(
        &mut self,
        owner_tournament_id: &TournamentId,
        place_number: &u8,
        prize_id: &String,
        prize: &RewardPrize
    ) {
        let tournament = self.tournaments_by_id.get(&owner_tournament_id).expect("Not found");
        let tournament_prize_id = tournament_prize_id(
            &tournament.owner_id,
            &tournament.tournament_id,
            &place_number,
            &prize_id
        );

        assert!(
            place_number.clone() > 0 && place_number.clone() <= tournament.players_number,
            "Invalid place"
        );
        assert!(self.prize_by_id.get(&prize_id).is_none(), "Prize id already taken");

        let mut tournament_prizes = self.prizes_by_tournament
            .get(&owner_tournament_id)
            .unwrap_or_else(|| {
                TreeMap::new(StorageKey::TournamentPrizesInner {
                    tournament_hash: env::sha256(owner_tournament_id.as_bytes()),
                })
            });
        let mut place_prizes = tournament_prizes.get(&place_number).unwrap_or_else(|| {
            UnorderedSet::new(StorageKey::TournamentPrizesPlaceInner {
                place_hash: env::sha256(place_number.to_string().as_bytes()),
            })
        });

        assert!(place_prizes.len() < 5, "Maximum 5 prizes per place");

        place_prizes.insert(&prize_id.to_string());
        tournament_prizes.insert(&place_number, &place_prizes);
        self.prizes_by_tournament.insert(&owner_tournament_id, &tournament_prizes);
        self.prize_by_id.insert(&tournament_prize_id, &prize);

        (TournamentAddPrize {
            tournament_id: &tournament.tournament_id,
            owner_id: &tournament.owner_id,
            prize,
        }).emit();
    }

  pub fn internal_add_prize_owner(&mut self, id: &String, account_id: &AccountId) {
    let mut whitelist: WhitelistFeature = self.whitelist_prize_owners.get(&id).unwrap_or_else(|| {
      WhitelistFeature::new(StorageKey::TournamentWhitelistPrizeOwnersInner {
        tournament_hash: env::sha256(id.as_bytes()),
      })
    });
    whitelist.whitelist_add(account_id.clone());

    self.whitelist_prize_owners.insert(&id, &whitelist);
  }
}
