pub use crate::symbology::*;
use chrono::{DateTime, Utc};
use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[grpc(package = "json.architect")]
#[grpc(
    service = "Symbology",
    name = "symbology_snapshot",
    response = "SymbologySnapshot"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbologySnapshotRequest {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbologySnapshot {
    pub epoch: DateTime<Utc>,
    pub seqno: u64,
    pub routes: Vec<Route>,
    pub venues: Vec<Venue>,
    pub products: Vec<Product>,
    pub markets: Vec<Market>,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Symbology",
    name = "subscribe_symbology_updates",
    response = "SymbologyUpdate",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeSymbologyUpdatesRequest {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbologyUpdate {
    pub epoch: DateTime<Utc>,
    pub seqno: u64,
    pub routes: Option<Vec<Route>>,
    pub venues: Option<Vec<Venue>>,
    pub products: Option<Vec<Product>>,
    pub markets: Option<Vec<Market>>,
}
