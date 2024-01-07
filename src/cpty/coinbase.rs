use crate::{
    folio::FolioMessage,
    orderflow::*,
    symbology::{market::NormalizedMarketInfo, MarketId},
};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
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

// CR alee: consider BatchCancel
#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum CoinbaseMessage {
    Order(CoinbaseOrder),
    Cancel(Cancel),
    Reject(Reject),
    Ack(Ack),
    Fill(CoinbaseFill),
    Out(Out),
    Folio(FolioMessage),
    ExchangeOrderUpdate(OrderId),
    ExchangeAck(OrderId, Uuid),
    ExchangeFills(Vec<CoinbaseFill>),
    ExchangeExternalOrderUpdate(MarketId, Uuid),
    ExchangeExternalOrderNew(Uuid, CoinbaseOrder),
    ExchangeExternalOrderOut(Uuid),
}

impl TryInto<OrderflowMessage> for &CoinbaseMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            CoinbaseMessage::Order(o) => Ok(OrderflowMessage::Order(**o)),
            CoinbaseMessage::Cancel(c) => Ok(OrderflowMessage::Cancel(*c)),
            CoinbaseMessage::Reject(r) => Ok(OrderflowMessage::Reject(*r)),
            CoinbaseMessage::Ack(a) => Ok(OrderflowMessage::Ack(*a)),
            CoinbaseMessage::Fill(f) => Ok(OrderflowMessage::Fill(**f)),
            CoinbaseMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            CoinbaseMessage::Folio(..)
            | CoinbaseMessage::ExchangeOrderUpdate(..)
            | CoinbaseMessage::ExchangeAck(..)
            | CoinbaseMessage::ExchangeFills(..)
            | CoinbaseMessage::ExchangeExternalOrderUpdate(..)
            | CoinbaseMessage::ExchangeExternalOrderNew(..)
            | CoinbaseMessage::ExchangeExternalOrderOut(..) => Err(()),
        }
    }
}

impl TryInto<FolioMessage> for &CoinbaseMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, ()> {
        match self {
            CoinbaseMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<&FolioMessage> for CoinbaseMessage {
    type Error = ();

    fn try_from(f: &FolioMessage) -> Result<Self, ()> {
        Ok(Self::Folio(f.clone()))
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct CoinbaseOrder {
    #[serde(flatten)]
    pub order: Order,
}

impl From<Order> for CoinbaseOrder {
    fn from(order: Order) -> Self {
        Self { order }
    }
}

impl Deref for CoinbaseOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

impl DerefMut for CoinbaseOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.order
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct CoinbaseFill {
    #[serde(flatten)]
    pub fill: Result<Fill, AberrantFill>,
    pub exchange_trade_id: Uuid,
    pub exchange_order_id: Uuid,
}

impl Deref for CoinbaseFill {
    type Target = Result<Fill, AberrantFill>;

    fn deref(&self) -> &Self::Target {
        &self.fill
    }
}

impl std::fmt::Display for CoinbaseMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}
