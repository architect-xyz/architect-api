use crate::{symbology_v2::*, utils::sequence::SequenceIdAndNumber};
use chrono::{DateTime, Utc};
use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;

/// List all symbols
#[grpc(package = "json.architect")]
#[grpc(service = "SymbologyV2", name = "symbols", response = "SymbolsResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbolsRequest {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbolsResponse {
    pub symbols: Vec<String>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "SymbologyV2", name = "symbology_v2", response = "SymbologySnapshot")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbologyV2Request {}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbologySnapshot {
    #[serde(flatten)]
    pub sequence: SequenceIdAndNumber,
    pub products: BTreeMap<Product, ProductInfo>,
    pub options_series: BTreeMap<OptionsSeries, OptionsSeriesInfo>,
    pub execution_info: BTreeMap<String, BTreeMap<ExecutionVenue, ExecutionInfo>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbologyUpdate {
    #[serde(flatten)]
    pub sequence: SequenceIdAndNumber,
    #[serde(default)]
    pub products: Option<SnapshotOrUpdate<Product, ProductInfo>>,
    #[serde(default)]
    pub options_series: Option<SnapshotOrUpdate<OptionsSeries, OptionsSeriesInfo>>,
    #[serde(default)]
    pub execution_info: Option<SnapshotOrUpdate2<String, ExecutionVenue, ExecutionInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AddOrRemove<Symbol, Info> {
    Add {
        symbol: Symbol,
        #[serde(flatten)]
        info: Info,
    },
    Remove {
        symbol: Symbol,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SnapshotOrUpdate<Symbol: Eq + Ord, Info> {
    Snapshot { snapshot: BTreeMap<Symbol, Info> },
    Update { updates: Vec<AddOrRemove<Symbol, Info>> },
}

impl<Symbol: Eq + Ord, Info> SnapshotOrUpdate<Symbol, Info> {
    pub fn apply(self, map: &mut BTreeMap<Symbol, Info>) {
        match self {
            SnapshotOrUpdate::Snapshot { snapshot } => {
                *map = snapshot;
            }
            SnapshotOrUpdate::Update { updates } => {
                for action in updates {
                    match action {
                        AddOrRemove::Add { symbol, info } => {
                            map.insert(symbol, info);
                        }
                        AddOrRemove::Remove { symbol } => {
                            map.remove(&symbol);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AddOrRemove2<Symbol, Venue, Info> {
    Add {
        symbol: Symbol,
        #[serde(flatten)]
        info: Info,
    },
    Remove {
        symbol: Symbol,
        venue: Venue,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SnapshotOrUpdate2<Symbol: Eq + Ord, Venue: Eq + Ord, Info> {
    Snapshot { snapshot: BTreeMap<Symbol, BTreeMap<Venue, Info>> },
    Update { updates: Vec<AddOrRemove2<Symbol, Venue, Info>> },
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "SymbologyV2",
    name = "subscribe_symbology_v2",
    response = "SymbologyUpdate",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeSymbologyV2 {}

#[grpc(package = "json.architect")]
#[grpc(
    service = "SymbologyV2",
    name = "upload_symbology_v2",
    response = "UploadSymbologyV2Response"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UploadSymbologyV2Request {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub products: BTreeMap<Product, ProductInfo>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub options_series: BTreeMap<OptionsSeries, OptionsSeriesInfo>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub execution_info: BTreeMap<String, BTreeMap<ExecutionVenue, ExecutionInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UploadSymbologyV2Response {}

// One-shot RPC to the symbol store to make it expire symbols
#[grpc(package = "json.architect")]
#[grpc(
    service = "SymbologyV2",
    name = "prune_expired_symbols",
    response = "PruneExpiredSymbolsResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PruneExpiredSymbolsRequest {
    /// If None then it will just use server current time;
    /// otherwise, specify a unix timestamp in seconds
    pub cutoff: Option<i64>,
}

impl PruneExpiredSymbolsRequest {
    pub fn new(cutoff: Option<DateTime<Utc>>) -> Self {
        Self { cutoff: cutoff.map(|dt| dt.timestamp()) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PruneExpiredSymbolsResponse {}
