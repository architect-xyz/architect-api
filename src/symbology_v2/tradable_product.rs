//! A tradable product describes a trading pair, e.g. BTC/USD.

use super::Product;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
#[serde(transparent)]
pub struct TradableProduct(String);

impl TradableProduct {
    pub fn new(base: &Product, quote: &Product) -> Self {
        Self(format!("{base}/{quote}"))
    }

    // TODO: base() and quote() using substr
}
