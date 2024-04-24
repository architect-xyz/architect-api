use crate::symbology::market::NormalizedMarketInfo;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash, clap::ValueEnum, Pack,
)]
#[serde(rename_all = "lowercase")]
pub enum ProductType {
    Spot,
    Linear,
    Inverse,
    Option,
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct BybitMarketInfo {
    pub tick_size: Decimal,
    pub step_size: Decimal,
    pub product_type: ProductType,
}

impl NormalizedMarketInfo for BybitMarketInfo {
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

impl std::fmt::Display for BybitMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}
