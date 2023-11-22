use crate::symbology::ProductId;
use derive::FromValue;
use fxhash::FxHashMap;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct LimitsFile {
    pub max_open_qty: FxHashMap<ProductId, Decimal>,
    pub max_open_buy_qty: FxHashMap<ProductId, Decimal>,
    pub max_open_sell_qty: FxHashMap<ProductId, Decimal>,
    pub max_position: FxHashMap<ProductId, Decimal>,
    pub default_max_open_qty: Option<Decimal>,
    pub default_max_open_buy_qty: Option<Decimal>,
    pub default_max_open_sell_qty: Option<Decimal>,
    pub default_max_position: Option<Decimal>,
}
