use crate::symbology::market::NormalizedMarketInfo;
use chrono::{DateTime, Utc};
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct DatabentoMarketInfo {
    pub tick_size: Decimal,
    pub step_size: Decimal,
    pub is_delisted: bool,
    pub underlying: String,
    pub underlying_root: String,
    pub expiration: DateTime<Utc>,
    pub raw_symbol: String,
    pub root_symbol: String,
}

impl NormalizedMarketInfo for DatabentoMarketInfo {
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

impl std::fmt::Display for DatabentoMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}
