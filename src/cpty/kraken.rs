use crate::{
    folio::FolioMessage,
    orderflow::{
        AberrantFill, Ack, Cancel, Fill, Order, OrderType, OrderflowMessage, Out, Reject,
        TimeInForce,
    },
    symbology::market::{MinOrderQuantityUnit, NormalizedMarketInfo},
    Amount, Dir, OrderId,
};
use chrono::{DateTime, Utc};
use derive::{FromStrJson, FromValue};
use netidx_derive::Pack;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use zeroize::Zeroize;

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub enum Status {
    #[serde(alias = "online")]
    Online,
    #[serde(alias = "cancel_only")]
    CancelOnly,
    #[serde(alias = "post_only")]
    PostOnly,
    #[serde(alias = "limit_only")]
    LimitOnly,
    #[serde(alias = "reduce_only")]
    ReduceOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct KrakenMarketInfo {
    pub altname: String,
    pub wsname: String,
    pub aclass_base: String,
    pub base: String,
    pub aclass_quote: String,
    pub quote: String,
    pub pair_decimals: u32,
    pub cost_decimals: u32,
    pub lot_decimals: u32,
    pub lot_multiplier: u32,
    pub margin_call: u32,
    pub margin_stop: u32,
    pub fee_volume_currency: String,
    pub ordermin: Decimal,
    pub costmin: Decimal,
    pub tick_size: Decimal,
    pub status: Status,
    pub long_position_limit: Option<u32>,
    pub short_position_limit: Option<u32>,
}

impl NormalizedMarketInfo for KrakenMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    fn step_size(&self) -> Decimal {
        Decimal::from_f64(10f64.powi(-(self.lot_decimals as i32)))
            .expect(&format!("could not compute step_size: {:?}", self))
    }

    fn min_order_quantity(&self) -> Amount<Decimal, MinOrderQuantityUnit> {
        return Amount::new(self.ordermin, MinOrderQuantityUnit::Base);
    }

    fn is_delisted(&self) -> bool {
        false
    }
}

impl std::fmt::Display for KrakenMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum KrakenMessage {
    Initialize,
    Order(KrakenOrder),
    Cancel(Cancel),
    Reject(Reject),
    Ack(Ack),
    Fill(KrakenFill),
    Out(Out),
    ExchangeOrderUpdate(
        KrakenUserRef,
        KrakenExchangeId,
        Option<(OrderId, KrakenExternalOrder)>,
        bool, /* out */
    ),
    ExchangeAck(OrderId, KrakenExchangeId),
    ExchangeFill(KrakenExternalFill),
    ExchangeExternalOrderNew(OrderId, KrakenExternalOrder),
    ExchangeExternalOrderOut(KrakenExchangeId),
    ExchangeExternalFill(KrakenExternalFill),
    Folio(FolioMessage),
}

impl TryInto<OrderflowMessage> for &KrakenMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            KrakenMessage::Order(o) => Ok(OrderflowMessage::Order(**o)),
            KrakenMessage::Cancel(c) => Ok(OrderflowMessage::Cancel(*c)),
            KrakenMessage::Reject(r) => Ok(OrderflowMessage::Reject(r.clone())),
            KrakenMessage::Ack(a) => Ok(OrderflowMessage::Ack(*a)),
            KrakenMessage::Fill(f) => Ok(OrderflowMessage::Fill(**f)),
            KrakenMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            KrakenMessage::ExchangeOrderUpdate(..)
            | KrakenMessage::Initialize
            | KrakenMessage::ExchangeAck(..)
            | KrakenMessage::ExchangeFill(..)
            | KrakenMessage::ExchangeExternalOrderNew(..)
            | KrakenMessage::ExchangeExternalFill(..)
            | KrakenMessage::ExchangeExternalOrderOut(..)
            | KrakenMessage::Folio(..) => Err(()),
        }
    }
}

impl TryInto<KrakenMessage> for &OrderflowMessage {
    type Error = ();

    fn try_into(self) -> Result<KrakenMessage, ()> {
        match self {
            OrderflowMessage::Order(o) => {
                Ok(KrakenMessage::Order(KrakenOrder { order: *o }))
            }
            OrderflowMessage::Cancel(c) => Ok(KrakenMessage::Cancel(*c)),
            OrderflowMessage::Reject(r) => Ok(KrakenMessage::Reject(r.clone())),
            OrderflowMessage::Ack(a) => Ok(KrakenMessage::Ack(*a)),
            OrderflowMessage::Fill(_) => Err(()),
            OrderflowMessage::Out(o) => Ok(KrakenMessage::Out(*o)),
        }
    }
}

impl TryInto<FolioMessage> for &KrakenMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, ()> {
        match self {
            KrakenMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<&FolioMessage> for KrakenMessage {
    type Error = ();

    fn try_from(f: &FolioMessage) -> Result<Self, ()> {
        Ok(Self::Folio(f.clone()))
    }
}

pub type KrakenExchangeId = String;
pub type KrakenUserRef = i32;

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct KrakenOrder {
    #[serde(flatten)]
    pub order: Order,
}

impl From<Order> for KrakenOrder {
    fn from(order: Order) -> Self {
        Self { order }
    }
}

impl Deref for KrakenOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

impl DerefMut for KrakenOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.order
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct KrakenFill {
    #[serde(flatten)]
    pub fill: Result<Fill, AberrantFill>,
    pub exchange_trade_id: KrakenExchangeId,
    pub exchange_order_id: KrakenExchangeId,
}

impl Deref for KrakenFill {
    type Target = Result<Fill, AberrantFill>;

    fn deref(&self) -> &Self::Target {
        &self.fill
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct KrakenExternalOrder {
    pub exchange_symbol: String,
    pub exchange_order_id: KrakenExchangeId,
    pub user_reference_id: KrakenUserRef,
    pub quantity: Decimal,
    pub trigger_price: Option<Decimal>,
    pub dir: Dir,
    pub expiration: Option<DateTime<Utc>>,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct KrakenExternalFill {
    pub exchange_order_id: KrakenExchangeId,
    pub exchange_trade_id: KrakenExchangeId,
    pub user_reference_id: Option<KrakenUserRef>,
    pub time: DateTime<Utc>,
    pub quantity: Decimal,
    pub price: Decimal,
    pub dir: Dir,
}

#[derive(Debug, Clone, Pack, FromValue, FromStrJson, Serialize, Deserialize, Zeroize)]
pub struct KrakenCredentials {
    /// Account name for the API key--must be user generated since there's
    /// no way to programmatically determine this.  If not provided, the
    /// account name defaults to "DEFAULT".
    #[serde(default)]
    pub account_name: Option<String>,
    pub api_key: String,
    pub api_secret: String,
}
