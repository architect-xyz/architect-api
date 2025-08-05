use super::ExecutionVenue;
use crate::Dir;
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
    ///
    /// For example, if the thresholds are [(100, 0.01), (200, 0.02)], valid
    /// prices include:
    ///
    /// ...199.98, 199.99, 200.00, 200.02, 200.04...
    #[schemars(title = "Varying")]
    Varying { thresholds: Vec<(Decimal, Decimal)> },
}

impl TickSize {
    pub fn simple(tick_size: Decimal) -> Self {
        Self::Simple(tick_size)
    }

    /// Increment the price by `n` ticks.  Negative values decrement.
    ///
    /// This method assumes:
    ///
    /// - `price` is a valid price (i.e. on a tick and not between ticks)
    /// - `price` is not on or below the first threshold, for negative `n`
    /// - `thresholds` is non-empty, if tick size is `Varying`
    /// - `thresholds` are well-formed[1]
    ///
    /// [1] Sequential thresholds must be arithemetically adjacent; e.g.
    /// `(100, 0.3), (200, _)` is an invalid threshold sequence because
    /// no iteration of ticks can get from 100 to 200.
    pub fn increment(&self, mut price: Decimal, mut n: i32) -> Option<Decimal> {
        if n == 0 {
            return Some(price);
        } else if n < 0 {
            return self.decrement(price, -n);
        }
        // INVARIANT: n > 0
        let thresholds = match self {
            TickSize::Simple(step) => {
                return Some(price + *step * Decimal::from(n));
            }
            TickSize::Varying { thresholds } => thresholds,
        };
        if thresholds.is_empty() {
            return None;
        }
        // INVARIANT: thresholds.len() > 0
        let mut i = thresholds.len() - 1;
        let mut t_bound;
        let mut t_step;
        let mut u_bound;
        loop {
            (t_bound, t_step) = thresholds[i];
            u_bound = thresholds.get(i + 1).map(|(b, _)| *b);
            if price >= t_bound {
                break;
            }
            if i == 0 {
                // didn't find a threshold
                return None;
            }
            i -= 1;
        }
        while n > 0 {
            price += t_step;
            if u_bound.is_some_and(|u| price >= u) {
                // INVARIANT: threshold[i + 1] exists
                // move to next threshold
                i += 1;
                (_, t_step) = thresholds[i];
                u_bound = thresholds.get(i + 1).map(|(b, _)| *b);
            }
            n -= 1;
        }
        Some(price)
    }

    pub fn decrement(&self, mut price: Decimal, mut n: i32) -> Option<Decimal> {
        if n == 0 {
            return Some(price);
        } else if n < 0 {
            return self.increment(price, -n);
        }
        // INVARIANT: n > 0
        let thresholds = match self {
            TickSize::Simple(step) => {
                return Some(price - *step * Decimal::from(n));
            }
            TickSize::Varying { thresholds } => thresholds,
        };
        if thresholds.is_empty() {
            return None;
        }
        // INVARIANT: thresholds.len() > 0
        let mut i = thresholds.len() - 1;
        let mut t_bound;
        let mut t_step;
        loop {
            (t_bound, t_step) = thresholds[i];
            if price >= t_bound {
                break;
            }
            if i == 0 {
                // didn't find a threshold
                return None;
            }
            i -= 1;
        }
        while n > 0 {
            if price == t_bound {
                // on boundary, use previous threshold tick size
                if i == 0 {
                    return None;
                }
                i -= 1;
                (t_bound, t_step) = thresholds[i];
            }
            price -= t_step;
            n -= 1;
        }
        Some(price)
    }

    /// Round the price to make the price more aggressive for the given direction.
    pub fn round_aggressive(&self, price: Decimal, dir: Dir) -> Option<Decimal> {
        match dir {
            Dir::Buy => self.round_up(price),
            Dir::Sell => self.round_down(price),
        }
    }

    /// Round the price to make the price more passive for the given direction.
    pub fn round_passive(&self, price: Decimal, dir: Dir) -> Option<Decimal> {
        match dir {
            Dir::Buy => self.round_down(price),
            Dir::Sell => self.round_up(price),
        }
    }

