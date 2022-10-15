// pub(crate) fn log_event(method: &String, data: String) {
//   env::log_str(&format!("{}{}{}{}{}", "EVENT_JSON:{\"standard\":\"mfight_rent\",\"version\":\"1.0.0\",\"event\":\"", method.to_string(), "\",\"data\": [", data, "] }"));
// }

//! Standard for nep171 (Non-Fungible Token) events.
//!
//! These events will be picked up by the NEAR indexer.
//!
//! <https://github.com/near/NEPs/blob/69f76c6c78c2ebf05d856347c9c98ae48ad84ebd/specs/Standards/NonFungibleToken/Event.md>
//!
//! This is an extension of the events format (nep-297):
//! <https://github.com/near/NEPs/blob/master/specs/Standards/EventsFormat.md>
//!
//! The three events in this standard are [`NftMint`], [`NftTransfer`], and [`NftBurn`].
//!
//! These events can be logged by calling `.emit()` on them if a single event, or calling
//! [`NftMint::emit_many`], [`NftTransfer::emit_many`],
//! or [`NftBurn::emit_many`] respectively.

use crate::event::NearEvent;
use near_sdk::AccountId;
use serde::Serialize;
use near_sdk::json_types::U128;
use crate::rent::TokenId;
use crate::rent::meta::SaleConditions;

/// Data to log for an NFT mint event. To log this event, call [`.emit()`](NftMint::emit).
#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct RentAdd<'a> {
    pub token_id: &'a TokenId,
    pub contract_id: &'a AccountId,
    pub owner_id: &'a AccountId,
    pub sale_conditions: &'a SaleConditions,
    pub min_time: &'a u64,
    pub max_time: &'a u64,
    pub created_at: &'a u64,
}

impl RentAdd<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an nft mint event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`NftMint`] represents the data of each mint.
    pub fn emit_many(data: &[RentAdd<'_>]) {
        new_mfight_rent_v1(MfRentEventKind::RentAdd(data)).emit()
    }
}

//

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct RentUpdate<'a> {
    pub token_id: &'a TokenId,
    pub contract_id: &'a AccountId,
    pub owner_id: &'a AccountId,
    pub ft_token_id: &'a AccountId,
    pub price: &'a U128,
    pub min_time: &'a u64,
    pub max_time: &'a u64,
    pub created_at: &'a u64,
}

impl RentUpdate<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an nft mint event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`NftMint`] represents the data of each mint.
    pub fn emit_many(data: &[RentUpdate<'_>]) {
        new_mfight_rent_v1(MfRentEventKind::RentUpdate(data)).emit()
    }
}

//

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct RentRemove<'a> {
    pub token_id: &'a TokenId,
    pub contract_id: &'a AccountId,
    pub account_id: &'a AccountId,
}

impl RentRemove<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an nft mint event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`NftMint`] represents the data of each mint.
    pub fn emit_many(data: &[RentRemove<'_>]) {
        new_mfight_rent_v1(MfRentEventKind::RentRemove(data)).emit()
    }
}

//

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct RentPay<'a> {
    pub token_id: &'a TokenId,
    pub contract_id: &'a AccountId,
    pub owner_id: &'a AccountId,
    pub receiver_id: &'a AccountId,
    pub time: &'a u64,
    pub end_time: &'a u64,
    pub price: &'a U128,
}

impl RentPay<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an nft mint event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`NftMint`] represents the data of each mint.
    pub fn emit_many(data: &[RentPay<'_>]) {
        new_mfight_rent_v1(MfRentEventKind::RentPay(data)).emit()
    }
}

//

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct RentClaim<'a> {
    pub token_id: &'a TokenId,
    pub contract_id: &'a AccountId,
    pub owner_id: &'a AccountId,
    pub renter_id: &'a AccountId,
}

impl RentClaim<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an nft mint event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`NftMint`] represents the data of each mint.
    pub fn emit_many(data: &[RentClaim<'_>]) {
        new_mfight_rent_v1(MfRentEventKind::RentClaim(data)).emit()
    }
}

//

#[derive(Serialize, Debug)]
pub(crate) struct MfRentEvent<'a> {
    version: &'static str,
    #[serde(flatten)]
    event_kind: MfRentEventKind<'a>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum MfRentEventKind<'a> {
    RentAdd(&'a [RentAdd<'a>]),
    RentUpdate(&'a [RentUpdate<'a>]),
    RentRemove(&'a [RentRemove<'a>]),
    RentPay(&'a [RentPay<'a>]),
    RentClaim(&'a [RentClaim<'a>]),
}

fn new_mfight_rent<'a>(version: &'static str, event_kind: MfRentEventKind<'a>) -> NearEvent<'a> {
    NearEvent::MfRent(MfRentEvent { version, event_kind })
}

fn new_mfight_rent_v1(event_kind: MfRentEventKind) -> NearEvent {
    new_mfight_rent("1.0.0", event_kind)
}
