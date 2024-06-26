#![cfg(feature = "netidx")]

use crate::{
    symbology::{MarketId, ProductId},
    Dir, DirPair,
};
use anyhow::anyhow;
use chrono::{DateTime, TimeDelta, Utc};
use derive::FromValue;
use enumflags2::bitflags;
use netidx::{path::Path, pool::Pooled};
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tokio_postgres::Row;

pub mod databento;
pub mod snapshots;

// CR alee: deprecate this in favor of [Symbolic]; would need to adjust how blockchain QFs work
/// Quotefeed path definitions for symbolics
pub trait NetidxFeedPaths {
    fn path_by_id(&self, base: &Path) -> Path;
    fn path_by_name(&self, base: &Path) -> Path;
    fn unalias_id(&self) -> Option<String>;
}

/// Book snapshot
#[derive(Debug, Clone, PartialEq, Eq, Pack)]
pub struct Snapshot {
    pub book: DirPair<Pooled<Vec<(Decimal, Decimal)>>>,
    #[pack(default)]
    pub timestamp: DateTime<Utc>,
}

/// Book update
#[derive(Debug, Clone, PartialEq, Eq, Pack)]
#[pack(unwrapped)]
pub enum Update {
    Remove { price: Decimal },
    Change { price: Decimal, size: Decimal },
}

/// Book updates
#[derive(Debug, Clone, PartialEq, Eq, Pack)]
pub struct Updates {
    pub book: DirPair<Pooled<Vec<Update>>>,
    #[pack(default)]
    pub timestamp: DateTime<Utc>,
}

impl Default for Updates {
    fn default() -> Self {
        Self {
            book: DirPair { buy: Pooled::orphan(vec![]), sell: Pooled::orphan(vec![]) },
            timestamp: DateTime::<Utc>::default(),
        }
    }
}

impl Updates {
    pub fn len(&self) -> usize {
        self.book.buy.len() + self.book.sell.len()
    }

    pub fn clear(&mut self) {
        self.book.buy.clear();
        self.book.sell.clear();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Pack)]
#[pack(unwrapped)]
pub enum MessageHeader {
    Updates,
    Snapshot,
}

#[derive(
    Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash, Serialize, Pack, FromValue,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[bitflags]
#[repr(u8)]
pub enum CandleWidth {
    OneSecond,
    FiveSecond,
    OneMinute,
    FifteenMinute,
    OneHour,
    OneDay,
}

impl CandleWidth {
    pub fn all() -> Vec<Self> {
        vec![
            Self::OneSecond,
            Self::FiveSecond,
            Self::OneMinute,
            Self::FifteenMinute,
            Self::OneHour,
            Self::OneDay,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OneSecond => "1s",
            Self::FiveSecond => "5s",
            Self::OneMinute => "1m",
            Self::FifteenMinute => "15m",
            Self::OneHour => "1h",
            Self::OneDay => "1d",
        }
    }

    pub fn as_seconds(&self) -> i64 {
        match self {
            Self::OneSecond => 1,
            Self::FiveSecond => 5,
            Self::OneMinute => 60,
            Self::FifteenMinute => 900,
            Self::OneHour => 3600,
            Self::OneDay => 86400,
        }
    }
}

impl FromStr for CandleWidth {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1s" => Ok(Self::OneSecond),
            "5s" => Ok(Self::FiveSecond),
            "1m" => Ok(Self::OneMinute),
            "15m" => Ok(Self::FifteenMinute),
            "1h" => Ok(Self::OneHour),
            "1d" => Ok(Self::OneDay),
            _ => Err(anyhow!("invalid candle width: {}", s)),
        }
    }
}

impl Into<TimeDelta> for CandleWidth {
    fn into(self) -> TimeDelta {
        TimeDelta::seconds(self.as_seconds())
    }
}

/// NB: buy_volume + sell_volume <> volume; volume may count trades
/// that don't have a discernible direction.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Deserialize,
    Serialize,
    Pack,
    FromValue,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct CandleV1 {
    pub time: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    pub buy_volume: Decimal,
    pub sell_volume: Decimal,
}

impl TryInto<CandleV1> for Row {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<CandleV1, Self::Error> {
        Ok(CandleV1 {
            time: self.try_get("ts")?,
            open: self.try_get("open_price")?,
            high: self.try_get("high_price")?,
            close: self.try_get("close_price")?,
            low: self.try_get("low_price")?,
            volume: self.try_get("volume")?,
            buy_volume: self.try_get("buy_volume")?,
            sell_volume: self.try_get("sell_volume")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Pack, FromValue)]
pub struct HistoricalCandlesV1 {
    pub candles: Vec<CandleV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Pack, FromValue)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct TradeV0 {
    pub time: Option<DateTime<Utc>>,
    pub direction: Dir,
    pub price: Decimal,
    pub size: Decimal,
}

impl Into<TradeV1> for TradeV0 {
    fn into(self) -> TradeV1 {
        TradeV1 {
            time: self.time,
            direction: Some(self.direction),
            price: self.price,
            size: self.size,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Pack, FromValue)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct TradeV1 {
    pub time: Option<DateTime<Utc>>,
    pub direction: Option<Dir>, // maker dir
    pub price: Decimal,
    pub size: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Pack, FromValue)]
pub struct TradeGlobalV1 {
    pub market: MarketId,
    pub time: Option<DateTime<Utc>>,
    pub direction: Option<Dir>, // maker dir
    pub price: Decimal,
    pub size: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Pack, FromValue)]
pub struct LiquidationV1 {
    pub time: DateTime<Utc>,
    pub direction: Dir,
    pub price: Decimal,
    pub size: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Pack, FromValue)]
pub struct LiquidationGlobalV1 {
    pub market: MarketId,
    pub time: DateTime<Utc>,
    pub direction: Dir,
    pub price: Decimal,
    pub size: Decimal,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Pack, FromValue,
)]
pub struct RfqRequest {
    pub base: ProductId,
    pub quote: ProductId,
    pub quantity: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Pack, FromValue)]
pub struct RfqResponse {
    pub market: MarketId,
    pub sides: DirPair<Result<RfqResponseSide, String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Pack, FromValue)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct RfqResponseSide {
    pub price: Decimal,
    pub quantity: Decimal,
    pub quote_id: Option<String>,
}
