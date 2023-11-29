use crate::orderflow::*;
use derive::FromValue;
use enumflags2::{bitflags, BitFlags};
use netidx_derive::Pack;
use schemars::JsonSchema_repr;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::ops::Deref;

pub mod limits_file;

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum OmsMessage {
    Order(Order),
    Cancel(Cancel),
    Reject(OmsReject),
    Ack(Ack),
    Fill(Result<Fill, AberrantFill>),
    FillWarning(OrderId, FillId, BitFlags<FillWarning>),
    Out(Out),
    Initialize(limits_file::LimitsFile),
    RetireOutedOrders,
}

impl From<&OrderflowMessage> for OmsMessage {
    fn from(msg: &OrderflowMessage) -> Self {
        match msg {
            OrderflowMessage::Order(msg) => OmsMessage::Order(*msg),
            OrderflowMessage::Cancel(msg) => OmsMessage::Cancel(*msg),
            OrderflowMessage::Reject(msg) => OmsMessage::Reject(OmsReject {
                reject: *msg,
                reason: OmsRejectReason::Unknown,
            }),
            OrderflowMessage::Ack(msg) => OmsMessage::Ack(*msg),
            OrderflowMessage::Fill(msg) => OmsMessage::Fill(*msg),
            OrderflowMessage::Out(msg) => OmsMessage::Out(*msg),
        }
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct OmsReject {
    pub reject: Reject,
    pub reason: OmsRejectReason,
}

impl Deref for OmsReject {
    type Target = Reject;

    fn deref(&self) -> &Self::Target {
        &self.reject
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub enum OmsRejectReason {
    OmsNotInitialized,
    OrderRateLimitExceeded,
    UnknownSymbology,
    UnsupportedMarketKind,
    WouldExceedOpenQtyLimit,
    WouldExceedOpenBuyQtyLimit,
    WouldExceedOpenSellQtyLimit,
    WouldExceedPositionLimitIfFilled,
    UnknownCptyForMarket,
    #[pack(other)]
    Unknown,
}

#[bitflags]
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, JsonSchema_repr)]
pub enum FillWarning {
    FillAfterOut,
    Overfilled,
}
