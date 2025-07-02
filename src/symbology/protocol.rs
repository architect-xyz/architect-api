use super::*;
use crate::SequenceIdAndNumber;
use chrono::{DateTime, Utc};
use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::{BTreeMap, BTreeSet};

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
#[grpc(
    service = "Symbology",
    name = "execution_info",
    response = "ExecutionInfoResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionInfoRequest {
    pub symbol: String,
    pub execution_venue: Option<ExecutionVenue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionInfoResponse {
    pub execution_info: BTreeMap<ExecutionVenue, ExecutionInfo>,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Symbology",
    name = "futures_series",
    response = "FuturesSeriesResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FuturesSeriesRequest {
    pub series_symbol: String,
    #[serde(default)]
    pub include_expired: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FuturesSeriesResponse {
    pub futures: Vec<Product>,
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
    #[serde(default)]
    pub product_catalog: BTreeMap<ExecutionVenue, BTreeMap<String, ProductCatalogInfo>>,
    pub options_series: BTreeMap<OptionsSeries, OptionsSeriesInfo>,
    pub execution_info:
        BTreeMap<TradableProduct, BTreeMap<ExecutionVenue, ExecutionInfo>>,
}

impl SymbologySnapshot {
    pub fn exchange_symbols(
        &self,
        venue: &ExecutionVenue,
    ) -> BTreeMap<TradableProduct, String> {
        let mut map = BTreeMap::new();
        for (symbol, infos) in &self.execution_info {
            if let Some(exchange_symbol) =
                infos.get(venue).and_then(|info| info.exchange_symbol.as_ref())
            {
                map.insert(symbol.clone(), exchange_symbol.clone());
            }
        }
        map
    }

    pub fn exchange_symbol(
        &self,
        venue: &str,
        symbol: &TradableProduct,
    ) -> Option<&String> {
        self.execution_info.get(symbol).and_then(|infos| {
            infos.get(venue).and_then(|info| info.exchange_symbol.as_ref())
        })
    }

    pub fn filter_venue(mut self, venue: &ExecutionVenue) -> Self {
        let mut out = Self::default();
        let mut products = BTreeSet::default();
        for (symbol, infos) in self.execution_info {
            for (venue_key, info) in infos {
                if &venue_key != venue {
                    continue;
                }
                out.execution_info
                    .entry(symbol.clone())
                    .or_default()
                    .insert(venue.clone(), info);
                products.insert(symbol.base());
                if let Some(quote) = symbol.quote() {
                    products.insert(quote);
                }
            }
        }
        for (alias_kind, alias_map) in self.product_aliases {
            for (alias, product) in alias_map {
                if products.contains(&product) {
                    out.product_aliases
                        .entry(alias_kind)
                        .or_default()
                        .insert(alias, product);
                }
            }
        }
        for product in products {
            if let Some(info) = self.products.remove(&product) {
                out.products.insert(product, info);
            }
        }
        out
    }
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbologyUpdate {
    #[serde(flatten)]
    pub sequence: SequenceIdAndNumber,
    #[serde(default)]
    pub products: Option<SnapshotOrUpdate<Product, ProductInfo>>,
    #[serde(default)]
    pub product_aliases:
        Option<SnapshotOrUpdate<AliasKind, SnapshotOrUpdate<String, Product>>>,
    #[serde(default)]
    pub product_catalog: Option<
        SnapshotOrUpdate<ExecutionVenue, SnapshotOrUpdate<String, ProductCatalogInfo>>,
    >,
    #[serde(default)]
    pub options_series: Option<SnapshotOrUpdate<OptionsSeries, OptionsSeriesInfo>>,
    #[serde(default)]
    pub execution_info: Option<
        SnapshotOrUpdate<
            TradableProduct,
            SnapshotOrUpdate<ExecutionVenue, ExecutionInfo>,
        >,
    >,
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
    pub execution_info:
        BTreeMap<TradableProduct, BTreeMap<ExecutionVenue, ExecutionInfo>>,
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

#[grpc(package = "json.architect")]
#[grpc(
    service = "Symbology",
    name = "upload_product_catalog",
    response = "UploadProductCatalogResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UploadProductCatalogRequest {
    pub exchange: ExecutionVenue,
    pub product_catalog: Vec<ProductCatalogInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UploadProductCatalogResponse {}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Symbology",
    name = "download_product_catalog",
    response = "DownloadProductCatalogResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DownloadProductCatalogRequest {
    pub exchange: ExecutionVenue,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DownloadProductCatalogResponse {
    pub product_catalog: Vec<ProductCatalogInfo>,
}
