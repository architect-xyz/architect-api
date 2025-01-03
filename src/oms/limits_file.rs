use crate::symbology::ProductId;
use derive::FromValue;
use fxhash::FxHashMap;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct LimitsFile {
    #[serde(default)]
    pub max_open_qty: FxHashMap<ProductId, Decimal>,
    #[serde(default)]
    pub max_open_buy_qty: FxHashMap<ProductId, Decimal>,
    #[serde(default)]
    pub max_open_sell_qty: FxHashMap<ProductId, Decimal>,
    #[serde(default)]
    pub max_position: FxHashMap<ProductId, Decimal>,
    #[serde(default)]
    pub default_max_open_qty: Option<Decimal>,
    #[serde(default)]
    pub default_max_open_buy_qty: Option<Decimal>,
    #[serde(default)]
    pub default_max_open_sell_qty: Option<Decimal>,
    #[serde(default)]
    pub default_max_position: Option<Decimal>,
}
