use crate::{folio::FolioMessage, orderflow::*, symbology::market::NormalizedMarketInfo};
#[cfg(feature = "netidx")]
use derive::FromValue;
use log::error;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct FalconXMarketInfo {}

impl NormalizedMarketInfo for FalconXMarketInfo {
    fn tick_size(&self) -> Decimal {
        dec!(0.0001)
    }

    fn step_size(&self) -> Decimal {
        dec!(0.000001)
    }

    fn is_delisted(&self) -> bool {
        false
    }
}

impl std::fmt::Display for FalconXMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub enum FalconXMessage {
    Order(FalconXOrder),
    Reject(Reject),
    Fill(FalconXFill),
    Out(Out),
    Folio(FolioMessage),
    ExchangeExternalFills(Vec<FalconXFill>),
}

impl TryInto<FalconXMessage> for &OrderflowMessage {
    type Error = ();

    fn try_into(self) -> Result<FalconXMessage, ()> {
        match self {
            OrderflowMessage::Order(o) => {
                Ok(FalconXMessage::Order(FalconXOrder { order: *o }))
            }
            OrderflowMessage::Cancel(_) => Err(()),
            OrderflowMessage::Reject(r) => Ok(FalconXMessage::Reject(r.clone())),
            OrderflowMessage::Ack(_) => Err(()),
            OrderflowMessage::Fill(f) => {
                Ok(FalconXMessage::Fill(FalconXFill { fill: *f }))
            }
            OrderflowMessage::Out(o) => Ok(FalconXMessage::Out(*o)),
            OrderflowMessage::CancelAll(_) => {
                Err(error!("Cancel all not implemented for FalconX"))
            }
        }
    }
}

impl TryInto<OrderflowMessage> for &FalconXMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            FalconXMessage::Order(o) => Ok(OrderflowMessage::Order(**o)),
            FalconXMessage::Reject(r) => Ok(OrderflowMessage::Reject(r.clone())),
            FalconXMessage::Fill(f) => Ok(OrderflowMessage::Fill(**f)),
            FalconXMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            FalconXMessage::Folio(_) => Err(()),
            FalconXMessage::ExchangeExternalFills(..) => Err(()),
        }
    }
}

impl TryInto<FolioMessage> for &FalconXMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, ()> {
        match self {
            FalconXMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<&FolioMessage> for FalconXMessage {
    type Error = ();

    fn try_from(f: &FolioMessage) -> Result<Self, ()> {
        Ok(Self::Folio(f.clone()))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct FalconXOrder {
    #[serde(flatten)]
    pub order: Order,
}

impl From<Order> for FalconXOrder {
    fn from(order: Order) -> Self {
        Self { order }
    }
}

impl Deref for FalconXOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

impl DerefMut for FalconXOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.order
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct FalconXFill {
    #[serde(flatten)]
    pub fill: Result<Fill, AberrantFill>,
}

impl Deref for FalconXFill {
    type Target = Result<Fill, AberrantFill>;

    fn deref(&self) -> &Self::Target {
        &self.fill
    }
}
