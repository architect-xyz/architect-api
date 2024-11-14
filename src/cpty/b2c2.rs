use crate::{orderflow::*, symbology::market::NormalizedMarketInfo};
#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct B2C2MarketInfo {
    pub tick_size: Decimal,
    pub step_size: Decimal,
    pub is_delisted: bool,
}

impl NormalizedMarketInfo for B2C2MarketInfo {
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

impl std::fmt::Display for B2C2MarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub enum B2C2Message {
    Order(B2C2Order),
    Reject(Reject),
    Fill(B2C2Fill),
    Out(Out),
    ExchangeExternalFills(Vec<B2C2Fill>),
}

impl TryInto<B2C2Message> for &OrderflowMessage {
    type Error = ();

    fn try_into(self) -> Result<B2C2Message, ()> {
        match self {
            OrderflowMessage::Order(o) => Ok(B2C2Message::Order(B2C2Order { order: *o })),
            OrderflowMessage::Cancel(_) => Err(()),
            OrderflowMessage::CancelAll(_) => Err(()),
            OrderflowMessage::Reject(r) => Ok(B2C2Message::Reject(r.clone())),
            OrderflowMessage::Ack(_) => Err(()),
            OrderflowMessage::Fill(f) => Ok(B2C2Message::Fill(B2C2Fill { fill: *f })),
            OrderflowMessage::Out(o) => Ok(B2C2Message::Out(*o)),
        }
    }
}

impl TryInto<OrderflowMessage> for &B2C2Message {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            B2C2Message::Order(o) => Ok(OrderflowMessage::Order(**o)),
            B2C2Message::Reject(r) => Ok(OrderflowMessage::Reject(r.clone())),
            B2C2Message::Fill(f) => Ok(OrderflowMessage::Fill(**f)),
            B2C2Message::Out(o) => Ok(OrderflowMessage::Out(*o)),
            B2C2Message::ExchangeExternalFills(..) => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct B2C2Order {
    #[serde(flatten)]
    pub order: Order,
}

impl From<Order> for B2C2Order {
    fn from(order: Order) -> Self {
        Self { order }
    }
}

impl Deref for B2C2Order {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

impl DerefMut for B2C2Order {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.order
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct B2C2Fill {
    #[serde(flatten)]
    pub fill: Result<Fill, AberrantFill>,
}

impl Deref for B2C2Fill {
    type Target = Result<Fill, AberrantFill>;

    fn deref(&self) -> &Self::Target {
        &self.fill
    }
}
