use near_sdk::{ ext_contract, AccountId };
use crate::nft_ido::{ TokenId, IdoId };

#[ext_contract(ext_self)]
trait ExtSelf {
    fn resolve_nft_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        ido_id: IdoId,
        contract_id: AccountId
    );
}