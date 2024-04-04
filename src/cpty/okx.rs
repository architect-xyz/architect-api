use crate::{
    symbology::market::{MinOrderQuantityUnit, NormalizedMarketInfo},
    Amount,
};
use compact_str::CompactString;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};

const LIVE: CompactString = CompactString::new_inline("live");

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct OkxMarketInfo {
    pub tick_sz: Decimal,
    pub min_sz: Decimal,
    pub state: Option<CompactString>,
}

impl NormalizedMarketInfo for OkxMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_sz
    }

    fn step_size(&self) -> Decimal {
        self.min_sz
    }

    fn min_order_quantity(&self) -> Amount<Decimal, MinOrderQuantityUnit> {
        return Amount::new(self.min_sz, MinOrderQuantityUnit::Base);
    }

    fn is_delisted(&self) -> bool {
        !(self.state == Some(LIVE))
    }
}

impl std::fmt::Display for OkxMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}
