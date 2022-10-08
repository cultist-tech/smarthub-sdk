use near_sdk::{ env, IntoStorageKey, AccountId, Balance, Promise, CryptoHash, BorshStorageKey };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::{ LookupMap, UnorderedMap, UnorderedSet, TreeMap };
use near_sdk::json_types::{ U128 };
use crate::tournament::events::{
    TournamentCreate,
    TournamentReward,
    TournamentStart,
    TournamentEnd,
};
use crate::tournament::metadata::{ TournamentId, Tournament, TournamentMetadata };
use crate::tournament::base::TournamentFactoryCore;
use crate::tournament::utils::{ contract_tournament_id, tournament_place_id, assert_tx_money };
use crate::tournament::metadata::TokenId;
use crate::tournament::{ TournamentPrizeId, RewardPrize, PrizeId, TournamentPlaceId };
use crate::tournament::utils::tournament_prize_id;
use crate::whitelist::{ WhitelistFeature, WhitelistFeatureCore };
use crate::nft::base::{ GAS_FOR_NFT_TRANSFER };
use crate::ft::base::core_impl::{ GAS_FOR_FT_TRANSFER };
use crate::ft::base::external::{ ext_ft };
use crate::nft::base::external::{ ext_nft };

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    PlayersPerTournamentInner {
        tournament_id_hash: CryptoHash,
    },
    TournamentsByOwnerInner {
        accont_hash: Vec<u8>,
    },
    TournamentPrizesInner {
        tournament_hash: Vec<u8>,
    },
    TournamentPrizesPlaceInner {
        place_hash: Vec<u8>,
    },
    TournamentWhitelistPrizeOwnersInner {
        tournament_hash: Vec<u8>,
    },
}

// ONE NEAR 1_000_000_000_000_000_000_000_000
const MIN_PRIZE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;

pub type WinnerPlace = u8;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TournamentFactory {
    //keeps track of all the players IDs for a given tournament
    pub players_per_tournament: LookupMap<TournamentId, UnorderedSet<AccountId>>,

    //keeps track of the tournament struct for a given tournament ID
    pub tournaments_by_id: TreeMap<TournamentId, Tournament>,

    //keeps track of the tournament metadata for a given tournament ID
    pub tournament_metadata_by_id: UnorderedMap<TournamentId, TournamentMetadata>,

    //

    pub prize_by_id: LookupMap<TournamentPrizeId, RewardPrize>,

    pub prizes_by_tournament: LookupMap<TournamentId, TreeMap<WinnerPlace, UnorderedSet<PrizeId>>>,

    pub prizes_per_place_rewarded: UnorderedSet<TournamentPlaceId>,

    //

    pub tournament_access_nft: LookupMap<TournamentId, UnorderedSet<TokenId>>,

    pub tournaments_by_owner: TreeMap<AccountId, UnorderedSet<TournamentId>>,

    pub whitelist_prize_owners: LookupMap<TournamentId, WhitelistFeature>,
}

impl TournamentFactory {
    pub fn new<P, W, W2, W3, TI, TM, TN, TO, TWP>(
        players_per_tournament_prefix: P,
        tournament_prizes_prefix: W,
        tournament_prizes_per_prefix: W2,
        tournament_prizes_rewarded_prefix: W3,
        tournaments_by_id: TI,
        tournament_metadata_by_id: TM,
        tournament_access_nft_prefix: TN,
        tournaments_by_owner_prefix: TO,
        tournament_whitelist_prize_owners: TWP
    )
        -> Self
        where
            P: IntoStorageKey,
            W: IntoStorageKey,
            W2: IntoStorageKey,
            W3: IntoStorageKey,
            TI: IntoStorageKey,
            TM: IntoStorageKey,
            TN: IntoStorageKey,
            TO: IntoStorageKey,
            TWP: IntoStorageKey
    {
        let this = Self {
            players_per_tournament: LookupMap::new(players_per_tournament_prefix),
            prize_by_id: LookupMap::new(tournament_prizes_prefix),
            prizes_by_tournament: LookupMap::new(tournament_prizes_per_prefix),
            prizes_per_place_rewarded: UnorderedSet::new(tournament_prizes_rewarded_prefix),
            tournaments_by_id: TreeMap::new(tournaments_by_id),
            tournament_metadata_by_id: UnorderedMap::new(tournament_metadata_by_id),
            tournament_access_nft: LookupMap::new(tournament_access_nft_prefix),
            tournaments_by_owner: TreeMap::new(tournaments_by_owner_prefix),
            whitelist_prize_owners: LookupMap::new(tournament_whitelist_prize_owners),
        };

        this
    }
}

