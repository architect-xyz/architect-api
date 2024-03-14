use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::symbology::market::NormalizedMarketInfo;

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct BinanceMarketInfo {
    pub tick_size: Decimal,
    pub step_size: Decimal,
    pub is_delisted: bool,
}

impl NormalizedMarketInfo for BinanceMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    fn step_size(&self) -> Decimal {
        self.step_size
    }

    fn is_delisted(&self) -> bool {
        self.is_delisted
    }
}

impl std::fmt::Display for BinanceMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}
