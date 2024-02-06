use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};

use crate::symbology::market::NormalizedMarketInfo;

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct DYDXMarketInfo {
    pub step_size: Decimal,
    pub tick_size: Decimal,
}

impl NormalizedMarketInfo for DYDXMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    fn step_size(&self) -> Decimal {
        self.step_size
    }

    fn is_delisted(&self) -> bool {
        false
    }
}

impl std::fmt::Display for DYDXMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}
