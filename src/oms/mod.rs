#![cfg(feature = "netidx")]

use crate::{orderflow::*, utils::messaging::MaybeRequest, ComponentId, HalfOpenRange};
use arcstr::{literal, ArcStr};
use chrono::{DateTime, Utc};
use derive::FromValue;
use enumflags2::{bitflags, BitFlags};
use netidx::pool::Pooled;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::JsonSchema_repr;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
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
    CancelAll(CancelAll),
    Reject(OmsReject),
    Ack(Ack),
    Fill(Result<Fill, AberrantFill>),
    FillWarning(OrderId, FillId, BitFlags<FillWarning>),
    Out(Out),
    Initialize(limits_file::LimitsFile),
    RetireOutedOrdersAndUnknownFills,
    // some of these are better queried via a follower Oms or StatsDb;
    // for latency sensitive applications, responding to these requests
    // blocks the Oms for too long; but the option is available
    GetOpenOrders(Uuid),
    GetOpenOrdersResponse(Uuid, Vec<OrderLog>),
    // retrieve outed orders that the oms knows about; outed orders are retired
    // from the oms after the configured interval
    GetOutedOrders(Uuid, HalfOpenRange<DateTime<Utc>>),
    GetOutedOrdersResponse(Uuid, Arc<Vec<OrderLog>>),
    GetOrder(Uuid, OrderId),
    GetOrderResponse(Uuid, Option<OrderLog>),
    GetFills(Uuid, OrderId),
    GetFillsResponse(Uuid, Result<GetFillsResponse, GetFillsError>),
    ReconcileOrders(Arc<Vec<ReconcileOrder>>),
}

#[derive(Debug, Copy, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct ReconcileOrder {
    pub order: Order,
    pub order_state: OrderState,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct OmsReject {
    pub order_id: OrderId,
    pub reason: OmsRejectReason,
}

impl OmsReject {
    pub fn new(order_id: OrderId, reason: OmsRejectReason) -> Self {
        Self { order_id, reason }
    }
}

impl Into<Reject> for OmsReject {
    fn into(self) -> Reject {
        Reject { order_id: self.order_id, reason: self.reason.into() }
    }
}

