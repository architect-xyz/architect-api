use crate::{
    folio::FolioMessage,
    orderflow::*,
    symbology::{market::NormalizedMarketInfo, ProductId},
};
use compact_str::CompactString;
use derive::FromValue;
use log::error;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde_derive::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct GalaxyMarketInfo {
    pub tick_size: Decimal,
}

impl NormalizedMarketInfo for GalaxyMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    // CR alee: copied from core, seems dubious...
    fn step_size(&self) -> Decimal {
        self.tick_size
    }

    fn is_delisted(&self) -> bool {
        false
    }
}

impl std::fmt::Display for GalaxyMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

impl Default for GalaxyMarketInfo {
    fn default() -> GalaxyMarketInfo {
        GalaxyMarketInfo {
            // Galaxy support says tick size is 0.00000001 for all pairs
            tick_size: dec!(0.00000001),
        }
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum GalaxyMessage {
    Order(GalaxyOrder),
    Cancel(Cancel),
    Ack(GalaxyAck),
    Reject(Reject),
    Fill(GalaxyFill),
    Out(Out),
    Balances(Vec<(ProductId, Decimal)>),
    Folio(FolioMessage),
}

impl TryInto<GalaxyMessage> for &OrderflowMessage {
    type Error = ();

    fn try_into(self) -> Result<GalaxyMessage, ()> {
        match self {
            OrderflowMessage::Order(o) => {
                Ok(GalaxyMessage::Order(GalaxyOrder { order: *o }))
            }
            OrderflowMessage::Cancel(cancel) => Ok(GalaxyMessage::Cancel(*cancel)),
            OrderflowMessage::CancelAll(_) => {
                Err(error!("Cancel all not implemented for Galaxy"))
            }
            OrderflowMessage::Ack(_)
            | OrderflowMessage::Reject(_)
            | OrderflowMessage::Fill(_)
            | OrderflowMessage::Out(_) => Err(()),
        }
    }
}

impl TryInto<OrderflowMessage> for &GalaxyMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            GalaxyMessage::Order(o) => Ok(OrderflowMessage::Order(**o)),
            GalaxyMessage::Ack(a) => Ok(OrderflowMessage::Ack(**a)),
            GalaxyMessage::Reject(r) => Ok(OrderflowMessage::Reject(r.clone())),
            GalaxyMessage::Fill(f, ..) => Ok(OrderflowMessage::Fill(**f)),
            GalaxyMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            GalaxyMessage::Cancel(c) => Ok(OrderflowMessage::Cancel(*c)),
            GalaxyMessage::Balances(_) | GalaxyMessage::Folio(_) => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct GalaxyOrder {
    #[serde(flatten)]
    pub order: Order,
}

impl Deref for GalaxyOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

impl DerefMut for GalaxyOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.order
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct GalaxyAck {
    #[serde(flatten)]
    pub ack: Ack,
    pub exchange_order_id: CompactString,
}

impl Deref for GalaxyAck {
    type Target = Ack;

    fn deref(&self) -> &Self::Target {
        &self.ack
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct GalaxyFill {
    #[serde(flatten)]
    pub fill: Result<Fill, AberrantFill>,
    pub leaves_qty: Decimal,
}

impl Deref for GalaxyFill {
    type Target = Result<Fill, AberrantFill>;

    fn deref(&self) -> &Self::Target {
        &self.fill
    }
}
