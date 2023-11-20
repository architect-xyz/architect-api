use crate::{symbology::MarketId, Dir, OrderId, Str};
use enumflags2::{bitflags, BitFlags};
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::JsonSchema_repr;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct Order {
    pub id: OrderId,
    pub market: MarketId,
    pub dir: Dir,
    pub price: Decimal,
    pub quantity: Decimal,
    pub account: Option<Str>,
}

/// The state of an order
#[bitflags]
#[repr(u8)]
#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, Pack, Serialize, Deserialize, JsonSchema_repr,
)]
pub enum OrderStateFlags {
    Open,
    Rejected,
    Acked,
    Filled,
    Canceling,
    Canceled,
    Out,
    Stale, // we were expecting some state change but it was never confirmed
}

pub type OrderState = BitFlags<OrderStateFlags>;

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct Cancel {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct Reject {
    pub order_id: OrderId,
}

impl Reject {
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct Ack {
    pub order_id: OrderId,
}

impl Ack {
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct Out {
    pub order_id: OrderId,
}

impl Out {
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
    }
}
