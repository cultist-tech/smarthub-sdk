#[macro_export]
macro_rules! impl_escrow_core {
    ($contract:ident, $tokens:ident, $assert_use:ident) => {
        use $crate::escrow::{EscrowResolver, EscrowCore};

        #[near_bindgen]
        impl EscrowCore for $contract {
          #[payable]
          fn escrow_remove_offer(&mut self, offer_id: $crate::escrow::EscrowOfferId) {
            self.$assert_use();

            self.$tokens.escrow_remove_offer(offer_id)
          }
        }

        #[near_bindgen]
        impl EscrowResolver for $contract {
          #[private]
          fn resolve_accept_offer(&mut self, owner_id: AccountId, receiver_id: AccountId, offer: $crate::escrow::EscrowEnum, offer_id: $crate::escrow::EscrowOfferId) -> bool {
            self.$tokens.resolve_accept_offer(owner_id, receiver_id, offer, offer_id)
          }
           #[private]
          fn resolve_remove_offer(&mut self, owner_id: AccountId, receiver_id: AccountId, offer: $crate::escrow::EscrowEnum, offer_id: $crate::escrow::EscrowOfferId) -> bool {
            self.$tokens.resolve_remove_offer(owner_id, receiver_id, offer, offer_id)
          }
        }
    };
}

#[macro_export]
macro_rules! impl_escrow_enumeration {
    ($contract:ident, $tokens:ident) => {
        use $crate::escrow::{JsonEscrow, EscrowEnumeration};

        #[near_bindgen]
        impl EscrowEnumeration for $contract {
          fn escrow_offer(&self, offer_id: $crate::escrow::EscrowOfferId) -> Option<JsonEscrow> {
            self.$tokens.escrow_offer(offer_id)
          }
          fn escrow_offers_by_owner(&self, account_id: AccountId, limit: Option<u64>, offset: Option<U128>) -> Vec<JsonEscrow> {
            self.$tokens.escrow_offers_by_owner(account_id, limit, offset)
          }
          fn escrow_offers_for_owner(&self, account_id: AccountId, limit: Option<u64>, offset: Option<U128>) -> Vec<JsonEscrow> {
            self.$tokens.escrow_offers_for_owner(account_id, limit, offset)
          }
          fn escrow_offers_total_by_owner(&self, account_id: AccountId) -> u64 {
            self.$tokens.escrow_offers_total_by_owner(account_id)
          }
          fn escrow_offers_total_for_owner(&self, account_id: AccountId) -> u64 {
            self.$tokens.escrow_offers_total_for_owner(account_id)
          }
        }
    };
}
