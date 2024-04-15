use crate::{
    folio::FolioMessage,
    orderflow::{AberrantFill, Ack, Cancel, Fill, Order, OrderflowMessage, Out, Reject},
    symbology::market::{MinOrderQuantityUnit, NormalizedMarketInfo},
    Amount, OrderId,
};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, sync::Arc};

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
    Ack(BinanceAck),
    Fill(Result<Fill, AberrantFill>),
    Out(Out),
    Folio(FolioMessage),
    ExchangeAccountSnapshot(Arc<BinanceSnapshot>),
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct BinanceAck {
    pub ack: Ack,
    pub exchange_order_id: u64,
}

impl Deref for BinanceAck {
    type Target = Ack;

    fn deref(&self) -> &Self::Target {
        &self.ack
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct BinanceSnapshot {
    pub open_oids: Vec<OrderId>,
}

impl TryFrom<&BinanceMessage> for OrderflowMessage {
    type Error = ();

    fn try_from(value: &BinanceMessage) -> Result<Self, Self::Error> {
        match value {
            BinanceMessage::Order(o) => Ok(OrderflowMessage::Order(*o)),
            BinanceMessage::Cancel(c) => Ok(OrderflowMessage::Cancel(*c)),
            BinanceMessage::Reject(r) => Ok(OrderflowMessage::Reject(r.clone())),
            BinanceMessage::Ack(a) => Ok(OrderflowMessage::Ack(**a)),
            BinanceMessage::Fill(f) => Ok(OrderflowMessage::Fill(f.clone())),
            BinanceMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            BinanceMessage::Folio(_) | BinanceMessage::ExchangeAccountSnapshot(..) => {
                Err(())
            }
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
            OrderflowMessage::Ack(_a) => Err(()),
            OrderflowMessage::Fill(f) => Ok(BinanceMessage::Fill(f.clone())),
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
