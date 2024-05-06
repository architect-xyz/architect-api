use crate::{
    folio::FolioMessage,
    orderflow::{Fill, Order, OrderflowMessage, Out, Reject},
    symbology::market::NormalizedMarketInfo,
};
use derive::FromValue;
use log::error;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum CumberlandMessage {
    Order(Order),
    Reject(Reject),
    Fill(Fill),
    Out(Out),
    ExchangeFill(Fill),
    ExchangeReject(Reject),
    Folio(FolioMessage),
}

impl TryInto<OrderflowMessage> for &CumberlandMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, Self::Error> {
        match self {
            CumberlandMessage::Order(o) => Ok(OrderflowMessage::Order(*o)),
            CumberlandMessage::Reject(r) => Ok(OrderflowMessage::Reject(r.clone())),
            CumberlandMessage::Fill(f) => Ok(OrderflowMessage::Fill(Ok(*f))),
            CumberlandMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            CumberlandMessage::ExchangeFill(..) => Err(()),
            CumberlandMessage::ExchangeReject(..) => Err(()),
            CumberlandMessage::Folio(..) => Err(()),
        }
    }
}

impl TryInto<CumberlandMessage> for &OrderflowMessage {
    type Error = ();

    fn try_into(self) -> Result<CumberlandMessage, Self::Error> {
        match self {
            OrderflowMessage::Order(o) => Ok(CumberlandMessage::Order(*o)),
            OrderflowMessage::Cancel(_) => Err(()),
            OrderflowMessage::Reject(r) => Ok(CumberlandMessage::Reject(r.clone())),
            OrderflowMessage::Ack(_) => Err(()),
            OrderflowMessage::Fill(f) => Ok(CumberlandMessage::Fill(f.map_err(|_| ())?)),
            OrderflowMessage::Out(o) => Ok(CumberlandMessage::Out(*o)),
            OrderflowMessage::CancelAll(_) => {
                Err(error!("Cancel all not implemented for Cumberland"))
            }
        }
    }
}

impl TryInto<FolioMessage> for &CumberlandMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, Self::Error> {
        match self {
            CumberlandMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl TryInto<CumberlandMessage> for &FolioMessage {
    type Error = ();

    fn try_into(self) -> Result<CumberlandMessage, Self::Error> {
        Ok(CumberlandMessage::Folio(self.clone()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct CumberlandMarketInfo {
    pub tick_size: Decimal,
    pub step_size: Decimal,
    pub is_delisted: bool,
}

impl NormalizedMarketInfo for CumberlandMarketInfo {
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

impl std::fmt::Display for CumberlandMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}
