use crate::symbology::market::NormalizedMarketInfo;
use chrono::{DateTime, Utc};
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct CqgMarketInfo {
    pub tick_size: Decimal,
    pub tick_value: Decimal,
    pub step_size: Decimal,
    pub contract_size: Option<String>,
    pub currency: String,
    pub is_active: bool,
    pub description: String,
    pub last_trading_date: DateTime<Utc>,
    pub maturity_month_year: Option<String>,
    pub cfi_code: String,
    pub country_code: String,
    pub settlement_method: Option<u32>,
    pub exercise_style: Option<u32>,
}

impl NormalizedMarketInfo for CqgMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    fn step_size(&self) -> Decimal {
        self.step_size
    }

    fn is_delisted(&self) -> bool {
        !self.is_active
    }
}

impl std::fmt::Display for CqgMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}
