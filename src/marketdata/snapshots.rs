use crate::symbology::MarketdataVenue;
use chrono::{DateTime, Utc};
use derive::grpc;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[grpc(package = "json.architect")]
#[grpc(
    service = "MarketdataSnapshots",
    name = "marketdata_snapshot",
    response = "MarketdataSnapshot"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MarketdataSnapshotRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<MarketdataVenue>,
    pub symbol: String,
    pub latest_at_or_before: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct MarketdataSnapshot {
    #[serde(rename = "s")]
    #[schemars(title = "symbol")]
    pub symbol: String,
    #[serde(rename = "ts")]
    #[schemars(title = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "timestamp_ns")]
    pub timestamp_ns: u32,
    #[serde(rename = "b")]
    #[schemars(title = "bid_price")]
    pub bid_price: Option<Decimal>,
    #[serde(rename = "a")]
    #[schemars(title = "ask_price")]
    pub ask_price: Option<Decimal>,
    #[serde(rename = "p")]
    #[schemars(title = "last_price")]
    pub last_price: Option<Decimal>,
    #[serde(rename = "o")]
    #[schemars(title = "open_24h")]
    pub open_24h: Option<Decimal>,
    #[serde(rename = "v")]
    #[schemars(title = "volume_24h")]
    pub volume_24h: Option<Decimal>,
    #[serde(rename = "l")]
    #[schemars(title = "low_24h")]
    pub low_24h: Option<Decimal>,
    #[serde(rename = "h")]
    #[schemars(title = "high_24h")]
    pub high_24h: Option<Decimal>,
    #[serde(rename = "oi")]
    #[schemars(title = "open_interest")]
    pub open_interest: Option<Decimal>,
}

impl MarketdataSnapshot {
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::<Utc>::from_timestamp(self.timestamp, self.timestamp_ns)
    }
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "MarketdataSnapshots",
    name = "marketdata_snapshots",
    response = "MarketdataSnapshotsResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MarketdataSnapshotsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<MarketdataVenue>,
    // CR alee: not clear if we should allow such a broad query;
    // surely we'd like to limit to venue OR max_results when
    // ordered by something
    pub latest_at_or_before: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MarketdataSnapshotsResponse {
    pub snapshots: Vec<MarketdataSnapshot>,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "MarketdataSnapshots",
    name = "subscribe_marketdata_snapshots",
    response = "SubscribeMarketdataSnapshotsResponse"
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeMarketdataSnapshotsRequest {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeMarketdataSnapshotsResponse {
    pub snapshots: Vec<MarketdataSnapshot>,
}
