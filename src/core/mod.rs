//! gRPC interface for the core system; configuration, administration, etc.

use crate::symbology::MarketdataVenue;
use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use url::Url;

#[grpc(package = "json.architect")]
#[grpc(service = "Core", name = "config", response = "ConfigResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConfigRequest {}

#[grpc(package = "json.architect")]
#[grpc(service = "Core", name = "config", response = "ConfigResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConfigResponse {
    pub marketdata: BTreeMap<MarketdataVenue, Url>,
}
