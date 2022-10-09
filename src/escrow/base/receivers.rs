use crate::escrow::{
    EscrowFeature,
    EscrowOnFtTransferArgs,
    EscrowOnNftTransferArgs,
    TokenId,
    EscrowEnum,
};
use crate::escrow::base::internal::{GAS_FOR_RESOLVE_ACCEPT};
use near_sdk::{ PromiseOrValue, AccountId, env, require };
use near_sdk::json_types::U128;

impl EscrowFeature {
    pub fn internal_on_ft_transfer(
        &mut self,
        args: &EscrowOnFtTransferArgs,
        ft_contract_id: &AccountId,
        amount: &U128,
        sender_id: &AccountId
    ) -> PromiseOrValue<U128> {
        let EscrowOnFtTransferArgs {
            ft_contract_id_out,
            ft_amount_out,
            nft_contract_id_out,
            nft_token_id_out,
            receiver_id,
            offer_id,
        } = args;

        if let Some(receiver_id) = receiver_id {
            assert_ne!(&sender_id, &receiver_id, "Not self");

            if let Some(ft_contract_id_out) = ft_contract_id_out {
                if let Some(ft_amount_out) = ft_amount_out {
                    assert_ne!(&ft_contract_id_out, &ft_contract_id, "Ft contracts does not equal");

                    self.internal_make_offer(
                        &(EscrowEnum::FtToFt {
                            ft_contract_id_in: ft_contract_id.clone(),
                            ft_contract_id_out: ft_contract_id_out.clone(),
                            amount_in: amount.clone(),
                            amount_out: ft_amount_out.clone(),
                        }),
                        &sender_id,
                        &receiver_id,
                        None
                    );

                    return PromiseOrValue::Value(U128(0));
                }
            }
            if let Some(nft_contract_id_out) = nft_contract_id_out {
                if let Some(nft_token_id_out) = nft_token_id_out {
                    self.internal_make_offer(
                        &(EscrowEnum::FtToNft {
                            ft_contract_id_in: ft_contract_id.clone(),
                            nft_contract_id_out: nft_contract_id_out.clone(),
                            amount_in: amount.clone(),
                            nft_token_id_out: nft_token_id_out.clone(),
                        }),
                        &sender_id,
                        &receiver_id,
                        None
                    );

                    return PromiseOrValue::Value(U128(0));
                }
            }

            env::panic_str(&"Invalid params");
        } else if let Some(offer_id) = offer_id {
          require!(
              env::prepaid_gas() > GAS_FOR_RESOLVE_ACCEPT,
              "More gas is required"
            );

            let owner_id = self.offer_owner_by_account.get(&offer_id).unwrap();
            self.internal_accept_offer_unknown_to_ft(
                &owner_id,
                &sender_id,
                &offer_id,
                &ft_contract_id,
                &amount
            );
        }

        return PromiseOrValue::Value(U128(0));
    }

    pub fn internal_on_nft_transfer(
        &mut self,
        args: &EscrowOnNftTransferArgs,
        contract_id: &AccountId,
        token_id: &TokenId,
        sender_id: &AccountId
    ) -> PromiseOrValue<bool> {
        let EscrowOnNftTransferArgs {
            ft_contract_id_out,
            ft_amount_out,
            nft_contract_id_out,
            nft_token_id_out,
            receiver_id,
            offer_id,
        } = args;

        if let Some(receiver_id) = receiver_id {
            assert_ne!(&sender_id, &receiver_id, "Not self");

            if let Some(ft_contract_id_out) = ft_contract_id_out {
                if let Some(ft_amount_out) = ft_amount_out {
                    self.internal_make_offer(
                        &(EscrowEnum::NftToFt {
                            nft_contract_id_in: contract_id.clone(),
                            ft_contract_id_out: ft_contract_id_out.clone(),
                            nft_token_id_in: token_id.clone(),
                            amount_out: ft_amount_out.clone(),
                        }),
                        &sender_id,
                        &receiver_id,
                        None
                    );

                    return PromiseOrValue::Value(false);
                }
            }
            if let Some(nft_contract_id_out) = nft_contract_id_out {
                if let Some(nft_token_id_out) = nft_token_id_out {
                    self.internal_make_offer(
                        &(EscrowEnum::NftToNft {
                            nft_contract_id_in: contract_id.clone(),
                            nft_contract_id_out: nft_contract_id_out.clone(),
                            nft_token_id_in: token_id.clone(),
                            nft_token_id_out: nft_token_id_out.clone(),
                        }),
                        &sender_id,
                        &receiver_id,
                        None
                    );

                    return PromiseOrValue::Value(false);
                }
            }

            env::panic_str(&"Invalid params");
        } else if let Some(offer_id) = offer_id {
          require!(
              env::prepaid_gas() > GAS_FOR_RESOLVE_ACCEPT,
              "More gas is required"
            );

            let owner_id = self.offer_owner_by_account.get(&offer_id).unwrap();
            self.internal_accept_offer_unknown_to_nft(
                &owner_id,
                &sender_id,
                &offer_id,
                &contract_id,
                &token_id
            );
        }

        PromiseOrValue::Value(false)
    }
}
