use super::ExecutionVenue;
use crate::symbology::market::MinOrderQuantityUnit;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Information about a symbol related to its execution route.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionInfo {
    pub execution_venue: ExecutionVenue,
    pub tick_size: TickSize,
    pub step_size: Decimal,
    pub min_order_quantity: Decimal,
    pub min_order_quantity_unit: MinOrderQuantityUnit,
    pub is_delisted: bool,
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
