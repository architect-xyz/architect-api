#![cfg(feature = "netidx")]

use crate::{
    folio::FolioMessage,
    orderflow::*,
    symbology::{MarketId, ProductId},
    UserId,
};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum SimCptyMessage {
    Init,
    Orderflow(OrderflowMessage),
    Folio(FolioMessage),
    Bbo(Bbo),
    SimBalanceChange(SimBalanceChange),
}

impl TryInto<OrderflowMessage> for &SimCptyMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            SimCptyMessage::Orderflow(o) => Ok(o.clone()),
            _ => Err(()),
        }
    }
}

impl Into<SimCptyMessage> for &OrderflowMessage {
    fn into(self) -> SimCptyMessage {
        SimCptyMessage::Orderflow(self.clone())
    }
}

impl TryInto<FolioMessage> for &SimCptyMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, ()> {
        match self {
            SimCptyMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl Into<SimCptyMessage> for &FolioMessage {
    fn into(self) -> SimCptyMessage {
        SimCptyMessage::Folio(self.clone())
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct Bbo {
    pub market_id: MarketId,
    pub bid: Option<Decimal>,
    pub ask: Option<Decimal>,
}

impl Bbo {
    pub fn default(market_id: MarketId) -> Self {
        Self { market_id, bid: None, ask: None }
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct SimBalanceChange {
    pub product: ProductId,
    pub user: UserId,
    pub amount: Decimal,
}
