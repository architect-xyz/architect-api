use crate::{marketdata::snapshots::MarketSnapshot, symbology::MarketId};
use chrono::{DateTime, Utc};
use derive::grpc;
use serde::{Deserialize, Serialize};

#[grpc(package = "json.architect")]
#[grpc(
    service = "MarketdataSnapshots",
    name = "marketdata_snapshot",
    response = "MarketdataSnapshot"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketdataSnapshotRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<MarketId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    pub latest_at_or_before: Option<DateTime<Utc>>,
}

// CR-soon alee: alias ahead of depecration/rename
pub type MarketdataSnapshot = MarketSnapshot;

#[grpc(package = "json.architect")]
#[grpc(
    service = "MarketdataSnapshots",
    name = "marketdata_snapshots",
    response = "MarketdataSnapshotsResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketdataSnapshotsRequest {
    // CR alee: not clear if we should allow such a broad query;
    // surely we'd like to limit to venue OR max_results when
    // ordered by something
    pub latest_at_or_before: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeMarketdataSnapshotsRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeMarketdataSnapshotsResponse {
    pub snapshots: Vec<MarketdataSnapshot>,
}
