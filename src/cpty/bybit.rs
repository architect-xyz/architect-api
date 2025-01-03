use crate::symbology::market::NormalizedMarketInfo;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[serde(rename_all = "lowercase")]
pub enum ProductType {
    Spot,
    Linear,
    Inverse,
    Option,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
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

impl std::fmt::Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let product_type = match self {
            ProductType::Spot => "spot",
            ProductType::Linear => "linear",
            ProductType::Inverse => "inverse",
            ProductType::Option => "option",
        };
        write!(f, "{}", product_type)?;
        Ok(())
    }
}
