use crate::symbology::MarketId;
use chrono::{DateTime, Utc};
use derive::grpc;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "l1_book_snapshot", response = "L1BookSnapshot")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1BookSnapshotRequest {
    pub market_id: MarketId,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "l1_book_snapshots", response = "L1BookSnapshot")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1BookSnapshotsRequest {
    pub market_ids: Vec<MarketId>,
}

pub type L1BookSnapshots = Vec<L1BookSnapshot>;

#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "subscribe_l1_book_snapshots",
    response = "L1BookSnapshot",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeL1BookSnapshotsRequest {
    /// If None, subscribe from all symbols on the feed
    pub market_ids: Option<Vec<MarketId>>,
}

impl From<Vec<MarketId>> for SubscribeL1BookSnapshotsRequest {
    fn from(market_ids: Vec<MarketId>) -> Self {
        Self { market_ids: Some(market_ids) }
    }
}

impl From<Option<MarketId>> for SubscribeL1BookSnapshotsRequest {
    fn from(market_id: Option<MarketId>) -> Self {
        Self { market_ids: market_id.map(|id| vec![id]) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1BookSnapshot {
    #[serde(rename = "m")]
    pub market_id: MarketId,
    #[serde(rename = "ts")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    pub timestamp_ns: u32,
    #[serde(rename = "e", skip_serializing_if = "Option::is_none", default)]
    pub epoch: Option<i64>,
    #[serde(rename = "n", skip_serializing_if = "Option::is_none", default)]
    pub seqno: Option<u64>,
    #[serde(rename = "b")]
    pub best_bid: Option<(Decimal, Decimal)>,
    #[serde(rename = "a")]
    pub best_ask: Option<(Decimal, Decimal)>,
}

impl L1BookSnapshot {
    pub fn new(
        market_id: MarketId,
        timestamp: DateTime<Utc>,
        epoch: Option<DateTime<Utc>>,
        seqno: Option<u64>,
        best_bid: Option<(Decimal, Decimal)>,
        best_ask: Option<(Decimal, Decimal)>,
    ) -> Self {
        Self {
            market_id,
            timestamp: timestamp.timestamp(),
            timestamp_ns: timestamp.timestamp_subsec_nanos(),
            epoch: epoch.map(|e| e.timestamp()),
            seqno,
            best_bid,
            best_ask,
        }
    }

    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        chrono::DateTime::from_timestamp(self.timestamp, self.timestamp_ns)
    }

    pub fn epoch(&self) -> Option<DateTime<Utc>> {
        chrono::DateTime::from_timestamp(self.epoch?, 0)
    }
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
