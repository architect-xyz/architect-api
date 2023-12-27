use crate::{
    symbology::{MarketId, ProductId},
    Dir, DirPair,
};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx::{path::Path, pool::Pooled};
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

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

/// NB: buy_volume + sell_volume <> volume; volume may count trades
/// that don't have a discernible direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Pack, FromValue)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Pack, FromValue)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct TradeV1 {
    pub time: Option<DateTime<Utc>>,
    pub direction: Option<Dir>,
    pub price: Decimal,
    pub size: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Pack, FromValue)]
pub struct TradeGlobalV1 {
    pub market: MarketId,
    pub time: Option<DateTime<Utc>>,
    pub direction: Option<Dir>,
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
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct RfqResponseSide {
    pub price: Decimal,
    pub quantity: Decimal,
    pub quote_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Pack, FromValue)]
pub struct RfqResponse {
    pub market: MarketId,
    pub sides: DirPair<Result<RfqResponseSide, String>>,
}

#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct GraphQLRfqResponse {
    pub market: MarketId,
    pub buy: Option<RfqResponseSide>,
    pub sell: Option<RfqResponseSide>,
}
