//! A product is a thing you can have a position in.

use super::*;
use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, NaiveDate, Utc};
use derive_more::{AsRef, Deref, Display, From};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::{EnumString, IntoStaticStr};

#[derive(
    Debug,
    Display,
    From,
    AsRef,
    Deref,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    JsonSchema,
)]
#[as_ref(forward)]
#[serde(transparent)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLScalar))]
#[cfg_attr(feature = "graphql", graphql(transparent))]
#[cfg_attr(feature = "postgres", derive(postgres_types::ToSql))]
#[cfg_attr(feature = "postgres", postgres(transparent))]
pub struct Product(pub(crate) String);

impl Product {
    fn new(
        symbol: &str,
        venue_discriminant: Option<&str>,
        product_kind: &str,
    ) -> Result<Self> {
        if symbol.contains('/')
            || venue_discriminant.map_or(false, |v| v.contains('/'))
            || product_kind.contains('/')
        {
            bail!("product symbol cannot contain the forward slash character '/'");
        }
        let inner = match venue_discriminant {
            Some(venue_discriminant) => {
                if venue_discriminant.is_empty() {
                    bail!("venue discriminant cannot be empty if provided");
                }
                format!(
                    "{} {} {}",
                    symbol,
                    venue_discriminant.to_uppercase(),
                    product_kind
                )
            }
            None => format!("{} {}", symbol, product_kind),
        };
        Ok(Self(inner))
    }

    pub fn fiat(symbol: &str) -> Result<Self> {
        if symbol.contains(char::is_whitespace) {
            bail!("fiat product symbol cannot contain whitespace");
        }
        if symbol.contains('/') {
            bail!("fiat product symbol cannot contain the forward slash character '/'");
        }
        Ok(Self(symbol.to_string()))
    }

    pub fn commodity(symbol: &str) -> Result<Self> {
        Self::new(symbol, None, "Commodity")
    }

    pub fn crypto(symbol: &str) -> Result<Self> {
        Self::new(symbol, None, "Crypto")
    }

    pub fn index(symbol: &str, venue_discriminant: Option<&str>) -> Result<Self> {
        Self::new(symbol, venue_discriminant, "Index")
    }

    pub fn equity(symbol: &str, country: &str) -> Result<Self> {
        let symbol = format!("{symbol} {country}");
        Self::new(&symbol, None, "Equity")
    }

    pub fn future(
        symbol: &str,
        expiration: NaiveDate,
        venue_discriminant: Option<&str>,
    ) -> Result<Self> {
        let symbol = format!("{symbol} {}", expiration.format("%Y%m%d"));
        Self::new(&symbol, venue_discriminant, "Future")
    }

    pub fn future_spread(symbol: &str, venue_discriminant: Option<&str>) -> Result<Self> {
        Self::new(symbol, venue_discriminant, "Future Spread")
    }

    pub fn perpetual(symbol: &str, venue_discriminant: Option<&str>) -> Result<Self> {
        Self::new(symbol, venue_discriminant, "Perpetual")
    }

    /// E.g. "AAPL US 20241227 300 C Option"
    pub fn option(
        stem: &str,
        expiration: NaiveDate,
        strike: Decimal,
        put_or_call: PutOrCall,
        venue_discriminant: Option<&str>,
    ) -> Result<Self> {
        let symbol = format!(
            "{stem} {} {} {put_or_call}",
            expiration.format("%Y%m%d"),
            strike.normalize()
        );
        Self::new(&symbol, venue_discriminant, "Option")
    }

    // pub fn is_series(&self) -> bool {
    //     self.0.ends_with("Option") || self.0.ends_with("Event Contract")
    // }
}

impl std::borrow::Borrow<str> for Product {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl FromStr for Product {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.contains('/') {
            bail!("product symbol cannot contain the forward slash character '/'");
        }
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ProductInfo {
    pub product_type: ProductType,
    pub primary_venue: Option<String>,
}

impl ProductInfo {
    pub fn series(&self) -> Option<&str> {
        match &self.product_type {
            ProductType::Future { series, .. } => series.as_deref(),
            _ => None,
        }
    }

