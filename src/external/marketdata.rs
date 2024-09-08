use crate::symbology::MarketId;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1BookSnapshot {
    pub timestamp: DateTime<Utc>,
    pub epoch: DateTime<Utc>,
    pub seqno: u64,
    pub best_bid: Option<(Decimal, Decimal)>,
    pub best_ask: Option<(Decimal, Decimal)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L2BookSnapshot {
    pub timestamp: DateTime<Utc>,
    pub epoch: DateTime<Utc>,
    pub seqno: u64,
    pub bids: Vec<(Decimal, Decimal)>,
    pub asks: Vec<(Decimal, Decimal)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryL2BookSnapshot {
    pub market_id: MarketId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L3BookSnapshot {
    pub timestamp: DateTime<Utc>,
    pub epoch: DateTime<Utc>,
    pub seqno: u64,
    pub bids: Vec<(u64, Decimal, Decimal)>,
    pub asks: Vec<(u64, Decimal, Decimal)>,
}
