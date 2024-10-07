#![cfg(feature = "netidx")]

use crate::{
    folio::FolioMessage,
    orderflow::{
        self, AberrantFill, Ack, Cancel, Fill, Order, OrderSource, OrderStateFlags,
        OrderType, OrderflowMessage, Out, Reject, TimeInForce,
    },
    symbology::{market::NormalizedMarketInfo, CptyId, MarketId},
    Address, Dir, HalfOpenRange, OrderId, Str, UserId,
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct DeribitMarketInfo {
    pub tick_size: Decimal,
    pub min_trade_amount: Decimal,
    pub is_active: bool,
}

impl NormalizedMarketInfo for DeribitMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    fn step_size(&self) -> Decimal {
        self.min_trade_amount
    }

    fn is_delisted(&self) -> bool {
        !self.is_active
    }
}

impl std::fmt::Display for DeribitMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum DeribitMessage {
    Order(DeribitOrder),
    Cancel(DeribitCancel),
    CancelAll(CancelAll),
    Reject(Reject),
    Ack(Ack),
    Fill(DeribitFill),
    Out(Out),
    Folio(FolioMessage),
    ExchangeOrderUpdate(DeribitExternalOrderAck),
    ExchangeAck(OrderId, DeribitExchangeId),
    ExchangeFill(DeribitExternalFill),
    ExchangeOrderOut(u64),
    BalanceRequest(DeribitBalanceQuery),
    TradeQueryRequest(DeribitTradeQuery),
    SupportedCurrencies(Vec<String>),
    FolioMessagesAllowed(bool),
}

impl TryInto<DeribitMessage> for &OrderflowMessage {
    type Error = ();

    fn try_into(self) -> Result<DeribitMessage, ()> {
        match self {
            OrderflowMessage::Order(o) => {
                Ok(DeribitMessage::Order(DeribitOrder { order: *o }))
            }
            OrderflowMessage::Cancel(c) => Ok(DeribitMessage::Cancel(DeribitCancel {
                cancel: *c,
                exchange_id: None,
            })),
            OrderflowMessage::Reject(r) => Ok(DeribitMessage::Reject(r.clone())),
            OrderflowMessage::Ack(a) => Ok(DeribitMessage::Ack(*a)),
            OrderflowMessage::Fill(_) => Err(()),
            OrderflowMessage::Out(o) => Ok(DeribitMessage::Out(*o)),
            OrderflowMessage::CancelAll(_) => Ok(DeribitMessage::CancelAll(CancelAll {})),
        }
    }
}

impl TryInto<FolioMessage> for &DeribitMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, ()> {
        match self {
            DeribitMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<&FolioMessage> for DeribitMessage {
    type Error = ();

    fn try_from(f: &FolioMessage) -> Result<Self, ()> {
        Ok(Self::Folio(f.clone()))
    }
}

impl TryInto<OrderflowMessage> for &DeribitMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            DeribitMessage::Order(o) => Ok(OrderflowMessage::Order(**o)),
            DeribitMessage::Cancel(c) => Ok(OrderflowMessage::Cancel(**c)),
            DeribitMessage::Reject(r) => Ok(OrderflowMessage::Reject(r.clone())),
            DeribitMessage::Ack(a) => Ok(OrderflowMessage::Ack(*a)),
            DeribitMessage::Fill(f) => Ok(OrderflowMessage::Fill(**f)),
            DeribitMessage::CancelAll(_) => {
                Ok(OrderflowMessage::CancelAll(orderflow::CancelAll::default()))
            }
            DeribitMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            DeribitMessage::Folio(..)
            | DeribitMessage::SupportedCurrencies(..)
            | DeribitMessage::FolioMessagesAllowed(..)
            | DeribitMessage::BalanceRequest(..)
            | DeribitMessage::TradeQueryRequest(..)
            | DeribitMessage::ExchangeOrderUpdate(..)
            | DeribitMessage::ExchangeAck(..)
            | DeribitMessage::ExchangeFill(..)
            | DeribitMessage::ExchangeOrderOut(..) => Err(()),
        }
    }
}

pub type DeribitExchangeId = String;
pub type DeribitUserRef = u64;

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct CancelAll {}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct DeribitOrder {
    #[serde(flatten)]
    pub order: Order,
}

impl From<Order> for DeribitOrder {
    fn from(order: Order) -> Self {
        Self { order }
    }
}

impl Deref for DeribitOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

impl DerefMut for DeribitOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.order
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct DeribitCancel {
    #[serde(flatten)]
    pub cancel: Cancel,
    pub exchange_id: Option<Str>,
}

impl From<Cancel> for DeribitCancel {
    fn from(cancel: Cancel) -> Self {
        Self { cancel, exchange_id: None }
    }
}

impl Deref for DeribitCancel {
    type Target = Cancel;

    fn deref(&self) -> &Self::Target {
        &self.cancel
    }
}

impl DerefMut for DeribitCancel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cancel
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct DeribitFill {
    #[serde(flatten)]
    pub fill: Result<Fill, AberrantFill>,
    pub exchange_trade_id: DeribitExchangeId,
    pub exchange_order_id: DeribitExchangeId,
}

impl Deref for DeribitFill {
    type Target = Result<Fill, AberrantFill>;

    fn deref(&self) -> &Self::Target {
        &self.fill
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct DeribitExternalOrderAck {
    pub exchange_symbol: String,
    pub exchange_order_id: DeribitExchangeId,
    pub user_reference_id: DeribitUserRef,
    pub quantity: Decimal,
    pub trigger_price: Option<Decimal>,
    pub dir: Dir,
    pub expiration: Option<DateTime<Utc>>,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub order_state: OrderStateFlags,
}

impl DeribitExternalOrderAck {
    pub fn to_deribit_order(
        &mut self,
        oid: OrderId,
        mid: MarketId,
        trader: Option<UserId>,
    ) -> Result<DeribitOrder> {
        let order = Order {
            id: oid,
            market: mid,
            dir: self.dir,
            quantity: self.quantity,
            trader,
            account: None,
            order_type: self.order_type,
            time_in_force: self.time_in_force,
            quote_id: None,
            source: OrderSource::External,
            parent_order: None,
        };
        Ok(DeribitOrder { order })
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct DeribitExternalFill {
    pub exchange_order_id: DeribitExchangeId,
    pub exchange_trade_id: DeribitExchangeId,
    pub user_reference_id: Option<DeribitUserRef>,
    pub time: DateTime<Utc>,
    pub quantity: Decimal,
    pub price: Decimal,
    pub dir: Dir,
    pub status: FillOrderStatus,
    pub unique_fill_id: String,
    pub fee: Option<Decimal>,
    pub fee_currency: Option<String>,
    pub exchange_symbol: Option<String>,
    pub is_maker: Option<bool>,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize, PartialEq, Eq)]
pub enum FillOrderStatus {
    #[serde(alias = "open")]
    Open,
    #[serde(alias = "filled")]
    Filled,
    #[serde(alias = "rejected")]
    Rejected,
    #[serde(alias = "cancelled")]
    Cancelled,
    #[serde(alias = "untriggered")]
    Untriggered,
    #[serde(alias = "archive")]
    Archive,
    Unknown,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct DeribitBalanceQuery {
    pub cpty_id: CptyId,
    pub reply_to: Address,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct DeribitTradeQuery {
    pub cpty_id: CptyId,
    pub reply_to: Address,
    pub range: HalfOpenRange<Option<DateTime<Utc>>>,
}