impl TournamentFactoryCore for TournamentFactory {
    //tournament creation method
    fn tournament_create(
        &mut self,
        tournament_id: TournamentId,
        players_number: u8,
        price: Option<U128>,
        name: String,
        media: Option<String>,
        summary: Option<String>,
        nft_access_contract: Option<AccountId>
    ) {
        assert_tx_money();

        let owner_id = env::predecessor_account_id();
        let id = contract_tournament_id(&owner_id, &tournament_id);

        if let Some(price) = price {
            assert!(u128::from(price) > 0, "Tournaments with zero in prise are not allowed");
        }

        //specify the tornament struct that contains the owner ID
        let tournament = Tournament {
            tournament_id: tournament_id.clone(),
            //set the owner ID equal to the tournament owner ID passed into the function
            owner_id: owner_id.clone(),
            ended_at: None,
            started_at: None,
            created_at: env::block_timestamp(),
            players_number,
            price,
            access_nft_contract: nft_access_contract,
        };

        // insert the tournament ID and tournament struct and make sure that the tournament doesn't exist
        assert!(
            self.tournaments_by_id.insert(&id, &tournament).is_none(),
            "Tournament already exists"
        );

        // specify the tournament metadata struct
        let metadata = TournamentMetadata {
            name,
            media,
            summary,
        };

        // insert the tournament ID and metadata
        self.tournament_metadata_by_id.insert(&id, &metadata);

        let mut list = self.tournaments_by_owner.get(&owner_id).unwrap_or_else(||
            UnorderedSet::new(StorageKey::TournamentsByOwnerInner {
                accont_hash: env::sha256(owner_id.as_bytes()),
            })
        );
        list.insert(&tournament_id);
        self.tournaments_by_owner.insert(&owner_id, &list);

        self.internal_add_prize_owner(&id, &owner_id);

        (TournamentCreate {
            tournament_id: &tournament_id,
            owner_id: &owner_id,
            players_number: &players_number,
            price: &price,
        }).emit();
    }

    fn tournament_start(&mut self, tournament_id: TournamentId) {
        let owner_id = env::predecessor_account_id();
        let id = contract_tournament_id(&owner_id, &tournament_id);

        let mut tournament = self.assert_tournament_not_started(&id);
        let players = self.internal_get_players_number_in_tournament(&id);

        assert_eq!(owner_id, tournament.owner_id, "Owner's method");
        assert_eq!(players, tournament.players_number, "Tournament does not filled");

        tournament.started_at = Some(env::block_timestamp());

        self.tournaments_by_id.insert(&id, &tournament);

        (TournamentStart {
            tournament_id: &id,
            owner_id: &owner_id,
            date: &tournament.started_at.unwrap(),
        }).emit();
    }

    fn tournament_end(&mut self, tournament_id: TournamentId) {
        let owner_id = env::predecessor_account_id();
        let id = contract_tournament_id(&owner_id, &tournament_id);

        let mut tournament = self.assert_tournament_started(&id);
        assert!(tournament.ended_at.is_none(), "Already ended");

        assert_eq!(owner_id, tournament.owner_id, "Owner's method");

        tournament.ended_at = Some(env::block_timestamp());

        self.tournaments_by_id.insert(&id, &tournament);

        (TournamentEnd {
            tournament_id: &id,
            owner_id: &owner_id,
            date: &env::block_timestamp(),
        }).emit();
    }

    //add player to the tournament with NEAR depositing
    //#[payable]
    fn tournament_join(&mut self, tournament_id: TournamentId, owner_id: AccountId) {
        let id = contract_tournament_id(&owner_id, &tournament_id);

        let account_id: &AccountId = &env::predecessor_account_id();
        let attached_deposit: Balance = env::attached_deposit();

        let tournament = self.assert_tournament_not_started(&id);
        let price = tournament.price.expect("Unavailable");

        // check the is enough deposit attached to players account
        assert!(
            attached_deposit >= price.0,
            "Deposit is too small. Attached: {}, Required: {}",
            attached_deposit,
            price.0
        );

        // check for double participation
        assert!(
            self.internal_add_player_to_tournament(&tournament_id, &owner_id, &account_id),
            "Already in the tournament"
        );

        //get the refund amount from the attached deposit - required cost
        let refund = attached_deposit - price.0;

        //if the refund is greater than 1 yocto NEAR, we refund the predecessor that amount
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }

