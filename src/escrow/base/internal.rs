use crate::escrow::{ EscrowFeature, JsonEscrow, EscrowEnum, ContractId, TokenId, EscrowOfferId };
use near_sdk::{ AccountId, require, env, Gas, Promise, BorshStorageKey };
use near_sdk::json_types::U128;
use near_sdk::collections::{ UnorderedSet };
use near_sdk::borsh::{ self, BorshSerialize };
use crate::nft::{ ext_nft };
use crate::ft::{ ext_ft };
use crate::ft::base::core_impl::GAS_FOR_FT_TRANSFER;
use crate::nft::base::GAS_FOR_NFT_TRANSFER;
use crate::escrow::base::{ ext_self };

const GAS_FOR_RESOLVE_REMOVE: Gas = Gas(15_000_000_000_000);
const GAS_FOR_RESOLVE_ACCEPT: Gas = Gas(15_000_000_000_000);

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    EscrowOffersByAccount {
        account_hash: Vec<u8>,
    },
    EscrowOffersForAccount {
        account_hash: Vec<u8>,
    },
}

impl EscrowFeature {
    //

    pub(crate) fn internal_make_offer(
        &mut self,
        offer: &EscrowEnum,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        offer_id: Option<EscrowOfferId>
    ) -> JsonEscrow {
        let total = self.internal_total_offers_by_owner(&sender_id);
        let offer_id = offer_id.unwrap_or_else(|| format!("{}-{}", sender_id.clone(), total + 1));

        let mut sender_offers = self.offers_by_account.get(&sender_id).unwrap_or_else(|| {
            UnorderedSet::new(StorageKey::EscrowOffersByAccount {
                account_hash: env::sha256(sender_id.as_bytes()),
            })
        });
        let mut receiver_offers = self.offers_for_account.get(&receiver_id).unwrap_or_else(|| {
            UnorderedSet::new(StorageKey::EscrowOffersForAccount {
                account_hash: env::sha256(receiver_id.as_bytes()),
            })
        });

        sender_offers.insert(&offer_id);
        receiver_offers.insert(&offer_id);

        self.offer_by_id.insert(&offer_id, &offer);
        self.offers_by_account.insert(&sender_id, &sender_offers);
        self.offers_for_account.insert(&receiver_id, &receiver_offers);
        self.offer_owner_by_account.insert(&offer_id, &sender_id);
        self.offer_receiver_by_account.insert(&offer_id, &receiver_id);

        JsonEscrow {
            offer_id,
            sender_id: sender_id.clone(),
            receiver_id: receiver_id.clone(),
            data: offer.clone(),
            is_accepted: false,
        }
    }

    pub(crate) fn internal_remove_offer(&mut self, offer_id: &EscrowOfferId) {
        let owner_id = self.offer_owner_by_account.get(&offer_id).expect("Not found");
        let receiver_id = self.offer_receiver_by_account.get(&offer_id).expect("Not found");

        let mut owner_offers = self.offers_by_account.get(&owner_id).expect("Not found sender");
        let mut receiver_offers = self.offers_for_account
            .get(&receiver_id)
            .expect("Not found receiver");

        owner_offers.remove(&offer_id);
        receiver_offers.remove(&offer_id);

        self.offer_by_id.remove(offer_id);
        self.offer_owner_by_account.remove(&offer_id);
        self.offer_receiver_by_account.remove(&offer_id);
        self.offers_by_account.insert(&owner_id, &owner_offers);
        self.offers_for_account.insert(&receiver_id, &receiver_offers);
    }

