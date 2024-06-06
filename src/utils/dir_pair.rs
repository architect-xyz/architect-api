use crate::Dir;
#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

/// A dirpair is a structure for holding things that depend on trading direction.
///
/// For example one might hold one's position in a particular coin in a `DirPair<Decimal>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub struct DirPair<T: 'static> {
    pub buy: T,
    pub sell: T,
}

impl<T: Default + 'static> Default for DirPair<T> {
    fn default() -> Self {
        Self { buy: T::default(), sell: T::default() }
    }
}

impl<T: 'static> DirPair<T> {
    /// get a shared reference to the field specified by dir
    pub fn get(&self, dir: Dir) -> &T {
        match dir {
            Dir::Buy => &self.buy,
            Dir::Sell => &self.sell,
        }
    }

    /// get a mutable reference to field side specified by dir
    pub fn get_mut(&mut self, dir: Dir) -> &mut T {
        match dir {
            Dir::Buy => &mut self.buy,
            Dir::Sell => &mut self.sell,
        }
    }
}

impl DirPair<Decimal> {
    /// true if both sides are 0
    pub fn is_empty(&self) -> bool {
        self.buy == dec!(0) && self.sell == dec!(0)
    }

    /// net the buy and the sell side (buy - sell)
    pub fn net(&self) -> Decimal {
        self.buy - self.sell
    }
}
