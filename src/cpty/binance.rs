use crate::{
    folio::FolioMessage,
    orderflow::{Ack, Cancel, Fill, Order, OrderflowMessage, Out, Reject},
    symbology::{
        market::{MinOrderQuantityUnit, NormalizedMarketInfo},
        CptyId,
    },
    Address, Amount, Dir, HalfOpenRange, OrderId,
};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct BinanceMarketInfo {
    pub tick_size: Decimal,
    pub step_size: Decimal,
    pub is_delisted: bool,
    #[pack(default)]
    pub min_order_quantity: Amount<Decimal, MinOrderQuantityUnit>,
}

impl NormalizedMarketInfo for BinanceMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    fn step_size(&self) -> Decimal {
        self.step_size
    }

    fn min_order_quantity(&self) -> Amount<Decimal, MinOrderQuantityUnit> {
        self.min_order_quantity
    }

    fn is_delisted(&self) -> bool {
        self.is_delisted
    }
}

impl std::fmt::Display for BinanceMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum BinanceMessage {
    Order(Order),
    Cancel(Cancel),
    Reject(Reject),
    Ack(Ack),
    Fill(Fill),
    Out(Out),
    ExchangeAck(BinanceExchangeAck),
    ExchangeHistoricalFills(BinanceExchangeHistoricalFills),
    ExchangeOrderFailed(OrderId),
    ExternalOrderUpdates(BinanceExternalOrderUpdates),
    Folio(FolioMessage),
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct BinanceExchangeAck {
    pub order_id: OrderId,
    pub exchange_order_id: u64,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct BinanceExchangeHistoricalFills {
    pub cpty: CptyId,
    pub range: HalfOpenRange<Option<DateTime<Utc>>>,
    pub fills: Vec<BinancePartialFill>,
    pub reply_to: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct BinancePartialFill {
    pub xoid: u64,
    pub quantity: Decimal,
    pub price: Decimal,
    pub side: Dir,
    pub trade_time: DateTime<Utc>,
    pub is_maker: Option<bool>,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct BinanceExternalOrderUpdates {
    pub fills: Vec<BinanceExternalOrderUpdate>,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct BinanceExternalOrderUpdate {
    pub exchange_order_id: u64,
    pub fill: Fill,
    pub original_qty: Decimal,
}

impl TryFrom<&BinanceMessage> for OrderflowMessage {
    type Error = ();

    fn try_from(value: &BinanceMessage) -> Result<Self, Self::Error> {
        match value {
            BinanceMessage::Order(o) => Ok(OrderflowMessage::Order(*o)),
            BinanceMessage::Cancel(c) => Ok(OrderflowMessage::Cancel(*c)),
            BinanceMessage::Reject(r) => Ok(OrderflowMessage::Reject(r.clone())),
            BinanceMessage::Ack(a) => Ok(OrderflowMessage::Ack(*a)),
            BinanceMessage::Fill(f) => Ok(OrderflowMessage::Fill(Ok(*f))),
            BinanceMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            BinanceMessage::ExchangeAck(_)
            | BinanceMessage::ExchangeHistoricalFills(_)
            | BinanceMessage::ExchangeOrderFailed(_)
            | BinanceMessage::ExternalOrderUpdates(_)
            | BinanceMessage::Folio(_) => Err(()),
        }
    }
}

impl TryFrom<&OrderflowMessage> for BinanceMessage {
    type Error = ();

    fn try_from(value: &OrderflowMessage) -> Result<Self, Self::Error> {
        match value {
            OrderflowMessage::Order(o) => Ok(BinanceMessage::Order(*o)),
            OrderflowMessage::Cancel(c) => Ok(BinanceMessage::Cancel(*c)),
            OrderflowMessage::Reject(r) => Ok(BinanceMessage::Reject(r.clone())),
            OrderflowMessage::Ack(a) => Ok(BinanceMessage::Ack(*a)),
            OrderflowMessage::Fill(f) => match f {
                Ok(f) => Ok(BinanceMessage::Fill(*f)),
                Err(_) => Err(()),
            },
            OrderflowMessage::Out(o) => Ok(BinanceMessage::Out(*o)),
        }
    }
}

impl TryFrom<&BinanceMessage> for FolioMessage {
    type Error = ();

    fn try_from(value: &BinanceMessage) -> Result<Self, Self::Error> {
        match value {
            BinanceMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<&FolioMessage> for BinanceMessage {
    type Error = ();

    fn try_from(value: &FolioMessage) -> Result<Self, Self::Error> {
        Ok(BinanceMessage::Folio(value.clone()))
    }
}
