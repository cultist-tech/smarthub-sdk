use crate::escrow::{ EscrowResolver, EscrowFeature, EscrowEnum, EscrowOfferId };
use near_sdk::{ AccountId, env, PromiseResult };

impl EscrowResolver for EscrowFeature {
    fn resolve_accept_offer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        offer: EscrowEnum,
        offer_id: EscrowOfferId
    ) -> bool {
        match env::promise_result(0) {
            PromiseResult::NotReady => env::abort(),
            PromiseResult::Successful(_value) => {
              match offer {
                EscrowEnum::FtToFt {
                  ft_contract_id_out,
                  ft_contract_id_in: _,
                  amount_out,
                  amount_in: _,
                } => {
                  self.internal_resolve_offer_ft_to_unknown(
                    &owner_id,
                    &ft_contract_id_out,
                    &amount_out
                  );
                }
                EscrowEnum::FtToNft {
                  ft_contract_id_in: _,
                  amount_in: _,
                  nft_contract_id_out,
                  nft_token_id_out,
                } => {
                  self.internal_resolve_offer_nft_to_unknown(
                    &owner_id,
                    &nft_contract_id_out,
                    &nft_token_id_out
                  );
                }
                EscrowEnum::NftToFt {
                  ft_contract_id_out,
                  amount_out,
                  nft_token_id_in: _,
                  nft_contract_id_in: _,
                } => {
                  self.internal_resolve_offer_ft_to_unknown(
                    &owner_id,
                    &ft_contract_id_out,
                    &amount_out
                  );
                }
                EscrowEnum::NftToNft {
                  nft_token_id_in: _,
                  nft_contract_id_out,
                  nft_token_id_out,
                  nft_contract_id_in: _,
                } => {
                  self.internal_resolve_offer_nft_to_unknown(
                    &owner_id,
                    &nft_contract_id_out,
                    &nft_token_id_out
                  );
                }
              }

              true
            }
            PromiseResult::Failed => {
              self.internal_make_offer(&offer, &owner_id, &receiver_id, Some(offer_id));

              false
            }
        }
    }

    fn resolve_remove_offer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        offer: EscrowEnum,
        offer_id: EscrowOfferId
    ) -> bool {
        match env::promise_result(0) {
            PromiseResult::NotReady => env::abort(),
            PromiseResult::Successful(_value) => { true }
            PromiseResult::Failed => {
                self.internal_make_offer(&offer, &owner_id, &receiver_id, Some(offer_id));

                false
            }
        }
    }
}
