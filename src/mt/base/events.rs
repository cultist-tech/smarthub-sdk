use near_sdk::json_types::U128;
use near_sdk::AccountId;
use serde::Serialize;
use crate::event::{ NearEvent };

/// Data to log for an FT mint event. To log this event, call [`.emit()`](MtMint::emit).
#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct MtMint<'a> {
    pub owner_id: &'a AccountId,
    pub token_ids: &'a Vec<AccountId>,
    pub amounts: &'a Vec<U128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
}

impl MtMint<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an FT mint event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`MtMint`] represents the data of each mint.
    pub fn emit_many(data: &[MtMint<'_>]) {
        new_245_v1(Nep245EventKind::MtMint(data)).emit()
    }
}

/// Data to log for an FT transfer event. To log this event,
/// call [`.emit()`](MtTransfer::emit).
#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct MtTransfer<'a> {
    pub old_owner_id: &'a AccountId,
    pub new_owner_id: &'a AccountId,

    pub token_ids: &'a Vec<AccountId>,
    pub amounts: &'a Vec<U128>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
}

impl MtTransfer<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an FT transfer event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`MtTransfer`] represents the data of each transfer.
    pub fn emit_many(data: &[MtTransfer<'_>]) {
        new_245_v1(Nep245EventKind::MtTransfer(data)).emit()
    }
}

/// Data to log for an FT burn event. To log this event, call [`.emit()`](MtBurn::emit).
#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct MtBurn<'a> {
    pub token_ids: &'a Vec<AccountId>,
    pub owner_id: &'a AccountId,
    pub amounts: &'a Vec<U128>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
}

impl MtBurn<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an FT burn event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`MtBurn`] represents the data of each burn.
    pub fn emit_many<'a>(data: &'a [MtBurn<'a>]) {
        new_245_v1(Nep245EventKind::MtBurn(data)).emit()
    }
}

#[derive(Serialize, Debug)]
pub(crate) struct Nep245Event<'a> {
    version: &'static str,
    #[serde(flatten)]
    event_kind: Nep245EventKind<'a>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum Nep245EventKind<'a> {
    MtMint(&'a [MtMint<'a>]),
    MtTransfer(&'a [MtTransfer<'a>]),
    MtBurn(&'a [MtBurn<'a>]),
}

fn new_245<'a>(version: &'static str, event_kind: Nep245EventKind<'a>) -> NearEvent<'a> {
    NearEvent::Nep245(Nep245Event { version, event_kind })
}

fn new_245_v1(event_kind: Nep245EventKind) -> NearEvent {
    new_245("1.0.0", event_kind)
}

//