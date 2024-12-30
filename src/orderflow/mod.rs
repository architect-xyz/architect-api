//! Generic orderflow types used for oms/risk gateways, translateable to/from
//! more specific cpty orderflow types

#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};

pub mod fill;
pub mod order;
pub mod order_id;

pub use fill::*;
pub use order::*;
pub use order_id::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub enum OrderflowMessage {
    // OrderRequest(OrderRequest),
    Order(Order),
    Cancel(Cancel),
    CancelAll(CancelAll),
    Reject(Reject),
    Ack(Ack),
    Fill(Result<Fill, AberrantFill>),
    Out(Out),
}
