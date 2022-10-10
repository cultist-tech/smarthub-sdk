use crate::nft_ido::{ NftIdoFeature, TokenId, IdoId };
use near_sdk::{ AccountId, is_promise_success, Promise, ext_contract };
use crate::nft_ido::utils::contract_token_id;
use crate::ft::base::external::{ ext_ft };
use crate::ft::base::core_impl::GAS_FOR_FT_TRANSFER;

#[ext_contract(ext_self)]
pub trait NftIdoResolvers {
  fn resolve_nft_transfer(
    &mut self,
    sender_id: AccountId,
    receiver_id: AccountId,
    token_id: TokenId,
    ido_id: IdoId,
    contract_id: AccountId
  ) -> bool;
}

impl NftIdoResolvers for NftIdoFeature {
    fn resolve_nft_transfer(
        &mut self,
        sender_id: AccountId,
        _receiver_id: AccountId,
        _token_id: TokenId,
        ido_id: IdoId,
        contract_id: AccountId
    ) -> bool {
        let id = contract_token_id(&contract_id, &ido_id);
        let ido = self.ido_by_id.get(&id).expect("Not found ido");
        let ft_token = self.ido_by_ft_token.get(&id);
        let price = ido.price;

        if !is_promise_success() {
            if let Some(ft_token) = ft_token {
                ext_ft
                    ::ext(ft_token.clone())
                    .with_static_gas(GAS_FOR_FT_TRANSFER)
                    .with_attached_deposit(1)
                    .ft_transfer(sender_id.clone(), price, None);
            } else {
                Promise::new(sender_id.clone()).transfer(price.0);
            }

            return false;
        }

        if let Some(ft_token) = ft_token {
            ext_ft
                ::ext(ft_token.clone())
                .with_static_gas(GAS_FOR_FT_TRANSFER)
                .with_attached_deposit(1)
                .ft_transfer(ido.contract_id.clone(), price.clone(), None);
        } else {
            Promise::new(ido.contract_id.clone()).transfer(price.0);
        }

        true
    }
}
