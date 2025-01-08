use super::{ExecutionVenue, Product};
use crate::symbology::market::MinOrderQuantityUnit;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Information about a symbol related to its execution route.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionInfo {
    pub execution_venue: ExecutionVenue,
    // NB: for series products, interpretation of `venue_raw_symbol` is venue-specific
    pub venue_raw_symbol: String,
    /// For products or series, if there's only one possible quote symbol
    pub only_possible_quote_symbol: Option<Product>,
    pub tick_size: TickSize,
    pub step_size: Decimal,
    pub min_order_quantity: Decimal,
    pub min_order_quantity_unit: MinOrderQuantityUnit,
    pub is_delisted: bool,
    pub additional_info: Option<BTreeMap<String, String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TickSize {
    Simple(Decimal),
    /// List of (threshold, tick_size) pairs.  For price greater than or equal
    /// to each threshold, the tick size is the corresponding value.
    Varying {
        thresholds: Vec<(Decimal, Decimal)>,
    },
}

impl TickSize {
    pub fn simple(tick_size: Decimal) -> Self {
        Self::Simple(tick_size)
    }
}
