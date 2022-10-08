use crate::tournament::{ TournamentFactory, TournamentId, TokenId };
use crate::tournament::nft_access::TournamentFactoryNftAccess;
use near_sdk::collections::UnorderedSet;
use near_sdk::{ env, AccountId, BorshStorageKey };
use crate::tournament::events::{ TournamentAddNftAccess };
use crate::tournament::utils::contract_tournament_id;
use near_sdk::borsh::{ self, BorshSerialize };

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    TournamentAccessNftInner {
        tournament_hash: Vec<u8>,
    },
}

impl TournamentFactoryNftAccess for TournamentFactory {
    fn tournament_nft_access(
        &self,
        tournament_id: TournamentId,
        owner_id: AccountId
    ) -> Vec<TokenId> {
        self.internal_nft_access(&tournament_id, &owner_id)
    }

    fn tournament_add_nft_access(&mut self, tournament_id: TournamentId, token_ids: Vec<TokenId>) {
        let owner_id = env::predecessor_account_id();
        let id = contract_tournament_id(&owner_id, &tournament_id);

        let tournament = self.assert_tournament_not_started(&id);

        // check the owner calls this method
        assert_eq!(&owner_id, &tournament.owner_id, "Owner's method");


        assert!(tournament.access_nft_contract.is_some(), "Nft access not available");

        let mut arr = self.tournament_access_nft.get(&id).unwrap_or_else(|| {
            UnorderedSet::new(StorageKey::TournamentAccessNftInner {
                tournament_hash: env::sha256(id.as_bytes()),
            })
        });

        token_ids.iter().for_each(|el| {
            arr.insert(&el);
        });

        self.tournament_access_nft.insert(&id, &arr);

        (TournamentAddNftAccess {
            tournament_id: &tournament_id,
            token_ids: &token_ids,
            owner_id: &owner_id,
        }).emit()
    }
}
