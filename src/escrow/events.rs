use near_sdk::AccountId;
use serde::Serialize;
use crate::event::{ NearEvent };

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct MfEscrowCreate<'a> {
    pub account_id: &'a AccountId,
}

impl MfEscrowCreate<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many<'a>(data: &'a [MfEscrowCreate<'a>]) {
        new_mf_escrow_v1(MfEscrowEventKind::MfEscrowCreate(data)).emit()
    }
}

//

#[derive(Serialize, Debug)]
pub(crate) struct MfEscrowEvent<'a> {
    version: &'static str,
    #[serde(flatten)]
    event_kind: MfEscrowEventKind<'a>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum MfEscrowEventKind<'a> {
    MfEscrowCreate(&'a [MfEscrowCreate<'a>]),
}

fn new_mf_escrow<'a>(version: &'static str, event_kind: MfEscrowEventKind<'a>) -> NearEvent<'a> {
    NearEvent::MfEscrow(MfEscrowEvent { version, event_kind })
}

fn new_mf_escrow_v1(event_kind: MfEscrowEventKind) -> NearEvent {
    new_mf_escrow("1.0.0", event_kind)
}