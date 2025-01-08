use super::MarketdataVenue;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Information about a symbol related to its marketdata source.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct MarketdataInfo {
    pub marketdata_venue: MarketdataVenue,
    pub venue_raw_symbol: String,
}
