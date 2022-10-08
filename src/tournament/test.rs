#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::tournament::{TournamentFactory, RewardPrize};
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{AccountId, BorshStorageKey, env, Balance, testing_env};
    use near_sdk::json_types::U128;
    use near_sdk::borsh::{self, BorshSerialize};
    use crate::tournament::base::TournamentFactoryCore;
    use crate::tournament::enumeration::TournamentFactoryEnumeration;
    use crate::tournament::nft_access::TournamentFactoryNftAccess;
    use crate::tournament::utils::{contract_tournament_id, tournament_prize_id, tournament_place_id};

    const PLAYERS_NUMBER: u8 = 4;
    const PRICE: U128 = U128(10_000_000_000_000_000_000_000);

    const ATTACHED_SUPPLY: Balance = 100_000_000_000_000_000_000_000;
    const ATTACHED_1_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

    /// Helper structure for keys of the persistent collections.
    #[derive(BorshSerialize, BorshStorageKey)]
    pub enum StorageKey {
        PlayersPerTournament,
        TournamentsById,
        TournamentMetadataById,
        TournamentPrizes,
        TournamentPrizesPer,
        TournamentRewardedPrizesByPlace,
        TournamentAccessNft,
        TournamentsByOwner,
        TournamentWhitelistPrizeOwners,
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn get_instance() -> TournamentFactory {
        TournamentFactory::new(
            StorageKey::PlayersPerTournament,
            StorageKey::TournamentPrizes,
            StorageKey::TournamentPrizesPer,
            StorageKey::TournamentRewardedPrizesByPlace,
            StorageKey::TournamentsById,
            StorageKey::TournamentMetadataById,
            StorageKey::TournamentAccessNft,
            StorageKey::TournamentsByOwner,
            StorageKey::TournamentWhitelistPrizeOwners,
        )
    }

    #[test]
    fn test_tournament_create() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),

        );

        let id = contract_tournament_id(&owner_id, &tournament_id);

        if let Some(tournament) = instance.tournaments_by_id.get(&id){
            assert_eq!(tournament.tournament_id, tournament_id);
            assert_eq!(tournament.owner_id, owner_id);
            assert_eq!(tournament.access_nft_contract, nft_access_contract);
            assert_eq!(tournament.players_number, PLAYERS_NUMBER);
            assert_eq!(tournament.price, price);
            assert_eq!(tournament.started_at, None);
            assert_eq!(tournament.ended_at, None);
            assert_eq!(tournament.created_at, env::block_timestamp());
        }

        if let Some(tournament_meta) = instance.tournament_metadata_by_id.get(&id){
            assert_eq!(tournament_meta.name, name);
            assert_eq!(tournament_meta.media, media);
            assert_eq!(tournament_meta.summary, summary);
        }

        if let Some(set) = instance.tournaments_by_owner.get(&owner_id){
            assert_eq!(set.contains(&tournament_id), true);
        }

        if let Some(list) = instance.whitelist_prize_owners.get(&id){
            assert_eq!(list.internal_is_whitelist(&owner_id), true);
        }
    }

     #[test]
    fn test_tournament_add_prize_and_whitelist_prize_owner() {
        let owner = accounts(1);

        let mut context = get_context(owner.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),
        );

        let place_number: u8 = 1;
        let prize_id = "Prize1".to_string();

        let prize_owner = accounts(3);

        let mut context = get_context(owner.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        instance.tournament_add_whitelist_prize_owner(
            tournament_id.clone(),
            prize_owner.clone(),
        );

        let mut context = get_context(prize_owner.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_1_NEAR)
            .build()
        );

        instance.tournament_add_prize(
            tournament_id.clone(),
            owner.clone(),
            place_number,
            prize_id.clone()
        );

         let id = contract_tournament_id(&owner, &tournament_id);

         if let Some(tree) = instance.prizes_by_tournament.get(&id){
            if let Some(set) = tree.get(&place_number) {
                assert_eq!(set.contains(&prize_id), true);
            }
         }

         let tournament_prize_id = tournament_prize_id(
             &prize_owner,
             &tournament_id,
             &place_number,
             &prize_id
         );

         if let Some(prize) = instance.prize_by_id.get(&tournament_prize_id) {
            if let RewardPrize::Near{ amount, owner_id} = prize {
                assert_eq!(amount, U128(ATTACHED_1_NEAR));
                assert_eq!(owner_id, Some(prize_owner));
            }
        }
    }

    #[test]
    fn test_tournament_join() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),
        );

        let player_id = accounts(3);

        let mut context = get_context(player_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let id = contract_tournament_id(&owner_id, &tournament_id);

        if let Some(set) = instance.players_per_tournament.get(&id){
            assert_eq!(set.contains(&player_id), true);
        }
    }

    #[test]
    fn test_tournament_start() {
        let owner_id = accounts(0);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(1));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),
        );

        let player1_id = accounts(2);
        let mut context = get_context(player1_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player2_id = accounts(3);
        let mut context = get_context(player2_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player3_id = accounts(4);
        let mut context = get_context(player3_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player4_id = accounts(5);
        let mut context = get_context(player4_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_start(
            tournament_id.clone(),
        );

        let id = contract_tournament_id(&owner_id, &tournament_id);

        if let Some(tournament) = instance.tournaments_by_id.get(&id){
            assert_eq!(tournament.started_at, Some(env::block_timestamp()));
        }
    }

     #[test]
    fn test_tournament_execute_reward() {
        let owner_id = accounts(0);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(1));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),
        );

        let place_number: u8 = 1;
        let prize_id = "Prize1".to_string();

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_1_NEAR)
            .build()
        );

        instance.tournament_add_prize(
            tournament_id.clone(),
            owner_id.clone(),
            place_number,
            prize_id.clone()
        );

        let player1_id = accounts(2);
        let mut context = get_context(player1_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player2_id = accounts(3);
        let mut context = get_context(player2_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player3_id = accounts(4);
        let mut context = get_context(player3_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player4_id = accounts(5);
        let mut context = get_context(player4_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_start(
            tournament_id.clone(),
        );
        let winner = player4_id;

        let winner_place = place_number;

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        instance.tournament_execute_reward(
            tournament_id.clone(),
            winner_place,
            winner.clone(),
        );

        let tournament_place_id = tournament_place_id(&owner_id, &tournament_id, &winner_place);

        assert_eq!(instance.prizes_per_place_rewarded.contains(&tournament_place_id), true);
    }

    #[test]
    fn test_tournament_end() {
        let owner_id = accounts(0);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(1));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),
        );

        let player1_id = accounts(2);
        let mut context = get_context(player1_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player2_id = accounts(3);
        let mut context = get_context(player2_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player3_id = accounts(4);
        let mut context = get_context(player3_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player4_id = accounts(5);
        let mut context = get_context(player4_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_start(
            tournament_id.clone(),
        );

        instance.tournament_end(
            tournament_id.clone(),
        );

        let id = contract_tournament_id(&owner_id, &tournament_id);

        if let Some(tournament) = instance.tournaments_by_id.get(&id){
            assert_eq!(tournament.ended_at, Some(env::block_timestamp()));
        }
    }

    #[test]
    fn test_enum_tournament() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),

        );

        //let id = contract_tournament_id(&owner_id, &tournament_id);

        if let Some(json_tournament) = instance.tournament(
            tournament_id.clone(),
            owner_id.clone()
        ){
            assert_eq!(json_tournament.tournament_id, tournament_id);
            assert_eq!(json_tournament.metadata.name, name);
            assert_eq!(json_tournament.metadata.media, media);
            assert_eq!(json_tournament.metadata.summary, summary);
            assert_eq!(json_tournament.started_at, None);
            assert_eq!(json_tournament.ended_at, None);
            assert_eq!(json_tournament.created_at, env::block_timestamp());
            assert_eq!(json_tournament.players_total, PLAYERS_NUMBER);
            assert_eq!(json_tournament.players_current, 0);
            assert_eq!(json_tournament.price, price);

        }
    }

    #[test]
    fn test_enum_tournaments() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),

        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let tournament_id2 = "Tournament2".to_string();
        let price = Some(PRICE);
        let name2 = "Tournament_name2".to_string();
        let media2 = Some("Media2".to_string());
        let summary2 = Some("Summary2".to_string());
        let nft_access_contract2 = Some(accounts(3));

        instance.tournament_create(
            tournament_id2.clone(),
            PLAYERS_NUMBER,
            price,
            name2.clone(),
            media2.clone(),
            summary2.clone(),
            nft_access_contract2.clone(),

        );

        let from_index = Some(U128(0));
        let limit = Some(3);

        let tournaments = instance.tournaments(
            owner_id.clone(),
            from_index,
            limit,
        );

        let tournament1 = &tournaments[0];

        assert_eq!(tournament1.tournament_id, tournament_id);
        assert_eq!(tournament1.metadata.name, name);
        assert_eq!(tournament1.metadata.media, media);
        assert_eq!(tournament1.metadata.summary, summary);
        assert_eq!(tournament1.started_at, None);
        assert_eq!(tournament1.ended_at, None);
        assert_eq!(tournament1.created_at, env::block_timestamp());
        assert_eq!(tournament1.players_total, PLAYERS_NUMBER);
        assert_eq!(tournament1.players_current, 0);
        assert_eq!(tournament1.price, price);

        let tournament2 = &tournaments[1];

        assert_eq!(tournament2.tournament_id, tournament_id2);
        assert_eq!(tournament2.metadata.name, name2);
        assert_eq!(tournament2.metadata.media, media2);
        assert_eq!(tournament2.metadata.summary, summary2);
        assert_eq!(tournament2.started_at, None);
        assert_eq!(tournament2.ended_at, None);
        assert_eq!(tournament2.created_at, env::block_timestamp());
        assert_eq!(tournament2.players_total, PLAYERS_NUMBER);
        assert_eq!(tournament2.players_current, 0);
        assert_eq!(tournament2.price, price);
    }

    #[test]
    fn test_enum_tournament_players() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),
        );

        let player_id1 = accounts(3);

        let mut context = get_context(player_id1.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player_id2 = accounts(4);

        let mut context = get_context(player_id2.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let players = instance.tournament_players(
            tournament_id.clone(),
            owner_id.clone(),
        );

        assert_eq!(players[0], player_id1);
        assert_eq!(players[1], player_id2);

    }

    #[test]
    fn test_enum_tournament_prizes() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),

        );


        let prize_owner = accounts(3);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        instance.tournament_add_whitelist_prize_owner(
            tournament_id.clone(),
            prize_owner.clone(),
        );

        let place_number1: u8 = 1;
        let prize_id1 = "Prize1".to_string();
        let amount1: Balance = ATTACHED_1_NEAR + 1_000_000_000;

        let mut context = get_context(prize_owner.clone());
        testing_env!(context
            .attached_deposit(amount1)
            .build()
        );

        instance.tournament_add_prize(
            tournament_id.clone(),
            owner_id.clone(),
            place_number1,
            prize_id1.clone()
        );

        let place_number2: u8 = 2;
        let prize_id2 = "Prize2".to_string();
        let amount2: Balance = ATTACHED_1_NEAR + 1_000_000;

        let mut context = get_context(prize_owner.clone());
        testing_env!(context
            .attached_deposit(amount2)
            .build()
        );

        instance.tournament_add_prize(
            tournament_id.clone(),
            owner_id.clone(),
            place_number2,
            prize_id2.clone()
        );

        let place_number3: u8 = 3;
        let prize_id3 = "Prize3".to_string();
        let amount3: Balance = ATTACHED_1_NEAR + 1_000;

        let mut context = get_context(prize_owner.clone());
        testing_env!(context
            .attached_deposit(amount3)
            .build()
        );

        instance.tournament_add_prize(
            tournament_id.clone(),
            owner_id.clone(),
            place_number3,
            prize_id3.clone()
        );

        let tournament_prizes = instance.tournament_prizes(
            tournament_id.clone(),
            owner_id.clone(),
        );

        if let Some(prizes) = tournament_prizes.get(&place_number1) {
            if let RewardPrize::Near{ amount, owner_id} = &prizes[0] {
                assert_eq!(*amount, U128(amount1));
                assert_eq!(*owner_id, Some(prize_owner.clone()));
            }
        }

        if let Some(prizes) = tournament_prizes.get(&place_number2) {
            if let RewardPrize::Near{ amount, owner_id} = &prizes[0] {
                assert_eq!(*amount, U128(amount2));
                assert_eq!(*owner_id, Some(prize_owner.clone()));
            }
        }

        if let Some(prizes) = tournament_prizes.get(&place_number3) {
            if let RewardPrize::Near{ amount, owner_id} = &prizes[0] {
                assert_eq!(*amount, U128(amount3));
                assert_eq!(*owner_id, Some(prize_owner.clone()));
            }
        }


    }

    #[test]
    fn test_enum_tournament_free_places() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),
        );

        let player_id1 = accounts(3);

        let mut context = get_context(player_id1.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player_id2 = accounts(4);

        let mut context = get_context(player_id2.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let free_places = instance.tournament_free_places(
            tournament_id.clone(),
            owner_id.clone(),
        );

        assert_eq!(free_places, Some(2));
    }

    #[test]
    fn test_enum_tournament_member() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),
        );

        let player_id = accounts(3);

        let mut context = get_context(player_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let is_member = instance.tournament_member(
            tournament_id.clone(),
            owner_id.clone(),
            player_id.clone()
        );

        assert_eq!(is_member, true);
    }

    #[test]
    fn test_enum_tournament_whitelist_prize_owners() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),
        );

        let prize_owner1 = accounts(3);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        instance.tournament_add_whitelist_prize_owner(
            tournament_id.clone(),
            prize_owner1.clone(),
        );

        let prize_owner2 = accounts(4);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_1_NEAR)
            .build()
        );

        instance.tournament_add_whitelist_prize_owner(
            tournament_id.clone(),
            prize_owner2.clone(),
        );

        let prize_owners = instance.tournament_whitelist_prize_owners(
            tournament_id.clone(),
            owner_id.clone(),
        );

        assert_eq!(prize_owners[0], owner_id);
        assert_eq!(prize_owners[1], prize_owner1);
        assert_eq!(prize_owners[2], prize_owner2);
    }

     #[test]
    fn test_enum_tournament_is_rewarded() {
        let owner_id = accounts(0);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(1));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),
        );

        let place_number: u8 = 1;
        let prize_id = "Prize1".to_string();

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_1_NEAR)
            .build()
        );

        instance.tournament_add_prize(
            tournament_id.clone(),
            owner_id.clone(),
            place_number,
            prize_id.clone()
        );

        let player1_id = accounts(2);
        let mut context = get_context(player1_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player2_id = accounts(3);
        let mut context = get_context(player2_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player3_id = accounts(4);
        let mut context = get_context(player3_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let player4_id = accounts(5);
        let mut context = get_context(player4_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_join(
            tournament_id.clone(),
            owner_id.clone(),
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );
        instance.tournament_start(
            tournament_id.clone(),
        );
        let winner = player4_id;

        let winner_place = place_number;

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        instance.tournament_execute_reward(
            tournament_id.clone(),
            winner_place,
            winner.clone(),
        );


        let is_rewarded = instance.tournament_is_rewarded(
            tournament_id.clone(),
            owner_id.clone(),
            winner_place,
        );

        assert_eq!(is_rewarded, true);
    }

    #[test]
    fn test_nft_tournament_add_nft_access() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),

        );

        let token_ids = vec!["token_1".to_string(), "token_2".to_string()];

        instance.tournament_add_nft_access(
            tournament_id.clone(),
            token_ids.clone(),
        );

        let id = contract_tournament_id(&owner_id, &tournament_id);

        if let Some(set) = instance.tournament_access_nft.get(&id) {
            assert_eq!(set.contains(&token_ids[0]), true);
            assert_eq!(set.contains(&token_ids[1]), true);
        }
    }

    #[test]
    fn test_nft_tournament_nft_access() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),

        );

        let token_ids = vec!["token_1".to_string(), "token_2".to_string()];

        instance.tournament_add_nft_access(
            tournament_id.clone(),
            token_ids.clone(),
        );

        let tokens = instance.tournament_nft_access(
            tournament_id.clone(),
            owner_id.clone(),
        );

        assert_eq!(tokens, token_ids);
    }

    #[test]
    fn test_nft_tournament_assert_nft_access() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),

        );

        let token_ids = vec!["token_1".to_string(), "token_2".to_string()];

        instance.tournament_add_nft_access(
            tournament_id.clone(),
            token_ids.clone(),
        );

        //let id = contract_tournament_id(&owner_id, &tournament_id);

        instance.assert_nft_access(
            &tournament_id,
            &owner_id,
            &token_ids[0]
        );
    }

    #[test]
    fn test_nft_tournament_internal_use_nft_access() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let tournament_id = "Tournament1".to_string();
        let price = Some(PRICE);
        let name = "Tournament_name".to_string();
        let media = Some("Media".to_string());
        let summary = Some("Summary".to_string());
        let nft_access_contract = Some(accounts(2));

        instance.tournament_create(
            tournament_id.clone(),
            PLAYERS_NUMBER,
            price,
            name.clone(),
            media.clone(),
            summary.clone(),
            nft_access_contract.clone(),

        );

        let token_ids = vec!["token_1".to_string(), "token_2".to_string()];

        instance.tournament_add_nft_access(
            tournament_id.clone(),
            token_ids.clone(),
        );

        let player_id = accounts(3);

        instance.internal_use_nft_access(
            &tournament_id,
            &owner_id,
            &token_ids[0],
            &player_id,
            &nft_access_contract.unwrap(),
        );

        let id = contract_tournament_id(&owner_id, &tournament_id);

        if let Some(set) = instance.players_per_tournament.get(&id){
            assert_eq!(set.contains(&player_id), true);
        }


    }
}