    pub fn round_up(&self, price: Decimal) -> Option<Decimal> {
        match self {
            TickSize::Simple(tick_size) => {
                if tick_size.is_zero() {
                    return None;
                }
                let remainder = price % tick_size;
                if remainder.is_zero() {
                    // Already on a tick
                    Some(price)
                } else if remainder > Decimal::ZERO {
                    // Positive remainder: round up to next tick
                    Some(price - remainder + tick_size)
                } else {
                    // Negative remainder: already rounded up
                    Some(price - remainder)
                }
            }
            TickSize::Varying { thresholds } => {
                if thresholds.is_empty() {
                    return None;
                }

                // Find the appropriate tick size for this price
                let mut tick_size = thresholds[0].1;
                for (threshold, size) in thresholds {
                    if price >= *threshold {
                        tick_size = *size;
                    } else {
                        break;
                    }
                }

                if tick_size.is_zero() {
                    return None;
                }

                let remainder = price % tick_size;
                if remainder.is_zero() {
                    // Already on a tick
                    Some(price)
                } else if remainder > Decimal::ZERO {
                    // Positive remainder: round up to next tick
                    Some(price - remainder + tick_size)
                } else {
                    // Negative remainder: already rounded up
                    Some(price - remainder)
                }
            }
        }
    }

