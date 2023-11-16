use crate::{symbology::MarketId, Dir};
use chrono::{DateTime, Utc};
use derive::FromValue;
use enumflags2::{bitflags, BitFlags};
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

/// The ID of a fill
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pack,
    FromValue,
    Serialize,
    Deserialize,
    JsonSchema,
)]
pub struct FillId(Uuid);

impl Default for FillId {
    fn default() -> Self {
        FillId(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct Fill {
    /// Bad things about the fill that don't quite make it aberrant
    pub warnings: BitFlags<FillWarning>,
    /// Corresponding order ID, if the order originated from Architect
    // TODO: OrderId where 0 is none
    pub order_id: u64,
    pub fill_id: FillId,
    pub market: MarketId,
    pub quantity: Decimal,
    pub price: Decimal,
    pub dir: Dir,
    /// When Architect received the fill, if realtime
    pub recv_time: Option<DateTime<Utc>>,
    /// When the cpty claims the trade happened
    pub trade_time: DateTime<Utc>,
}

#[bitflags]
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, JsonSchema_repr)]
pub enum FillWarning {
    FillAfterOut,
}
