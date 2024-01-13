use crate::{symbology::MarketId, Dir, OrderId, Str};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use enumflags2::{bitflags, BitFlags};
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::JsonSchema_repr;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct Order {
    pub id: OrderId,
    pub market: MarketId,
    pub dir: Dir,
    pub quantity: Decimal,
    pub account: Option<Str>,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
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
    ) -> Self {
        Self(Order {
            id,
            market,
            dir,
            quantity,
            account: None,
            order_type: OrderType::Limit(LimitOrderType { limit_price, post_only }),
            time_in_force: TimeInForce::GoodTilCancel,
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
            time_in_force: time_in_force,
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
            time_in_force: time_in_force,
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
    ImmediateOrCancel,
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
            "IOC" => Ok(Self::ImmediateOrCancel),
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
            Self::ImmediateOrCancel => "IOC",
        }
    }

    fn good_til_date(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::GoodTilDate(d) => Some(*d),
            _ => None,
        }
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

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct Reject {
    pub order_id: OrderId,
}

impl Reject {
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
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
