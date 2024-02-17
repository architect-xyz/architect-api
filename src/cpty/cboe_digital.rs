use crate::symbology::market::NormalizedMarketInfo;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct CboeDigitalMarketInfo {
    pub tick_size: Decimal,
    pub step_size: Decimal,
}

impl NormalizedMarketInfo for CboeDigitalMarketInfo {
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

impl std::fmt::Display for CboeDigitalMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}