    pub(crate) fn internal_withdraw_offer(
        &mut self,
        offer_id: &EscrowOfferId,
        owner_id: &AccountId,
        receiver_id: &AccountId
    ) {
        let offer = self.offer_by_id.get(&offer_id).expect("Not found");

        self.internal_remove_offer(&offer_id);

        match &offer {
            EscrowEnum::FtToFt {
                ft_contract_id_in,
                amount_in,
                ft_contract_id_out: _,
                amount_out: _,
            } => {
                self.internal_transfer_ft(&owner_id, &ft_contract_id_in, &amount_in).then(
                    ext_self
                        ::ext(env::current_account_id())
                        .with_static_gas(GAS_FOR_RESOLVE_REMOVE)
                        .resolve_remove_offer(
                            owner_id.clone(),
                            receiver_id.clone(),
                            offer.clone(),
                            offer_id.clone()
                        )
                );
            }
            EscrowEnum::FtToNft {
                ft_contract_id_in,
                amount_in,
                nft_token_id_out: _,
                nft_contract_id_out: _,
            } => {
                self.internal_transfer_ft(&owner_id, &ft_contract_id_in, &amount_in).then(
                    ext_self
                        ::ext(env::current_account_id())
                        .with_static_gas(GAS_FOR_RESOLVE_REMOVE)
                        .resolve_remove_offer(
                            owner_id.clone(),
                            receiver_id.clone(),
                            offer.clone(),
                            offer_id.clone()
                        )
                );
            }
            EscrowEnum::NftToFt {
                nft_contract_id_in,
                nft_token_id_in,
                ft_contract_id_out: _,
                amount_out: _,
            } => {
                self.internal_transfer_nft(&owner_id, &nft_contract_id_in, &nft_token_id_in).then(
                    ext_self
                        ::ext(env::current_account_id())
                        .with_static_gas(GAS_FOR_RESOLVE_REMOVE)
                        .resolve_remove_offer(
                            owner_id.clone(),
                            receiver_id.clone(),
                            offer.clone(),
                            offer_id.clone()
                        )
                );
            }
            EscrowEnum::NftToNft {
                nft_contract_id_in,
                nft_token_id_in,
                nft_contract_id_out: _,
                nft_token_id_out: _,
            } => {
                self.internal_transfer_nft(&owner_id, &nft_contract_id_in, &nft_token_id_in).then(
                    ext_self
                        ::ext(env::current_account_id())
                        .with_static_gas(GAS_FOR_RESOLVE_REMOVE)
                        .resolve_remove_offer(
                            owner_id.clone(),
                            receiver_id.clone(),
                            offer.clone(),
                            offer_id.clone()
                        )
                );
            }
        }
    }

    //

    pub(crate) fn internal_transfer_ft(
        &mut self,
        receiver_id: &AccountId,
        ft_contract_id: &ContractId,
        amount: &U128
    ) -> Promise {
        ext_ft
            ::ext(ft_contract_id.clone())
            .with_static_gas(GAS_FOR_FT_TRANSFER)
            .with_attached_deposit(1)
            .ft_transfer(receiver_id.clone(), amount.clone(), Some("Escrow transfer".to_string()))
    }

    pub(crate) fn internal_transfer_nft(
        &mut self,
        receiver_id: &AccountId,
        nft_contract_id: &ContractId,
        token_id: &TokenId
    ) -> Promise {
        ext_nft
            ::ext(nft_contract_id.clone())
            .with_static_gas(GAS_FOR_NFT_TRANSFER)
            .with_attached_deposit(1)
            .nft_transfer(
                receiver_id.clone(),
                token_id.clone(),
                None,
                Some("Escrow transfer".to_string())
            )
    }

    //

    pub(crate) fn internal_accept_offer_unknown_to_ft(
        &mut self,
        owner_id: &AccountId,
        receiver_id: &AccountId,
        offer_id: &EscrowOfferId,
        ft_contract_id: &ContractId,
        amount: &U128
    ) {
        let offer = self.offer_by_id.get(&offer_id).expect("Not found offer");
        let receiver_offers = self.offers_for_account.get(&receiver_id).expect("Not found offers");

        assert!(receiver_offers.contains(&offer_id), "Not found offer");

        match &offer {
            EscrowEnum::FtToFt { ft_contract_id_out, amount_out, ft_contract_id_in, amount_in } => {
                assert_eq!(&ft_contract_id_out, &ft_contract_id);
                assert_eq!(&amount_out, &amount);

                self.internal_remove_offer(&offer_id);

                self.internal_transfer_ft(&receiver_id, &ft_contract_id_in, &amount_in).then(
                    ext_self
                        ::ext(env::current_account_id())
                        .with_static_gas(GAS_FOR_RESOLVE_ACCEPT)
                        .resolve_accept_offer(
                            owner_id.clone(),
                            receiver_id.clone(),
                            offer.clone(),
                            offer_id.clone()
                        )
                );
            }
            EscrowEnum::NftToFt {
                nft_token_id_in,
                nft_contract_id_in,
                ft_contract_id_out,
                amount_out,
            } => {
                assert_eq!(&ft_contract_id_out, &ft_contract_id);
                assert_eq!(&amount_out, &amount);

                self.internal_remove_offer(&offer_id);

                self.internal_transfer_nft(
                    &receiver_id,
                    &nft_contract_id_in,
                    &nft_token_id_in
                ).then(
                    ext_self
                        ::ext(env::current_account_id())
                        .with_static_gas(GAS_FOR_RESOLVE_ACCEPT)
                        .resolve_accept_offer(
                            owner_id.clone(),
                            receiver_id.clone(),
                            offer.clone(),
                            offer_id.clone()
                        )
                );
            }
            _ => {
                env::panic_str(&"Method unresolved");
            }
        }
    }