    pub fn multiplier(&self) -> Option<Decimal> {
        match &self.product_type {
            ProductType::Crypto
            | ProductType::Fiat
            | ProductType::Equity
            | ProductType::Index
            | ProductType::Commodity
            | ProductType::Unknown
            | ProductType::Option { .. }
            | ProductType::EventContract { .. }
            | ProductType::FutureSpread { .. } => None,
            ProductType::Perpetual { multiplier, .. }
            | ProductType::Future { multiplier, .. } => Some(*multiplier),
        }
    }

    pub fn underlying(&self) -> Option<&Product> {
        match &self.product_type {
            ProductType::Crypto
            | ProductType::Fiat
            | ProductType::Equity
            | ProductType::Index
            | ProductType::Commodity
            | ProductType::Unknown
            | ProductType::Option { .. }
            | ProductType::EventContract { .. }
            | ProductType::FutureSpread { .. } => None,
            ProductType::Perpetual { underlying, .. }
            | ProductType::Future { underlying, .. } => underlying.as_ref(),
        }
    }

    pub fn expiration(&self) -> Option<DateTime<Utc>> {
        match &self.product_type {
            ProductType::Crypto
            | ProductType::Fiat
            | ProductType::Equity
            | ProductType::Index
            | ProductType::Commodity
            | ProductType::Unknown
            | ProductType::Perpetual { .. }
            | ProductType::FutureSpread { .. } => None,
            ProductType::Option {
                instance: OptionsSeriesInstance { expiration, .. },
                ..
            } => Some(*expiration),
            ProductType::EventContract { instance, .. } => instance.expiration(),
            ProductType::Future { expiration, .. } => Some(*expiration),
        }
    }

    pub fn is_expired(&self, cutoff: DateTime<Utc>) -> bool {
        if let Some(expiration) = self.expiration() {
            expiration <= cutoff
        } else {
            false
        }
    }

    pub fn derivative_kind(&self) -> Option<DerivativeKind> {
        match &self.product_type {
            ProductType::Future { derivative_kind, .. } => Some(*derivative_kind),
            _ => None,
        }
    }

    pub fn first_notice_date(&self) -> Option<NaiveDate> {
        match &self.product_type {
            ProductType::Future { first_notice_date, .. } => *first_notice_date,
            _ => None,
        }
    }
}

#[derive(Debug, Clone, IntoStaticStr, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "product_type")]
pub enum ProductType {
    Fiat,
    Commodity,
    Crypto,
    Equity,
    Index,
    Future {
        series: Option<String>,
        underlying: Option<Product>,
        multiplier: Decimal,
        expiration: DateTime<Utc>,
        derivative_kind: DerivativeKind,
        #[serde(default)]
        first_notice_date: Option<NaiveDate>,
    },
    FutureSpread {
        legs: Vec<SpreadLeg>,
    },
    Perpetual {
        underlying: Option<Product>,
        multiplier: Decimal,
        derivative_kind: DerivativeKind,
    },
    Option {
        series: OptionsSeries,
        instance: OptionsSeriesInstance,
    },
    EventContract {
        series: EventContractSeries,
        instance: EventContractSeriesInstance,
    },
    #[serde(other)]
    Unknown,
}

#[derive(
    Debug, Copy, Clone, EnumString, IntoStaticStr, Serialize, Deserialize, JsonSchema,
)]
#[strum(ascii_case_insensitive)]
pub enum DerivativeKind {
    /// Normal futures
    Linear,
    /// Futures settled in the base currency
    Inverse,
    /// Quote currency different from settle currency
    Quanto,
}

#[cfg(feature = "postgres")]
crate::to_sql_str!(DerivativeKind);

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SpreadLeg {
    pub product: Product,
    /// Some spreads have different ratios for their legs, like buy 1 A, sell 2 B, buy 1 C;
    /// We would represent that with quantities in the legs: 1, -2, 1
    pub quantity: Decimal,
}

impl std::fmt::Display for SpreadLeg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.quantity > Decimal::ZERO {
            write!(f, "+{} {}", self.quantity, self.product)
        } else {
            write!(f, "{} {}", self.quantity, self.product)
        }
    }
}

impl std::str::FromStr for SpreadLeg {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (quantity, product) =
            s.split_once(' ').ok_or_else(|| anyhow!("invalid leg format"))?;
        Ok(Self { product: product.parse()?, quantity: quantity.parse()? })
    }
}

pub type AliasKind = String;
