use near_sdk::env;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(tag = "standard")]
#[must_use = "don't forget to `.emit()` this event"]
#[serde(rename_all = "snake_case")]
pub(crate) enum NearEvent<'a> {
    Nep141(crate::ft::base::events::Nep141Event<'a>),
    Nep171(crate::nft::events::Nep171Event<'a>),
    Nep245(crate::mt::base::events::Nep245Event<'a>),

    MfMarket(crate::market::events::MfMarketEvent<'a>),
    MfRent(crate::rent::events::MfRentEvent<'a>),
    MfNftIdo(crate::nft_ido::events::MfNftIdoEvent<'a>),
    MfEscrow(crate::escrow::events::MfEscrowEvent<'a>),
    NftFractionation(crate::nft_fractionation::events::MfFractEvent<'a>),
    MfTournament(crate::tournament::events::EventLog<'a>),
    CultReferral(crate::referral::events::CultReferralEvent<'a>),
}

impl<'a> NearEvent<'a> {
    fn to_json_string(&self) -> String {
        // Events cannot fail to serialize so fine to panic on error
        #[allow(clippy::redundant_closure)]
        serde_json
            ::to_string(self)
            .ok()
            .unwrap_or_else(|| env::abort())
    }

    fn to_json_event_string(&self) -> String {
        format!("EVENT_JSON:{}", self.to_json_string())
    }

    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub(crate) fn emit(self) {
        near_sdk::env::log_str(&self.to_json_event_string());
    }
}
