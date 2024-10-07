#[cfg(feature = "netidx")]
use crate::{folio::FolioMessage, symbology::MarketId, utils::messaging::MaybeRequest};
use crate::{orderflow::*, Dir, DirPair};
#[cfg(feature = "netidx")]
use arcstr::ArcStr;
use chrono::{DateTime, Utc};
#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Protocol for ExternalCpty component to talk to the external cpty;
/// Ser/de is optimized for JSON communication.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
#[serde(tag = "type")]
pub enum ExternalCptyProtocol {
    GetSymbology,
    Symbology(ExternalSymbology),
    GetOpenOrders,
    OpenOrders {
        open_orders: Arc<Vec<OrderId>>,
    },
    GetBalances {
        id: Uuid,
    },
    Balances {
        id: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        result: Option<ExternalBalances>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<String>,
    },
    Order(ExternalOrder),
    Cancel(Cancel),
    Reject(ExternalReject),
    Ack(Ack),
    Fill(ExternalFill),
    Out(Out),
    GetBookSnapshot {
        id: Uuid,
        market: String,
    },
    BookSnapshot {
        id: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        result: Option<ExternalBookSnapshot>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub struct ExternalSymbology {
    pub markets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub struct ExternalBalances {
    pub balances: Vec<(String, Decimal)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub struct ExternalOrder {
    pub id: OrderId,
    pub market: String,
    pub dir: Dir,
    pub quantity: Decimal,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub struct ExternalReject {
    pub order_id: OrderId,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub struct ExternalFill {
    pub kind: FillKind,
    pub fill_id: FillId,
    pub order_id: Option<OrderId>,
    pub market: String,
    pub quantity: Decimal,
    pub price: Decimal,
    pub dir: Dir,
    #[serde(default)]
    pub is_maker: Option<bool>,
    pub trade_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub struct ExternalBookSnapshot {
    pub book: DirPair<Arc<Vec<(Decimal, Decimal)>>>, // (price, size)
    pub timestamp: DateTime<Utc>,
}

/// Internal core message type for the ExternalCpty component.
#[cfg(feature = "netidx")]
#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum ExternalCptyMessage {
    Orderflow(OrderflowMessage),
    Folio(FolioMessage),
    External(ExternalCptyProtocol),
    Initialize,
    GetBookSnapshot(Uuid, MarketId),
    BookSnapshot(Uuid, Result<ExternalBookSnapshot, ArcStr>),
}

#[cfg(feature = "netidx")]
impl MaybeRequest for ExternalCptyMessage {
    fn request_id(&self) -> Option<Uuid> {
        match self {
            ExternalCptyMessage::GetBookSnapshot(id, _) => Some(*id),
            _ => None,
        }
    }

    fn response_id(&self) -> Option<Uuid> {
        match self {
            ExternalCptyMessage::BookSnapshot(id, _) => Some(*id),
            _ => None,
        }
    }
}

#[cfg(feature = "netidx")]
impl TryInto<OrderflowMessage> for &ExternalCptyMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            ExternalCptyMessage::Orderflow(o) => Ok(o.clone()),
            _ => Err(()),
        }
    }
}

#[cfg(feature = "netidx")]
impl TryInto<ExternalCptyMessage> for &OrderflowMessage {
    type Error = ();

    fn try_into(self) -> Result<ExternalCptyMessage, ()> {
        Ok(ExternalCptyMessage::Orderflow(self.clone()))
    }
}

#[cfg(feature = "netidx")]
impl TryInto<FolioMessage> for &ExternalCptyMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, ()> {
        match self {
            ExternalCptyMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

#[cfg(feature = "netidx")]
impl TryInto<ExternalCptyMessage> for &FolioMessage {
    type Error = ();

    fn try_into(self) -> Result<ExternalCptyMessage, ()> {
        Ok(ExternalCptyMessage::Folio(self.clone()))
    }
}
