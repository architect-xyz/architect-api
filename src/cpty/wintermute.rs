use crate::{
    folio::FolioMessage,
    orderflow::*,
    symbology::{market::NormalizedMarketInfo, ProductId},
    Dir, Str,
};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct WintermuteMarketInfo {
    pub tick_size: Decimal,
    pub step_size: Decimal,
    pub is_delisted: bool,
}

impl NormalizedMarketInfo for WintermuteMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    fn step_size(&self) -> Decimal {
        self.step_size
    }

    fn is_delisted(&self) -> bool {
        self.is_delisted
    }
}

impl std::fmt::Display for WintermuteMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum WintermuteMessage {
    Order(WintermuteOrder),
    Ack(Ack),
    Reject(Reject),
    Fill(WintermuteFill),
    Out(Out),
    ExecutionReport(ExecutionReport),
    Balances(Vec<(ProductId, Decimal)>),
    Folio(FolioMessage),
}

impl TryInto<WintermuteMessage> for &OrderflowMessage {
    type Error = ();

    fn try_into(self) -> Result<WintermuteMessage, ()> {
        match self {
            OrderflowMessage::Order(o) => {
                Ok(WintermuteMessage::Order(WintermuteOrder { order: *o }))
            }
            OrderflowMessage::Ack(a) => Ok(WintermuteMessage::Ack(*a)),
            OrderflowMessage::Cancel(_) => Err(()),
            OrderflowMessage::Reject(r) => Ok(WintermuteMessage::Reject(*r)),
            OrderflowMessage::Fill(f) => {
                Ok(WintermuteMessage::Fill(WintermuteFill { fill: *f }))
            }
            OrderflowMessage::Out(o) => Ok(WintermuteMessage::Out(*o)),
        }
    }
}

impl TryInto<OrderflowMessage> for &WintermuteMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            WintermuteMessage::Order(o) => Ok(OrderflowMessage::Order(**o)),
            WintermuteMessage::Ack(a) => Ok(OrderflowMessage::Ack(*a)),
            WintermuteMessage::Reject(r) => Ok(OrderflowMessage::Reject(*r)),
            WintermuteMessage::Fill(f) => Ok(OrderflowMessage::Fill(**f)),
            WintermuteMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            WintermuteMessage::Balances(_)
            | WintermuteMessage::Folio(_)
            | WintermuteMessage::ExecutionReport(_) => Err(()),
        }
    }
}

impl TryInto<FolioMessage> for &WintermuteMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, ()> {
        match self {
            WintermuteMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<&FolioMessage> for WintermuteMessage {
    type Error = ();

    fn try_from(f: &FolioMessage) -> Result<Self, ()> {
        Ok(Self::Folio(f.clone()))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, FromValue, Pack)]
pub struct ExecutionReport {
    pub client_order_id: Str,
    pub order_id: Str,
    pub execution_id: Str,
    pub exec_type: ExecType,
    pub order_status: Str,
    pub symbol: Str,
    pub product: Str,
    pub currency: Str,
    pub side: Dir,
    pub order_qty: Decimal,
    pub last_qty: Decimal,
    pub cum_qty: Decimal,
    pub leaves_qty: Decimal,
    pub price: Option<Decimal>,
    pub transact_time: DateTime<Utc>,
    pub text: Option<Str>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, FromValue, Pack)]
pub enum ExecType {
    Canceled,
    Rejected,
    Expired,
    Trade,
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct WintermuteOrder {
    #[serde(flatten)]
    pub order: Order,
}

impl From<Order> for WintermuteOrder {
    fn from(order: Order) -> Self {
        Self { order }
    }
}

impl Deref for WintermuteOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

impl DerefMut for WintermuteOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.order
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct WintermuteFill {
    #[serde(flatten)]
    pub fill: Result<Fill, AberrantFill>,
}

impl Deref for WintermuteFill {
    type Target = Result<Fill, AberrantFill>;

    fn deref(&self) -> &Self::Target {
        &self.fill
    }
}
