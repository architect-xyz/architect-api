use crate::{
    symbology::{MarketId, VenueId},
    AccountId, Dir, OrderId, Str, UserId,
};
use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use enumflags2::{bitflags, BitFlags};
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::JsonSchema_repr;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Builder, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct Order {
    pub id: OrderId,
    pub market: MarketId,
    pub dir: Dir,
    pub quantity: Decimal,
    #[builder(setter(strip_option), default)]
    pub trader: Option<UserId>,
    #[builder(setter(strip_option), default)]
    pub account: Option<AccountId>,
    pub order_type: OrderType,
    #[builder(default = "TimeInForce::GoodTilCancel")]
    pub time_in_force: TimeInForce,
    #[builder(setter(strip_option), default)]
    pub quote_id: Option<Str>,
    pub source: OrderSource,
    #[builder(setter(strip_option), default)]
    pub parent_order: Option<ParentOrder>,
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[repr(u8)]
pub enum OrderSource {
    API,
    GUI,
    Algo,
    External,
    CLI,
    Telegram,
    #[serde(other)]
    #[cfg_attr(feature = "netidx", pack(other))]
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[repr(u8)]
pub enum ParentOrderKind {
    Algo,
    Order,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct ParentOrder {
    pub kind: ParentOrderKind,
    pub id: OrderId,
}

impl ParentOrder {
    pub fn new(kind: ParentOrderKind, id: OrderId) -> Self {
        Self { kind, id }
    }
}

#[cfg(feature = "netidx")]
impl OrderBuilder {
    pub fn new(id: OrderId, source: OrderSource, market: MarketId) -> Self {
        let mut t = Self::default();
        t.id(id);
        t.source(source);
        t.market(market);
        t
    }

    /// Option version of trader(&mut self, ..)
    pub fn with_trader(&mut self, trader: Option<UserId>) -> &mut Self {
        self.trader = Some(trader);
        self
    }

    /// Option version of account(&mut self, ..)
    pub fn with_account(&mut self, account: Option<AccountId>) -> &mut Self {
        self.account = Some(account);
        self
    }

    pub fn limit(
        &mut self,
        dir: Dir,
        quantity: Decimal,
        limit_price: Decimal,
        post_only: bool,
    ) -> &mut Self {
        self.dir(dir);
        self.quantity(quantity);
        self.order_type(OrderType::Limit(LimitOrderType { limit_price, post_only }));
        self
    }

    pub fn stop_loss_limit(
        &mut self,
        dir: Dir,
        quantity: Decimal,
        limit_price: Decimal,
        trigger_price: Decimal,
    ) -> &mut Self {
        self.dir(dir);
        self.quantity(quantity);
        self.order_type(OrderType::StopLossLimit(StopLossLimitOrderType {
            limit_price,
            trigger_price,
        }));
        self
    }

    pub fn take_profit_limit(
        &mut self,
        dir: Dir,
        quantity: Decimal,
        limit_price: Decimal,
        trigger_price: Decimal,
    ) -> &mut Self {
        self.dir(dir);
        self.quantity(quantity);
        self.order_type(OrderType::TakeProfitLimit(TakeProfitLimitOrderType {
            limit_price,
            trigger_price,
        }));
        self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLUnion))]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[serde(tag = "type")]
pub enum OrderType {
    Limit(LimitOrderType),
    StopLossLimit(StopLossLimitOrderType),
    TakeProfitLimit(TakeProfitLimitOrderType),
}

impl OrderType {
    pub fn limit_price(&self) -> Decimal {
        match self {
            OrderType::Limit(limit_order_type) => limit_order_type.limit_price,
            OrderType::StopLossLimit(stop_loss_limit_order_type) => {
                stop_loss_limit_order_type.limit_price
            }
            OrderType::TakeProfitLimit(take_profit_limit_order_type) => {
                take_profit_limit_order_type.limit_price
            }
        }
    }

    pub fn post_only(&self) -> Option<bool> {
        match self {
            OrderType::Limit(limit_order_type) => Some(limit_order_type.post_only),
            OrderType::StopLossLimit(_) | OrderType::TakeProfitLimit(_) => None,
        }
    }

    pub fn trigger_price(&self) -> Option<Decimal> {
        match self {
            OrderType::StopLossLimit(sllot) => Some(sllot.trigger_price),
            OrderType::TakeProfitLimit(tplot) => Some(tplot.trigger_price),
            OrderType::Limit(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct LimitOrderType {
    pub limit_price: Decimal,
    pub post_only: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct StopLossLimitOrderType {
    pub limit_price: Decimal,
    pub trigger_price: Decimal,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct TakeProfitLimitOrderType {
    pub limit_price: Decimal,
    pub trigger_price: Decimal,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[serde(tag = "type", content = "value")]
pub enum TimeInForce {
    GoodTilCancel,
    GoodTilDate(DateTime<Utc>),
    /// Day order--the specific time which this expires will be dependent on the venue
    GoodTilDay,
    ImmediateOrCancel,
    FillOrKill,
}

impl TimeInForce {
    pub fn from_instruction(
        instruction: &str,
        good_til_date: Option<DateTime<Utc>>,
    ) -> Result<Self> {
        match instruction {
            "GTC" => Ok(Self::GoodTilCancel),
            "GTD" => Ok(Self::GoodTilDate(
                good_til_date.ok_or_else(|| anyhow!("GTD requires good_til_date"))?,
            )),
            "DAY" => Ok(Self::GoodTilDay),
            "IOC" => Ok(Self::ImmediateOrCancel),
            "FOK" => Ok(Self::FillOrKill),
            _ => Err(anyhow!("unknown time-in-force instruction: {}", instruction)),
        }
    }
}

#[cfg(feature = "juniper")]
#[cfg_attr(feature = "juniper", juniper::graphql_object)]
impl TimeInForce {
    pub fn instruction(&self) -> &'static str {
        match self {
            Self::GoodTilCancel => "GTC",
            Self::GoodTilDate(_) => "GTD",
            Self::GoodTilDay => "DAY",
            Self::ImmediateOrCancel => "IOC",
            Self::FillOrKill => "FOK",
        }
    }

    pub fn good_til_date(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::GoodTilDate(d) => Some(*d),
            _ => None,
        }
    }
}

#[cfg(feature = "clap")]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct TimeInForceArgs {
    /// GTC, GTD, IOC, DAY, FOK
    #[arg(long, default_value = "GTC")]
    time_in_force: String,
    /// If TIF instruction is GTD, the datetime or relative duration from now;
    /// e.g. +1d or 2021-01-01T00:00:00Z
    #[arg(long)]
    good_til_date: Option<String>,
}

#[cfg(feature = "clap")]
impl TryInto<TimeInForce> for TimeInForceArgs {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<TimeInForce> {
        let good_til_date = self
            .good_til_date
            .map(|s| {
                if s.starts_with('+') {
                    let dur_s = &s[1..];
                    let dur = crate::utils::duration::parse_duration(&dur_s)?;
                    let now = Utc::now();
                    Ok::<_, anyhow::Error>(now + dur)
                } else {
                    let dt = DateTime::parse_from_rfc3339(&s)?;
                    Ok::<_, anyhow::Error>(dt.with_timezone(&Utc))
                }
            })
            .transpose()?;
        TimeInForce::from_instruction(&self.time_in_force, good_til_date)
    }
}

/// The state of an order
#[bitflags]
#[repr(u8)]
#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, JsonSchema_repr,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub enum OrderStateFlags {
    Open,
    Rejected,
    Acked,
    Filled,
    Canceling,
    Canceled,
    Out,
    Stale, // we were expecting some state change but it was never confirmed
}

pub type OrderState = BitFlags<OrderStateFlags>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct Cancel {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct CancelAll {
    pub venue_id: Option<VenueId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct Reject {
    pub order_id: OrderId,
    pub reason: RejectReason,
}

impl Reject {
    pub fn new(order_id: OrderId, reason: RejectReason) -> Self {
        Self { order_id, reason }
    }

    pub fn order_id(&self) -> OrderId {
        self.order_id
    }

    pub fn reason(&self) -> String {
        self.reason.to_string()
    }
}

/// Reject reason, includes common reasons as unit enum variants,
/// but leaves room for custom reasons if needed; although, performance
/// sensitive components should still supertype their own rejects.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub enum RejectReason {
    // custom message...can be slow b/c sending the whole string
    Literal(ArcStr),
    ComponentNotInitialized,
    UnknownCpty,
    UnknownMarket,
    DuplicateOrderId,
    InvalidQuantity,
    MissingRequiredAccount,
    NoAccount,
    NotAuthorized,
    NotAuthorizedForAccount,
    #[cfg_attr(feature = "netidx", pack(other))]
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for RejectReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RejectReason::*;
        match self {
            Literal(s) => write!(f, "{}", s),
            ComponentNotInitialized => write!(f, "component not initialized"),
            UnknownCpty => write!(f, "unknown cpty"),
            UnknownMarket => write!(f, "unknown market"),
            DuplicateOrderId => write!(f, "duplicate order id"),
            InvalidQuantity => write!(f, "invalid quantity"),
            MissingRequiredAccount => write!(f, "missing required account"),
            NoAccount => write!(f, "no account"),
            NotAuthorized => write!(f, "not authorized to perform action"),
            NotAuthorizedForAccount => write!(f, "not authorized for account"),
            Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct Ack {
    pub order_id: OrderId,
}

impl Ack {
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct Out {
    pub order_id: OrderId,
}

impl Out {
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
    }
}
