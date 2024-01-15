use crate::{orderflow::*, utils::messaging::MaybeRequest, ComponentId};
use chrono::{DateTime, Utc};
use derive::FromValue;
use enumflags2::{bitflags, BitFlags};
use netidx::pool::Pooled;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::JsonSchema_repr;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::ops::Deref;
use uuid::Uuid;

pub mod limits_file;

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct ForwardOrderflow {
    pub to: ComponentId,
    pub rule: ForwardOrderflowRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, Serialize, Deserialize)]
pub enum ForwardOrderflowRule {
    Always,
    OnlyIfNoCptyMatch,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum OmsMessage {
    Order(Order),
    OrderUpdate(OmsOrderUpdate),
    Cancel(Cancel),
    Reject(OmsReject),
    Ack(Ack),
    Fill(Result<Fill, AberrantFill>),
    FillWarning(OrderId, FillId, BitFlags<FillWarning>),
    Out(Out),
    Initialize(limits_file::LimitsFile),
    RetireOutedOrders,
    // some of these are better queried via a follower Oms or StatsDb;
    // for latency sensitive applications, responding to these requests
    // blocks the Oms for too long; but the option is available
    GetOpenOrders(Uuid),
    GetOpenOrdersResponse(Uuid, Vec<OpenOrder>),
    GetFills(Uuid, OrderId),
    GetFillsResponse(Uuid, Result<GetFillsResponse, GetFillsError>),
}

impl MaybeRequest for OmsMessage {
    fn request_id(&self) -> Option<Uuid> {
        match self {
            OmsMessage::GetFills(id, ..) => Some(*id),
            _ => None,
        }
    }

    fn response_id(&self) -> Option<Uuid> {
        match self {
            OmsMessage::GetFillsResponse(id, ..) => Some(*id),
            _ => None,
        }
    }
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

impl TryInto<OrderflowMessage> for &OmsMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            OmsMessage::Order(msg) => Ok(OrderflowMessage::Order(*msg)),
            OmsMessage::Cancel(msg) => Ok(OrderflowMessage::Cancel(*msg)),
            OmsMessage::Reject(msg) => Ok(OrderflowMessage::Reject(msg.reject)),
            OmsMessage::Ack(msg) => Ok(OrderflowMessage::Ack(*msg)),
            OmsMessage::Fill(msg) => Ok(OrderflowMessage::Fill(*msg)),
            OmsMessage::Out(msg) => Ok(OrderflowMessage::Out(*msg)),
            OmsMessage::OrderUpdate(..)
            | OmsMessage::Initialize(..)
            | OmsMessage::RetireOutedOrders
            | OmsMessage::FillWarning(..)
            | OmsMessage::GetOpenOrders(_)
            | OmsMessage::GetOpenOrdersResponse(..)
            | OmsMessage::GetFills(..)
            | OmsMessage::GetFillsResponse(..) => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct OpenOrder {
    pub timestamp: DateTime<Utc>,
    pub order: Order,
    pub filled_qty: Decimal,
    pub avg_fill_price: Option<Decimal>,
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct OmsOrderUpdate {
    pub order_id: OrderId,
    pub state: OrderState,
    pub filled_qty: Decimal,
    pub avg_fill_price: Option<Decimal>,
}

#[cfg(feature = "juniper")]
#[juniper::graphql_object]
impl OmsOrderUpdate {
    pub fn order_id(&self) -> OrderId {
        self.order_id
    }

    pub fn state(&self) -> Vec<OrderStateFlags> {
        self.state.iter().collect()
    }

    pub fn filled_qty(&self) -> Decimal {
        self.filled_qty
    }

    pub fn avg_fill_price(&self) -> Option<Decimal> {
        self.avg_fill_price
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

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct GetFillsResponse {
    pub fills: Option<Pooled<Vec<Fill>>>,
    pub aberrant_fills: Option<Pooled<Vec<AberrantFill>>>,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub enum GetFillsError {
    OrderNotFound,
}

impl std::fmt::Display for GetFillsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetFillsError::OrderNotFound => write!(f, "order not found"),
        }
    }
}

impl std::error::Error for GetFillsError {}
