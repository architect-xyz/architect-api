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
//! ```text
//! let series = EventContractSeriesInfo {
//!     event_contract_series: "ECES 20241227 CME Event Contracts".into(),
//!     quote_symbol: "USD".into(),
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
//! ```text
//! let series = EventContractSeriesInfo {
//!     event_contract_series: "2024 Presidential Election KALSHI Event Contracts".into(),
//!     quote_symbol: "USD".into(),
//!     underlying: None,
//!     expiration: None,
//!     outcomes: EventContractOutcomes::Enumerated {
//!         outcomes: vec!["Biden".into(), "Trump".into(), "Other".into()],
//!     },
//!     outcomes_side: Some(YesOrNo::Yes),
//!     outcomes_are_mutually_exclusive: true,
//! }
//! ```

use super::Product;
use anyhow::{bail, Result};
use chrono::{DateTime, NaiveTime, Utc};
use derive_more::Display;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};
use strum_macros::{EnumString, IntoStaticStr};

#[derive(
    Debug,
    Display,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize,
    Serialize,
    JsonSchema,
)]
#[serde(transparent)]
#[cfg_attr(feature = "postgres", derive(postgres_types::ToSql))]
#[cfg_attr(feature = "postgres", postgres(transparent))]
pub struct EventContractSeries(String);

impl EventContractSeries {
    pub(crate) fn new_unchecked(name: impl AsRef<str>) -> Self {
        Self(name.as_ref().to_string())
    }

    pub fn new(name: &str, venue_discriminant: Option<&str>) -> Result<Self> {
        if name.contains('/') || venue_discriminant.is_some_and(|v| v.contains('/')) {
            bail!("Event contract series name cannot contain the forward slash character '/'");
        }
        let inner = match venue_discriminant {
            Some(venue_discriminant) => {
                if venue_discriminant.is_empty() {
                    bail!("Venue discriminant cannot be empty if provided");
                }
                format!(
                    "{} {} Event Contract Series",
                    name,
                    venue_discriminant.to_uppercase()
                )
            }
            None => format!("{} Event Contract Series", name),
        };
        Ok(Self(inner))
    }
}

impl FromStr for EventContractSeries {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // CR arao: add validation
        Ok(Self::new_unchecked(s))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct EventContractSeriesInfo {
    pub event_contract_series: EventContractSeries,
    pub quote_symbol: Product,
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

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventContractOutcomes {
    Enumerated {
        outcomes: Vec<Outcome>,
    },
    OptionLike {
        expiration_time_of_day: NaiveTime,
        #[schemars(with = "String")]
        expiration_time_zone: chrono_tz::Tz,
        strikes_by_expiration: BTreeMap<DateTime<Utc>, BTreeSet<Decimal>>,
    },
}

impl EventContractOutcomes {
    pub fn is_enumerated(&self) -> bool {
        matches!(self, Self::Enumerated { .. })
    }

    pub fn is_option_like(&self) -> bool {
        matches!(self, Self::OptionLike { .. })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Outcome {
    pub name: String,
}

#[derive(
    Debug, Clone, Copy, EnumString, IntoStaticStr, Deserialize, Serialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum YesOrNo {
    Yes,
    No,
}

#[cfg(feature = "postgres")]
crate::to_sql_str!(YesOrNo);
