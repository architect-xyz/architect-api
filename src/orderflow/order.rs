use crate::{symbology::MarketId, Dir};
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct Order {
    // TODO: OrderId where 0 is none
    pub id: u64,
    pub market: MarketId,
    pub dir: Dir,
    pub price: Decimal,
    pub quantity: Decimal,
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct Reject {
    pub order_id: u64,
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct Ack {
    pub order_id: u64,
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct Out {
    pub order_id: u64,
}
