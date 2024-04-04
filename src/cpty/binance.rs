use crate::{
    symbology::market::{MinOrderQuantityUnit, NormalizedMarketInfo},
    Amount,
};
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct BinanceMarketInfo {
    pub tick_size: Decimal,
    pub step_size: Decimal,
    pub is_delisted: bool,
    #[pack(default)]
    pub min_order_quantity: Amount<Decimal, MinOrderQuantityUnit>,
}

impl NormalizedMarketInfo for BinanceMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    fn step_size(&self) -> Decimal {
        self.step_size
    }

    fn min_order_quantity(&self) -> Amount<Decimal, MinOrderQuantityUnit> {
        return self.min_order_quantity.clone();
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
