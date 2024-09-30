#![cfg(feature = "netidx")]
use rust_decimal_macros::dec;
use crate::{
    MaybeSecret, folio::FolioMessage, orderflow::{
        AberrantFill, Ack, Cancel, CancelAll, Fill, Order, OrderflowMessage, Out, Reject,
    }, symbology::market::NormalizedMarketInfo, OrderId
};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KalshiCredentials {
    /// API key UUID from Kalshi
    pub api_key: String,
    /// PEM encoded RSA private key from Kalshi
    pub api_private_key: MaybeSecret<String>,
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct KalshiOrder {
    #[serde(flatten)]
    pub order: Order,
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct KalshiMarketInfo {
    pub is_delisted: bool,
    pub risk_limit_cents: i64,
    pub can_close_early: bool,
    pub series_ticker: String,
    pub strike_type: KalshiStrikeType,
    pub cap_strike: Option<Decimal>,
    pub floor_strike: Option<Decimal>,
    pub series: Option<KalshiSeries>,
}


impl NormalizedMarketInfo for KalshiMarketInfo {
    fn tick_size(&self) -> Decimal {
        dec!(0.01)
    }

    fn step_size(&self) -> Decimal {
        dec!(0.01)
    }

    fn is_delisted(&self) -> bool {
        self.is_delisted
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct KalshiSeries {
    pub category: String,
    pub contract_url: String,
    pub frequency: String,
    pub settlement_source: Vec<KalshiSettlementSource>,
    pub tags: Vec<String>,
    pub ticker: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct KalshiSettlementSource {
    pub name: String,
    pub url: String,
}



impl std::fmt::Display for KalshiMarketInfo{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

impl Deref for KalshiOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct KalshiTrade {
    pub order_id: OrderId,
    pub exec_id: String,
    pub scaled_price: i64,
    pub qty: Decimal,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize, PartialEq)]
pub enum KalshiOrderStatus {
    Canceled,
    Executed,
    Resting,
    Pending,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum KalshiStrikeType {
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Between,
    Functional,
    Custom
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct KalshiOrderState {
    pub internal_order_id: OrderId,
    pub last_update_time: Option<DateTime<Utc>>,
    pub status: KalshiOrderStatus,
    pub fills: Vec<Result<Fill, AberrantFill>>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum KalshiMessage {
    Order(KalshiOrder),
    Cancel(Cancel),
    CancelAll,
    Ack(Ack),
    Out(Out),
    Fill(Result<Fill, AberrantFill>),
    Reject(Reject),
    Folio(FolioMessage),
    PollOrders,
    ExchangeAck { order_id: OrderId, kalshi_order_id: String },
    ExchangeOrderState(KalshiOrderState),
}

impl TryInto<OrderflowMessage> for &KalshiMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            KalshiMessage::Order(o) => Ok(OrderflowMessage::Order(**o)),
            KalshiMessage::Cancel(c) => Ok(OrderflowMessage::Cancel(*c)),
            KalshiMessage::CancelAll => {
                Ok(OrderflowMessage::CancelAll(CancelAll { venue_id: None }))
            }
            KalshiMessage::Ack(a) => Ok(OrderflowMessage::Ack(*a)),
            KalshiMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            KalshiMessage::Fill(f) => Ok(OrderflowMessage::Fill(*f)),
            KalshiMessage::Reject(r) => Ok(OrderflowMessage::Reject(r.clone())),
            KalshiMessage::Folio(_)
            | KalshiMessage::PollOrders
            | KalshiMessage::ExchangeAck { .. }
            | KalshiMessage::ExchangeOrderState(_) => Err(()),
        }
    }
}

impl TryInto<KalshiMessage> for &OrderflowMessage {
    type Error = ();

    fn try_into(self) -> Result<KalshiMessage, ()> {
        match self {
            OrderflowMessage::Order(o) => {
                Ok(KalshiMessage::Order(KalshiOrder { order: *o }))
            }
            OrderflowMessage::Cancel(c) => Ok(KalshiMessage::Cancel(*c)),
            OrderflowMessage::CancelAll(_) => Ok(KalshiMessage::CancelAll),
            OrderflowMessage::Ack(a) => Ok(KalshiMessage::Ack(*a)),
            OrderflowMessage::Out(o) => Ok(KalshiMessage::Out(*o)),
            OrderflowMessage::Reject(r) => Ok(KalshiMessage::Reject(r.clone())),
            OrderflowMessage::Fill(f) => Ok(KalshiMessage::Fill(*f)),
        }
    }
}

impl TryInto<FolioMessage> for &KalshiMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, ()> {
        match self {
            KalshiMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<&FolioMessage> for KalshiMessage {
    type Error = ();

    fn try_from(f: &FolioMessage) -> Result<Self, ()> {
        Ok(Self::Folio(f.clone()))
    }
}
