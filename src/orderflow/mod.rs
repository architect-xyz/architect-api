//! Generic orderflow types used for oms/risk engines, translateable to/from
//! more specific cpty orderflow types

use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};

pub mod fill;
pub mod order;

pub use fill::*;
pub use order::*;

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub enum OrderflowMessage {
    Order(Order),
    Reject(Reject),
    Ack(Ack),
    Fill(Fill),
    Out(Out),
}
