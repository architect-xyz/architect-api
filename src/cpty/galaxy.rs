use crate::symbology::market::NormalizedMarketInfo;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct GalaxyMarketInfo {
    pub tick_size: Decimal,
}

impl NormalizedMarketInfo for GalaxyMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    // CR alee: copied from core, seems dubious...
    fn step_size(&self) -> Decimal {
        self.tick_size
    }

    fn is_delisted(&self) -> bool {
        false
    }
}

impl std::fmt::Display for GalaxyMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

impl Default for GalaxyMarketInfo {
    fn default() -> GalaxyMarketInfo {
        GalaxyMarketInfo {
            // Galaxy support says tick size is 0.00000001 for all pairs
            tick_size: dec!(0.00000001),
        }
    }
}
