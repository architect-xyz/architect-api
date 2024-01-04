//! Generic orderflow types used for oms/risk gateways, translateable to/from
//! more specific cpty orderflow types

use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};

pub mod algo;
pub mod channel_authority;
pub mod fill;
pub mod order;
pub mod order_id;

pub use channel_authority::*;
pub use fill::*;
pub use order::*;
pub use order_id::*;

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub enum OrderflowMessage {
    Order(Order),
    Cancel(Cancel),
    Reject(Reject),
    Ack(Ack),
    Fill(Result<Fill, AberrantFill>),
    Out(Out),
}
