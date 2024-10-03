use crate::symbology::{MarketId, ProductId};
use chrono::{DateTime, Utc};
#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
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
    #[serde(default)]
    #[cfg_attr(feature = "netidx", pack(default))]
    pub open_interest: Option<Decimal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub struct OptionsMarketSnapshot {
    pub market: MarketId,
    pub underlying: ProductId,
    pub snapshot_ts: DateTime<Utc>,
    pub delta: Option<Decimal>,
    pub gamma: Option<Decimal>,
    pub theta: Option<Decimal>,
    pub vega: Option<Decimal>,
    pub rho: Option<Decimal>,
    pub bid_iv: Option<Decimal>,
    pub ask_iv: Option<Decimal>,
    pub und_price: Option<Decimal>,
}
