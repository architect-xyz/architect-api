use crate::{
    orderflow::{Ack, Fill, Order, OrderflowMessage, Out, Reject},
    symbology::{market::NormalizedMarketInfo, MarketId},
};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct CoinbaseMarketInfo {
    pub min_market_funds: Decimal,
    pub status_message: Option<String>,
    pub base_increment: Decimal,
    pub quote_increment: Decimal,
    pub trading_disabled: bool,
    pub cancel_only: bool,
    pub post_only: bool,
    pub limit_only: bool,
    pub fx_stablecoin: bool,
    pub auction_mode: bool,
}

impl NormalizedMarketInfo for CoinbaseMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.quote_increment
    }

    fn step_size(&self) -> Decimal {
        self.base_increment
    }

    fn is_delisted(&self) -> bool {
        // CR alee: not really true?
        self.trading_disabled
    }
}

#[derive(Debug, Clone, Pack, FromValue)]
pub enum CoinbaseMessage {
    CoinbaseOrder(CoinbaseOrder),
    Order(Order),
    Reject(Reject),
    Ack(Ack),
    CoinbaseFill(CoinbaseFill),
    Fill(Fill),
    Out(Out),
    ExchangeOrderUpdate(u64),
    ExchangeFills(Vec<CoinbaseFill>),
    ExchangeExternalOrderUpdate(MarketId, Uuid),
    ExchangeExternalOrderNew(Uuid, CoinbaseOrder),
    ExchangeExternalOrderOut(Uuid),
}

impl TryFrom<OrderflowMessage> for CoinbaseMessage {
    type Error = ();

    fn try_from(msg: OrderflowMessage) -> Result<Self, Self::Error> {
        match msg {
            OrderflowMessage::Order(o) => Ok(Self::Order(o)),
            OrderflowMessage::Reject(r) => Ok(Self::Reject(r)),
            OrderflowMessage::Ack(a) => Ok(Self::Ack(a)),
            OrderflowMessage::Fill(f) => Ok(Self::Fill(f)),
            OrderflowMessage::Out(o) => Ok(Self::Out(o)),
        }
    }
}

impl TryInto<OrderflowMessage> for CoinbaseMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, Self::Error> {
        match self {
            Self::CoinbaseOrder(o) => Ok(OrderflowMessage::Order(*o)),
            Self::Order(o) => Ok(OrderflowMessage::Order(o)),
            Self::Reject(r) => Ok(OrderflowMessage::Reject(r)),
            Self::Ack(a) => Ok(OrderflowMessage::Ack(a)),
            Self::CoinbaseFill(f) => Ok(OrderflowMessage::Fill(*f)),
            Self::Fill(f) => Ok(OrderflowMessage::Fill(f)),
            Self::Out(o) => Ok(OrderflowMessage::Out(o)),
            Self::ExchangeOrderUpdate(..) => Err(()),
            Self::ExchangeFills(..) => Err(()),
            Self::ExchangeExternalOrderUpdate(..) => Err(()),
            Self::ExchangeExternalOrderNew(..) => Err(()),
            Self::ExchangeExternalOrderOut(..) => Err(()),
        }
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct CoinbaseOrder {
    pub order: Order,
    #[allow(dead_code)]
    pub special_coinbase_flag: (),
}

impl Deref for CoinbaseOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct CoinbaseFill {
    #[serde(flatten)]
    pub fill: Fill,
    pub exchange_trade_id: Uuid,
    pub exchange_order_id: Uuid,
}

impl Deref for CoinbaseFill {
    type Target = Fill;

    fn deref(&self) -> &Self::Target {
        &self.fill
    }
}
