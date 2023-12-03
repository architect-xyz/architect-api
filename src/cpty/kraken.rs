use crate::symbology::market::NormalizedMarketInfo;
use netidx_derive::Pack;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub enum Status {
    #[serde(alias = "online")]
    Online,
    #[serde(alias = "cancel_only")]
    CancelOnly,
    #[serde(alias = "post_only")]
    PostOnly,
    #[serde(alias = "limit_only")]
    LimitOnly,
    #[serde(alias = "reduce_only")]
    ReduceOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct KrakenMarketInfo {
    pub altname: String,
    pub wsname: String,
    pub aclass_base: String,
    pub base: String,
    pub aclass_quote: String,
    pub quote: String,
    pub pair_decimals: u32,
    pub cost_decimals: u32,
    pub lot_decimals: u32,
    pub lot_multiplier: u32,
    pub margin_call: u32,
    pub margin_stop: u32,
    pub fee_volume_currency: String,
    pub ordermin: Decimal,
    pub costmin: Decimal,
    pub tick_size: Decimal,
    pub status: Status,
    pub long_position_limit: Option<u32>,
    pub short_position_limit: Option<u32>,
}

impl NormalizedMarketInfo for KrakenMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    fn step_size(&self) -> Decimal {
        Decimal::from_f64(10f64.powi(-(self.lot_decimals as i32)))
            .expect(&format!("could not compute step_size: {:?}", self))
    }

    fn is_delisted(&self) -> bool {
        false
    }
}

impl std::fmt::Display for KrakenMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}
