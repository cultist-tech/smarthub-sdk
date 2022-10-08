use crate::rent::{ RentFeature, RentOnNftApproveArgs, TokenId };
use near_sdk::{ AccountId, Promise, PromiseOrValue };
use crate::rent::meta::RentOnFtTransferArgs;
use near_sdk::json_types::{ U128 };
use crate::utils::near_ft;

impl RentFeature {
    pub fn internal_on_nft_approve(
        &mut self,
        args: &RentOnNftApproveArgs,
        contract_id: &AccountId,
        token_id: &TokenId,
        owner_id: &AccountId
    ) -> PromiseOrValue<String> {
        let RentOnNftApproveArgs { sale_conditions, max_time, min_time } = args;

        self.internal_rent_add(
            &contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &min_time,
            &max_time
        );

        PromiseOrValue::Value("true".to_string())
    }

    pub fn internal_on_ft_transfer(
        &mut self,
        args: &RentOnFtTransferArgs,
        ft_token_id: &AccountId,
        amount: &U128,
        sender_id: &AccountId
    ) -> PromiseOrValue<U128> {
        let RentOnFtTransferArgs { token_id, contract_id, receiver_id, time } = args;

        self.internal_process_purchase(
            &contract_id,
            &token_id,
            &sender_id,
            &receiver_id,
            &time,
            &ft_token_id,
            &amount
        ).into()
    }
}
