use crate::event::NearEvent;
use near_sdk::AccountId;
use serde::Serialize;
use crate::nft_ido::{ Ido, IdoId };
use near_sdk::json_types::{U128};

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct IdoCreate<'a> {
    pub ido_id: &'a IdoId,
    pub contract_id: &'a AccountId,

    pub name: &'a String,
    pub media: &'a Option<String>,
    pub amount: &'a u64,
    pub price: &'a U128,
    pub buy_max: &'a u64,
    pub per_transaction_min: &'a u64,
    pub per_transaction_max: &'a u64,
}

impl IdoCreate<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[IdoCreate<'_>]) {
        new_mf_nft_ido_v1(MfNftIdoEventKind::IdoCreate(data)).emit()
    }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct IdoStart<'a> {
    pub ido_id: &'a IdoId,
    pub contract_id: &'a AccountId,
    pub date: &'a u64,
}

impl IdoStart<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[IdoStart<'_>]) {
        new_mf_nft_ido_v1(MfNftIdoEventKind::IdoStart(data)).emit()
    }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct IdoUpdate<'a> {
    pub ido_id: &'a IdoId,
    pub contract_id: &'a AccountId,

    pub date: &'a u64,
    pub per_transaction_max: &'a u64,
    pub per_transaction_min: &'a u64,
    pub buy_max: &'a u64,
}

impl IdoUpdate<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[IdoUpdate<'_>]) {
        new_mf_nft_ido_v1(MfNftIdoEventKind::IdoUpdate(data)).emit()
    }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct IdoPause<'a> {
    pub ido_id: &'a IdoId,
    pub contract_id: &'a AccountId,

    pub pause: &'a bool,
}

impl IdoPause<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[IdoPause<'_>]) {
        new_mf_nft_ido_v1(MfNftIdoEventKind::IdoPause(data)).emit()
    }
}

// #

#[derive(Serialize, Debug)]
pub(crate) struct MfNftIdoEvent<'a> {
    version: &'static str,
    #[serde(flatten)]
    event_kind: MfNftIdoEventKind<'a>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum MfNftIdoEventKind<'a> {
    IdoCreate(&'a [IdoCreate<'a>]),
    IdoStart(&'a [IdoStart<'a>]),
    IdoUpdate(&'a [IdoUpdate<'a>]),
    IdoPause(&'a [IdoPause<'a>]),
}

fn new_mf_nft_ido<'a>(version: &'static str, event_kind: MfNftIdoEventKind<'a>) -> NearEvent<'a> {
    NearEvent::MfNftIdo(MfNftIdoEvent { version, event_kind })
}

fn new_mf_nft_ido_v1(event_kind: MfNftIdoEventKind) -> NearEvent {
    new_mf_nft_ido("1.0.0", event_kind)
}
