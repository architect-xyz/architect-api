use crate::{
    folio::FolioMessage, orderflow::*, symbology::market::NormalizedMarketInfo, Dir, Str,
};
use chrono::{DateTime, Utc};
#[cfg(feature = "netidx")]
use derive::FromValue;
use log::error;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct CoinbasePrimeMarketInfo {
    pub base_increment: Decimal,
    pub quote_increment: Decimal,
    pub base_min_size: Decimal,
    pub quote_min_size: Decimal,
    pub base_max_size: Decimal,
    pub quote_max_size: Decimal,
}

impl NormalizedMarketInfo for CoinbasePrimeMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.quote_increment
    }

    fn step_size(&self) -> Decimal {
        self.base_increment
    }

    fn is_delisted(&self) -> bool {
        false
    }
}

impl std::fmt::Display for CoinbasePrimeMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub enum CoinbasePrimeMessage {
    Order(CoinbasePrimeOrder),
    Cancel(Cancel),
    Ack(Ack),
    Reject(Reject),
    Fill(CoinbasePrimeFill),
    Out(Out),
    ExchangeExecutionReport(ExchangeExecutionReport),
    Folio(FolioMessage),
}

impl TryInto<CoinbasePrimeMessage> for &OrderflowMessage {
    type Error = ();

    fn try_into(self) -> Result<CoinbasePrimeMessage, ()> {
        match self {
            OrderflowMessage::Order(o) => {
                Ok(CoinbasePrimeMessage::Order(CoinbasePrimeOrder { order: *o }))
            }
            OrderflowMessage::Ack(a) => Ok(CoinbasePrimeMessage::Ack(*a)),
            OrderflowMessage::Cancel(c) => Ok(CoinbasePrimeMessage::Cancel(*c)),
            OrderflowMessage::Reject(r) => Ok(CoinbasePrimeMessage::Reject(r.clone())),
            OrderflowMessage::Fill(f) => {
                Ok(CoinbasePrimeMessage::Fill(CoinbasePrimeFill { fill: *f }))
            }
            OrderflowMessage::Out(o) => Ok(CoinbasePrimeMessage::Out(*o)),
            OrderflowMessage::CancelAll(_) => {
                Err(error!("Cancel all not implemented for CoinbasePrime"))
            }
        }
    }
}

impl TryInto<OrderflowMessage> for &CoinbasePrimeMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            CoinbasePrimeMessage::Order(o) => Ok(OrderflowMessage::Order(**o)),
            CoinbasePrimeMessage::Ack(a) => Ok(OrderflowMessage::Ack(*a)),
            CoinbasePrimeMessage::Cancel(c) => Ok(OrderflowMessage::Cancel(*c)),
            CoinbasePrimeMessage::Reject(r) => Ok(OrderflowMessage::Reject(r.clone())),
            CoinbasePrimeMessage::Fill(f) => Ok(OrderflowMessage::Fill(**f)),
            CoinbasePrimeMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            CoinbasePrimeMessage::Folio(_)
            | CoinbasePrimeMessage::ExchangeExecutionReport(_) => Err(()),
        }
    }
}

impl TryInto<FolioMessage> for &CoinbasePrimeMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, ()> {
        match self {
            CoinbasePrimeMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<&FolioMessage> for CoinbasePrimeMessage {
    type Error = ();

    fn try_from(f: &FolioMessage) -> Result<Self, ()> {
        Ok(Self::Folio(f.clone()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub enum ExchangeExecType {
    NewOrder,
    PartialFill,
    Filled,
    Done,
    Canceled,
    PendingCancel,
    Stopped,
    Rejected,
    Restated,
    PendingNew,
    OrderStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub struct ExchangeExecutionReport {
    pub order_average_price: Decimal,
    pub client_order_id: Str,
    pub commission: Option<Decimal>,
    pub cum_qty: Decimal,
    pub execution_id: Str,
    pub last_price: Option<Decimal>,
    pub last_qty: Option<Decimal>,
    pub order_id: Str,
    pub order_qty: Decimal,
    pub order_status: Str,
    pub sender_sub_id: Str,
    pub side: Dir,
    pub symbol: Str,
    pub order_reject_reason: Option<ExchangeOrderRejectReason>,
    pub exec_type: ExchangeExecType,
    pub leaves_qty: Decimal,
    //pub cash_order_qty: Option<Decimal>,
    pub last_mkt: Option<Str>,
    pub transact_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub enum ExchangeOrderRejectReason {
    BrokerOption,
    UnknownSymbol,
    ExchangeClosed,
    OrderExceedsLimit,
    TooLateToEnter,
    UnknownOrder,
    DuplicateOrder,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub struct CoinbasePrimeOrder {
    #[serde(flatten)]
    pub order: Order,
}

impl From<Order> for CoinbasePrimeOrder {
    fn from(order: Order) -> Self {
        Self { order }
    }
}

impl Deref for CoinbasePrimeOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

impl DerefMut for CoinbasePrimeOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.order
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct CoinbasePrimeFill {
    #[serde(flatten)]
    pub fill: Result<Fill, AberrantFill>,
}

impl Deref for CoinbasePrimeFill {
    type Target = Result<Fill, AberrantFill>;

    fn deref(&self) -> &Self::Target {
        &self.fill
    }
}
