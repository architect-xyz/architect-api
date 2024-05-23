use crate::{folio::FolioMessage, orderflow::*, Dir};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Protocol for ExternalCpty component to talk to the external cpty;
/// Ser/de is optimized for JSON communication.
#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct ExternalSymbology {
    pub markets: Vec<String>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct ExternalBalances {
    pub balances: Vec<(String, Decimal)>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct ExternalOrder {
    pub id: OrderId,
    pub market: String,
    pub dir: Dir,
    pub quantity: Decimal,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct ExternalReject {
    pub order_id: OrderId,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
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

/// Internal core message type for the ExternalCpty component.
#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum ExternalCptyMessage {
    Orderflow(OrderflowMessage),
    Folio(FolioMessage),
    External(ExternalCptyProtocol),
    Initialize,
}

impl TryInto<OrderflowMessage> for &ExternalCptyMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            ExternalCptyMessage::Orderflow(o) => Ok(o.clone()),
            _ => Err(()),
        }
    }
}

impl TryInto<ExternalCptyMessage> for &OrderflowMessage {
    type Error = ();

    fn try_into(self) -> Result<ExternalCptyMessage, ()> {
        Ok(ExternalCptyMessage::Orderflow(self.clone()))
    }
}

impl TryInto<FolioMessage> for &ExternalCptyMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, ()> {
        match self {
            ExternalCptyMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl TryInto<ExternalCptyMessage> for &FolioMessage {
    type Error = ();

    fn try_into(self) -> Result<ExternalCptyMessage, ()> {
        Ok(ExternalCptyMessage::Folio(self.clone()))
    }
}
