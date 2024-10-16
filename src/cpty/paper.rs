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
pub enum PaperCptyMessage {
    Init,
    Orderflow(OrderflowMessage),
    Folio(FolioMessage),
    Bbo(Bbo),
    PaperBalanceChange(PaperBalanceChange),
}

impl TryInto<OrderflowMessage> for &PaperCptyMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            PaperCptyMessage::Orderflow(o) => Ok(o.clone()),
            _ => Err(()),
        }
    }
}

impl Into<PaperCptyMessage> for &OrderflowMessage {
    fn into(self) -> PaperCptyMessage {
        PaperCptyMessage::Orderflow(self.clone())
    }
}

impl TryInto<FolioMessage> for &PaperCptyMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, ()> {
        match self {
            PaperCptyMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl Into<PaperCptyMessage> for &FolioMessage {
    fn into(self) -> PaperCptyMessage {
        PaperCptyMessage::Folio(self.clone())
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
pub struct PaperBalanceChange {
    pub product: ProductId,
    pub user: UserId,
    pub amount: Decimal,
}
