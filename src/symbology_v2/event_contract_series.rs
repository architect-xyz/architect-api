//! Some event contracts can be abbreviated/compressed as a series of products, in the
//! same way that options can.  The kinds of dimensions that event contracts could vary
//! on are more varied than options.
//!
//! Additionally, event contracts have a notion of `outcome_side` which denotes whether
//! a contract represents the "Yes" or "No" side of an outcome.  Some event contract
//! venues have separate physical products for each side; others have just one product
//! where the opposite side is just a different viewpoint.
//!
//! ## Example: S&P 500 binary options (CME event contracts)
//!
//! These are option-like event contract series.  The strikes are minted around the
//! spot price of the underlying future.
//!
//! ```
//! let series = EventContractSeriesInfo {
//!     event_contract_series: "ECES 20241227 CME Event Contracts".into(),
//!     quote_currency: "USD".into(),
//!     underlying: Some("ES 20241227 Future".into()),
//!     expiration: Some("2024-12-27T15:00:00Z".parse().unwrap()),
//!     outcomes: EventContractOutcomes::OptionLike {
//!         strikes: OptionsStrikes {
//!             start: dec!(2000.0),
//!             end: dec!(3000.0),
//!             stride: dec!(1.0),
//!         },
//!         expirations: None,
//!     },
//!     outcomes_side: Some(YesOrNo::Yes),
//!     outcomes_are_mutually_exclusive: false,
//! }
//! ```
//!
//! ## Example: Presidential election 2024
//!
//! These are one time events that are better explicitly enumerated.
//!
//! ```
//! let series = EventContractSeriesInfo {
//!     event_contract_series: "2024 Presidential Election KALSHI Event Contracts".into(),
//!     quote_currency: "USD".into(),
//!     underlying: None,
//!     expiration: None,
//!     outcomes: EventContractOutcomes::Enumerated {
//!         outcomes: vec!["Biden".into(), "Trump".into(), "Other".into()],
//!     },
//!     outcomes_side: Some(YesOrNo::Yes),
//!     outcomes_are_mutually_exclusive: true,
//! }
//! ```

use super::{OptionsExpirations, OptionsStrikes, Product};
use anyhow::Result;
use chrono::{DateTime, Utc};
use derive_more::Display;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
#[serde(transparent)]
pub struct EventContractSeries(String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventContractSeriesInfo {
    pub event_contract_series: EventContractSeries,
    pub quote_currency: Product,
    pub underlying: Option<Product>,
    pub expiration: Option<DateTime<Utc>>,
    pub outcomes: EventContractOutcomes,
    pub outcomes_side: Option<YesOrNo>,
    pub outcomes_are_mutually_exclusive: bool,
}

impl EventContractSeriesInfo {
    pub fn get_product(&self, _instance: EventContractSeriesInstance) -> Result<Product> {
        todo!()
    }

    pub fn get_tradable_product(&self) -> Result<Product> {
        todo!()
    }

    pub fn parse_instance(
        &self,
        _symbol: impl AsRef<str>,
    ) -> Result<EventContractSeriesInstance> {
        todo!()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum EventContractSeriesInstance {
    Enumerated { outcome: Outcome },
    OptionLike { strike: Decimal, expiration: Option<DateTime<Utc>> },
}

impl EventContractSeriesInstance {
    pub fn expiration(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Enumerated { .. } => None,
            Self::OptionLike { expiration, .. } => *expiration,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventContractOutcomes {
    Enumerated { outcomes: Vec<Outcome> },
    OptionLike { strikes: OptionsStrikes, expirations: Option<OptionsExpirations> },
}

impl EventContractOutcomes {
    pub fn is_enumerated(&self) -> bool {
        matches!(self, Self::Enumerated { .. })
    }

    pub fn is_option_like(&self) -> bool {
        matches!(self, Self::OptionLike { .. })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Outcome {
    pub name: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum YesOrNo {
    Yes,
    No,
}
