use crate::{symbology::MarketId, Dir, OrderId};
use chrono::{DateTime, Utc};
use derive::FromValue;
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
    pub kind: FillKind,
    pub fill_id: FillId,
    /// Corresponding order ID, if the order originated from Architect
    pub order_id: OrderId,
    pub market: MarketId,
    pub quantity: Decimal,
    pub price: Decimal,
    pub dir: Dir,
    /// When Architect received the fill, if realtime
    pub recv_time: Option<DateTime<Utc>>,
    /// When the cpty claims the trade happened
    pub trade_time: DateTime<Utc>,
}

impl Fill {
    pub fn into_aberrant(self) -> AberrantFill {
        AberrantFill {
            kind: Some(self.kind),
            fill_id: self.fill_id,
            order_id: Some(self.order_id),
            market: Some(self.market),
            quantity: Some(self.quantity),
            price: Some(self.price),
            dir: Some(self.dir),
            recv_time: self.recv_time,
            trade_time: Some(self.trade_time),
        }
    }
}

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, Pack, Serialize, Deserialize, JsonSchema_repr,
)]
#[repr(u8)]
pub enum FillKind {
    Normal,
    Reversal,
    Correction,
}

/// Fills which we received but couldn't parse fully, return details
/// best effort
#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct AberrantFill {
    pub kind: Option<FillKind>,
    pub fill_id: FillId,
    pub order_id: Option<OrderId>,
    pub market: Option<MarketId>,
    pub quantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub dir: Option<Dir>,
    pub recv_time: Option<DateTime<Utc>>,
    pub trade_time: Option<DateTime<Utc>>,
}
