use crate::symbology_v2::*;
use chrono::{DateTime, Utc};
use derive::grpc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AddOrRemove<Symbol, Metadata> {
    Add { symbol: Symbol, metadata: Metadata },
    Remove { symbol: Symbol },
}

// TODO: maybe too obtuse?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AddOrRemoveVenue<Symbol, Venue, Metadata> {
    Add { symbol: Symbol, venue: Venue, metadata: Metadata },
    Remove { symbol: Symbol, venue: Venue },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SymbologyUpdate {
    Product(AddOrRemove<Product, ProductInfo>),
    TradableProduct(AddOrRemove<TradableProduct, TradableProductInfo>),
    TradableProductMarketdata(
        AddOrRemoveVenue<TradableProduct, MarketdataVenue, MarketdataInfo>,
    ),
    TradableProductExecution(
        AddOrRemoveVenue<TradableProduct, ExecutionVenue, ExecutionInfo>,
    ),
    OptionsSeries(AddOrRemove<OptionsSeries, OptionsSeriesInfo>),
    OptionsSeriesMarketdata(
        AddOrRemoveVenue<OptionsSeries, MarketdataVenue, MarketdataInfo>,
    ),
    OptionsSeriesExecution(
        AddOrRemoveVenue<OptionsSeries, ExecutionVenue, ExecutionInfo>,
    ),
    EventContractSeries(AddOrRemove<EventContractSeries, EventContractSeriesInfo>),
    EventContractSeriesMarketdata(
        AddOrRemoveVenue<EventContractSeries, MarketdataVenue, MarketdataInfo>,
    ),
    EventContractSeriesExecution(
        AddOrRemoveVenue<EventContractSeries, ExecutionVenue, ExecutionInfo>,
    ),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbologyTransaction {
    Begin,
    Update(Vec<SymbologyUpdate>),
    Commit,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "SymbologyV2",
    name = "subscribe_symbology_v2",
    response = "SymbologyTransaction",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeSymbologyV2 {}

#[grpc(package = "json.architect")]
#[grpc(
    service = "SymbologyV2",
    name = "upload_symbology_v2",
    request = "SymbologyTransaction",
    client_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadSymbologyV2Response {}

// One-shot RPC to the symbol store to make it expire symbols
#[grpc(package = "json.architect")]
#[grpc(
    service = "SymbologyV2",
    name = "prune_expired_symbols",
    response = "PruneExpiredSymbolsResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PruneExpiredSymbolsResponse {}
