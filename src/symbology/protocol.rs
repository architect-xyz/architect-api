use super::*;
use crate::SequenceIdAndNumber;
use chrono::{DateTime, Utc};
use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;

/// List all symbols
#[grpc(package = "json.architect")]
#[grpc(service = "Symbology", name = "symbols", response = "SymbolsResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbolsRequest {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbolsResponse {
    pub symbols: Vec<String>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Symbology", name = "symbology", response = "SymbologySnapshot")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbologyRequest {}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbologySnapshot {
    #[serde(flatten)]
    pub sequence: SequenceIdAndNumber,
    pub products: BTreeMap<Product, ProductInfo>,
    #[serde(default)]
    pub product_aliases: BTreeMap<AliasKind, BTreeMap<String, Product>>,
    pub options_series: BTreeMap<OptionsSeries, OptionsSeriesInfo>,
    pub execution_info: BTreeMap<String, BTreeMap<ExecutionVenue, ExecutionInfo>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbologyUpdate {
    #[serde(flatten)]
    pub sequence: SequenceIdAndNumber,
    #[serde(default)]
    pub products: Option<SnapshotOrUpdate1<Product, ProductInfo>>,
    #[serde(default)]
    pub product_aliases:
        Option<SnapshotOrUpdate<AliasKind, SnapshotOrUpdate<String, Product>>>,
    #[serde(default)]
    pub options_series: Option<SnapshotOrUpdate1<OptionsSeries, OptionsSeriesInfo>>,
    #[serde(default)]
    pub execution_info: Option<SnapshotOrUpdate2<String, ExecutionVenue, ExecutionInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum SnapshotOrUpdate<K: Eq + Ord, V> {
    Snapshot { snapshot: BTreeMap<K, V> },
    Update { updates: Vec<(K, Option<V>)> },
}

impl<K: Eq + Ord, V> From<BTreeMap<K, V>> for SnapshotOrUpdate<K, V> {
    fn from(map: BTreeMap<K, V>) -> Self {
        SnapshotOrUpdate::Snapshot { snapshot: map }
    }
}

impl<K: Eq + Ord, V> SnapshotOrUpdate<K, V> {
    pub fn apply(self, map: &mut BTreeMap<K, V>) {
        match self {
            Self::Snapshot { snapshot } => {
                *map = snapshot;
            }
            Self::Update { updates } => {
                for (k, v) in updates {
                    if let Some(v) = v {
                        map.insert(k, v);
                    } else {
                        map.remove(&k);
                    }
                }
            }
        }
    }
}

impl<K0: Eq + Ord, K1: Eq + Ord, V> From<BTreeMap<K0, BTreeMap<K1, V>>>
    for SnapshotOrUpdate<K0, SnapshotOrUpdate<K1, V>>
{
    fn from(map: BTreeMap<K0, BTreeMap<K1, V>>) -> Self {
        SnapshotOrUpdate::Snapshot {
            snapshot: map
                .into_iter()
                .map(|(k, v)| (k, SnapshotOrUpdate::Snapshot { snapshot: v }))
                .collect(),
        }
    }
}

impl<K0: Eq + Ord, K1: Eq + Ord, V> SnapshotOrUpdate<K0, SnapshotOrUpdate<K1, V>> {
    pub fn apply2(self, map: &mut BTreeMap<K0, BTreeMap<K1, V>>) {
        match self {
            Self::Snapshot { snapshot } => {
                map.clear();
                for (k, t) in snapshot {
                    match t {
                        SnapshotOrUpdate::Snapshot { snapshot } => {
                            map.insert(k, snapshot);
                        }
                        u @ SnapshotOrUpdate::Update { .. } => {
                            let mut v = BTreeMap::new();
                            u.apply(&mut v);
                            map.insert(k, v);
                        }
                    }
                }
            }
            Self::Update { updates } => {
                for (k, t) in updates {
                    match t {
                        Some(SnapshotOrUpdate::Snapshot { snapshot }) => {
                            map.insert(k, snapshot);
                        }
                        Some(u @ SnapshotOrUpdate::Update { .. }) => {
                            let v = map.entry(k).or_default();
                            u.apply(v);
                        }
                        None => {
                            map.remove(&k);
                        }
                    }
                }
            }
        }
    }
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

// CR alee: deprecate in favor of `SnapshotOrUpdate`
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SnapshotOrUpdate1<Symbol: Eq + Ord, Info> {
    Snapshot { snapshot: BTreeMap<Symbol, Info> },
    Update { updates: Vec<AddOrRemove<Symbol, Info>> },
}

impl<Symbol: Eq + Ord, Info> SnapshotOrUpdate1<Symbol, Info> {
    pub fn apply(self, map: &mut BTreeMap<Symbol, Info>) {
        match self {
            SnapshotOrUpdate1::Snapshot { snapshot } => {
                *map = snapshot;
            }
            SnapshotOrUpdate1::Update { updates } => {
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
    service = "Symbology",
    name = "subscribe_symbology",
    response = "SymbologyUpdate",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeSymbology {}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Symbology",
    name = "upload_symbology",
    response = "UploadSymbologyResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UploadSymbologyRequest {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub products: BTreeMap<Product, ProductInfo>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub product_aliases: BTreeMap<AliasKind, BTreeMap<String, Product>>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub options_series: BTreeMap<OptionsSeries, OptionsSeriesInfo>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub execution_info: BTreeMap<String, BTreeMap<ExecutionVenue, ExecutionInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UploadSymbologyResponse {}

// One-shot RPC to the symbol store to make it expire symbols
#[grpc(package = "json.architect")]
#[grpc(
    service = "Symbology",
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
