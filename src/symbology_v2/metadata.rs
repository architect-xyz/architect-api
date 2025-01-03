use super::{
    EventContractSeries, EventContractSeriesInstance, ExecutionVenue, MarketdataVenue,
    OptionsSeries, OptionsSeriesInstance, Product,
};
use crate::symbology::market::MinOrderQuantityUnit;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProductInfo {
    Crypto,
    Fiat,
    Equity,
    Index,
    Commodity,
    Perpetual {
        underlying: Product,
        multiplier: Decimal,
        derivative_kind: DerivativeKind,
    },
    Future {
        underlying: Product,
        multiplier: Decimal,
        expiration: DateTime<Utc>,
        derivative_kind: DerivativeKind,
    },
    FutureSpread {
        legs: Vec<SpreadLeg>,
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

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct TradableProductInfo {
    pub quote: Product,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DerivativeKind {
    /// Normal futures
    Linear,
    /// Futures settled in the base currency
    Inverse,
    /// Quote currency different from settle currency
    Quanto,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SpreadLeg {
    pub product: Product,
    /// Some spreads have different ratios for their legs, like buy 1 A, sell 2 B, buy 1 C;
    /// We would represent that with quantities in the legs: 1, -2, 1
    pub quantity: Decimal,
}

impl ProductInfo {
    pub fn expiration(&self) -> Option<DateTime<Utc>> {
        match self {
            ProductInfo::Crypto
            | ProductInfo::Fiat
            | ProductInfo::Equity
            | ProductInfo::Index
            | ProductInfo::Commodity
            | ProductInfo::Unknown
            | ProductInfo::Perpetual { .. }
            | ProductInfo::FutureSpread { .. } => None,
            ProductInfo::Option {
                instance: OptionsSeriesInstance { expiration, .. },
                ..
            } => Some(*expiration),
            ProductInfo::EventContract { instance, .. } => instance.expiration(),
            ProductInfo::Future { expiration, .. } => Some(*expiration),
        }
    }

    pub fn is_expired(&self, cutoff: DateTime<Utc>) -> bool {
        if let Some(expiration) = self.expiration() {
            expiration <= cutoff
        } else {
            false
        }
    }
}

/// Information about a symbol related to its marketdata source.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct MarketdataInfo {
    pub marketdata_venue: MarketdataVenue,
    pub source_raw_symbol: String,
}

/// Information about a symbol related to its execution route.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionInfo {
    pub execution_venue: ExecutionVenue,
    // NB: for series products, interpretation of `venue_raw_symbol` is venue-specific
    pub venue_raw_symbol: String,
    pub only_possible_quote_symbol: Option<Product>,
    pub tick_size: Vec<TickSize>,
    pub step_size: Decimal,
    pub min_order_quantity: Decimal,
    pub min_order_quantity_unit: MinOrderQuantityUnit,
    pub is_delisted: bool,
    pub additional_info: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TickSize {
    Simple(Decimal),
    Varying { thresholds: Vec<(Decimal, Decimal)> },
}

impl TickSize {
    pub fn simple(tick_size: Decimal) -> Self {
        Self::Simple(tick_size)
    }
}
