use crate::{symbology::MarketId, Dir, OrderId, Str};
use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use chrono::{DateTime, Utc};
use enumflags2::{bitflags, BitFlags};
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::JsonSchema_repr;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct Order {
    pub id: OrderId,
    pub market: MarketId,
    pub dir: Dir,
    pub quantity: Decimal,
    pub account: Option<Str>,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub quote_id: Option<Str>,
}

pub struct OrderBuilder(Order);

impl OrderBuilder {
    pub fn limit(
        id: OrderId,
        market: MarketId,
        dir: Dir,
        quantity: Decimal,
        limit_price: Decimal,
        post_only: bool,
        quote_id: Option<Str>,
    ) -> Self {
        Self(Order {
            id,
            market,
            dir,
            quantity,
            account: None,
            order_type: OrderType::Limit(LimitOrderType { limit_price, post_only }),
            time_in_force: TimeInForce::GoodTilCancel,
            quote_id,
        })
    }

    pub fn stop_loss_limit(
        id: OrderId,
        market: MarketId,
        dir: Dir,
        quantity: Decimal,
        limit_price: Decimal,
        trigger_price: Decimal,
        time_in_force: TimeInForce,
    ) -> Self {
        Self(Order {
            id,
            market,
            dir,
            quantity,
            account: None,
            order_type: OrderType::StopLossLimit(StopLossLimitOrderType {
                trigger_price,
                limit_price,
            }),
            time_in_force,
            quote_id: None,
        })
    }

    pub fn take_profit_limit(
        id: OrderId,
        market: MarketId,
        dir: Dir,
        quantity: Decimal,
        limit_price: Decimal,
        trigger_price: Decimal,
        time_in_force: TimeInForce,
    ) -> Self {
        Self(Order {
            id,
            market,
            dir,
            quantity,
            account: None,
            order_type: OrderType::TakeProfitLimit(TakeProfitLimitOrderType {
                trigger_price,
                limit_price,
            }),
            time_in_force,
            quote_id: None,
        })
    }

    pub fn account(self, account: Option<Str>) -> Self {
        Self(Order { account, ..self.0 })
    }

    pub fn time_in_force(self, time_in_force: TimeInForce) -> Self {
        Self(Order { time_in_force, ..self.0 })
    }

    pub fn build(self) -> Order {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLUnion))]
pub enum OrderType {
    Limit(LimitOrderType),
    StopLossLimit(StopLossLimitOrderType),
    TakeProfitLimit(TakeProfitLimitOrderType),
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct LimitOrderType {
    pub limit_price: Decimal,
    pub post_only: bool,
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct StopLossLimitOrderType {
    pub limit_price: Decimal,
    pub trigger_price: Decimal,
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct TakeProfitLimitOrderType {
    pub limit_price: Decimal,
    pub trigger_price: Decimal,
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
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
    fn instruction(&self) -> &'static str {
        match self {
            Self::GoodTilCancel => "GTC",
            Self::GoodTilDate(_) => "GTD",
            Self::GoodTilDay => "DAY",
            Self::ImmediateOrCancel => "IOC",
            Self::FillOrKill => "FOK",
        }
    }

    fn good_til_date(&self) -> Option<DateTime<Utc>> {
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
    Debug, Clone, Copy, Hash, PartialEq, Eq, Pack, Serialize, Deserialize, JsonSchema_repr,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
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

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct Cancel {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct Reject {
    pub order_id: OrderId,
    pub reason: RejectReason,
}

impl Reject {
    pub fn new(order_id: OrderId, reason: RejectReason) -> Self {
        Self { order_id, reason }
    }
}

#[cfg(feature = "juniper")]
#[cfg_attr(feature = "juniper", juniper::graphql_object)]
impl Reject {
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
#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub enum RejectReason {
    // custom message...can be slow b/c sending the whole string
    Literal(ArcStr),
    ComponentNotInitialized,
    UnknownCpty,
    UnknownMarket,
    DuplicateOrderId,
    #[pack(other)]
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
            Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct Ack {
    pub order_id: OrderId,
}

impl Ack {
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct Out {
    pub order_id: OrderId,
}

impl Out {
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
    }
}
