use crate::symbology::MarketId;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookSnapshot {
    pub timestamp: DateTime<Utc>,
    pub seqno: u64,
    pub bids: Vec<(Decimal, Decimal)>,
    pub asks: Vec<(Decimal, Decimal)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryBookSnapshot {
    pub market_id: MarketId,
}
