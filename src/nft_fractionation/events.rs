use crate::event::NearEvent;
use near_sdk::AccountId;
use serde::Serialize;
use crate::nft_fractionation::{ TokenId };

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct FractionationCreate<'a> {
    pub token_id: &'a TokenId,
    pub contract_id: &'a AccountId,
    pub entries: &'a Vec<TokenId>,
}

impl FractionationCreate<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[FractionationCreate<'_>]) {
        new_mf_fract_v1(MfFractEventKind::FractionationCreate(data)).emit()
    }
}

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct FractionationComplete<'a> {
    pub token_id: &'a TokenId,
    pub contract_id: &'a AccountId,
    pub receiver_id: &'a AccountId,
}

impl FractionationComplete<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[FractionationComplete<'_>]) {
        new_mf_fract_v1(MfFractEventKind::FractionationComplete(data)).emit()
    }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct FractionationProcess<'a> {    
    pub token_id: &'a TokenId,
    pub contract_id: &'a AccountId,
    pub account_id: &'a AccountId,
}

impl FractionationProcess<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[FractionationProcess<'_>]) {
        new_mf_fract_v1(MfFractEventKind::FractionationProcess(data)).emit()
    }
}

#[derive(Serialize, Debug)]
pub(crate) struct MfFractEvent<'a> {
    version: &'static str,
    #[serde(flatten)]
    event_kind: MfFractEventKind<'a>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum MfFractEventKind<'a> {
    FractionationCreate(&'a [FractionationCreate<'a>]),
    FractionationComplete(&'a [FractionationComplete<'a>]),
    FractionationProcess(&'a[FractionationProcess<'a>]),
}

fn new_mf_fract<'a>(version: &'static str, event_kind: MfFractEventKind<'a>) -> NearEvent<'a> {
    NearEvent::NftFractionation(MfFractEvent { version, event_kind })
}

fn new_mf_fract_v1(event_kind: MfFractEventKind) -> NearEvent {
    new_mf_fract("1.0.0", event_kind)
}
