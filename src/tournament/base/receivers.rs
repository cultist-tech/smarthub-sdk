use crate::tournament::{
    TournamentFactory,
    TournamentOnNftTransferArgs,
    TokenId,
    RewardPrize,
    TournamentOnFtTransferArgs,
};
use near_sdk::{ AccountId, PromiseOrValue };
use crate::tournament::utils::contract_tournament_id;
use near_sdk::json_types::U128;

impl TournamentFactory {
    pub fn internal_on_ft_transfer(
        &mut self,
        args: &TournamentOnFtTransferArgs,
        ft_contract_id: &AccountId,
        amount: &U128,
        sender_id: &AccountId
    ) -> PromiseOrValue<U128> {
        let TournamentOnFtTransferArgs { tournament_id, place, owner_id, prize_id } = args;
        let id = contract_tournament_id(&owner_id, &tournament_id);

        if let Some(place) = place {
            let tournament = self.assert_tournament_not_started(&id);
            assert_eq!(owner_id, &tournament.owner_id, "Owner's method");

            self.internal_tournament_add_prize(
                &id,
                &place,
                &prize_id.clone().expect("Invalid args"),
                &(RewardPrize::Ft {
                    amount: amount.clone(),
                    ft_contract_id: ft_contract_id.clone(),
                    owner_id: Some(owner_id.clone()),
                })
            );
        } else {
            unimplemented!();
        }

        PromiseOrValue::Value(U128::from(0))
    }

    pub fn internal_on_nft_transfer(
        &mut self,
        args: &TournamentOnNftTransferArgs,
        nft_contract_id: &AccountId,
        token_id: &TokenId,
        account_id: &AccountId
    ) -> PromiseOrValue<bool> {
        let TournamentOnNftTransferArgs { tournament_id, place, owner_id, prize_id } = args;
        let id = contract_tournament_id(&owner_id, &tournament_id);

        if let Some(place) = place {
            let tournament = self.assert_tournament_not_started(&id);
            assert_eq!(account_id, &tournament.owner_id, "Owner's method");
            assert_eq!(
                nft_contract_id,
                &tournament.access_nft_contract.expect("Nft access not available"),
                "Invalid contract"
            );

            self.internal_tournament_add_prize(
                &id,
                &place,
                &prize_id.clone().expect("Invalid args"),
                &(RewardPrize::Nft {
                    token_id: token_id.clone(),
                    nft_contract_id: nft_contract_id.clone(),
                    owner_id: Some(account_id.clone()),
                })
            );
        } else {
            self.internal_use_nft_access(
                &tournament_id,
                &owner_id,
                &token_id,
                &account_id,
                &nft_contract_id
            );
        }

        PromiseOrValue::Value(false)
    }
}
