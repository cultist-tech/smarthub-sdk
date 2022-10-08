use crate::event::NearEvent;
use near_sdk::{ AccountId };
use near_sdk::json_types::{ U128 };
use near_sdk::serde::{ Serialize };
use crate::tournament::{ RewardPrize, TokenId };
use crate::tournament::TournamentId;

/// Enum that represents the data type of the EventLog.
#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
#[serde(crate = "near_sdk::serde")]
pub enum EventLogVariant<'a> {
    TournamentCreate(&'a [TournamentCreate<'a>]),
    TournamentJoin(&'a [TournamentJoin<'a>]),
    TournamentStart(&'a [TournamentStart<'a>]),
    TournamentEnd(&'a [TournamentEnd<'a>]),
    TournamentReward(&'a [TournamentReward<'a>]),
    TournamentAddPrize(&'a [TournamentAddPrize<'a>]),
    TournamentAddNftAccess(&'a [TournamentAddNftAccess<'a>]),
}

/// Interface to capture data about an event
///
/// Arguments:
/// * `standard`: name of standard e.g. nep171
/// * `version`: e.g. 1.0.0
/// * `event`: associate event data
#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub(crate) struct EventLog<'a> {
    pub version: &'static str,

    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
    #[serde(flatten)]
    pub event: EventLogVariant<'a>,
}

/// An event log to capture tournament creation
///
/// Arguments
/// * `tournament_id`: "tournament-1"
/// * `players_number`: 8
/// * `price`: "100000"
#[must_use]
#[derive(Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentCreate<'a> {
    pub tournament_id: &'a String,
    pub owner_id: &'a AccountId,

    pub players_number: &'a u8,
    pub price: &'a Option<U128>,
}

impl TournamentCreate<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[TournamentCreate<'_>]) {
        new_mf1_v1(EventLogVariant::TournamentCreate(data)).emit()
    }
}

/// An event log to capture tournament entrance
///
/// Arguments
/// * `partisipator_id`: "partisipator.near"
/// * `tournament_id`: "tournament-1"
#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentJoin<'a> {
    pub account_id: &'a AccountId,
    pub tournament_id: &'a String,
    pub owner_id: &'a AccountId,
}

impl TournamentJoin<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[TournamentJoin<'_>]) {
        new_mf1_v1(EventLogVariant::TournamentJoin(data)).emit()
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentStart<'a> {
    pub tournament_id: &'a String,
    pub owner_id: &'a AccountId,

    pub date: &'a u64,
}

impl TournamentStart<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[TournamentStart<'_>]) {
        new_mf1_v1(EventLogVariant::TournamentStart(data)).emit()
    }
}

/// An event log to capture tournament prize rewarding
///
/// Arguments
/// * `tournament_id`: "tournament-1"
/// * `rewarded_amount`: "100000000"
#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentEnd<'a> {
    pub tournament_id: &'a String,
    pub owner_id: &'a AccountId,

    pub date: &'a u64,
}

impl TournamentEnd<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[TournamentEnd<'_>]) {
        new_mf1_v1(EventLogVariant::TournamentEnd(data)).emit()
    }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentReward<'a> {
    pub tournament_id: &'a String,
    pub owner_id: &'a AccountId,

    pub place: &'a u8,
    pub prize: &'a RewardPrize,
}

impl TournamentReward<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[TournamentReward<'_>]) {
        new_mf1_v1(EventLogVariant::TournamentReward(data)).emit()
    }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentAddPrize<'a> {
    pub tournament_id: &'a TournamentId,
    pub owner_id: &'a AccountId,

    pub prize: &'a RewardPrize,
}

impl TournamentAddPrize<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[TournamentAddPrize<'_>]) {
        new_mf1_v1(EventLogVariant::TournamentAddPrize(data)).emit()
    }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentAddNftAccess<'a> {
    pub tournament_id: &'a String,
    pub owner_id: &'a AccountId,

    pub token_ids: &'a Vec<TokenId>,
}

impl TournamentAddNftAccess<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[TournamentAddNftAccess<'_>]) {
        new_mf1_v1(EventLogVariant::TournamentAddNftAccess(data)).emit()
    }
}

fn new_mf1<'a>(version: &'static str, event: EventLogVariant<'a>) -> NearEvent<'a> {
    NearEvent::MfTournament(EventLog { version, event })
}

fn new_mf1_v1(event: EventLogVariant) -> NearEvent {
    new_mf1("1.0.0", event)
}