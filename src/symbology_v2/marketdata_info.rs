use super::MarketdataVenue;
use crate::symbology_v2::Product;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::Not;

/// Information about a symbol related to its marketdata source.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct MarketdataInfo {
    pub marketdata_venue: MarketdataVenue,
    pub venue_raw_symbol: String,
    /// For products or series, if there's only one possible quote symbol
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only_possible_quote_symbol: Option<Product>,
    /// If true, this symbol is a scalar (e.g. index value);
    /// `only_possible_quote_symbol` isn't applicable in this case.
    #[serde(default, skip_serializing_if = "Not::not")]
    pub is_scalar: bool,
}
