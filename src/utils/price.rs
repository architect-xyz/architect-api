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

pub fn is_less_agg_than(price: Decimal, than: Decimal, dir: Dir) -> bool {
    match dir {
        Dir::Buy => price < than,
        Dir::Sell => price > than,
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

pub fn min_agg(p1: Decimal, p2: Decimal, dir: Dir) -> Decimal {
    match dir {
        Dir::Buy => p1.min(p2),
        Dir::Sell => p1.max(p2)
    }
}


pub fn max_agg(p1: Decimal, p2: Decimal, dir: Dir) -> Decimal {
    match dir {
        Dir::Buy => p1.max(p2),
        Dir::Sell => p1.min(p2)
    }
}

pub fn incr_agg(price: Decimal, by: Decimal, dir: Dir) -> Decimal {
    match dir {
        Dir::Buy => price + by,
        Dir::Sell => price - by,
    }
}

pub fn decr_agg(price: Decimal, by: Decimal, dir: Dir) -> Decimal {
    match dir {
        Dir::Buy => price - by,
        Dir::Sell => price + by,
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
