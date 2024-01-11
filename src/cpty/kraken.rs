use crate::{
    orderflow::{
        AberrantFill, Ack, Cancel, Fill, Order, OrderType, OrderflowMessage, Out, Reject,
        TimeInForce,
    },
    symbology::market::NormalizedMarketInfo,
    Dir, OrderId,
};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

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
    Order(KrakenOrder),
    Cancel(Cancel),
    Reject(Reject),
    Ack(Ack),
    Fill(KrakenFill),
    Out(Out),
    ExchangeOrderUpdate(
        KrakenUserRef,
        KrakenExchangeId,
        Option<KrakenExternalOrder>,
        bool, /* out */
    ),
    ExchangeAck(OrderId, KrakenExchangeId),
    ExchangeFill(KrakenExternalFill),
    ExchangeExternalOrderNew(KrakenExternalOrder),
    ExchangeExternalOrderOut(KrakenExchangeId),
    ExchangeExternalFill(KrakenExternalFill),
}

impl TryInto<OrderflowMessage> for &KrakenMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            KrakenMessage::Order(o) => Ok(OrderflowMessage::Order(**o)),
            KrakenMessage::Cancel(c) => Ok(OrderflowMessage::Cancel(*c)),
            KrakenMessage::Reject(r) => Ok(OrderflowMessage::Reject(*r)),
            KrakenMessage::Ack(a) => Ok(OrderflowMessage::Ack(*a)),
            KrakenMessage::Fill(f) => Ok(OrderflowMessage::Fill(**f)),
            KrakenMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            KrakenMessage::ExchangeOrderUpdate(..)
            | KrakenMessage::ExchangeAck(..)
            | KrakenMessage::ExchangeFill(..)
            | KrakenMessage::ExchangeExternalOrderNew(..)
            | KrakenMessage::ExchangeExternalFill(..)
            | KrakenMessage::ExchangeExternalOrderOut(..) => Err(()),
        }
    }
}

pub type KrakenExchangeId = String;
pub type KrakenUserRef = i32;

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct KrakenOrder {
    #[serde(flatten)]
    pub order: Order,
    #[allow(dead_code)]
    pub special_kraken_flag: (),
}

impl From<Order> for KrakenOrder {
    fn from(order: Order) -> Self {
        Self { order, special_kraken_flag: () }
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
