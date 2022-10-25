use crate::nft_ido::{ NftIdoFeature, NftIdoOnFtTransferArgs, NftIdoOnNftTransferArgs, TokenId };
use near_sdk::{ env, AccountId, PromiseOrValue };
use crate::nft_ido::utils::contract_token_id;
use crate::nft_ido::events::IdoAddToken;
use near_sdk::json_types::U128;

impl NftIdoFeature {
    pub fn internal_on_ft_transfer(
        &mut self,
        args: &NftIdoOnFtTransferArgs,
        ft_token_id: &AccountId,
        attached_money: &U128,
        sender_id: &AccountId
    ) -> PromiseOrValue<U128> {
        let NftIdoOnFtTransferArgs { ido_id, contract_id, receiver_id, mint_amount } = args;

        let id = contract_token_id(&contract_id, &ido_id);
        let sale = self.ido_by_id.get(&id).expect("Not found sale");
        let ft_token = self.ido_by_ft_token.get(&id).expect("Mint only with NEAR");

        assert_eq!(
            sale.price.0 * (mint_amount.clone() as u128),
            attached_money.0,
            "Invalid attached price"
        );
        assert_eq!(ft_token_id, &ft_token, "Unavailable ft");

        self.internal_random_mint(
            &env::signer_account_id(),
            &contract_id,
            &ido_id,
            &receiver_id,
            &mint_amount
        );

        PromiseOrValue::Value(U128(0))
    }

    pub fn internal_on_nft_transfer(
        &mut self,
        args: &NftIdoOnNftTransferArgs,
        contract_id: &AccountId,
        token_id: &TokenId,
        sender_id: &AccountId
    ) -> PromiseOrValue<bool> {
        let NftIdoOnNftTransferArgs { ido_id } = args;

        self.internal_ido_add_token(&contract_id, &ido_id, &token_id);
        
        (IdoAddToken {
            ido_id: &ido_id,
            contract_id: &contract_id,
            token_id: &token_id,
        }).emit();

        PromiseOrValue::Value(false)
    }
}
