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
use crate::nft::{Token, TokenId, TokenRarity, TokenTypes, PriceType};
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use serde::Serialize;
use std::collections::HashMap;

/// Data to log for an NFT mint event. To log this event, call [`.emit()`](NftMint::emit).
#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftMint<'a> {
    pub owner_id: &'a AccountId,
    pub token_ids: &'a Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
}

impl NftMint<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an nft mint event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`NftMint`] represents the data of each mint.
    pub fn emit_many(data: &[NftMint<'_>]) {
        new_171_v1(Nep171EventKind::NftMint(data)).emit()
    }
}

/// Data to log for an NFT transfer event. To log this event,
/// call [`.emit()`](NftTransfer::emit).
#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftTransfer<'a> {
    pub old_owner_id: &'a AccountId,
    pub new_owner_id: &'a AccountId,
    pub token_ids: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_id: Option<&'a AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
}

impl NftTransfer<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an nft transfer event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`NftTransfer`] represents the data of each transfer.
    pub fn emit_many(data: &[NftTransfer<'_>]) {
        new_171_v1(Nep171EventKind::NftTransfer(data)).emit()
    }
}

/// Data to log for an NFT burn event. To log this event, call [`.emit()`](NftBurn::emit).
#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftBurn<'a> {
    pub owner_id: &'a AccountId,
    pub token_ids: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_id: Option<&'a AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
}

impl NftBurn<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an nft burn event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`NftBurn`] represents the data of each burn.
    pub fn emit_many<'a>(data: &'a [NftBurn<'a>]) {
        new_171_v1(Nep171EventKind::NftBurn(data)).emit()
    }
}

/// Data to log for an NFT reveal event. To log this event, call [`.emit()`](NftReveal::emit).
#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftReveal<'a> {
    pub owner_id: &'a AccountId,
    pub token_ids: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_id: Option<&'a AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
}

impl NftReveal<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an nft reveal event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`NftReveal`] represents the data of each reveal.
    pub fn emit_many<'a>(data: &'a [NftReveal<'a>]) {
        new_171_v1(Nep171EventKind::NftReveal(data)).emit()
    }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftTransferPayout<'a> {
    pub token_id: &'a TokenId,
    pub sender_id: &'a AccountId,
    pub receiver_id: &'a AccountId,
    pub balance: &'a U128,
    pub payout: &'a HashMap<AccountId, U128>,
}

impl NftTransferPayout<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[NftTransferPayout<'_>]) {
        new_171_v1(Nep171EventKind::NftTransferPayout(data)).emit()
    }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftCreate<'a> {
    pub token: &'a Token,
}

impl NftCreate<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    pub fn emit_many(data: &[NftCreate<'_>]) {
        new_171_v1(Nep171EventKind::NftCreate(data)).emit()
    }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftUpgrade<'a> {
  pub owner_id: &'a AccountId,
  pub rarity: &'a TokenRarity,
  pub token_id: &'a TokenId,
}

impl NftUpgrade<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[NftUpgrade<'_>]) {
    new_171_v1(Nep171EventKind::NftUpgrade(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftSetUpgradePrice<'a> {  
  pub rarity: &'a TokenRarity,
  pub types: &'a Option<TokenTypes>,
  pub ft_token: &'a AccountId,
  pub price: &'a U128,
}

impl NftSetUpgradePrice<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[NftSetUpgradePrice<'_>]) {
    new_171_v1(Nep171EventKind::NftSetUpgradePrice(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftSetBurnerPrice<'a> {  
  pub rarity: &'a TokenRarity,
  pub types: &'a Option<TokenTypes>,
  pub burning_rarity_sum: &'a u8,  
}

impl NftSetBurnerPrice<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[NftSetBurnerPrice<'_>]) {
    new_171_v1(Nep171EventKind::NftSetBurnerPrice(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftRemoveUpgradePrice<'a> {
  pub price_type: &'a PriceType,
  pub rarity: &'a TokenRarity,
  pub types: &'a Option<TokenTypes>,  
}

impl NftRemoveUpgradePrice<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[NftRemoveUpgradePrice<'_>]) {
    new_171_v1(Nep171EventKind::NftRemoveUpgradePrice(data)).emit()
  }
}

#[derive(Serialize, Debug)]
pub(crate) struct Nep171Event<'a> {
    version: &'static str,
    #[serde(flatten)]
    event_kind: Nep171EventKind<'a>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum Nep171EventKind<'a> {
    NftMint(&'a [NftMint<'a>]),
    NftTransfer(&'a [NftTransfer<'a>]),
    NftBurn(&'a [NftBurn<'a>]),
    NftReveal(&'a [NftReveal<'a>]),

    NftCreate(&'a [NftCreate<'a>]),
    NftTransferPayout(&'a [NftTransferPayout<'a>]),
    NftUpgrade(&'a [NftUpgrade<'a>]),
    NftSetUpgradePrice(&'a [NftSetUpgradePrice<'a>]),
    NftSetBurnerPrice(&'a [NftSetBurnerPrice<'a>]),  
    NftRemoveUpgradePrice(&'a [NftRemoveUpgradePrice<'a>]),      
}