    pub(crate) fn internal_accept_offer_unknown_to_nft(
        &mut self,
        owner_id: &AccountId,
        receiver_id: &AccountId,
        offer_id: &EscrowOfferId,
        nft_contract_id: &ContractId,
        token_id: &TokenId
    ) {
        let offer = self.offer_by_id.get(&offer_id).expect("Not found offer");
        let receiver_offers = self.offers_for_account.get(&receiver_id).expect("Not found offers");

        assert!(receiver_offers.contains(&offer_id), "Not found offer");

        match &offer {
            EscrowEnum::FtToNft {
                nft_token_id_out,
                nft_contract_id_out,
                ft_contract_id_in,
                amount_in,
            } => {
                assert_eq!(&nft_contract_id_out, &nft_contract_id);
                assert_eq!(&nft_token_id_out, &token_id);

                self.internal_remove_offer(&offer_id);

                self.internal_transfer_ft(&receiver_id, &ft_contract_id_in, &amount_in).then(
                    ext_self
                        ::ext(env::current_account_id())
                        .with_static_gas(GAS_FOR_RESOLVE_ACCEPT)
                        .resolve_accept_offer(
                            owner_id.clone(),
                            receiver_id.clone(),
                            offer.clone(),
                            offer_id.clone()
                        )
                );
            }
            EscrowEnum::NftToNft {
                nft_token_id_in,
                nft_contract_id_in,
                nft_token_id_out,
                nft_contract_id_out,
            } => {
                assert_eq!(&nft_contract_id_out, &nft_contract_id);
                assert_eq!(&nft_token_id_out, &token_id);

                self.internal_remove_offer(&offer_id);

                self.internal_transfer_nft(
                    &receiver_id,
                    &nft_contract_id_in,
                    &nft_token_id_in
                ).then(
                    ext_self
                        ::ext(env::current_account_id())
                        .with_static_gas(GAS_FOR_RESOLVE_ACCEPT)
                        .resolve_accept_offer(
                            owner_id.clone(),
                            receiver_id.clone(),
                            offer.clone(),
                            offer_id.clone()
                        )
                );
            }
            _ => {
                env::panic_str(&"Method unresolved");
            }
        }
    }

    pub(crate) fn internal_resolve_offer_ft_to_unknown(
        &mut self,
        owner_id: &AccountId,
        ft_contract_id: &AccountId,
        amount: &U128
    ) {
        self.internal_transfer_ft(&owner_id, &ft_contract_id, &amount);
    }

    pub(crate) fn internal_resolve_offer_nft_to_unknown(
        &mut self,
        owner_id: &AccountId,
        nft_contract_id: &AccountId,
        token_id: &TokenId
    ) {
        self.internal_transfer_nft(&owner_id, &nft_contract_id, &token_id);
    }

    //

    pub(crate) fn enum_get_offer(&self, offer_id: &EscrowOfferId) -> Option<JsonEscrow> {
        let offer = self.offer_by_id.get(&offer_id);

        if let Some(offer) = offer {
            return Some(JsonEscrow {
                offer_id: offer_id.clone(),
                data: offer,
                receiver_id: self.offer_receiver_by_account.get(&offer_id).expect("Not receiver"),
                sender_id: self.offer_owner_by_account.get(&offer_id).expect("Not found owner"),
                is_accepted: self.offer_accepted_by_id.get(&offer_id).unwrap_or_else(|| false),
            });
        }

        None
    }

    pub(crate) fn internal_find_offers_by_owner(
        &self,
        account_id: &AccountId,
        limit: &Option<u64>,
        offset: &Option<U128>
    ) -> Vec<JsonEscrow> {
        let arr = if let Some(arr) = self.offers_by_account.get(&account_id) {
            arr
        } else {
            return vec![];
        };

        let start_index: u128 = offset.map(From::from).unwrap_or_default();

        if (arr.len() as u128) <= start_index {
            return vec![];
        }

        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        require!(limit != 0, "Cannot provide limit of 0.");

        arr.iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|offer_id| { self.enum_get_offer(&offer_id).expect("Not found offer") })
            .collect()
    }

    pub(crate) fn internal_find_offers_for_owner(
        &self,
        account_id: &AccountId,
        limit: &Option<u64>,
        offset: &Option<U128>
    ) -> Vec<JsonEscrow> {
        let arr = if let Some(arr) = self.offers_for_account.get(&account_id) {
            arr
        } else {
            return vec![];
        };

        let start_index: u128 = offset.map(From::from).unwrap_or_default();

        if (arr.len() as u128) <= start_index {
            return vec![];
        }

        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        require!(limit != 0, "Cannot provide limit of 0.");

        arr.iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|offer_id| { self.enum_get_offer(&offer_id).expect("Not found offer") })
            .collect()
    }

    pub(crate) fn internal_total_offers_by_owner(&self, account_id: &AccountId) -> u64 {
        let offers = self.offers_by_account.get(&account_id);

        if let Some(offers) = offers {
            return offers.len();
        }

        0
    }

    pub(crate) fn internal_total_offers_for_owner(&self, account_id: &AccountId) -> u64 {
        let offers = self.offers_for_account.get(&account_id);

        if let Some(offers) = offers {
            return offers.len();
        }

        0
    }
}
