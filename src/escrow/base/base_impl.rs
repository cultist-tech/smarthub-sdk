use near_sdk::{ env, AccountId, IntoStorageKey };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::{ TreeMap, LookupMap, UnorderedSet };
use crate::escrow::base::{ EscrowCore, EscrowEnumeration };
use near_sdk::json_types::U128;
use crate::escrow::metadata::{ EscrowEnum, JsonEscrow };
use crate::escrow::{ EscrowOfferId };
use crate::utils::assert_tx_money;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct EscrowFeature {
    pub offer_accepted_by_id: LookupMap<EscrowOfferId, bool>,

    pub offer_by_id: TreeMap<EscrowOfferId, EscrowEnum>,
    pub offers_by_account: TreeMap<AccountId, UnorderedSet<EscrowOfferId>>,
    pub offers_for_account: TreeMap<AccountId, UnorderedSet<EscrowOfferId>>,

    pub offer_owner_by_account: LookupMap<EscrowOfferId, AccountId>,
    pub offer_receiver_by_account: LookupMap<EscrowOfferId, AccountId>,
}

impl EscrowFeature {
    pub fn new<O, OT, OF, OO, OR, OA>(
        offer_prefix: O,
        offer_to_account_prefix: OT,
        offer_from_account_prefix: OF,
        offer_owner_account_prefix: OO,
        offer_receiver_account_prefix: OR,
        offer_accepted_prefix: OA
    )
        -> Self
        where
            O: IntoStorageKey,
            OT: IntoStorageKey,
            OF: IntoStorageKey,
            OO: IntoStorageKey,
            OR: IntoStorageKey,
            OA: IntoStorageKey
    {
        let this = Self {
            offer_accepted_by_id: LookupMap::new(offer_accepted_prefix),
            offer_by_id: TreeMap::new(offer_prefix),
            offers_by_account: TreeMap::new(offer_to_account_prefix),
            offers_for_account: TreeMap::new(offer_from_account_prefix),
            offer_owner_by_account: LookupMap::new(offer_owner_account_prefix),
            offer_receiver_by_account: LookupMap::new(offer_receiver_account_prefix),
        };

        this
    }
}

impl EscrowCore for EscrowFeature {
    fn escrow_remove_offer(&mut self, offer_id: EscrowOfferId) {
        assert_tx_money();

        let signer_id = env::predecessor_account_id();
        let owner_id = self.offer_owner_by_account.get(&offer_id).expect("Not found");
        let receiver_id = self.offer_receiver_by_account.get(&offer_id).expect("Not found");

        assert_eq!(owner_id, signer_id, "Unauthorized");

        self.internal_withdraw_offer(&offer_id, &owner_id, &receiver_id)
    }
}

impl EscrowEnumeration for EscrowFeature {
    fn escrow_offer(&self, offer_id: EscrowOfferId) -> Option<JsonEscrow> {
        self.enum_get_offer(&offer_id)
    }

    fn escrow_offers_by_owner(
        &self,
        account_id: AccountId,
        limit: Option<u64>,
        offset: Option<U128>
    ) -> Vec<JsonEscrow> {
        self.internal_find_offers_by_owner(&account_id, &limit, &offset)
    }

    fn escrow_offers_for_owner(
        &self,
        account_id: AccountId,
        limit: Option<u64>,
        offset: Option<U128>
    ) -> Vec<JsonEscrow> {
        self.internal_find_offers_for_owner(&account_id, &limit, &offset)
    }

    fn escrow_offers_total_by_owner(&self, account_id: AccountId) -> u64 {
        self.internal_total_offers_by_owner(&account_id)
    }

    fn escrow_offers_total_for_owner(&self, account_id: AccountId) -> u64 {
        self.internal_total_offers_for_owner(&account_id)
    }
}
