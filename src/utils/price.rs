//! Utility functions for working with prices.

use crate::Dir;
use rust_decimal::Decimal;

/// Return true if the first price is more aggressive than the
/// second for the specified direction.
///
/// e.g. is_more_agg_then(dec!(3), dec!(2), Dir::Sell) -> false
/// e.g. is_more_agg_then(dec!(3), dec!(2), Dir::Buy) -> true
pub fn is_more_agg_than(price: Decimal, than: Decimal, dir: Dir) -> bool {
    match dir {
        Dir::Buy => price > than,
        Dir::Sell => price < than,
    }
}

pub fn is_equal_or_more_agg_than(price: Decimal, than: Decimal, dir: Dir) -> bool {
    match dir {
        Dir::Buy => price >= than,
        Dir::Sell => price <= than,
    }
}

pub fn is_less_agg_than(price: Decimal, than: Decimal, dir: Dir) -> bool {
    match dir {
        Dir::Buy => price < than,
        Dir::Sell => price > than,
    }
}

pub fn is_equal_or_less_agg_than(price: Decimal, than: Decimal, dir: Dir) -> bool {
    match dir {
        Dir::Buy => price <= than,
        Dir::Sell => price >= than,
    }
}

pub fn more_agg_by(price: Decimal, by: Decimal, dir: Dir) -> Decimal {
    match dir {
        Dir::Buy => price + by,
        Dir::Sell => price - by,
    }
}

pub fn less_agg_by(price: Decimal, by: Decimal, dir: Dir) -> Decimal {
    match dir {
        Dir::Buy => price - by,
        Dir::Sell => price + by,
    }
}

pub fn least_agg(p1: Decimal, p2: Decimal, dir: Dir) -> Decimal {
    match dir {
        Dir::Buy => p1.min(p2),
        Dir::Sell => p1.max(p2),
    }
}

pub fn most_agg(p1: Decimal, p2: Decimal, dir: Dir) -> Decimal {
    match dir {
        Dir::Buy => p1.max(p2),
        Dir::Sell => p1.min(p2),
    }
}

/// Round the decimal to the nearest multiple of increment in the
/// direction of zero
pub fn alias_to_step_size(x: Decimal, step_size: Option<Decimal>) -> Decimal {
    if let Some(step_size) = step_size {
        if x.is_sign_positive() || x.is_zero() {
            (x / step_size).floor() * step_size
        } else {
            (x / step_size).ceil() * step_size
        }
    } else {
        x
    }
}

pub fn tick_size_round_less_agg(x: Decimal, dir: Dir, tick_size: Decimal) -> Decimal {
    match dir {
        Dir::Buy => (x / tick_size).floor() * tick_size,
        Dir::Sell => (x / tick_size).ceil() * tick_size,
    }
}
