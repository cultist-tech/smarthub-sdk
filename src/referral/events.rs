use crate::event::NearEvent;
use near_sdk::AccountId;
use serde::Serialize;
use crate::nft_ido::{ Ido, IdoId, TokenId };
use near_sdk::json_types::{U128};
use crate::referral::ReferralProgramMetadata;

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct ProgramCreate<'a> {
    pub contract_id: &'a AccountId,
    pub influencer_id: &'a AccountId,
    pub program_id: &'a String,
    pub royalty_percent: &'a Option<u64>,
    pub code: &'a String,
    pub metadata: &'a Option<ReferralProgramMetadata>,
}

impl ProgramCreate<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[ProgramCreate<'_>]) {
        new_cult_referral_v1(CultReferralEventKind::ProgramCreate(data)).emit()
    }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct ReferralAccept<'a> {
    pub contract_id: &'a AccountId,
    pub influencer_id: &'a AccountId,
    pub program_id: &'a String,
    pub account_id: &'a AccountId,
}

impl ReferralAccept<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[ReferralAccept<'_>]) {
        new_cult_referral_v1(CultReferralEventKind::ReferralAccept(data)).emit()
    }
}

// #

#[derive(Serialize, Debug)]
pub(crate) struct CultReferralEvent<'a> {
    version: &'static str,
    #[serde(flatten)]
    event_kind: CultReferralEventKind<'a>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum CultReferralEventKind<'a> {
    ProgramCreate(&'a [ProgramCreate<'a>]),
    ReferralAccept(&'a [ReferralAccept<'a>]),
}

fn new_cult_referral<'a>(version: &'static str, event_kind: CultReferralEventKind<'a>) -> NearEvent<'a> {
    NearEvent::CultReferral(CultReferralEvent { version, event_kind })
}

fn new_cult_referral_v1(event_kind: CultReferralEventKind) -> NearEvent {
    new_cult_referral("1.0.0", event_kind)
}
