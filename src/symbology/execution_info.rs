use super::ExecutionVenue;
use derive_more::Display;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, IntoStaticStr};

/// Information about a symbol related to its execution route.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionInfo {
    pub execution_venue: ExecutionVenue,
    pub tick_size: TickSize,
    pub step_size: Decimal,
    pub min_order_quantity: Decimal,
    pub min_order_quantity_unit: MinOrderQuantityUnit,
    pub is_delisted: bool,
    pub initial_margin: Option<Decimal>,
    pub maintenance_margin: Option<Decimal>,
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

// TODO: un snake_case this
#[derive(
    Default,
    Debug,
    Display,
    Clone,
    Copy,
    EnumString,
    IntoStaticStr,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[serde(tag = "unit", rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MinOrderQuantityUnit {
    #[default]
    Base,
    Quote,
}

#[cfg(feature = "postgres")]
crate::to_sql_str!(MinOrderQuantityUnit);