fn new_171<'a>(version: &'static str, event_kind: Nep171EventKind<'a>) -> NearEvent<'a> {
    NearEvent::Nep171(Nep171Event {
        version,
        event_kind,
    })
}

fn new_171_v1(event_kind: Nep171EventKind) -> NearEvent {
    new_171("1.0.0", event_kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{test_utils, AccountId};

    fn bob() -> AccountId {
        AccountId::new_unchecked("bob".to_string())
    }

    fn alice() -> AccountId {
        AccountId::new_unchecked("alice".to_string())
    }

    #[test]
    fn nft_mint() {
        let owner_id = &bob();
        let token_ids = &vec!["0".to_string(), "1".to_string()];
        (NftMint {
            owner_id,
            token_ids,
            memo: None,
        })
        .emit();
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"bob","token_ids":["0","1"]}]}"#
        );
    }

    #[test]
    fn nft_mints() {
        let owner_id = &bob();
        let token_ids = &vec!["0".to_string(), "1".to_string()];
        let mint_log = NftMint {
            owner_id,
            token_ids,
            memo: None,
        };
        NftMint::emit_many(&[
            mint_log,
            NftMint {
                owner_id: &alice(),
                token_ids: &vec!["2".to_string(), "3".to_string()],
                memo: Some("has memo"),
            },
        ]);
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"bob","token_ids":["0","1"]},{"owner_id":"alice","token_ids":["2","3"],"memo":"has memo"}]}"#
        );
    }

    #[test]
    fn nft_burn() {
        let owner_id = &bob();
        let token_ids = &["0", "1"];
        (NftBurn {
            owner_id,
            token_ids,
            authorized_id: None,
            memo: None,
        })
        .emit();
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_burn","data":[{"owner_id":"bob","token_ids":["0","1"]}]}"#
        );
    }

    #[test]
    fn nft_burns() {
        let owner_id = &bob();
        let token_ids = &["0", "1"];
        NftBurn::emit_many(&[
            NftBurn {
                owner_id: &alice(),
                token_ids: &["2", "3"],
                authorized_id: Some(&bob()),
                memo: Some("has memo"),
            },
            NftBurn {
                owner_id,
                token_ids,
                authorized_id: None,
                memo: None,
            },
        ]);
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_burn","data":[{"owner_id":"alice","token_ids":["2","3"],"authorized_id":"bob","memo":"has memo"},{"owner_id":"bob","token_ids":["0","1"]}]}"#
        );
    }

    #[test]
    fn nft_transfer() {
        let old_owner_id = &bob();
        let new_owner_id = &alice();
        let token_ids = &["0", "1"];
        (NftTransfer {
            old_owner_id,
            new_owner_id,
            token_ids,
            authorized_id: None,
            memo: None,
        })
        .emit();
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_transfer","data":[{"old_owner_id":"bob","new_owner_id":"alice","token_ids":["0","1"]}]}"#
        );
    }

    #[test]
    fn nft_transfers() {
        let old_owner_id = &bob();
        let new_owner_id = &alice();
        let token_ids = &["0", "1"];
        NftTransfer::emit_many(&[
            NftTransfer {
                old_owner_id: &alice(),
                new_owner_id: &bob(),
                token_ids: &["2", "3"],
                authorized_id: Some(&bob()),
                memo: Some("has memo"),
            },
            NftTransfer {
                old_owner_id,
                new_owner_id,
                token_ids,
                authorized_id: None,
                memo: None,
            },
        ]);
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_transfer","data":[{"old_owner_id":"alice","new_owner_id":"bob","token_ids":["2","3"],"authorized_id":"bob","memo":"has memo"},{"old_owner_id":"bob","new_owner_id":"alice","token_ids":["0","1"]}]}"#
        );
    }

    #[test]
    fn nft_reveal() {
        let owner_id = &bob();
        let token_ids = &["0", "1"];
        (NftReveal {
            owner_id,
            token_ids,
            authorized_id: None,
            memo: None,
        })
        .emit();
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_reveal","data":[{"owner_id":"bob","token_ids":["0","1"]}]}"#
        );
    }

    #[test]
    fn nft_reveals() {
        let owner_id = &bob();
        let token_ids = &["0", "1"];
        NftReveal::emit_many(&[
            NftReveal {
                owner_id: &alice(),
                token_ids: &["2", "3"],
                authorized_id: Some(&bob()),
                memo: Some("has memo"),
            },
            NftReveal {
                owner_id,
                token_ids,
                authorized_id: None,
                memo: None,
            },
        ]);
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_reveal","data":[{"owner_id":"alice","token_ids":["2","3"],"authorized_id":"bob","memo":"has memo"},{"owner_id":"bob","token_ids":["0","1"]}]}"#
        );
    }
}