impl Into<OmsReject> for &Reject {
    fn into(self) -> OmsReject {
        OmsReject { order_id: self.order_id, reason: (&self.reason).into() }
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub enum OmsRejectReason {
    NotInitialized,
    RateLimitExceeded,
    UnknownMarket,
    InvalidMarketKind,
    UnknownCpty,
    WouldExceedOpenBuyQty,
    WouldExceedOpenSellQty,
    WouldExceedOpenQty,
    WouldExceedPosLimit,
    Literal(ArcStr),
    NoAccount,
    NotAuthorized,
    NotAuthorizedForAccount,
    #[pack(other)]
    #[serde(other)]
    Unknown,
}

const RATE_LIMIT_EXCEEDED: ArcStr = literal!("order rate limit exceeded");
const INVALID_MARKET_KIND: ArcStr = literal!("invalid market kind");
const WOULD_EXCEED_OPEN_BUY_QTY: ArcStr = literal!("would exceed open buy qty limit");
const WOULD_EXCEED_OPEN_SELL_QTY: ArcStr = literal!("would exceed open sell qty limit");
const WOULD_EXCEED_OPEN_QTY: ArcStr = literal!("would exceed open qty limit");
const WOULD_EXCEED_POS_LIMIT: ArcStr = literal!("would exceed position limit if filled");

impl Into<RejectReason> for OmsRejectReason {
    fn into(self) -> RejectReason {
        use OmsRejectReason::*;
        use RejectReason as R;
        match self {
            NotInitialized => R::ComponentNotInitialized,
            RateLimitExceeded => R::Literal(RATE_LIMIT_EXCEEDED),
            UnknownMarket => R::UnknownMarket,
            InvalidMarketKind => R::Literal(INVALID_MARKET_KIND),
            UnknownCpty => R::UnknownCpty,
            WouldExceedOpenBuyQty => R::Literal(WOULD_EXCEED_OPEN_BUY_QTY),
            WouldExceedOpenSellQty => R::Literal(WOULD_EXCEED_OPEN_SELL_QTY),
            WouldExceedOpenQty => R::Literal(WOULD_EXCEED_OPEN_QTY),
            WouldExceedPosLimit => R::Literal(WOULD_EXCEED_POS_LIMIT),
            Literal(s) => R::Literal(s),
            NoAccount => R::NoAccount,
            NotAuthorized => R::NotAuthorized,
            NotAuthorizedForAccount => R::NotAuthorizedForAccount,
            Unknown => R::Unknown,
        }
    }
}

impl Into<OmsRejectReason> for &RejectReason {
    fn into(self) -> OmsRejectReason {
        use OmsRejectReason::*;
        use RejectReason as R;
        match self {
            R::ComponentNotInitialized => NotInitialized,
            R::UnknownMarket => UnknownMarket,
            R::UnknownCpty => UnknownCpty,
            R::NoAccount => NoAccount,
            R::NotAuthorized => NotAuthorized,
            R::NotAuthorizedForAccount => NotAuthorizedForAccount,
            R::Literal(s) if s == &RATE_LIMIT_EXCEEDED => RateLimitExceeded,
            R::Literal(s) if s == &INVALID_MARKET_KIND => InvalidMarketKind,
            R::Literal(s) if s == &WOULD_EXCEED_OPEN_BUY_QTY => WouldExceedOpenBuyQty,
            R::Literal(s) if s == &WOULD_EXCEED_OPEN_SELL_QTY => WouldExceedOpenSellQty,
            R::Literal(s) if s == &WOULD_EXCEED_OPEN_QTY => WouldExceedOpenQty,
            R::Literal(s) if s == &WOULD_EXCEED_POS_LIMIT => WouldExceedPosLimit,
            R::Literal(s) => Literal(s.clone()),
            _ => Unknown,
        }
    }
}

impl MaybeRequest for OmsMessage {
    fn request_id(&self) -> Option<Uuid> {
        match self {
            OmsMessage::GetOpenOrders(id)
            | OmsMessage::GetOutedOrders(id, ..)
            | OmsMessage::GetOrder(id, ..)
            | OmsMessage::GetFills(id, ..) => Some(*id),
            _ => None,
        }
    }

    fn response_id(&self) -> Option<Uuid> {
        match self {
            OmsMessage::GetOpenOrdersResponse(id, ..)
            | OmsMessage::GetOutedOrdersResponse(id, ..)
            | OmsMessage::GetOrderResponse(id, ..)
            | OmsMessage::GetFillsResponse(id, ..) => Some(*id),
            _ => None,
        }
    }
}

impl From<&OrderflowMessage> for OmsMessage {
    fn from(msg: &OrderflowMessage) -> Self {
        match msg {
            OrderflowMessage::Order(msg) => OmsMessage::Order(*msg),
            OrderflowMessage::Cancel(msg) => OmsMessage::Cancel(*msg),
            OrderflowMessage::CancelAll(msg) => OmsMessage::CancelAll(msg.clone()),
            OrderflowMessage::Reject(msg) => OmsMessage::Reject(msg.into()),
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
            OmsMessage::CancelAll(msg) => Ok(OrderflowMessage::CancelAll(msg.clone())),
            OmsMessage::Reject(msg) => Ok(OrderflowMessage::Reject(msg.clone().into())),
            OmsMessage::Ack(msg) => Ok(OrderflowMessage::Ack(*msg)),
            OmsMessage::Fill(msg) => Ok(OrderflowMessage::Fill(*msg)),
            OmsMessage::Out(msg) => Ok(OrderflowMessage::Out(*msg)),
            OmsMessage::OrderUpdate(..)
            | OmsMessage::Initialize(..)
            | OmsMessage::RetireOutedOrdersAndUnknownFills
            | OmsMessage::ReconcileOrders(..)
            | OmsMessage::FillWarning(..)
            | OmsMessage::GetOpenOrders(_)
            | OmsMessage::GetOpenOrdersResponse(..)
            | OmsMessage::GetOutedOrders(..)
            | OmsMessage::GetOutedOrdersResponse(..)
            | OmsMessage::GetFills(..)
            | OmsMessage::GetFillsResponse(..)
            | OmsMessage::GetOrder(..)
            | OmsMessage::GetOrderResponse(..) => Err(()),
        }
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct OrderLog {
    pub timestamp: DateTime<Utc>,
    pub order: Order,
    pub order_state: OrderState,
    pub filled_qty: Decimal,
    pub avg_fill_price: Option<Decimal>,
    pub reject_reason: Option<OmsRejectReason>,
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

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub enum GetOrderError {
    OrderNotFound,
}

impl std::fmt::Display for GetOrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetOrderError::OrderNotFound => write!(f, "order not found"),
        }
    }
}

impl std::error::Error for GetOrderError {}
