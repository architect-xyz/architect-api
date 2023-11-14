use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;

pub mod cpty;
pub mod marketdata;
pub mod symbology;
pub mod utils;

// common, basic types which should cover a lot of use cases
pub use utils::component_id::ComponentId;
pub use utils::dir::Dir;
pub use utils::dir_pair::DirPair;
pub use utils::envelope::Envelope;
pub use utils::hcstr::Str;

// common orderflow types suitable for generic oms/risk engine work
// maybe move this to orderflow.rs

#[derive(Debug, Clone, Pack, FromValue)]
pub enum OrderflowMessage {
    Order(Order),
    Reject(Reject),
    Ack(Ack),
    Fill(Fill),
    Out(Out),
}

#[derive(Debug, Clone, Pack)]
pub struct Order {
    pub id: u64,
    pub target: String,
    pub dir: Dir,
    pub price: Decimal,
    pub quantity: Decimal,
}

#[derive(Debug, Clone, Pack)]
pub struct Fill {
    pub order_id: u64,
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub quantity: Decimal,
}

#[derive(Debug, Clone, Pack)]
pub struct Reject {
    pub id: u64,
}

#[derive(Debug, Clone, Pack)]
pub struct Ack {
    pub id: u64,
}

#[derive(Debug, Clone, Pack)]
pub struct Out {
    pub id: u64,
}
