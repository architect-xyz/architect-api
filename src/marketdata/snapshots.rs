use crate::symbology::{MarketId, ProductId};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Pack, FromValue)]
pub struct MarketSnapshot {
    pub market: MarketId,
    pub snapshot_ts: DateTime<Utc>,
    pub bid_price: Option<Decimal>,
    pub ask_price: Option<Decimal>,
    pub last_price: Option<Decimal>,
    pub open_24h: Option<Decimal>,
    pub high_24h: Option<Decimal>,
    pub low_24h: Option<Decimal>,
    pub volume_24h: Option<Decimal>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Pack, FromValue)]
pub struct OptionsMarketSnapshot {
    pub market: MarketId,
    pub underlying: ProductId,
    pub snapshot_ts: DateTime<Utc>,
    pub open_interest: Option<Decimal>,
    pub delta: Option<Decimal>,
    pub gamma: Option<Decimal>,
    pub theta: Option<Decimal>,
    pub vega: Option<Decimal>,
    pub rho: Option<Decimal>,
    pub bid_iv: Option<Decimal>,
    pub ask_iv: Option<Decimal>,
    pub und_price: Option<Decimal>,
}