    pub fn round_down(&self, price: Decimal) -> Option<Decimal> {
        match self {
            TickSize::Simple(tick_size) => {
                if tick_size.is_zero() {
                    return None;
                }
                let remainder = price % tick_size;
                if remainder.is_zero() {
                    // Already on a tick
                    Some(price)
                } else if remainder > Decimal::ZERO {
                    // Positive remainder: round down
                    Some(price - remainder)
                } else {
                    // Negative remainder: need to go down one more tick
                    Some(price - remainder - tick_size)
                }
            }
            TickSize::Varying { thresholds } => {
                if thresholds.is_empty() {
                    return None;
                }

                // Find the appropriate tick size for this price
                let mut tick_size = thresholds[0].1;
                for (threshold, size) in thresholds {
                    if price >= *threshold {
                        tick_size = *size;
                    } else {
                        break;
                    }
                }

                if tick_size.is_zero() {
                    return None;
                }

                let remainder = price % tick_size;
                if remainder.is_zero() {
                    // Already on a tick
                    Some(price)
                } else if remainder > Decimal::ZERO {
                    // Positive remainder: round down
                    Some(price - remainder)
                } else {
                    // Negative remainder: need to go down one more tick
                    Some(price - remainder - tick_size)
                }
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_tick_size_decrement() {
        let tick = TickSize::Simple(dec!(0.01));
        assert_eq!(tick.increment(dec!(100.00), -1), Some(dec!(99.99)));

        let tick_varying = TickSize::Varying {
            thresholds: vec![
                (dec!(0), dec!(0.01)),
                (dec!(100), dec!(0.05)),
                (dec!(500), dec!(0.10)),
            ],
        };

        // Test on boundary with negative increment
        assert_eq!(tick_varying.increment(dec!(100.00), -1), Some(dec!(99.99)));
        assert_eq!(tick_varying.increment(dec!(100.05), -1), Some(dec!(100)));
        assert_eq!(tick_varying.increment(dec!(500.00), -1), Some(dec!(499.95)));
    }

    #[test]
    fn test_tick_size_zero() {
        let tick = TickSize::Varying {
            thresholds: vec![
                (dec!(0), dec!(0.01)),
                (dec!(100), dec!(0.05)),
                (dec!(500), dec!(0.10)),
            ],
        };

        assert_eq!(tick.increment(dec!(100.00), 0), Some(dec!(100.00)));
        assert_eq!(tick.increment(dec!(150.00), 0), Some(dec!(150.00)));
        assert_eq!(tick.increment(dec!(50.00), 0), Some(dec!(50.00)));

        // Note: With the assumption that price is NOT below first threshold,
        // this test case is no longer valid
    }

    #[test]
    fn test_tick_size_boundary_behavior() {
        let tick = TickSize::Varying {
            thresholds: vec![
                (dec!(0), dec!(0.01)),
                (dec!(100), dec!(0.05)),
                (dec!(500), dec!(0.10)),
            ],
        };

        // On boundary going up uses new tick size
        assert_eq!(tick.increment(dec!(100.00), 1), Some(dec!(100.05)));
        assert_eq!(tick.increment(dec!(500.00), 1), Some(dec!(500.10)));

        // On boundary going down uses previous tick size
        assert_eq!(tick.increment(dec!(100.00), -1), Some(dec!(99.99)));
        assert_eq!(tick.increment(dec!(500.00), -1), Some(dec!(499.95)));
    }

    #[test]
    fn test_tick_size_multiple_crossings() {
        let tick = TickSize::Varying {
            thresholds: vec![
                (dec!(0), dec!(0.01)),
                (dec!(100), dec!(0.05)),
                (dec!(500), dec!(0.10)),
            ],
        };

        // Cross multiple thresholds going up
        let result = tick.increment(dec!(50), 12000);
        assert_eq!(result, Some(dec!(450)));

        // Test crossing multiple thresholds going down
        // With the boundary handling:
        // - 1000 ticks to get from 600 to 500
        // - Then we're at 500, one more tick at 0.05 gets us to 499.95
        // - Remaining ticks: 5100 - 1000 - 1 = 4099
        // - 4099 * 0.05 = 204.95 movement
        // - 499.95 - 204.95 = 295.00
        let result = tick.increment(dec!(600), -5100);
        assert_eq!(result, Some(dec!(295.00)));
    }

    #[test]
    fn test_round_up_simple() {
        let tick = TickSize::Simple(dec!(0.01));

        // Already on tick
        assert_eq!(tick.round_up(dec!(100.00)), Some(dec!(100.00)));

        // Between ticks - should round up
        assert_eq!(tick.round_up(dec!(100.001)), Some(dec!(100.01)));
        assert_eq!(tick.round_up(dec!(100.005)), Some(dec!(100.01)));
        assert_eq!(tick.round_up(dec!(100.009)), Some(dec!(100.01)));

        // Negative prices
        assert_eq!(tick.round_up(dec!(-99.995)), Some(dec!(-99.99)));
        assert_eq!(tick.round_up(dec!(-100.00)), Some(dec!(-100.00)));

        // Zero tick size
        let zero_tick = TickSize::Simple(dec!(0));
        assert_eq!(zero_tick.round_up(dec!(100.123)), None);
    }

    #[test]
    fn test_round_down_simple() {
        let tick = TickSize::Simple(dec!(0.01));

        // Already on tick
        assert_eq!(tick.round_down(dec!(100.00)), Some(dec!(100.00)));

        // Between ticks - should round down
        assert_eq!(tick.round_down(dec!(100.001)), Some(dec!(100.00)));
        assert_eq!(tick.round_down(dec!(100.005)), Some(dec!(100.00)));
        assert_eq!(tick.round_down(dec!(100.009)), Some(dec!(100.00)));

        // Negative prices
        assert_eq!(tick.round_down(dec!(-99.995)), Some(dec!(-100.00)));
        assert_eq!(tick.round_down(dec!(-100.00)), Some(dec!(-100.00)));

        // Zero tick size
        let zero_tick = TickSize::Simple(dec!(0));
        assert_eq!(zero_tick.round_down(dec!(100.123)), None);
    }

    #[test]
    fn test_round_up_varying() {
        let tick = TickSize::Varying {
            thresholds: vec![
                (dec!(0), dec!(0.01)),
                (dec!(100), dec!(0.05)),
                (dec!(500), dec!(0.10)),
            ],
        };

        // In first threshold range
        assert_eq!(tick.round_up(dec!(50.00)), Some(dec!(50.00)));
        assert_eq!(tick.round_up(dec!(50.001)), Some(dec!(50.01)));
        assert_eq!(tick.round_up(dec!(99.996)), Some(dec!(100.00)));

        // In second threshold range
        assert_eq!(tick.round_up(dec!(100.00)), Some(dec!(100.00)));
        assert_eq!(tick.round_up(dec!(100.01)), Some(dec!(100.05)));
        assert_eq!(tick.round_up(dec!(100.03)), Some(dec!(100.05)));
        assert_eq!(tick.round_up(dec!(100.05)), Some(dec!(100.05)));

        // In third threshold range
        assert_eq!(tick.round_up(dec!(500.00)), Some(dec!(500.00)));
        assert_eq!(tick.round_up(dec!(500.01)), Some(dec!(500.10)));
        assert_eq!(tick.round_up(dec!(500.05)), Some(dec!(500.10)));
        assert_eq!(tick.round_up(dec!(500.09)), Some(dec!(500.10)));
        assert_eq!(tick.round_up(dec!(500.10)), Some(dec!(500.10)));
    }

    #[test]
    fn test_round_down_varying() {
        let tick = TickSize::Varying {
            thresholds: vec![
                (dec!(0), dec!(0.01)),
                (dec!(100), dec!(0.05)),
                (dec!(500), dec!(0.10)),
            ],
        };

        // In first threshold range
        assert_eq!(tick.round_down(dec!(50.00)), Some(dec!(50.00)));
        assert_eq!(tick.round_down(dec!(50.001)), Some(dec!(50.00)));
        assert_eq!(tick.round_down(dec!(99.999)), Some(dec!(99.99)));

        // In second threshold range
        assert_eq!(tick.round_down(dec!(100.00)), Some(dec!(100.00)));
        assert_eq!(tick.round_down(dec!(100.01)), Some(dec!(100.00)));
        assert_eq!(tick.round_down(dec!(100.04)), Some(dec!(100.00)));
        assert_eq!(tick.round_down(dec!(100.05)), Some(dec!(100.05)));
        assert_eq!(tick.round_down(dec!(100.09)), Some(dec!(100.05)));

        // In third threshold range
        assert_eq!(tick.round_down(dec!(500.00)), Some(dec!(500.00)));
        assert_eq!(tick.round_down(dec!(500.01)), Some(dec!(500.00)));
        assert_eq!(tick.round_down(dec!(500.09)), Some(dec!(500.00)));
        assert_eq!(tick.round_down(dec!(500.10)), Some(dec!(500.10)));
        assert_eq!(tick.round_down(dec!(500.19)), Some(dec!(500.10)));
    }

    #[test]
    fn test_round_aggressive() {
        let tick = TickSize::Simple(dec!(0.01));

        // Buy direction - rounds up (more aggressive = higher price)
        assert_eq!(tick.round_aggressive(dec!(100.001), Dir::Buy), Some(dec!(100.01)));
        assert_eq!(tick.round_aggressive(dec!(100.00), Dir::Buy), Some(dec!(100.00)));

        // Sell direction - rounds down (more aggressive = lower price)
        assert_eq!(tick.round_aggressive(dec!(100.009), Dir::Sell), Some(dec!(100.00)));
        assert_eq!(tick.round_aggressive(dec!(100.00), Dir::Sell), Some(dec!(100.00)));
    }

    #[test]
    fn test_round_passive() {
        let tick = TickSize::Simple(dec!(0.01));

        // Buy direction - rounds down (more passive = lower price)
        assert_eq!(tick.round_passive(dec!(100.009), Dir::Buy), Some(dec!(100.00)));
        assert_eq!(tick.round_passive(dec!(100.00), Dir::Buy), Some(dec!(100.00)));

        // Sell direction - rounds up (more passive = higher price)
        assert_eq!(tick.round_passive(dec!(100.001), Dir::Sell), Some(dec!(100.01)));
        assert_eq!(tick.round_passive(dec!(100.00), Dir::Sell), Some(dec!(100.00)));
    }

    #[test]
    fn test_rounding_with_large_tick_sizes() {
        // Test with tick size of 0.25
        let tick = TickSize::Simple(dec!(0.25));

        assert_eq!(tick.round_up(dec!(10.00)), Some(dec!(10.00)));
        assert_eq!(tick.round_up(dec!(10.10)), Some(dec!(10.25)));
        assert_eq!(tick.round_up(dec!(10.25)), Some(dec!(10.25)));
        assert_eq!(tick.round_up(dec!(10.30)), Some(dec!(10.50)));

        assert_eq!(tick.round_down(dec!(10.00)), Some(dec!(10.00)));
        assert_eq!(tick.round_down(dec!(10.10)), Some(dec!(10.00)));
        assert_eq!(tick.round_down(dec!(10.25)), Some(dec!(10.25)));
        assert_eq!(tick.round_down(dec!(10.30)), Some(dec!(10.25)));
        assert_eq!(tick.round_down(dec!(10.50)), Some(dec!(10.50)));
        assert_eq!(tick.round_down(dec!(10.60)), Some(dec!(10.50)));
    }
}
