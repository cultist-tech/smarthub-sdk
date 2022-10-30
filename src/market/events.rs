use crate::event::NearEvent;
use near_sdk::AccountId;
use serde::Serialize;
use near_sdk::json_types::U128;
use crate::market::{ TokenId, Sale };
use crate::nft::royalty::Payout;
use std::collections::HashMap;

/// Data to log for an NFT mint event. To log this event, call [`.emit()`](NftMint::emit).
#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct MarketCreateSale<'a> {
    pub owner_id: &'a AccountId,
    pub nft_contract_id: &'a AccountId,
    pub token_id: &'a TokenId,
    pub sale: &'a Sale,
}

impl MarketCreateSale<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an nft mint event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`NftMint`] represents the data of each mint.
    pub fn emit_many(data: &[MarketCreateSale<'_>]) {
        new_mfight_market_v1(MfMarketEventKind::MarketCreateSale(data)).emit()
    }
}

//

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct MarketUpdateSale<'a> {
    pub owner_id: &'a AccountId,
    pub nft_contract_id: &'a AccountId,
    pub token_id: &'a TokenId,
    pub ft_token_id: &'a AccountId,
    pub price: &'a U128,
}

impl MarketUpdateSale<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an nft mint event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`NftMint`] represents the data of each mint.
    pub fn emit_many(data: &[MarketUpdateSale<'_>]) {
        new_mfight_market_v1(MfMarketEventKind::MarketUpdateSale(data)).emit()
    }
}

//

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct MarketRemoveSale<'a> {
    pub owner_id: &'a AccountId,
    pub nft_contract_id: &'a AccountId,
    pub token_id: &'a TokenId,
}

impl MarketRemoveSale<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an nft mint event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`NftMint`] represents the data of each mint.
    pub fn emit_many(data: &[MarketRemoveSale<'_>]) {
        new_mfight_market_v1(MfMarketEventKind::MarketRemoveSale(data)).emit()
    }
}

//

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct MarketOffer<'a> {
    pub owner_id: &'a AccountId,

    pub receiver_id: &'a AccountId,
    pub nft_contract_id: &'a AccountId,
    pub token_id: &'a TokenId,
    pub payout: &'a HashMap<AccountId, U128>,
    pub ft_token_id: &'a AccountId,
    pub price: &'a U128,
}

impl MarketOffer<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an nft mint event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`NftMint`] represents the data of each mint.
    pub fn emit_many(data: &[MarketOffer<'_>]) {
        new_mfight_market_v1(MfMarketEventKind::MarketOffer(data)).emit()
    }
}

//

#[derive(Serialize, Debug)]
pub struct MfMarketEvent<'a> {
    version: &'static str,
    #[serde(flatten)]
    event_kind: MfMarketEventKind<'a>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum MfMarketEventKind<'a> {
    MarketCreateSale(&'a [MarketCreateSale<'a>]),
    MarketUpdateSale(&'a [MarketUpdateSale<'a>]),
    MarketRemoveSale(&'a [MarketRemoveSale<'a>]),
    MarketOffer(&'a [MarketOffer<'a>]),
}

fn new_mfight_market<'a>(
    version: &'static str,
    event_kind: MfMarketEventKind<'a>
) -> NearEvent<'a> {
    NearEvent::MfMarket(MfMarketEvent { version, event_kind })
}

fn new_mfight_market_v1(event_kind: MfMarketEventKind) -> NearEvent {
    new_mfight_market("1.0.0", event_kind)
}
