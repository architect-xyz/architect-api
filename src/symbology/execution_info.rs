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
    /// If the execution venue has stable symbology, this may be populated
    pub exchange_symbol: Option<String>,
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
    #[schemars(title = "Simple|Decimal")]
    Simple(Decimal),
    /// List of (threshold, tick_size) pairs.  For price greater than or equal
    /// to each threshold, the tick size is the corresponding value.
    #[schemars(title = "Varying")]
    Varying { thresholds: Vec<(Decimal, Decimal)> },
}

impl TickSize {
    pub fn simple(tick_size: Decimal) -> Self {
        Self::Simple(tick_size)
    }

    /// Number of ticks between `from` to `to` based on the tick size.
    ///
    /// If `from` is less than `to`, the result is positive.  If `from` is
    /// greater than `to`, the result is negative.
    ///
    /// If step size is zero, return None.
    pub fn signed_tick_distance(&self, from: Decimal, to: Decimal) -> Option<Decimal> {
        if from == to {
            return Some(Decimal::ZERO);
        } else if from > to {
            return self.signed_tick_distance(to, from).map(|d| -d);
        }
        // INVARIANT: from < to
        match self {
            TickSize::Simple(step) => {
                if step.is_zero() {
                    None
                } else {
                    Some((to - from) / *step)
                }
            }
            TickSize::Varying { thresholds } => {
                let mut ticks = Decimal::ZERO;
                let mut price = from;
                let mut i = thresholds.iter();
                let mut step = None;
                // find the first threshold including the price
                while let Some((lower, lower_step)) = i.next() {
                    if price >= *lower {
                        step = Some(*lower_step);
                        break;
                    }
                }
                let mut step = step?; // threshold doesn't include price at all, impossible to compute
                loop {
                    if step.is_zero() {
                        return None;
                    }
                    match i.next() {
                        Some((upper, next_step)) => {
                            // `to` is beyond the upper threshold, compute distance to next threshold
                            if to >= *upper {
                                ticks += (upper - price) / step;
                                price = *upper;
                                step = *next_step;
                            } else {
                                // `to` is below the upper threshold, compute distance to `to` and done
                                ticks += (to - price) / step;
                                break;
                            }
                        }
                        None => {
                            // no more thresholds, compute straightforward distance
                            ticks += (to - price) / step;
                            break;
                        }
                    }
                }
                Some(ticks)
            }
        }
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
    #[schemars(title = "Base")]
    Base,
    #[schemars(title = "Quote")]
    Quote,
}

#[cfg(feature = "postgres")]
crate::to_sql_str!(MinOrderQuantityUnit);