        Promise::new(tournament.owner_id).transfer(price.0);
    }

    fn tournament_add_prize(
        &mut self,
        tournament_id: TournamentId,
        owner_id: AccountId,
        place_number: u8,
        prize_id: String
    ) {
        let id = contract_tournament_id(&owner_id, &tournament_id);

        let attached_price = env::attached_deposit();
        assert!(attached_price >= MIN_PRIZE_NEAR, "Minimum 1 NEAR");

        let tournament = self.assert_tournament_not_started(&id);

        if tournament.owner_id != env::predecessor_account_id() {
            self.whitelist_prize_owners
                .get(&id)
                .unwrap()
                .assert_whitelist(&env::predecessor_account_id());
        }

        self.internal_tournament_add_prize(
            &id,
            &place_number,
            &prize_id,
            &(RewardPrize::Near {
                amount: U128::from(attached_price),
                owner_id: Some(env::predecessor_account_id()),
            })
        );
    }

    // refunds the prizes for the winners
    fn tournament_execute_reward(
        &mut self,
        tournament_id: TournamentId,
        winner_place: u8,
        account_id: AccountId
    ) {
        let owner_id = env::predecessor_account_id();
        let id = contract_tournament_id(&owner_id, &tournament_id);

        let tournament = self.assert_tournament_started(&id);
        let winner = account_id.clone();

        // check the owner calls this method
        assert_eq!(&owner_id, &tournament.owner_id, "Owner's method");

        let members = self.players_per_tournament.get(&id).expect("Not found");
        assert!(members.contains(&account_id), "Not found member");

        let tournament_place_id = tournament_place_id(&owner_id, &tournament_id, &winner_place);

        assert!(!self.prizes_per_place_rewarded.contains(&tournament_place_id), "Already rewarded");
        self.prizes_per_place_rewarded.insert(&tournament_place_id);

        //

        let prizes: UnorderedSet<TournamentPrizeId> = self.prizes_by_tournament
            .get(&id)
            .unwrap()
            .get(&winner_place)
            .unwrap();

        prizes
            .as_vector()
            .iter()
            .for_each(|prize_id| {
                let tournament_prize_id = tournament_prize_id(
                    &owner_id,
                    &tournament_id,
                    &winner_place,
                    &prize_id
                );
                let prize = self.prize_by_id.get(&tournament_prize_id).expect("Not found");

                match &prize {
                    RewardPrize::Near { amount, owner_id: _ } => {
                        Promise::new(winner.clone()).transfer(amount.0);
                    }
                    RewardPrize::Ft { amount, ft_contract_id, owner_id: _ } => {
                        ext_ft
                            ::ext(ft_contract_id.clone())
                            .with_static_gas(GAS_FOR_FT_TRANSFER)
                            .with_attached_deposit(1)
                            .ft_transfer(
                                winner.clone(),
                                amount.clone(),
                                Some(format!("Tournament {} place", winner_place.to_string()))
                            );
                    }
                    RewardPrize::Nft { token_id, nft_contract_id, owner_id: _ } => {
                        ext_nft
                            ::ext(nft_contract_id.clone())
                            .with_static_gas(GAS_FOR_NFT_TRANSFER)
                            .with_attached_deposit(1)
                            .nft_transfer(
                                winner.clone(),
                                token_id.clone(),
                                None,
                                Some(format!("Tournament {} place", winner_place.to_string()))
                            );
                    }
                }

                (TournamentReward {
                    tournament_id: &tournament_id,
                    owner_id: &tournament.owner_id,
                    place: &winner_place,
                    prize: &prize,
                }).emit();
            });
    }

    fn tournament_add_whitelist_prize_owner(
        &mut self,
        tournament_id: TournamentId,
        account_id: AccountId
    ) {
        let id = contract_tournament_id(&env::predecessor_account_id(), &tournament_id);
        let tournament = self.assert_tournament_not_started(&id);

        assert_eq!(&tournament.owner_id, &env::predecessor_account_id(), "Access denied");

        self.internal_add_prize_owner(&id, &account_id)
    }
}
